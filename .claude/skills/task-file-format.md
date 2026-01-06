# task-file-format

## 概要

**タスクファイル形式スキル** - Markdownタスクファイルのデータ構造を定義

### 参照元ガイドライン

- task-registration-agent.md（タスク作成）
- task-manager-agent.md（タスク管理）

---

## ファイル配置

### タスクタイプによる配置

| タスクタイプ | ID形式 | 配置場所 | worktree |
|------------|-------|---------|----------|
| game-dev | `30XXX` | `project/tasks/` | ✅ 有効 |
| project-wide | `PXXX` | `project/tasks/` | ❌ 無効 |
| framework | `FXXX` | `tasks/` | ❌ 無効 |

### ディレクトリ構造

```
tasks/                           # フレームワーク開発タスク
├── .taskrc.yaml                 # 設定ファイル
├── 1_todo/                      # 未着手
├── 2_in-progress/               # 実装中
├── 3_in-review/                 # レビュー中
└── 4_archive/                   # 完了・キャンセル

project/tasks/                   # ゲーム開発・プロジェクト横断タスク
├── .taskrc.yaml                 # 設定ファイル
├── 1_todo/                      # 未着手
├── 2_in-progress/               # 実装中
├── 3_in-review/                 # レビュー中
└── 4_archive/                   # 完了・キャンセル
```

---

## ファイル名形式

**形式: `{ID}-{タイトル}.md`**

```
30101-ジャンプ機能実装.md
P001-CI-CD構築.md
F001-エージェント更新.md
30101-1-ジャンプ機能-仕様書.md  # 子タスク
```

---

## Frontmatter必須フィールド

```yaml
---
id: "30101"                          # タスクID（必須）
title: "ジャンプ機能実装"             # タイトル（必須）
type: "game-dev"                     # タスクタイプ（必須）: game-dev, project-wide, framework
status: "todo"                       # ステータス（必須）: todo, in-progress, in-review, done, cancelled, planning
priority: "medium"                   # 優先度（必須）: high, medium, low
spec_ids: ["30201"]                  # 関連仕様書ID（任意）
blocked_by: []                       # ブロック元タスクID（任意）
blocks: []                           # ブロック先タスクID（任意）
branch_name: null                    # Gitブランチ名（game-devのみ）
worktree_path: null                  # worktreeパス（game-devのみ）
plan_file: "~/.claude/plans/xxx.md"  # プランファイルパス（任意）
tags: ["player", "physics"]          # タグ（任意）
parent_task_id: null                 # 親タスクID（子タスクのみ）
created_at: "2025-12-29T10:00:00"    # 作成日時（必須）
updated_at: "2025-12-29T10:00:00"    # 更新日時（必須）
completed_at: null                   # 完了日時（完了時のみ）
---
```

### フィールド詳細

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|-----|------|
| `id` | string | ✅ | タスクID（30101, P001, F001, 30101-1） |
| `title` | string | ✅ | タスクタイトル |
| `type` | string | ✅ | game-dev, project-wide, framework |
| `status` | string | ✅ | todo, in-progress, in-review, done, cancelled, planning |
| `priority` | string | ✅ | high, medium, low |
| `spec_ids` | array | - | 関連する仕様書のID |
| `blocked_by` | array | - | このタスクをブロックしているタスクID |
| `blocks` | array | - | このタスクがブロックしているタスクID |
| `branch_name` | string | - | Gitブランチ名（game-devのみ） |
| `worktree_path` | string | - | worktreeパス（game-devのみ） |
| `plan_file` | string | - | プランファイルのパス |
| `tags` | array | - | タグ（検索用） |
| `parent_task_id` | string | - | 親タスクのID（子タスクのみ） |
| `created_at` | string | ✅ | ISO 8601形式 |
| `updated_at` | string | ✅ | ISO 8601形式 |
| `completed_at` | string | - | ISO 8601形式（完了時のみ） |

---

## タイムスタンプ形式

**ISO 8601形式を使用:**

```
2025-12-29T10:00:00
2026-01-04T21:00:00+09:00
```

---

## 本文構造

### 基本構造

```markdown
---
(frontmatter)
---

# Task {ID}: {title}

## Summary

タスクの概要説明

## Related Specifications

- 関連仕様書へのリンク

## Progress

### Completed

- [x] 完了した作業1
- [x] 完了した作業2

## Next Actions

1. 次のアクション1
2. 次のアクション2

## Dependencies

- **Blocked By:** 依存タスク
- **Blocks:** ブロックしているタスク

## Detailed Implementation Plan

> プランファイルから埋め込まれた詳細計画

## メモ

- 重要な決定事項
- 注意点
```

### Progress セクション

タスクの進捗履歴を記録するセクション。

```markdown
## Progress

### Completed

- [x] 仕様書作成完了
- [x] 2025-12-31 14:00 feat: ジャンプ機能実装 (abc1234)
- [x] レビュー完了 - 問題なし（PASS: 15, FAIL: 0, WARN: 0）

### In Progress

- [ ] PR作成準備中
```

### Next Actions セクション

次に実行すべきアクションのリスト。

```markdown
## Next Actions

1. テスト実行
2. review-agent によるレビュー依頼
3. 問題があれば修正、問題なければPR作成
```

---

## 親子タスク構造【将来実装】

親子タスク機能の詳細（パターン、ワークフロー、構造例）は `skills/task-lifecycle.md` を参照。

**概要**:
- 親タスク: `status="planning"` で固定、`parent_task_id: null`
- 子タスク: `parent_task_id` で親を参照、ID形式は `30101-1`

---

## 関連ドキュメント

- `skills/task-lifecycle.md` - タスク状態遷移
- `skills/task-operations.md` - タスク操作
- `agents/task-registration-agent.md` - タスク登録ガイドライン
