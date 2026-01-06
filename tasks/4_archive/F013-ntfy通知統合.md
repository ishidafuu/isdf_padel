---
id: "F013"
title: "ntfy通知統合"
type: "framework"
status: "done"
priority: "high"
spec_ids: []
blocked_by: []
blocks: []
plan_file: "~/.claude/plans/sorted-wishing-quilt.md"
tags: ["infrastructure", "notification"]
created_at: "2026-01-03T17:55:00+09:00"
updated_at: "2026-01-03T19:30:00+09:00"
completed_at: "2026-01-03T19:30:00+09:00"
---

# F013: ntfy通知統合

## 概要

Claude Codeの承認通知問題を解決し、ntfy経由でリモート通知を実現する。

### 目的
- **問題**: 承認待ちに気づかず、タイムアウトで作業が止まる
- **解決策**: ntfy によるプッシュ通知
- **優先度**: 高（すぐに解決が必要）

### 方針変更
当初は Discord Bot（claude-code-discord）を検討したが、以下の理由で ntfy に変更:
- Discord Bot は ANTHROPIC_API_KEY が必要 → API料金が発生
- ntfy は無料、アカウント不要、シンプル

## 実装内容

### Hook スクリプト更新
- `~/.claude/hooks/permission-requested.sh` - ntfy 通知追加
- `~/.claude/hooks/execution-complete.sh` - ntfy 通知追加

### ドキュメント
- `.claude/skills/ntfy-notification.md` - 新規作成
- `docs/reference/tools-reference.md` - 外部ツール統合セクション更新
- `docs/index.md` - リンク更新

### 削除（Discord Bot関連）
- `.claude/skills/discord-integration.md` - 削除
- `docs/guides/discord-bot-setup.md` - 削除

## 成功基準

- [x] 権限要求時に ntfy 通知が届く
- [x] 実行完了時に ntfy 通知が届く
- [x] 既存の macOS 通知も引き続き動作する
- [x] ドキュメントが ntfy に更新されている

## Progress

### 2026-01-03 18:00 - タスク開始
- タスクファイル作成、ステータスを in-progress に変更
- 当初は Discord Bot 統合を計画

### 2026-01-03 18:10 - Discord Bot ドキュメント作成
- `.claude/skills/discord-integration.md` 作成
- `docs/guides/discord-bot-setup.md` 作成

### 2026-01-03 19:00 - 方針変更（Discord → ntfy）
- Discord Bot は API 料金が発生することが判明
- ntfy（無料、シンプル）に変更

### 2026-01-03 19:30 - ntfy 統合完了
- Hook スクリプト更新（permission-requested.sh, execution-complete.sh）
- ntfy 通知動作確認済み
- ドキュメント更新（Discord → ntfy）
- タスク完了
