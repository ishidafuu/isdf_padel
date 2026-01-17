# task-workflow

## 概要

**タスクライフサイクル管理スキル** - Markdownファイルベースのタスク管理ワークフロー

このスキルは以下のファイルに分割されています：

| ファイル | 内容 |
|---------|------|
| **task-lifecycle.md** | タスク作成フロー、状態遷移、親子タスク【将来実装】、セッション引き継ぎ |
| **task-file-format.md** | ファイル配置、Frontmatter形式、本文構造 |
| **task-operations.md** | 基本操作、検索、依存関係管理、worktree管理 |
| **task-status.md** | タスク状況表示形式、アイコン定義 |
| **task-planning.md** | プランモードフロー |
| **task-registration.md** | タスク登録（配置ルール、バリデーション） |

### 人間向け詳細ガイド

詳細な使用例・参考実装は `docs/concepts/tasks.md` を参照。

### 参照元ガイドライン

- task-manager-agent.md（主担当）
- impl-agent.md（タスク参照・更新）
- session-manager-agent.md（worktree連携）

### タスク登録

**タスク登録時は必ず `task-registration.md` を参照。**

---

## クイックリファレンス

### タスク作成フロー

1. **プラン作成** - ユーザーがプランモードで実行
2. **タスク登録** - task-registration.md を参照して実行

詳細: `task-lifecycle.md`, `task-planning.md`

### タスク状態遷移

```
todo → in-progress → in-review → done
                                  ↑
         cancelled ───────────────┘
```

詳細: `task-lifecycle.md`

### タスクタイプと配置

| タスクタイプ | ID形式 | 配置場所 | worktree |
|------------|-------|---------|----------|
| game-dev | `30XXX` | `project/tasks/` | ✅ |
| project-wide | `PXXX` | `project/tasks/` | ❌ |
| framework | `FXXX` | `tasks/` | ❌ |

詳細: `task-file-format.md`

### 基本操作

- タスク開始/完了: `task-operations.md`
- タスク検索: `task-operations.md`
- 状況表示: `task-status.md`

### バグバックログ

バグ発見時の一時記録からタスク化までのフロー。

詳細: [bug-backlog.md](bug-backlog.md)

---

## 関連ドキュメント

### 分割ファイル（Claude用スキル）

- `skills/task-lifecycle.md` - タスク状態遷移
- `skills/task-file-format.md` - タスクファイル形式
- `skills/task-operations.md` - タスク操作
- `skills/task-status.md` - 状況表示形式
- `skills/task-planning.md` - プランモードフロー
- `skills/task-registration.md` - タスク登録

### 人間向けドキュメント

- `docs/concepts/tasks.md` - タスク管理詳細ガイド（使用例、参考実装）

### ガイドライン

- `agents/task-manager-agent.md` - タスク管理専門ガイドライン
- `agents/session-manager-agent.md` - 並列セッション管理

### コマンド

- `commands/handover.md` - セッション引き継ぎ
- `commands/resume-handover.md` - セッション再開

### 設定

- `project/tasks/.taskrc.yaml` - プロジェクトタスク設定
- `tasks/.taskrc.yaml` - フレームワークタスク設定
