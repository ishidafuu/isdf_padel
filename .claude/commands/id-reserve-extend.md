---
description: ID予約の期限を延長 (project)
argument-hint: <reservation-id> <hours>
---

# /id-reserve-extend コマンド

ID予約の有効期限を延長します。

**引数**: $ARGUMENTS（予約ID 延長時間）

## 使用者

**自動実行用** - session-manager-agent ガイドラインに従って Claude Code が実行

## 使用方法

```bash
# 4時間延長
/id-reserve-extend res-001 4

# 24時間延長（翌日まで）
/id-reserve-extend res-001 24
```

## オプション

| 引数 | 説明 |
|-----|------|
| `<reservation-id>` | 延長する予約のID（例: res-001） |
| `<hours>` | 延長時間（時間単位、最大48時間） |

## 処理フロー

### 1. 対象予約の確認

```bash
# .id-reservations.yml から対象を検索
grep -A10 "id: res-001" .id-reservations.yml
```

### 2. 有効期限の更新

```yaml
# expires_at を更新
- id: res-001
  ...
  expires_at: 2026-01-04T18:00:00+09:00  # 旧
  ↓
  expires_at: 2026-01-04T22:00:00+09:00  # 新（+4時間）
```

### 3. 結果出力

## 出力例

### 正常な延長

```
ユーザー: /id-reserve-extend res-001 4

Claude:
予約期限を延長しました:

- ID: res-001
- Type: REQ
- Folder: 30101 (docs/3_ingame/301_player/)
- Range: 001-050

期限:
- 旧: 2026-01-04 18:00（残り2時間）
- 新: 2026-01-04 22:00（残り6時間）
- 延長: +4時間
```

### 期限切れ予約の復活

```
ユーザー: /id-reserve-extend res-003 8

Claude:
期限切れ予約を復活させました:

- ID: res-003
- Type: TST
- Folder: 30105 (docs/3_ingame/301_player/)
- Range: 001-100

期限:
- 旧: 2026-01-04 10:00（2時間前に失効）
- 新: 2026-01-04 20:00（残り8時間）
- 延長: +8時間（失効分 + 延長分）

⚠️  注意: 失効中に他のセッションが予約していた場合、範囲が重複する可能性があります。
/id-reserve-status で確認してください。
```

## エラーハンドリング

| 条件 | メッセージ |
|------|-----------|
| 予約が存在しない | `Reservation not found: res-001` |
| 延長時間が不正 | `Invalid hours: must be 1-48` |
| 最大期限超過 | `Cannot extend beyond 48 hours from now` |

## 制限事項

- 最大延長時間: 48時間
- 期限切れ予約の復活は可能だが、範囲重複に注意

## 関連コマンド

- `/id-reserve` - ID範囲を予約
- `/id-reserve-status` - 予約状況一覧
- `/id-reserve-release` - 予約解放
- `/id-reserve-cleanup` - 期限切れ予約削除
