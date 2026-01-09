#!/usr/bin/env python3
"""
id-next: 次の連番IDを取得するスクリプト

Usage:
    python3 scripts/id-next.py <種別>

Examples:
    python3 scripts/id-next.py REQ-30101    # 仕様書ID採番
    python3 scripts/id-next.py FXXX         # フレームワークタスクID採番
    python3 scripts/id-next.py 30XXX        # ゲーム開発タスクID採番
    python3 scripts/id-next.py PXXX         # プロジェクト横断タスクID採番
    python3 scripts/id-next.py B30101       # バグ修正ID採番
    python3 scripts/id-next.py R30201       # リファクタID採番
"""

import re
import sys
from pathlib import Path


def find_project_root() -> Path:
    """スクリプトからプロジェクトルートを特定"""
    return Path(__file__).resolve().parent.parent


def get_spec_file_suffix(prefix: str) -> str:
    """ID種別から対応する仕様書ファイル拡張子を返す"""
    suffix_map = {
        "REQ": "_spec.md",
        "DES": "_design.md",
        "BHV": "_behavior.md",
        "TST": "_test.md",
    }
    return suffix_map.get(prefix, "_spec.md")


def find_spec_file(project_root: Path, prefix: str, file_number: str) -> Path | None:
    """仕様書ファイルを検索"""
    suffix = get_spec_file_suffix(prefix)
    docs_dir = project_root / "project" / "docs"

    # file_number にマッチするファイルを検索
    pattern = f"**/{file_number}*{suffix}"
    matches = list(docs_dir.glob(pattern))

    if matches:
        return matches[0]
    return None


def extract_spec_ids(file_path: Path, prefix: str, file_number: str) -> list[str]:
    """仕様書ファイルからIDを抽出"""
    if not file_path.exists():
        return []

    content = file_path.read_text(encoding="utf-8")

    # パターン: REQ-30101-001, DES-30102-002 など
    pattern = rf"{prefix}-{file_number}-(\d{{3}})"
    matches = re.findall(pattern, content)

    return sorted(set(matches))


def get_next_spec_id(project_root: Path, arg: str) -> tuple[str, list[str]]:
    """仕様書IDの次の番号を計算"""
    # 引数パース: REQ-30101
    match = re.match(r"^([A-Z]+)-(\d+)$", arg)
    if not match:
        return "", []

    prefix = match.group(1)
    file_number = match.group(2)

    spec_file = find_spec_file(project_root, prefix, file_number)
    if not spec_file:
        # ファイルが見つからない場合は 001 から開始
        next_id = f"{prefix}-{file_number}-001"
        return next_id, []

    existing = extract_spec_ids(spec_file, prefix, file_number)

    if not existing:
        next_seq = 1
    else:
        # 最大番号 + 1
        max_seq = max(int(seq) for seq in existing)
        next_seq = max_seq + 1

    next_id = f"{prefix}-{file_number}-{next_seq:03d}"
    existing_ids = [f"{prefix}-{file_number}-{seq}" for seq in existing]

    return next_id, existing_ids


def find_task_files(project_root: Path, task_type: str) -> list[Path]:
    """タスクファイルを検索"""
    if task_type == "framework":
        # FXXX: tasks/ 配下
        tasks_dir = project_root / "tasks"
    else:
        # 30XXX, PXXX, BXXX, RXXX: project/tasks/ 配下
        tasks_dir = project_root / "project" / "tasks"

    if not tasks_dir.exists():
        return []

    return list(tasks_dir.glob("**/*.md"))


def extract_task_ids(files: list[Path], pattern: str) -> list[str]:
    """タスクファイル名からIDを抽出"""
    ids = []
    regex = re.compile(pattern)

    for f in files:
        match = regex.match(f.name)
        if match:
            ids.append(match.group(1))

    return sorted(set(ids))


