#!/usr/bin/env python3
"""
タスクファイルのfrontmatterバリデーション

タスクファイルに必要なfrontmatterが設定されているかを検証
"""

import os
import re
import sys
from pathlib import Path

# プロジェクトルート
PROJECT_DIR = os.environ.get("CLAUDE_PROJECT_DIR", os.getcwd())

# チェック対象のタスクディレクトリ
TASK_ROOTS = [
    os.path.join(PROJECT_DIR, "tasks"),
    os.path.join(PROJECT_DIR, "project", "tasks"),
]

# チェック対象のサブディレクトリ
# - 0_backlog: バグ専用で別形式なので除外
# - 4_archive: レガシー形式が混在するため除外
TARGET_SUBDIRS = ["1_todo", "2_in-progress", "3_in-review"]

# 必須フィールド
REQUIRED_FIELDS = ["id", "title", "status", "priority"]

# 有効な値
VALID_STATUSES = ["todo", "in-progress", "in-review", "done", "cancelled"]
VALID_PRIORITIES = ["high", "medium", "low"]
VALID_TYPES = ["feature", "bugfix", "refactor", "infrastructure"]

# ディレクトリと期待されるstatus
DIR_STATUS_MAP = {
    "1_todo": "todo",
    "2_in-progress": "in-progress",
    "3_in-review": "in-review",
    "4_archive": ["done", "cancelled"],
}


def parse_frontmatter(file_path: Path) -> dict | None:
    """YAML Frontmatter を抽出してパース"""
    try:
        content = file_path.read_text(encoding="utf-8")
    except Exception:
        return None

    # Frontmatter を抽出（--- で囲まれた部分）
    match = re.match(r"^---\n(.*?)\n---", content, re.DOTALL)
    if not match:
        return None

    frontmatter = match.group(1)

    # 簡易 YAML パース
    def parse_value(val: str) -> str | list[str]:
        val = val.strip()
        # リスト形式 ["a", "b"]
        if val.startswith("[") and val.endswith("]"):
            inner = val[1:-1]
            if not inner.strip():
                return []
            items = re.findall(r'"([^"]*)"', inner)
            return items
        # 文字列
        if val.startswith('"') and val.endswith('"'):
            return val[1:-1]
        # null
        if val == "null":
            return None
        return val

    data: dict = {}
    for line in frontmatter.split("\n"):
        if ":" in line:
            key, val = line.split(":", 1)
            data[key.strip()] = parse_value(val)

    return data


def validate_task_file(file_path: Path, subdir: str) -> list[str]:
    """単一タスクファイルをバリデーション"""
    errors = []
    rel_path = os.path.relpath(file_path, PROJECT_DIR)

    # frontmatterをパース
    data = parse_frontmatter(file_path)

    if data is None:
        errors.append(f"{rel_path}: frontmatterがありません（---で囲まれた部分が必要）")
        return errors

    # 必須フィールドチェック
    for field in REQUIRED_FIELDS:
        if field not in data or data[field] is None or data[field] == "":
            errors.append(f"{rel_path}: 必須フィールド '{field}' がありません")

    # statusの値チェック
    if "status" in data and data["status"]:
        status = data["status"]
        if status not in VALID_STATUSES:
            errors.append(
                f"{rel_path}: status '{status}' は無効です（有効: {', '.join(VALID_STATUSES)}）"
            )
        else:
            # ディレクトリとstatusの整合性チェック
            expected = DIR_STATUS_MAP.get(subdir)
            if expected:
                if isinstance(expected, list):
                    if status not in expected:
                        errors.append(
                            f"{rel_path}: {subdir}/内のstatusは {expected} のいずれかである必要があります（現在: {status}）"
                        )
                elif status != expected:
                    errors.append(
                        f"{rel_path}: {subdir}/内のstatusは '{expected}' である必要があります（現在: {status}）"
                    )

    # priorityの値チェック
    if "priority" in data and data["priority"]:
        priority = data["priority"]
        if priority not in VALID_PRIORITIES:
            errors.append(
                f"{rel_path}: priority '{priority}' は無効です（有効: {', '.join(VALID_PRIORITIES)}）"
            )

    # typeの値チェック（存在する場合のみ）
    if "type" in data and data["type"]:
        task_type = data["type"]
        if task_type not in VALID_TYPES:
            errors.append(
                f"{rel_path}: type '{task_type}' は無効です（有効: {', '.join(VALID_TYPES)}）"
            )

    return errors


def validate_all_tasks() -> list[str]:
    """全タスクファイルをバリデーション"""
    all_errors = []

    for task_root in TASK_ROOTS:
        if not os.path.exists(task_root):
            continue

        for subdir in TARGET_SUBDIRS:
            subdir_path = os.path.join(task_root, subdir)
            if not os.path.exists(subdir_path):
                continue

            for file_name in os.listdir(subdir_path):
                # .mdファイルのみ対象
                if not file_name.endswith(".md"):
                    continue

                # テンプレートファイルは除外
                if file_name.startswith("_"):
                    continue

                file_path = Path(subdir_path) / file_name
                errors = validate_task_file(file_path, subdir)
                all_errors.extend(errors)

    return all_errors


def main():
    errors = validate_all_tasks()

    if errors:
        print("=" * 60, file=sys.stderr)
        print("ERROR: タスクファイルのfrontmatterに問題があります", file=sys.stderr)
        print("=" * 60, file=sys.stderr)
        print("", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        print("", file=sys.stderr)
        print("必要なfrontmatter形式:", file=sys.stderr)
        print("  ---", file=sys.stderr)
        print('  id: "TASK-ID"', file=sys.stderr)
        print('  title: "タスクタイトル"', file=sys.stderr)
        print('  status: "todo"  # todo/in-progress/in-review/done/cancelled', file=sys.stderr)
        print('  priority: "medium"  # high/medium/low', file=sys.stderr)
        print("  ---", file=sys.stderr)
        print("=" * 60, file=sys.stderr)
        sys.exit(1)

    # 正常終了（出力なし）
    sys.exit(0)


if __name__ == "__main__":
    main()
