---
id: "F011"
title: "handover/resume-handover タスクファイル統合"
type: "framework"
status: "done"
priority: "medium"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/s13219/.claude/plans/rosy-forging-codd.md"
tags: ["commands", "handover", "task-management"]
created_at: "2026-01-03T13:30:00+09:00"
updated_at: "2026-01-03T16:00:00+09:00"
completed_at: "2026-01-03T16:05:00+09:00"
---

# Task F011: handover/resume-handover タスクファイル統合

## Summary

`/handover` と `/resume-handover` コマンドをタスクファイルベースに統合する。
`.claude/handover.md` を廃止し、タスクファイルが Single Source of Truth となる。

## 背景

- 現在 handover.md とタスクファイルで情報が重複している
- タスクファイルに Progress / Next Actions が既にある
- Single Source of Truth 原則に違反している

## 設計決定

| 項目 | 決定 |
|------|------|
| 重要な決定事項 | 既存の `## メモ` セクションに記載 |
| Git状態 | タスクファイルに保存しない（毎回取得） |
| handover.md | 廃止 |

## 実装内容

- [x] `.claude/commands/handover.md` を更新
  - `--task <id>`, `--all`, `--archive` オプション追加
  - タスクファイルの Progress / Next Actions / メモ を更新
  - handover.md 出力を廃止
- [x] `.claude/commands/resume-handover.md` を更新
  - `--task <id>` オプション追加
  - タスクファイルから状態読み込み
  - Git状態はリアルタイム取得
- [x] `.claude/skills/task-workflow.md` に handover 連携説明を追加
- [x] `.claude/CLAUDE.md` のコマンド説明を更新

## メモ

- 後方互換性: 既存 handover.md からの読み込みは deprecated 警告付きで維持
- アーカイブ機能: `.claude/handover/archive/YYYY-MM-DD-<task-id>.md` 形式

## 依存関係

- **Blocked By:** なし
- **Blocks:** なし

---

## Progress

### Completed

- [x] 設計完了（プランファイル作成済み）
- [x] タスクを開始状態に移行
- [x] handover.md コマンドを更新（新仕様に完全対応）
- [x] resume-handover.md コマンドを更新（新仕様に完全対応）
- [x] task-workflow.md に「セッション引き継ぎ（Handover連携）」セクション追加
- [x] CLAUDE.md の人間専用コマンド説明を更新

## Next Actions

1. コミット
2. タスクを done に移行

---

## Detailed Implementation Plan

> このセクションは、タスク作成時のプランモードで生成されたプランファイルの内容です。

### `/handover` コマンドの新仕様

**引数:**
```
/handover [--task <task-id>] [--all] [--archive]
```

**処理フロー:**
1. 対象タスク特定（引数 or 自動検出）
2. タスクファイルの更新:
   - `## Progress` に完了作業を追加（追記）
   - `## Next Actions` を更新（上書き）
   - `## メモ` に重要決定事項を追加（任意）
   - `updated_at` を更新
3. `--archive` 指定時: スナップショット保存

### `/resume-handover` コマンドの新仕様

**引数:**
```
/resume-handover [--task <task-id>] [<archive-file>] [additional-prompt...]
```

**出力内容:**
- タスク概要（ID, タイトル, ステータス）
- 直近の Progress（最新5件）
- Next Actions
- メモ
- Git状態（毎回取得）
- worktree情報
- 追加プロンプト

### エラーハンドリング

| 条件 | メッセージ |
|------|-----------|
| in-progress タスクなし | `No in-progress tasks found. Create or start a task first.` |
| 複数タスク選択必要 | `Multiple in-progress tasks found. Please specify --task <id>:` + 一覧 |
| タスク存在しない | `Task not found: {task-id}` |

### 変更対象ファイル

1. `.claude/commands/handover.md`
2. `.claude/commands/resume-handover.md`
3. `.claude/skills/task-workflow.md`
4. `.claude/CLAUDE.md`
