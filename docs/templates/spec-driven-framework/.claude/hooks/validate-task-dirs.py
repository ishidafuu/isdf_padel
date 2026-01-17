#!/usr/bin/env python3
"""
タスクディレクトリのバリデーション

許可されたディレクトリ以外が存在する場合にエラーを出力
"""

import os
import sys

# プロジェクトルート
PROJECT_DIR = os.environ.get("CLAUDE_PROJECT_DIR", os.getcwd())

# 許可されたディレクトリ名
ALLOWED_DIRS = {
    "0_backlog",
    "1_todo",
    "2_in-progress",
    "3_in-review",
    "4_archive",
}

# チェック対象のタスクディレクトリ
TASK_ROOTS = [
    os.path.join(PROJECT_DIR, "tasks"),
    os.path.join(PROJECT_DIR, "project", "tasks"),
]

def validate_task_dirs():
    """タスクディレクトリをバリデーション"""
    errors = []

    for task_root in TASK_ROOTS:
        if not os.path.exists(task_root):
            continue

        for item in os.listdir(task_root):
            item_path = os.path.join(task_root, item)

            # ディレクトリのみチェック
            if not os.path.isdir(item_path):
                continue

            # 隠しディレクトリは除外
            if item.startswith("."):
                continue

            # 許可リストにないディレクトリはエラー
            if item not in ALLOWED_DIRS:
                rel_path = os.path.relpath(item_path, PROJECT_DIR)
                errors.append(f"  - {rel_path}/ (許可: {', '.join(sorted(ALLOWED_DIRS))})")

    return errors

def main():
    errors = validate_task_dirs()

    if errors:
        print("=" * 60, file=sys.stderr)
        print("ERROR: 不正なタスクディレクトリを検出", file=sys.stderr)
        print("=" * 60, file=sys.stderr)
        print("", file=sys.stderr)
        print("以下のディレクトリは許可されていません:", file=sys.stderr)
        for error in errors:
            print(error, file=sys.stderr)
        print("", file=sys.stderr)
        print("許可されたディレクトリ:", file=sys.stderr)
        for d in sorted(ALLOWED_DIRS):
            print(f"  - {d}/", file=sys.stderr)
        print("", file=sys.stderr)
        print("対処方法:", file=sys.stderr)
        print("  1. 間違ったディレクトリ内のファイルを正しいディレクトリに移動", file=sys.stderr)
        print("  2. 間違ったディレクトリを削除", file=sys.stderr)
        print("=" * 60, file=sys.stderr)
        sys.exit(1)

    # 正常終了（出力なし）
    sys.exit(0)

if __name__ == "__main__":
    main()
