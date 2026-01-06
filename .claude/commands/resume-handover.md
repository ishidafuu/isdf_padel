---
description: handoverドキュメントを読み込んでセッションを再開 (project)
argument-hint: [--task <id>] [archive-file] [additional-prompt...]
---

# /resume-handover コマンド

タスクファイルから引き継ぎ情報を読み込み、作業を再開します。

**オプション/引数**: $ARGUMENTS

## 使用者

**人間専用コマンド** - セッション開始時に使用

## 設計思想

**タスクファイルが Single Source of Truth**

- タスクファイルの `## Progress` / `## Next Actions` / `## メモ` から状態を復元
- Git状態はリアルタイムで取得（保存された情報は使わない）
- worktree情報も表示

## 使用方法

```bash
/resume-handover                              # in-progress タスクを自動検出
/resume-handover --task F011                  # 指定タスクから再開
/resume-handover .claude/handover/archive/... # アーカイブから復元（deprecated）
/resume-handover 今日はテストから始めたい      # 追加プロンプト付き
```

## オプション

| オプション | 説明 |
|-----------|------|
| `--task <id>` | 対象タスクIDを指定（例: F011, 30101, P001） |
| `<archive-file>` | アーカイブファイルから復元（deprecated） |
| `<additional-prompt>` | 追加の作業指示 |

## 処理フロー

### 1. 対象タスク特定

```bash
# in-progress タスクを検索
ls tasks/2_in-progress/ project/tasks/2_in-progress/
```

**判定ロジック:**
- `--task <id>` 指定時: 該当タスクを使用
- アーカイブファイル指定時: そのファイルを使用（deprecated 警告表示）
- 引数なし + 1件のみ: そのタスクを使用
- 引数なし + 複数件: 一覧表示して選択を促す
- 引数なし + 0件: エラー

### 2. Git状態のリアルタイム取得

```bash
git branch --show-current
git log --oneline -3
git status --short
git stash list
```

### 3. タスク情報の抽出

タスクファイルから以下を抽出:
- フロントマター（id, title, status, priority, spec_ids, worktree_path）
- `## Progress` セクション
- `## Next Actions` セクション
- `## メモ` セクション

### 4. 出力フォーマット

```markdown
# セッション再開

## タスク情報

| 項目 | 値 |
|------|-----|
| **ID** | F011 |
| **タイトル** | handover/resume-handover タスクファイル統合 |
| **ステータス** | in-progress |
| **優先度** | medium |
| **関連仕様** | - |
| **ファイル** | tasks/2_in-progress/F011-handover-タスクファイル統合.md |

---

## Git状態

- **ブランチ**: master
- **最新コミット**:
  - abc1234 - [F010] done: docs ディレクトリ整頓
  - def5678 - [F010] WIP: docs ディレクトリ整頓
  - ghi9012 - [F009] done: docs/index.md整頓

- **変更ファイル**:
  M .claude/commands/handover.md
  M tasks/2_in-progress/F011-...

- **Stash**: なし

---

## Progress（直近5件）

- [x] 設計完了（プランファイル作成済み）
- [x] handover.md コマンドを更新

---

## Next Actions

1. resume-handover.md コマンドを更新
2. task-workflow.md にセクション追加
3. CLAUDE.md を更新

---

## メモ

- 後方互換性: 既存 handover.md からの読み込みは deprecated 警告付きで維持

---

## 追加の指示

[追加プロンプトがあれば表示]

---

## 推奨される次のアクション

Next Actions の第1項目「resume-handover.md コマンドを更新」から開始することをお勧めします。

どのように進めますか？
```

## エラーハンドリング

| 条件 | メッセージ |
|------|-----------|
| in-progress タスクなし | `No in-progress tasks found. Start a task first with task-manager-agent.` |
| 複数タスク + 未指定 | `Multiple in-progress tasks found. Please specify --task <id>:` + 一覧表示 |
| タスク存在しない | `Task not found: {task-id}` |

## 後方互換性（Deprecated）

### 従来の handover.md からの読み込み

```bash
/resume-handover .claude/handover.md
```

```
WARNING: .claude/handover.md is deprecated.
Please migrate to task-based handover.

Reading legacy handover file...
[従来形式で内容を表示]
```

### アーカイブからの読み込み

```bash
/resume-handover .claude/handover/archive/2026-01-03-F011.md
```

アーカイブファイルからの読み込みは引き続きサポート。

## 実行例

### 基本（単一タスク）

```
ユーザー: /resume-handover

Claude:
# セッション再開

## タスク情報
| 項目 | 値 |
|------|-----|
| **ID** | F011 |
| **タイトル** | handover/resume-handover タスクファイル統合 |
...

## 推奨される次のアクション
Next Actions の第1項目から開始することをお勧めします。

どのように進めますか？
```

### 複数タスク選択

```
ユーザー: /resume-handover

Claude:
Multiple in-progress tasks found. Please specify --task <id>:

  F011  handover/resume-handover タスクファイル統合
  30105 ダッシュ機能実装

Usage: /resume-handover --task F011
```

### 追加プロンプト付き

```
ユーザー: /resume-handover 今日はテストから始めたい

Claude:
# セッション再開

## タスク情報
...

## 追加の指示
今日はテストから始めたい

## 推奨される次のアクション
追加の指示に従い、テストの実行から開始します。

どのように進めますか？
```

## worktree情報の表示

ゲーム開発タスク（30XXX）で worktree が設定されている場合:

```markdown
## Worktree情報

| 項目 | 値 |
|------|-----|
| **パス** | ../spec-driven-framework-player |
| **ブランチ** | auto-12345-player |

注意: このタスクは worktree で作業中です。
正しいディレクトリで作業していることを確認してください。
```

## 注意事項

- **自動実行しない** - 内容表示・提案のみ。作業開始はユーザー指示を待つ
- Git状態は毎回リアルタイム取得（タスクファイルに保存しない）
- worktree 設定時は正しいディレクトリで作業するよう注意喚起

## 関連コマンド

- `/handover` - セッション引き継ぎドキュメントを生成
- `/status` - プロジェクト全体の進捗状況を確認
- `/task-status` - タスク状況を確認
