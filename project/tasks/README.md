# プロジェクトタスク管理（Markdownベース）

このディレクトリは、ゲームプロジェクトのタスクをMarkdownファイルで管理します。

---

## ディレクトリ構成

```
project/tasks/
├── .taskrc.yaml          # タスク管理設定
├── 1_todo/               # 未着手タスク
│   └── 30101-ジャンプ機能実装.md
├── 2_in-progress/        # 実装中タスク
│   └── P001-CI-CD構築.md
├── 3_in-review/          # レビュー中タスク
│   └── 30103-攻撃機能実装.md
└── 4_archive/            # 完了・キャンセル済みタスク
    ├── 30100-テストタスク.md (done)
    └── 30099-廃止機能.md (cancelled)
```

---

## タスク種別

### ゲーム開発タスク（30XXX形式）

`project/` 配下の仕様書・実装を対象とするタスク。

**ID形式**: `30XXX`（仕様書の番号と一致）

**例**:
- `30101-ジャンプ機能実装.md` - Player ジャンプ機能
- `30201-スライム実装.md` - Enemy スライム

**worktree対応**: ✅ 有効
- タスク開始時に自動でworktree/ブランチが作成される
- 並列実行が可能

### プロジェクト横断タスク（PXXX形式）

リポジトリ全体に影響するタスク（CI/CD、インフラ、全体設計等）。

**ID形式**: `PXXX`

**例**:
- `P001-CI-CD構築.md` - GitHub Actions 設定
- `P002-デバッグツール実装.md` - デバッグUI作成

**worktree対応**: ❌ 無効
- リポジトリ全体に影響するため、worktree非対応
- 順次実行を推奨

---

## タスクファイル構造

```markdown
---
id: 30101
type: game-dev
title: ジャンプ機能実装
status: todo
priority: high
tags: [feat, player]
created_at: 2025-01-15T10:00:00Z
updated_at: 2025-01-15T10:00:00Z
worktree: null
branch: null
plan_file: null
blocked_by: []
blocks: []
---

# 実装計画

## 対象仕様書
- project/docs/3_ingame/301_player/30101_player_spec.md
- project/docs/3_ingame/301_player/30102_player_design.md
- project/docs/3_ingame/301_player/30103_player_behavior.md

## タスク内容
Player のジャンプ機能を実装する。

## チェックリスト
- [ ] JumpSystem 実装
- [ ] PlayerJumpComponent 作成
- [ ] 仕様書との対応確認（@spec コメント）
- [ ] テスト実行
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
| `in-progress` | 作業中（worktree作成済み） |
| `in-review` | レビュー待ち（PR作成済み） |
| `done` | 完了（マージ済み） |
| `cancelled` | キャンセル |

---

## タスク操作（基本）

### タスク状況確認（人間専用コマンド）

```bash
# 全アクティブタスク表示
/task-status

# ゲーム開発タスクのみ
/task-status --type game-dev

# 進行中タスクのみ
/task-status --status in-progress

# 高優先度タスクのみ
/task-status --priority high
```

### タスク操作（task-manager-agent が実行）

**人間は以下のように指示するだけ:**

```
「Player ジャンプ機能のタスクを作成して」
「タスク30101を開始して」
「タスク30101をレビュー待ちにして」
「タスク30101を完了にして」
```

Claude Code がタスクファイルを直接操作し、ステータス変更時はファイルを適切なディレクトリに移動します。

---

## worktree管理（ゲーム開発タスクのみ）

### worktreeとは

Git の機能で、同じリポジトリの複数のブランチを独立したディレクトリで同時に作業できる仕組み。

### 自動作成

タスクを `in-progress` にすると自動的に以下が作成される:

```bash
# タスク30101の場合
worktree: ../spec-driven-framework-30101
branch: auto-30101-jump
```

### 並列実行

```
Terminal 1: タスク30101（Player）実装中
Terminal 2: タスク30201（Enemy）実装中
Terminal 3: タスク30301（Stage）実装中
```

それぞれ独立したworktreeで並列に作業可能。

---

## タスク依存関係

### blocked_by / blocks

タスク間の依存関係を記述できる:

```markdown
---
id: 30102
blocked_by: [30101]  # 30101が完了しないと開始できない
blocks: [30103]      # 30103は本タスク完了を待つ
---
```

---

## .taskrc.yaml の設定

```yaml
# プロジェクト用タスク管理設定
id_prefix:
  game-dev: 30000      # ゲーム開発タスクのID開始番号
  project-wide: P000   # プロジェクト横断タスクのID開始番号

archive_after_days: 30  # 完了後30日でアーカイブ

worktree_enabled: true  # worktree機能を有効化
worktree_base_dir: ..   # worktree作成場所

branch_prefix: auto-    # ブランチ名接頭辞
```

---

## 参照ドキュメント

- [タスク管理ワークフロー完全ガイド](../../skills/task-workflow.md)
- [並列セッション実行ガイド](../../skills/parallel-sessions.md)
- [フレームワーク仕様書](../../docs/reference/framework-spec.md)
