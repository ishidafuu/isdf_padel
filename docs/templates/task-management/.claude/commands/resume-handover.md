---
description: タスクファイルからセッションを再開
argument-hint: [--task <id>] [additional-prompt...]
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

## 使用方法

```bash
/resume-handover                              # in-progress タスクを自動検出
/resume-handover --task T001                  # 指定タスクから再開
/resume-handover 今日はテストから始めたい      # 追加プロンプト付き
```

## オプション

| オプション | 説明 |
|-----------|------|
| `--task <id>` | 対象タスクIDを指定 |
| `<additional-prompt>` | 追加の作業指示 |

## 処理フロー

### 1. 対象タスク特定

```bash
ls tasks/2_in-progress/
```

**判定ロジック:**
- `--task <id>` 指定時: 該当タスクを使用
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
- フロントマター（id, title, status, priority）
- `## Progress` セクション
- `## Next Actions` セクション
- `## メモ` セクション

### 4. 出力フォーマット

```markdown
# セッション再開

## タスク情報

| 項目 | 値 |
|------|-----|
| **ID** | T001 |
| **タイトル** | ログイン機能実装 |
| **ステータス** | in-progress |
| **優先度** | medium |
| **ファイル** | tasks/2_in-progress/T001-ログイン機能実装.md |

---

## Git状態

- **ブランチ**: main
- **最新コミット**:
  - abc1234 - feat: ログイン画面追加
  - def5678 - chore: 初期設定
  - ghi9012 - Initial commit

- **変更ファイル**:
  M src/auth/login.js
  A src/auth/logout.js

- **Stash**: なし

---

## Progress（直近5件）

- [x] 設計完了
- [x] ログイン画面UI作成

---

## Next Actions

1. バリデーション実装
2. API連携
3. テスト作成

---

## メモ

- OAuth対応は次フェーズで検討

---

## 追加の指示

[追加プロンプトがあれば表示]

---

## 推奨される次のアクション

Next Actions の第1項目から開始することをお勧めします。

どのように進めますか？
```

## エラーハンドリング

| 条件 | メッセージ |
|------|-----------|
| in-progress タスクなし | `No in-progress tasks found. Start a task first.` |
| 複数タスク + 未指定 | `Multiple in-progress tasks found. Please specify --task <id>:` + 一覧 |
| タスク存在しない | `Task not found: {task-id}` |

## 実行例

### 基本（単一タスク）

```
ユーザー: /resume-handover

Claude:
# セッション再開

## タスク情報
| 項目 | 値 |
|------|-----|
| **ID** | T001 |
...

## 推奨される次のアクション
Next Actions の第1項目から開始することをお勧めします。

どのように進めますか？
```

### 追加プロンプト付き

```
ユーザー: /resume-handover 今日はテストから始めたい

Claude:
# セッション再開

...

## 追加の指示
今日はテストから始めたい

## 推奨される次のアクション
追加の指示に従い、テストの実行から開始します。

どのように進めますか？
```

## 注意事項

- **自動実行しない** - 内容表示・提案のみ。作業開始はユーザー指示を待つ
- Git状態は毎回リアルタイム取得（タスクファイルに保存しない）

## 関連コマンド

- `/handover` - セッション引き継ぎ情報を記録
