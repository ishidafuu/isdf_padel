---
description: セッション状況確認（アクティブセッション、ID予約状況） (project)
argument-hint: [--verbose]
---

# /session-status コマンド

アクティブな並列セッションの状況を表示します。

**オプション**: $ARGUMENTS

## 使用者

**自動実行用** - session-manager-agent ガイドラインに従って Claude Code が実行

## オプション

| オプション | 説明 |
|-----------|------|
| `--verbose` | 各セッションの詳細情報（変更ファイル一覧）を表示 |

## 処理フロー

### 1. セッション情報読み込み

```bash
# .session-locks.yml からアクティブセッションを読み込み
cat .session-locks.yml
```

### 2. 各worktreeの状態確認

```bash
# 各worktreeのコミット数と変更ファイルを確認
git -C ../spec-driven-framework-player log --oneline master..HEAD | wc -l
git -C ../spec-driven-framework-player diff --stat HEAD
```

### 3. ID予約状況確認

```bash
/id-reserve-status
```

### 4. 競合チェック

```bash
# 共有リソース（dependencies.md等）の変更を確認
git -C ../spec-driven-framework-player diff master -- project/docs/2_architecture/
git -C ../spec-driven-framework-enemy diff master -- project/docs/2_architecture/
```

## 出力例

### 基本出力

```
ユーザー: 「セッション状態を確認して」

Claude:
アクティブセッション: 3

| セッション | コミット | 機能 | ID範囲 |
|-----------|---------|------|--------|
| auto-12345-player | 4 | Player | REQ-30101-001～050 |
| auto-12346-enemy | 3 | Enemy | REQ-30201-001～050 |
| auto-12347-stage | 2 | Stage | REQ-30301-001～050 |

競合リスク: なし
```

### --verbose 出力

```
ユーザー: /session-status --verbose

Claude:
アクティブセッション: 3

## auto-12345-player (4 commits)
機能: Player
ID範囲: REQ-30101-001～050（使用済: 12）
変更ファイル:
  - project/src/player/player.gd (+150/-20)
  - project/src/player/player_state.gd (+80/-0)
  - project/tests/test_player.gd (+60/-0)

## auto-12346-enemy (3 commits)
機能: Enemy
ID範囲: REQ-30201-001～050（使用済: 8）
変更ファイル:
  - project/src/enemy/enemy.gd (+120/-15)
  - project/docs/2_architecture/dependencies.md (+5/-0)  ⚠️ 共有リソース

## auto-12347-stage (2 commits)
機能: Stage
ID範囲: REQ-30301-001～050（使用済: 5）
変更ファイル:
  - project/src/stage/stage.gd (+90/-10)

競合リスク:
⚠️  dependencies.md を auto-12346-enemy が変更しています
推奨: このセッションを先にマージしてください
```

## エラーハンドリング

| 条件 | メッセージ |
|------|-----------|
| セッションなし | `No active sessions found. Use /session-init to start.` |
| worktree消失 | `Worktree not found: {path}. Session may have been cleaned up.` |
| .session-locks.yml破損 | `Session lock file corrupted. Manual cleanup required.` |

## 出力フォーマット

### 競合リスクの判定基準

| リスク | 条件 |
|--------|------|
| なし | 各セッションが独立したフォルダのみ変更 |
| 低 | テストファイルのみ重複 |
| 中 | 共有リソース（dependencies.md等）を1セッションが変更 |
| 高 | 共有リソースを複数セッションが変更 |

## 関連ドキュメント

- `/session-init` - セッション初期化
- `/session-merge` - セッションマージ
- `/id-reserve-status` - ID予約状況
- `skills/parallel-sessions.md` - 並列セッション実行ガイド
