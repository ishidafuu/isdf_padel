---
description: ID予約状況一覧表示 (project)
argument-hint: [<folder>] [--active] [--expired]
---

# /id-reserve-status コマンド

ID予約の状況を一覧表示します。

**オプション**: $ARGUMENTS

## 使用者

**自動実行用** - session-manager-agent ガイドラインに従って Claude Code が実行

## オプション

| オプション | 説明 |
|-----------|------|
| `<folder>` | 特定フォルダの予約のみ表示（例: 30101） |
| `--active` | アクティブな予約のみ表示 |
| `--expired` | 期限切れの予約のみ表示 |

## 処理フロー

### 1. 予約ファイル読み込み

```bash
# .id-reservations.yml を読み込み
cat .id-reservations.yml
```

### 2. 予約状態の判定

各予約について:
- `expires_at` < 現在時刻 → EXPIRED
- `expires_at` >= 現在時刻 → ACTIVE

### 3. 使用状況の確認（オプション）

```bash
# 実際に使われているIDを数える
grep -oE "REQ-30101-[0-9]{3}" project/docs/3_ingame/301_player/30101_spec.md | sort -u | wc -l
```

## 出力例

### 全予約の確認

```
ユーザー: /id-reserve-status

Claude:
=== ID Reservations ===

[ACTIVE] res-001
- Type: REQ
- Folder: 30101 (docs/3_ingame/301_player/)
- Range: 001-050
- Used: 45/50
- Session: auto-12345-player
- Created: 2 hours ago
- Expires: in 6 hours

[ACTIVE] res-002
- Type: DES
- Folder: 30101 (docs/3_ingame/301_player/)
- Range: 001-030
- Used: 30/30
- Session: auto-12345-player
- Created: 1 hour ago
- Expires: in 7 hours

[EXPIRED] res-003
- Type: TST
- Folder: 30105 (docs/3_ingame/301_player/)
- Range: 001-100
- Used: 0/100
- Session: session-old
- Expired: 2 hours ago

Total: 2 active, 1 expired
```

### 特定フォルダの確認

```
ユーザー: /id-reserve-status 30101

Claude:
=== ID Reservations for 30101 ===
Folder: docs/3_ingame/301_player/

| Type | Range | Used | Status | Session | Expires |
|------|-------|------|--------|---------|---------|
| REQ | 001-050 | 45/50 | active | auto-12345-player | in 6h |
| DES | 001-030 | 30/30 | active | auto-12345-player | in 7h |
| BHV | (none) | - | - | - | - |
| TST | 001-100 | 0/100 | expired | session-old | -2h ago |
```

### アクティブのみ

```
ユーザー: /id-reserve-status --active

Claude:
=== Active ID Reservations ===

| ID | Type | Folder | Range | Used | Session | Expires |
|----|------|--------|-------|------|---------|---------|
| res-001 | REQ | 30101 | 001-050 | 45/50 | auto-12345-player | in 6h |
| res-002 | DES | 30101 | 001-030 | 30/30 | auto-12345-player | in 7h |
| res-004 | REQ | 30201 | 001-050 | 20/50 | auto-12346-enemy | in 5h |

Total: 3 active reservations
```

## エラーハンドリング

| 条件 | メッセージ |
|------|-----------|
| 予約なし | `No reservations found.` |
| 予約ファイルなし | `.id-reservations.yml not found. No reservations exist.` |
| 無効なフォルダ番号 | `Invalid folder number: {folder}` |

## 関連コマンド

- `/id-reserve` - ID範囲を予約
- `/id-reserve-release` - 予約解放
- `/id-reserve-extend` - 予約延長
- `/id-reserve-cleanup` - 期限切れ予約削除
