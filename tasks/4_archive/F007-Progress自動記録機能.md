---
id: "F007"
title: "タスク Progress/Next Actions 自動記録機能"
type: "framework"
status: "done"
priority: "high"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/s13219/.claude/plans/breezy-orbiting-biscuit.md"
tags: ["task-management", "cli", "session-resume"]
created_at: "2025-12-31T15:00:00+09:00"
updated_at: "2025-12-31T15:50:00+09:00"
completed_at: "2025-12-31T15:50:00+09:00"
---

# Task F007: タスク Progress/Next Actions 自動記録機能

## 説明

セッションクリア時にタスクの進行状況を保持し、再開時に参照できるようにする。
既存の Python CLI 基盤を活用し、コミット単位での自動記録と再開情報表示を実装。

## 背景

### 現状
1. 仕組みは存在するが未使用: `update_progress()`, `update_next_actions()` は実装済みだが、実際のワークフローで使われていない
2. 手動入力が必要: 現在はコミット情報を手動で指定する必要がある
3. CLIパスが古い: impl-agent.md のパスが `scripts/task.py` と記載（実際は `tools/task_manager/cli.py`）
4. 再開情報の表示がない: タスク詳細表示に Progress/Next Actions が含まれていない

### 改修理由
- セッションクリア後に作業再開する際、前回の進捗が分からない
- コミット情報を自動取得することで、手動入力の手間を削減
- 再開情報を表示することで、スムーズな作業再開を実現

## 実装内容

### Phase 1: コミット情報自動取得機能
- [x] `tools/task_manager/storage.py` に `add_commit_progress()` メソッド追加

### Phase 2: CLI コマンド追加
- [x] `tools/task_manager/cli.py` に `add-commit-progress` コマンド追加
- [x] `tools/task_manager/cli.py` に `show --resume` オプション追加

### Phase 3: タスク詳細表示の強化
- [x] `tools/task_manager/board.py` の `render_task_detail()` 拡張
- [x] `tools/task_manager/board.py` に `render_resume_info()` メソッド追加

### Phase 4: Next Actions 自動生成の改善
- [x] `tools/task_manager/cli.py` の `_generate_smart_next_actions()` ロジック改善
- [x] Progress内容を分析してインテリジェントな次アクション提案

### Phase 5: ドキュメント更新
- [x] `.claude/agents/impl-agent.md` のCLIパス修正
- [x] 新コマンド使用例を追加
- [x] planning_dir参照を削除（F006対応）

## 依存関係

- **ブロック**: なし
- **ブロックされる**: なし
- **関連ドキュメント**:
  - `tools/task_manager/storage.py`
  - `tools/task_manager/cli.py`
  - `tools/task_manager/board.py`
  - `.claude/agents/impl-agent.md`
