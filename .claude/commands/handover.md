---
description: セッション引き継ぎドキュメントを生成 (project)
argument-hint: [--task <id>] [--all] [--archive]
---

# /handover コマンド

現在のセッション状態を分析し、タスクファイルに引き継ぎ情報を記録します。

**オプション**: $ARGUMENTS

## 使用者

**人間専用コマンド** - セッション終了前に使用

## 設計思想

**タスクファイルが Single Source of Truth**

- 引き継ぎ情報はタスクファイルの `## Progress` / `## Next Actions` / `## メモ` に記録
- Git状態は毎回取得するため保存しない
- 従来の `.claude/handover.md` は廃止

## 使用方法

```bash
/handover                    # in-progress タスクを自動検出して更新
/handover --task F011        # 指定タスクを更新
/handover --all              # 全 in-progress タスクを更新
/handover --archive          # アーカイブも保存
```

## オプション

| オプション | 説明 |
|-----------|------|
| `--task <id>` | 対象タスクIDを指定（例: F011, 30101, P001） |
| `--all` | 全ての in-progress タスクを更新 |
| `--archive` | `.claude/handover/archive/` にスナップショット保存 |

## 処理フロー

### 1. 対象タスク特定

```bash
# in-progress タスクを検索
ls tasks/2_in-progress/ project/tasks/2_in-progress/
```

**判定ロジック:**
- `--task <id>` 指定時: 該当タスクを使用
- `--all` 指定時: 全 in-progress タスクを対象
- 引数なし + 1件のみ: そのタスクを使用
- 引数なし + 複数件: エラー（選択を促す）
- 引数なし + 0件: エラー

### 2. タスクファイル更新

タスクファイルの以下セクションを更新:

```markdown
## Progress

### Completed

- [x] 設計完了（プランファイル作成済み）
- [x] handover.md コマンドを更新        ← 追記
- [x] resume-handover.md コマンドを更新  ← 追記

## Next Actions

1. task-workflow.md にセクション追加    ← 上書き更新
2. CLAUDE.md を更新
3. テスト実行

## メモ

- 後方互換性: 既存 handover.md からの読み込みは deprecated 警告付きで維持
- アーカイブ機能実装済み              ← 追記（重要な決定事項）
```

### 3. メタデータ更新

```yaml
updated_at: "2026-01-03T16:00:00+09:00"  # 現在時刻に更新
```

### 4. アーカイブ保存（--archive 指定時）

```bash
mkdir -p .claude/handover/archive/
cp <task-file> .claude/handover/archive/$(date +%Y-%m-%d)-<task-id>.md
```

## エラーハンドリング

| 条件 | メッセージ |
|------|-----------|
| in-progress タスクなし | `No in-progress tasks found. Create or start a task first.` |
| 複数タスク + 未指定 | `Multiple in-progress tasks found. Please specify --task <id>:` + 一覧表示 |
| タスク存在しない | `Task not found: {task-id}` |

## 実行例

### 基本（単一タスク）

```
ユーザー: /handover

Claude:
タスク F011 の引き継ぎ情報を更新しました。

更新内容:
- Progress: 3件の完了作業を追記
- Next Actions: 2件に更新
- メモ: 1件の決定事項を追記

ファイル: tasks/2_in-progress/F011-handover-タスクファイル統合.md
```

### 複数タスク選択

```
ユーザー: /handover

Claude:
Multiple in-progress tasks found. Please specify --task <id>:

  F011  handover/resume-handover タスクファイル統合
  30105 ダッシュ機能実装

Usage: /handover --task F011
```

### 全タスク更新

```
ユーザー: /handover --all

Claude:
2件のタスクを更新しました:

  [F011] handover/resume-handover タスクファイル統合
    - Progress: 3件追記
    - Next Actions: 2件に更新

  [30105] ダッシュ機能実装
    - Progress: 1件追記
    - Next Actions: 3件に更新
```

### アーカイブ付き

```
ユーザー: /handover --archive

Claude:
タスク F011 の引き継ぎ情報を更新しました。

更新内容:
- Progress: 3件の完了作業を追記
- Next Actions: 2件に更新

アーカイブ: .claude/handover/archive/2026-01-03-F011.md
```

## 後方互換性（Deprecated）

従来の `.claude/handover.md` を使用している場合:

```
WARNING: .claude/handover.md is deprecated.
Please use task-based handover instead.
Migration: Move content to task file's Progress/Next Actions/メモ sections.
```

読み込みは可能だが、新規作成は行わない。

## 出力先

```
tasks/
└── 2_in-progress/
    └── F011-handover-タスクファイル統合.md  # タスクファイル内に記録

.claude/
└── handover/
    └── archive/                              # アーカイブ（--archive 時のみ）
        └── 2026-01-03-F011.md
```

## 注意事項

- **人間専用コマンド** - エージェントは使用しない
- タスクファイルは Git 管理されるため、コミット推奨
- アーカイブディレクトリは `.claudeignore` で除外推奨
- Git状態は保存しない（`/resume-handover` 時に毎回取得）