def get_next_task_id(project_root: Path, arg: str) -> tuple[str, list[str]]:
    """タスクIDの次の番号を計算"""
    arg_upper = arg.upper()

    if arg_upper == "FXXX":
        # フレームワークタスク
        files = find_task_files(project_root, "framework")
        existing = extract_task_ids(files, r"^(F\d+)-.+\.md$")

        if not existing:
            next_seq = 1
        else:
            max_seq = max(int(id[1:]) for id in existing)
            next_seq = max_seq + 1

        next_id = f"F{next_seq:03d}"
        return next_id, existing

    elif arg_upper == "30XXX":
        # ゲーム開発タスク
        files = find_task_files(project_root, "game")
        existing = extract_task_ids(files, r"^(30\d+)-.+\.md$")

        if not existing:
            next_seq = 30001
        else:
            max_seq = max(int(id) for id in existing)
            next_seq = max_seq + 1

        next_id = str(next_seq)
        return next_id, existing

    elif arg_upper == "PXXX":
        # プロジェクト横断タスク
        files = find_task_files(project_root, "project")
        # P003 形式だが、サブタスク P003-001 は除外
        existing = extract_task_ids(files, r"^(P\d+)-[^0-9].+\.md$")

        if not existing:
            next_seq = 1
        else:
            max_seq = max(int(id[1:]) for id in existing)
            next_seq = max_seq + 1

        next_id = f"P{next_seq:03d}"
        return next_id, existing

    return "", []


def get_next_bugfix_id(project_root: Path, arg: str) -> tuple[str, list[str]]:
    """バグ修正/リファクタIDの次の番号を計算"""
    # B30101 or R30201 形式
    match = re.match(r"^([BR])(\d+)$", arg.upper())
    if not match:
        return "", []

    prefix = match.group(1)
    base_task = match.group(2)

    files = find_task_files(project_root, "game")

    # B30101-001 形式のIDを抽出
    pattern = rf"^{prefix}{base_task}-(\d{{3}})-.+\.md$"
    existing_seqs = []

    for f in files:
        m = re.match(pattern, f.name)
        if m:
            existing_seqs.append(m.group(1))

    existing_seqs = sorted(set(existing_seqs))

    if not existing_seqs:
        next_seq = 1
    else:
        max_seq = max(int(seq) for seq in existing_seqs)
        next_seq = max_seq + 1

    next_id = f"{prefix}{base_task}-{next_seq:03d}"
    existing_ids = [f"{prefix}{base_task}-{seq}" for seq in existing_seqs]

    return next_id, existing_ids


def detect_id_type(arg: str) -> str:
    """引数からID種別を判定"""
    arg_upper = arg.upper()

    # 仕様書ID: REQ-30101, DES-30102, BHV-30103, TST-30104
    if re.match(r"^(REQ|DES|BHV|TST)-\d+$", arg_upper):
        return "spec"

    # タスクID: FXXX, 30XXX, PXXX
    if arg_upper in ("FXXX", "30XXX", "PXXX"):
        return "task"

    # バグ修正/リファクタID: B30101, R30201
    if re.match(r"^[BR]\d+$", arg_upper):
        return "bugfix"

    return "unknown"


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: python3 scripts/id-next.py <種別>")
        print()
        print("Examples:")
        print("  python3 scripts/id-next.py REQ-30101  # 仕様書ID採番")
        print("  python3 scripts/id-next.py FXXX       # フレームワークタスクID採番")
        print("  python3 scripts/id-next.py 30XXX      # ゲーム開発タスクID採番")
        print("  python3 scripts/id-next.py PXXX       # プロジェクト横断タスクID採番")
        print("  python3 scripts/id-next.py B30101     # バグ修正ID採番")
        sys.exit(1)

    arg = sys.argv[1]
    project_root = find_project_root()

    id_type = detect_id_type(arg)

    if id_type == "spec":
        next_id, existing = get_next_spec_id(project_root, arg)
    elif id_type == "task":
        next_id, existing = get_next_task_id(project_root, arg)
    elif id_type == "bugfix":
        next_id, existing = get_next_bugfix_id(project_root, arg)
    else:
        print(f"Error: 不明な種別 '{arg}'")
        print()
        print("対応形式:")
        print("  REQ-30101, DES-30102, BHV-30103, TST-30104  (仕様書ID)")
        print("  FXXX, 30XXX, PXXX                           (タスクID)")
        print("  B30101, R30201                              (バグ修正/リファクタID)")
        sys.exit(1)

    if not next_id:
        print(f"Error: ID計算に失敗しました: {arg}")
        sys.exit(1)

    # 出力
    print(f"次のID: {next_id}")
    print()

    if existing:
        print("既存ID:")
        for eid in existing:
            print(f"- {eid}")
    else:
        print("既存ID: なし")


if __name__ == "__main__":
    main()
