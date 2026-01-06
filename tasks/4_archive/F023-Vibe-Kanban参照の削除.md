---
id: "F023"
title: "Vibe Kanban参照の削除"
type: "framework"
status: "done"
priority: "high"
spec_ids: []
blocked_by: ["F019"]
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/s13219/.claude/plans/sharded-stargazing-zephyr.md"
tags: ["agents", "cleanup"]
created_at: "2026-01-04T16:00:00+09:00"
updated_at: "2026-01-04T16:30:00+09:00"
completed_at: "2026-01-04T16:30:00+09:00"
---

# Task F023: Vibe Kanban参照の削除

## 説明

廃止済みの Vibe Kanban（GitHub Projects）への参照を全エージェントファイルから削除する。

## 背景

### 現状

- task-manager-agent で Vibe Kanban は明確に禁止
- しかし impl-agent、refactor-agent に Vibe 参照が残存
- PR本文に「Vibe Task」が含まれる例がある

### 改修理由

- 廃止済み機能への参照は混乱を招く
- Markdown ファイルベースのタスク管理に統一

## 実装内容

### 1. impl-agent.md

**該当箇所**: L456-491（PR本文テンプレート）

**修正**:
```markdown
Before:
Vibe Task: #<タスクID>

After:
Task: project/tasks/*/30101-*.md
```

または「Vibe Task」行を完全削除

### 2. refactor-agent.md

**該当箇所**: L64

**修正**:
```markdown
Before:
- [ ] 関連 Vibe タスクを Done に移動

After:
- [ ] 関連タスクファイルを 4_archive/ に移動
```

## 対象ファイル

| ファイル | 該当行 | 修正内容 |
|---------|-------|---------|
| `.claude/agents/impl-agent.md` | L456-491 | PR本文から「Vibe Task」削除 |
| `.claude/agents/refactor-agent.md` | L64 | チェックリスト修正 |

## 依存関係

- **ブロック**: なし
- **ブロックされる**: F019
- **関連レビュー**:
  - tasks/1_todo/agent-review-g4.md（I-01）
  - tasks/1_todo/agent-review-g5.md（RF-01）

## メモ

- 2ファイルのみの軽微な修正
- 約10行の修正
