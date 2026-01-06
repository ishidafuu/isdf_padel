---
id: "F019"
title: "Python CLI参照の一括削除"
type: "framework"
status: "done"
priority: "high"
spec_ids: []
blocked_by: []
blocks: ["F020", "F021", "F022", "F023", "F024", "F025", "F026", "F027"]
branch_name: null
worktree_path: null
plan_file: "/Users/s13219/.claude/plans/sharded-stargazing-zephyr.md"
tags: ["agents", "cleanup", "critical"]
created_at: "2026-01-04T16:00:00+09:00"
updated_at: "2026-01-04T16:00:00+09:00"
completed_at: "2026-01-04T17:00:00+09:00"
---

# Task F019: Python CLI参照の一括削除

## 説明

全17エージェントファイルから存在しない `tools.task_manager.cli` および `scripts/task.py` への参照を削除する。

## 背景

### 現状

- 全エージェントの Phase 0 に `python3 -m tools.task_manager.cli list ...` が記載
- `tools.task_manager.cli` は存在しない（Markdown ファイルベースに移行済み）
- `scripts/task.py` も存在しない
- 約100行が無効なコード

### 改修理由

- 動作しないコマンドがガイドラインに記載されている
- メインが参照しても実行できない
- ユーザー・開発者の混乱を招く

## 実装内容

- [x] 全17エージェントファイルから Python CLI 参照を削除
- [x] Phase 0 セクションを Markdown タスク確認に置き換え

### 対象ファイル

| ファイル | 該当行（概算） |
|---------|---------------|
| setup-agent.md | L91-97 |
| requirements-agent.md | L71-72 |
| spec-agent.md | L59-60 |
| critic-agent.md | L66-87 |
| module-design-agent.md | L59-60 |
| design-agent.md | L65-66 |
| behavior-agent.md | L59-60 |
| test-agent.md | L59-60 |
| task-manager-agent.md | L361-391（約30行） |
| impl-agent.md | L59, L506-555（約50行） |
| review-agent.md | L74, L534-559（約30行） |
| architecture-agent.md | L60 |
| deps-agent.md | L144 |
| data-agent.md | L130 |
| refactor-agent.md | L143 |
| legacy-analyzer-agent.md | L71-72 |
| game-reference-agent.md | L73-74 |
| session-manager-agent.md | L81 |

### 置き換えパターン

```markdown
Before:
/usr/bin/python3 -m tools.task_manager.cli list --root project --type game-dev --status in-progress

After:
# タスク状況確認
ls tasks/2_in-progress/
ls project/tasks/2_in-progress/
```

## 依存関係

- **ブロック**: F020〜F027（このタスク完了後に開始）
- **ブロックされる**: なし
- **関連レビュー**: tasks/1_todo/agent-review-g1.md〜g6.md

## メモ

- 機械的な置き換えが中心
- task-manager-agent は約30行削除（create-group コマンド等）
- impl-agent, review-agent は約50行削除（Progress/Next Actions 参照）
