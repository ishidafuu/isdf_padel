#!/usr/bin/env python3
"""
task-next: 次に着手可能なタスクを提案するスクリプト

Usage:
    python3 scripts/task-next.py [--limit N]
"""

import argparse
import glob
import re
import sys
from pathlib import Path
from typing import TypedDict


class TaskInfo(TypedDict):
    id: str
    title: str
    priority: str
    status: str
    blocked_by: list[str]
    blocks: list[str]


class BugInfo(TypedDict):
    id: str
    title: str
    severity: str  # critical / major / minor
    discovered: str
    related_feature: str
    status: str


def parse_frontmatter(file_path: Path) -> TaskInfo | None:
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

    # 簡易 YAML パース（PyYAML 不要）
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
        return val

    data: dict[str, str | list[str]] = {}
    for line in frontmatter.split("\n"):
        if ":" in line:
            key, val = line.split(":", 1)
            data[key.strip()] = parse_value(val)

    # 必須フィールドチェック
    if "id" not in data or "status" not in data:
        return None

    return TaskInfo(
        id=str(data.get("id", "")),
        title=str(data.get("title", "")),
        priority=str(data.get("priority", "medium")),
        status=str(data.get("status", "")),
        blocked_by=data.get("blocked_by", []) if isinstance(data.get("blocked_by"), list) else [],
        blocks=data.get("blocks", []) if isinstance(data.get("blocks"), list) else [],
    )


def parse_bug_frontmatter(file_path: Path) -> BugInfo | None:
    """バグファイルの YAML Frontmatter を抽出してパース"""
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
    def parse_value(val: str) -> str:
        val = val.strip()
        if val.startswith('"') and val.endswith('"'):
            return val[1:-1]
        return val

    data: dict[str, str] = {}
    for line in frontmatter.split("\n"):
        if ":" in line:
            key, val = line.split(":", 1)
            data[key.strip()] = parse_value(val)

    # 必須フィールドチェック
    if "status" not in data:
        return None

    # ID をファイル名から抽出（BUG-001-xxx.md → BUG-001）
    filename = file_path.stem
    id_match = re.match(r"^(BUG-\d+)", filename)
    bug_id = id_match.group(1) if id_match else filename

    return BugInfo(
        id=bug_id,
        title=str(data.get("title", "")),
        severity=str(data.get("severity", "minor")),
        discovered=str(data.get("discovered", "")),
        related_feature=str(data.get("related_feature", "")),
        status=str(data.get("status", "")),
    )


def get_priority_order(priority: str) -> int:
    """優先度のソート順を返す（小さいほど高優先）"""
    return {"high": 0, "medium": 1, "low": 2}.get(priority, 1)


def get_priority_icon(priority: str) -> str:
    """優先度アイコンを返す"""
    return {"high": "🔴", "medium": "🟡", "low": "🟢"}.get(priority, "🟡")


def get_severity_icon(severity: str) -> str:
    """深刻度アイコンを返す"""
    return {"critical": "🔴", "major": "🟠", "minor": "🟡"}.get(severity, "🟡")


