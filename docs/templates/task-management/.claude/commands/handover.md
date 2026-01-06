---
description: セッション引き継ぎ情報をタスクファイルに記録
argument-hint: [--task <id>] [--all]
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

## 使用方法

```bash
/handover                    # in-progress タスクを自動検出して更新
/handover --task T001        # 指定タスクを更新
/handover --all              # 全 in-progress タスクを更新
```

## オプション

| オプション | 説明 |
|-----------|------|
| `--task <id>` | 対象タスクIDを指定 |
| `--all` | 全ての in-progress タスクを更新 |

## 処理フロー

### 1. 対象タスク特定

```bash
ls tasks/2_in-progress/
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

- [x] 既存の完了作業
- [x] 今回完了した作業        ← 追記

## Next Actions

1. 次のアクション1            ← 上書き更新
2. 次のアクション2

## メモ

- 既存のメモ
- 重要な決定事項              ← 追記
```

### 3. メタデータ更新

```yaml
updated_at: "現在時刻"
```

## エラーハンドリング

| 条件 | メッセージ |
|------|-----------|
| in-progress タスクなし | `No in-progress tasks found.` |
| 複数タスク + 未指定 | `Multiple in-progress tasks found. Please specify --task <id>:` + 一覧 |
| タスク存在しない | `Task not found: {task-id}` |

## 実行例

### 基本（単一タスク）

```
ユーザー: /handover

Claude:
タスク T001 の引き継ぎ情報を更新しました。

更新内容:
- Progress: 3件の完了作業を追記
- Next Actions: 2件に更新
- メモ: 1件の決定事項を追記

ファイル: tasks/2_in-progress/T001-ログイン機能実装.md
```

### 複数タスク選択

```
ユーザー: /handover

Claude:
Multiple in-progress tasks found. Please specify --task <id>:

  T001  ログイン機能実装
  T002  ダッシュボード作成

Usage: /handover --task T001
```

## 注意事項

- **人間専用コマンド** - エージェントは使用しない
- タスクファイルは Git 管理されるため、コミット推奨
