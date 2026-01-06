---
description: 期限切れID予約を削除 (project)
argument-hint: [--unused] [--dry-run] [--force]
---

# /id-reserve-cleanup コマンド

期限切れのID予約を削除してクリーンアップします。

**オプション**: $ARGUMENTS

## 使用者

**自動実行用** - session-manager-agent ガイドラインに従って Claude Code が実行

## オプション

| オプション | 説明 |
|-----------|------|
| `--unused` | 未使用予約（1つもIDを使っていない）も検出 |
| `--dry-run` | 削除せずに対象を表示のみ |
| `--force` | 確認なしで削除 |

## 処理フロー

### 1. 期限切れ予約の検出

```bash
# .id-reservations.yml を読み込み、expires_at < 現在時刻 のものを抽出
```

### 2. 未使用予約の検出（--unused時）

```bash
# 各予約について、実際に使用されているIDを確認
grep -oE "REQ-30101-[0-9]{3}" project/docs/3_ingame/301_player/30101_spec.md | wc -l
# → 0件なら未使用
```

### 3. 削除実行

```yaml
# .id-reservations.yml から該当エントリを削除
```

## 出力例

### 期限切れ予約の削除

```
ユーザー: /id-reserve-cleanup

Claude:
期限切れ予約を検出中...

削除対象:
| ID | Type | Folder | Range | Session | Expired |
|----|------|--------|-------|---------|---------|
| res-003 | TST | 30105 | 001-100 | session-old | 2時間前 |
| res-007 | REQ | 30201 | 051-100 | session-old | 1日前 |

2件の期限切れ予約を削除しますか？ (y/n)

> y

✅ 削除完了
- res-003: deleted
- res-007: deleted

Total cleaned: 2
```

### 未使用予約の検出

```
ユーザー: /id-reserve-cleanup --unused

Claude:
未使用予約を検出中...

⚠️  未使用予約（アクティブだがIDを1つも使っていない）:

| ID | Type | Folder | Range | Session | Created |
|----|------|--------|-------|---------|---------|
| res-008 | BHV | 30301 | 001-050 | auto-12347-stage | 3時間前 |
| res-009 | TST | 30301 | 001-100 | auto-12347-stage | 3時間前 |

これらを解放しますか？ (y/n)

> y

✅ 解放完了
- res-008: released (0/50 used)
- res-009: released (0/100 used)

期限切れ予約:
| ID | Type | Folder | Range | Session | Expired |
|----|------|--------|-------|---------|---------|
| res-003 | TST | 30105 | 001-100 | session-old | 2時間前 |

1件の期限切れ予約を削除しますか？ (y/n)

> y

✅ 削除完了
Total: 2 released, 1 deleted
```

### ドライラン

```
ユーザー: /id-reserve-cleanup --dry-run

Claude:
[DRY-RUN] 期限切れ予約を検出中...

削除対象（実際には削除しません）:
| ID | Type | Folder | Range | Session | Expired |
|----|------|--------|-------|---------|---------|
| res-003 | TST | 30105 | 001-100 | session-old | 2時間前 |
| res-007 | REQ | 30201 | 051-100 | session-old | 1日前 |

Would delete: 2 reservations

実際に削除するには --dry-run を外して実行してください。
```

### 強制削除

```
ユーザー: /id-reserve-cleanup --force

Claude:
期限切れ予約を削除中（確認なし）...

✅ res-003: deleted (expired 2h ago)
✅ res-007: deleted (expired 1d ago)

Total cleaned: 2
```

## エラーハンドリング

| 条件 | メッセージ |
|------|-----------|
| 対象なし | `No expired reservations found. Nothing to clean up.` |
| 予約ファイルなし | `.id-reservations.yml not found. Nothing to clean up.` |
| 書き込み権限なし | `Cannot write to .id-reservations.yml. Check permissions.` |

## 推奨実行タイミング

- セッション終了時
- 1日の終わり
- 並列セッション開始前

## 関連コマンド

- `/id-reserve` - ID範囲を予約
- `/id-reserve-status` - 予約状況一覧
- `/id-reserve-release` - 予約解放
- `/id-reserve-extend` - 予約延長
- `/session-merge` - セッションマージ（自動でクリーンアップを実行）
