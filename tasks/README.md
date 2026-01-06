# フレームワークタスク管理（Markdownベース）

このディレクトリは、仕様書駆動開発フレームワーク自体の開発タスクをMarkdownファイルで管理します。

---

## ディレクトリ構成

```
tasks/
├── .taskrc.yaml          # タスク管理設定
├── 1_todo/               # 未着手タスク
│   └── F001-ドキュメント整合性確認.md
├── 2_in-progress/        # 実装中タスク
├── 3_in-review/          # レビュー中タスク
└── 4_archive/            # 完了・キャンセル済みタスク
```

---

## タスク種別

### フレームワーク開発タスク（FXXX形式）

`agents/`, `docs/`, `commands/` の更新を対象とするタスク。

**ID形式**: `FXXX`

**例**:
- `F001-エージェント更新.md` - spec-agent の機能追加
- `F002-ドキュメント整備.md` - ユーザーズガイド更新
- `F003-コマンド実装.md` - 新規スラッシュコマンド追加

**worktree対応**: ❌ 無効
- フレームワーク全体に影響するため、worktree非対応
- 順次実行を推奨

---

## タスクファイル構造

```markdown
---
id: F001
type: framework
title: spec-agent の EARS 記法拡張
status: todo
priority: high
tags: [agent, spec]
created_at: 2025-01-15T10:00:00Z
updated_at: 2025-01-15T10:00:00Z
worktree: null
branch: null
plan_file: null
blocked_by: []
blocks: []
---

# 実装計画

## 対象ファイル
- agents/spec-agent.md
- skills/ears.md
- docs/reference/spec-writing-guide.md

## タスク内容
spec-agent に EARS 記法の Optional パターンを追加する。

## チェックリスト
- [ ] spec-agent.md の更新
- [ ] ears.md の Example 追加
- [ ] ドキュメント更新
- [ ] 動作確認
```

---

## タスクライフサイクル

### 状態遷移

```
todo → in-progress → in-review → done
                    ↓
                 cancelled
```

### 状態の意味

| 状態 | 説明 |
|-----|------|
| `todo` | 未着手 |
| `in-progress` | 作業中 |
| `in-review` | レビュー待ち（PR作成済み） |
| `done` | 完了（マージ済み） |
| `cancelled` | キャンセル |

---

## タスク操作（基本）

### タスク状況確認（人間専用コマンド）

```bash
# 全アクティブタスク表示
/task-status --type framework

# 進行中タスクのみ
/task-status --type framework --status in-progress

# 高優先度タスクのみ
/task-status --type framework --priority high
```

### タスク操作（task-manager-agent が実行）

**人間は以下のように指示するだけ:**

```
「spec-agent の EARS 記法拡張タスクを作成して」
「タスクF001を開始して」
「タスクF001をレビュー待ちにして」
「タスクF001を完了にして」
```

Claude Code がタスクファイルを直接操作し、ステータス変更時はファイルを適切なディレクトリに移動します。

---

## worktree管理（非対応）

フレームワーク開発タスクは、リポジトリ全体に影響するため、**worktree機能は無効**です。

- 並列実行は非推奨
- 順次実行を推奨
- ブランチ戦略は通常のGitフローに従う

---

## タスク依存関係

### blocked_by / blocks

タスク間の依存関係を記述できる:

```markdown
---
id: F002
blocked_by: [F001]  # F001が完了しないと開始できない
blocks: [F003]      # F003は本タスク完了を待つ
---
```

---

## .taskrc.yaml の設定

```yaml
# フレームワーク用タスク管理設定
id_prefix:
  framework: F000   # フレームワークタスクのID開始番号

archive_after_days: 30  # 完了後30日でアーカイブ

worktree_enabled: false  # worktree機能を無効化
```

---

## 参照ドキュメント

- [タスク管理ワークフロー完全ガイド](../skills/task-workflow.md)
- [フレームワーク仕様書](../docs/reference/framework-spec.md)
- [設計判断集](../docs/reference/design-decisions.md)
