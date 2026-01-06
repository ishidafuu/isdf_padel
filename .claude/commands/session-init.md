---
description: 並列セッションの初期化（worktree作成、ブランチ作成） (project)
argument-hint: <features...>
---

# /session-init コマンド

並列実装のためのセッションを初期化します。worktree作成、ブランチ作成、ID範囲予約を行います。

**引数**: $ARGUMENTS（例: `player enemy stage`）

## 使用者

**自動実行用** - session-manager-agent ガイドラインに従って Claude Code が実行

## 前提条件

- 対象機能の仕様書（spec.md）が作成済み
- 機能フォルダが `project/docs/3_ingame/` に存在

## 処理フロー

### 1. 既存worktreeのクリーンアップ確認

```bash
# 既存worktreeを検索
git worktree list

# 古いworktreeが見つかった場合
⚠️  既存のworktreeが見つかりました:
/Users/user/repo/spec-driven-framework-old-player

これらのworktreeをクリーンアップしますか？ (y/n)
```

### 2. 機能フォルダの特定

```bash
# 指定された機能名から対応するフォルダを特定
player → 301_player
enemy → 302_enemy
stage → 303_stage
```

### 3. worktree作成

```bash
# 各機能に対してworktreeを作成
git worktree add ../spec-driven-framework-player -b auto-{PID}-player
git worktree add ../spec-driven-framework-enemy -b auto-{PID}-enemy
git worktree add ../spec-driven-framework-stage -b auto-{PID}-stage
```

### 4. ID範囲予約

```bash
# 各セッションにID範囲を予約（50個ずつ）
/id-reserve REQ-30101 001-050  # player
/id-reserve REQ-30201 001-050  # enemy
/id-reserve REQ-30301 001-050  # stage
```

### 5. セッション情報記録

`.session-locks.yml` に記録:

```yaml
sessions:
  - branch: auto-12345-player
    worktree: ../spec-driven-framework-player
    folder: 301_player
    id_range: REQ-30101-001～050
    created_at: "2026-01-04T10:00:00+09:00"
  - branch: auto-12346-enemy
    worktree: ../spec-driven-framework-enemy
    folder: 302_enemy
    id_range: REQ-30201-001～050
    created_at: "2026-01-04T10:00:00+09:00"
  - branch: auto-12347-stage
    worktree: ../spec-driven-framework-stage
    folder: 303_stage
    id_range: REQ-30301-001～050
    created_at: "2026-01-04T10:00:00+09:00"
```

## 出力例

```
ユーザー: 「Player、Enemy、Stageを並列実装したい」

Claude（session-manager-agentガイドライン参照）:
準備されたworktree:
- ../spec-driven-framework-player (Branch: auto-12345-player)
  ID範囲: REQ-30101-001～050
- ../spec-driven-framework-enemy (Branch: auto-12346-enemy)
  ID範囲: REQ-30201-001～050
- ../spec-driven-framework-stage (Branch: auto-12347-stage)
  ID範囲: REQ-30301-001～050

次のコマンドを各Terminalで実行してください:

# Terminal 2:
cd ../spec-driven-framework-enemy && claude

# Terminal 3:
cd ../spec-driven-framework-stage && claude
```

## エラーハンドリング

| 条件 | メッセージ |
|------|-----------|
| 機能フォルダが存在しない | `Feature folder not found: 301_player. Create spec first.` |
| worktree作成失敗 | `Failed to create worktree: {path}. Check disk space.` |
| ブランチ名衝突 | `Branch already exists: auto-12345-player. Use different PID or cleanup.` |

## 注意事項

- 仕様策定フェーズでの並列実行は非推奨
- 1日の終わりに全セッションをマージすること
- 同じフォルダに複数セッションを割り当てない

## 関連ドキュメント

- `skills/parallel-sessions.md` - 並列セッション実行ガイド
- `agents/session-manager-agent.md` - セッション管理ガイドライン
- `/session-status` - セッション状態確認
- `/session-merge` - セッションマージ
