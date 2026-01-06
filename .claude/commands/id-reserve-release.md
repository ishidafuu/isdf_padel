---
description: ID予約を解放 (project)
argument-hint: <reservation-id> | --session <session-name> | --all
---

# /id-reserve-release コマンド

ID予約を即座に解放します。

**引数**: $ARGUMENTS

## 使用者

**自動実行用** - session-manager-agent ガイドラインに従って Claude Code が実行

## 使用方法

```bash
# 特定の予約を解放
/id-reserve-release res-001

# セッションの全予約を解放
/id-reserve-release --session auto-12345-player

# 全ての予約を解放（危険）
/id-reserve-release --all
```

## オプション

| オプション | 説明 |
|-----------|------|
| `<reservation-id>` | 特定の予約IDを解放（例: res-001） |
| `--session <name>` | 指定セッションの全予約を解放 |
| `--all` | 全ての予約を解放（確認あり） |

## 処理フロー

### 1. 対象予約の特定

```bash
# .id-reservations.yml から対象を検索
grep "id: res-001" .id-reservations.yml
```

### 2. 予約の削除

```yaml
# .id-reservations.yml から該当エントリを削除
reservations:
  - id: res-001  # ← 削除
    ...
```

### 3. 結果出力

解放した予約の情報を表示

## 出力例

### 特定予約の解放

```
ユーザー: /id-reserve-release res-001

Claude:
予約を解放しました:

- ID: res-001
- Type: REQ
- Folder: 30101 (docs/3_ingame/301_player/)
- Range: 001-050
- Used: 45/50
- Session: auto-12345-player

注意: 使用済みID（001-045）は引き続き有効です。
未使用範囲（046-050）は他のセッションが予約可能になりました。
```

### セッション全解放

```
ユーザー: /id-reserve-release --session auto-12345-player

Claude:
セッション auto-12345-player の予約を全て解放しました:

| ID | Type | Folder | Range | Used |
|----|------|--------|-------|------|
| res-001 | REQ | 30101 | 001-050 | 45/50 |
| res-002 | DES | 30101 | 001-030 | 30/30 |
| res-005 | BHV | 30101 | 001-050 | 25/50 |

Total: 3 reservations released
```

### 全解放（確認あり）

```
ユーザー: /id-reserve-release --all

Claude:
⚠️  全ての予約を解放しようとしています。

現在の予約:
- 5 active reservations
- 3 sessions affected

この操作を実行しますか？ (y/n)

> y

全ての予約を解放しました:
- Released: 5 reservations
- Sessions affected: auto-12345-player, auto-12346-enemy, auto-12347-stage
```

## エラーハンドリング

| 条件 | メッセージ |
|------|-----------|
| 予約が存在しない | `Reservation not found: res-001` |
| セッションが存在しない | `No reservations found for session: {session}` |
| 既に解放済み | `Reservation already released: res-001` |

## 注意事項

- 解放しても、既に使用されたIDは有効なまま
- 解放された範囲は他のセッションが予約可能になる
- セッション終了時は自動的に解放されるが、手動解放を推奨

## 関連コマンド

- `/id-reserve` - ID範囲を予約
- `/id-reserve-status` - 予約状況一覧
- `/id-reserve-extend` - 予約延長
- `/id-reserve-cleanup` - 期限切れ予約削除