def main() -> None:
    parser = argparse.ArgumentParser(description="次に着手可能なタスクを提案")
    parser.add_argument("--limit", type=int, default=0, help="表示件数を制限（0=全件）")
    args = parser.parse_args()

    # プロジェクトルートを特定
    script_dir = Path(__file__).resolve().parent
    project_root = script_dir.parent
    tasks_dir = project_root / "project" / "tasks"

    # タスクファイルを収集
    todo_files = list(tasks_dir.glob("1_todo/*.md"))
    archive_files = list(tasks_dir.glob("4_archive/*.md"))
    in_progress_files = list(tasks_dir.glob("2_in-progress/*.md"))
    in_review_files = list(tasks_dir.glob("3_in-review/*.md"))

    # バックログファイルを収集（テンプレートを除外）
    backlog_files = [
        f for f in tasks_dir.glob("0_backlog/*.md") if not f.name.startswith("_")
    ]

    # reviewed バグを抽出
    reviewed_bugs: list[BugInfo] = []
    for f in backlog_files:
        bug = parse_bug_frontmatter(f)
        if bug and bug["status"] == "reviewed":
            reviewed_bugs.append(bug)

    # 完了済みタスクID（status: "done" のみ）
    done_ids: set[str] = set()
    for f in archive_files:
        task = parse_frontmatter(f)
        if task and task["status"] == "done":
            done_ids.add(task["id"])

    # 進行中タスク（2_in-progress）
    in_progress_tasks: list[TaskInfo] = []
    for f in in_progress_files:
        task = parse_frontmatter(f)
        if task:
            in_progress_tasks.append(task)

    # レビュー待ちタスク（3_in-review）
    in_review_tasks: list[TaskInfo] = []
    for f in in_review_files:
        task = parse_frontmatter(f)
        if task:
            in_review_tasks.append(task)

    # 並列判定用に両方を合わせる
    active_tasks = in_progress_tasks + in_review_tasks
    in_progress_ids = {t["id"] for t in active_tasks}

    # Todo タスクを解析
    todo_tasks: list[TaskInfo] = []
    for f in todo_files:
        task = parse_frontmatter(f)
        if task:
            todo_tasks.append(task)

    # READY 判定
    ready_tasks: list[TaskInfo] = []
    blocked_tasks: list[TaskInfo] = []

    for task in todo_tasks:
        blocked_by = task["blocked_by"]
        # blocked_by の全IDが完了済み(done)なら READY
        if all(dep in done_ids for dep in blocked_by):
            ready_tasks.append(task)
        else:
            blocked_tasks.append(task)

    # 並列可能判定（進行中タスクと相互依存がないか）
    def is_parallel_ok(task: TaskInfo) -> tuple[bool, str]:
        task_id = task["id"]
        for ip_task in active_tasks:
            # 進行中タスクが自分を blocks していたら不可
            if task_id in ip_task["blocks"]:
                return False, f"{ip_task['id']} と相互依存"
            # 自分が進行中タスクを blocks していたら不可
            if ip_task["id"] in task["blocks"]:
                return False, f"{ip_task['id']} と相互依存"
        return True, ""

    # ソート: priority > blocks.length > id
    ready_tasks.sort(
        key=lambda t: (
            get_priority_order(t["priority"]),
            -len(t["blocks"]),  # 多い順
            t["id"],
        )
    )

    # 出力

    # レビュー待ちセクション
    if in_review_tasks:
        print(f"🔍 レビュー待ち ({len(in_review_tasks)}件):")
        print()
        for task in in_review_tasks:
            icon = get_priority_icon(task["priority"])
            print(f"{icon} 🔍 [{task['id']}] {task['title']}")
        print()

    # 進行中セクション
    if in_progress_tasks:
        print(f"🔄 進行中 ({len(in_progress_tasks)}件):")
        print()
        for task in in_progress_tasks:
            icon = get_priority_icon(task["priority"])
            print(f"{icon} 🔄 [{task['id']}] {task['title']}")
        print()

    # セパレータ（レビュー待ちまたは進行中があった場合）
    if in_review_tasks or in_progress_tasks:
        print("---")
        print()

    if not ready_tasks:
        print("着手可能なタスクはありません。")
        print()
        if blocked_tasks:
            print(f"待機中: {len(blocked_tasks)}件（依存関係で blocked）")
        sys.exit(0)

    # 表示件数制限
    display_tasks = ready_tasks
    if args.limit > 0:
        display_tasks = ready_tasks[: args.limit]

    # バグセクション出力
    if reviewed_bugs:
        print("🐛 精査済みバグ（タスク化待ち）:")
        print()
        for bug in reviewed_bugs:
            icon = get_severity_icon(bug["severity"])
            print(f"{icon} [{bug['id']}] {bug['title']}")
            print(
                f"   └─ 深刻度: {bug['severity']} | 関連: {bug['related_feature']} | 発見: {bug['discovered']}"
            )
            print()
        print("---")
        print()

    print(f"次に着手可能なタスク ({len(ready_tasks)}件):")
    print()

    for task in display_tasks:
        icon = get_priority_icon(task["priority"])
        print(f"{icon} ⬜ [{task['id']}] {task['title']}")

        # Blocks 情報
        blocks = task["blocks"]
        if blocks:
            blocks_str = ", ".join(blocks)
            print(f"   └─ Blocks: {blocks_str} ({len(blocks)}件解除)")
        else:
            print("   └─ Blocks: なし")

        # 並列可能判定
        parallel_ok, reason = is_parallel_ok(task)
        if parallel_ok:
            print("   └─ 並列: ✅ 可能")
        else:
            print(f"   └─ 並列: ⚠️ 不可（{reason}）")
        print()

    # 推奨タスク（blocks が最も多いタスクを推奨）
    tasks_with_blocks = [t for t in ready_tasks if t["blocks"]]
    if tasks_with_blocks:
        best = max(tasks_with_blocks, key=lambda t: len(t["blocks"]))
        print("---")
        print(
            f"推奨: {best['id']}（{best['title']}）を先に実装すると"
            f"{len(best['blocks'])}タスクが着手可能になります"
        )


if __name__ == "__main__":
    main()
