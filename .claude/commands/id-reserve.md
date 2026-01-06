---
description: ID範囲を予約して衝突を防ぐ
argument-hint: <ID-type> <folder-number> <count>
---

# /id-reserve コマンド

並列セッション実行時にID範囲を事前に予約し、ID衝突を防ぎます。

**引数**: $ARGUMENTS（IDタイプ フォルダ番号 予約数）

## 使用者

**🔀 人間・エージェント共用コマンド**

### 人間の使用

並列セッション開始時にID範囲を予約する際に使用（推奨）

### エージェントの使用

| エージェント | 使用タイミング | 目的 |
|------------|--------------|------|
| session-manager-agent | セッション初期化時（将来実装） | ID範囲の自動予約 |

**注**: 現在は手動管理。将来的に /session-init で自動実行予定。

## 使用方法

```
/id-reserve REQ 30101 50
/id-reserve DES 30102 30
/id-reserve TST 30105 100
```

## 目的

**並列セッションでのID衝突を事前に防ぐ**

- 複数セッションが同じフォルダで作業する際のID重複を防止
- 自動的に次の空き範囲を予約
- 予約状況を `.id-reservations.yml` で管理

## 使用シナリオ

### シナリオ1: 新機能の仕様書作成開始

```bash
# Terminal 1: Player機能の仕様書作成
/id-reserve REQ 30101 50

✅ ID範囲予約完了
- Type: REQ
- Folder: 30101 (docs/3_ingame/301_player/)
- Range: REQ-30101-001 ～ REQ-30101-050
- Session: session-player-20251220
- Expires: 2025-12-20 18:00

この範囲内でREQ-IDを使用してください。
```

### シナリオ2: 追加予約

```bash
# 50個を使い切った場合
/id-reserve REQ 30101 50

✅ ID範囲予約完了（追加）
- Type: REQ
- Folder: 30101
- Range: REQ-30101-051 ～ REQ-30101-100
- Previous: 001-050 (used)
- Session: session-player-20251220
```

### シナリオ3: 並列セッションでの競合回避

```bash
# Terminal 1
/id-reserve REQ 30101 50
# → 001-050 を予約

# Terminal 2（同じフォルダで作業開始）
/id-reserve REQ 30101 50

⚠️ 既存の予約を検出しました

Reserved by: session-player-20251220
Range: REQ-30101-001 ～ 050
Created: 5 minutes ago

次の空き範囲を予約しますか？
→ REQ-30101-051 ～ 100

[y/n]: y

✅ ID範囲予約完了
- Range: REQ-30101-051 ～ REQ-30101-100
```

## 予約ファイル構造

`.id-reservations.yml` に予約情報を記録：

```yaml
reservations:
  - id: res-001
    type: REQ
    folder: 30101
    folder_path: docs/3_ingame/301_player/
    range_start: 1
    range_end: 50
    session: session-player-20251220
    created_at: 2025-12-20T10:00:00Z
    expires_at: 2025-12-20T18:00:00Z
    status: active

  - id: res-002
    type: DES
    folder: 30102
    folder_path: docs/3_ingame/301_player/
    range_start: 1
    range_end: 30
    session: session-player-20251220
    created_at: 2025-12-20T10:05:00Z
    expires_at: 2025-12-20T18:00:00Z
    status: active
```

## 指示

引数として渡された情報（`$ARGUMENTS`）から予約を実行します。

### Step 1: 引数のパース

```
$ARGUMENTS = "REQ 30101 50"
↓
type: REQ
folder: 30101
count: 50
```

### Step 2: フォルダパスの特定

5桁番号からフォルダパスを特定：

```
30101 → docs/3_ingame/301_player/
30102 → docs/3_ingame/301_player/
40101 → docs/4_outgame/401_title/
```

### Step 3: 既存の予約確認

`.id-reservations.yml` を読み込み、同じ folder + type の予約を検索：

```yaml
# 既存の予約がある場合
- type: REQ
  folder: 30101
  range_start: 1
  range_end: 50
  status: active
```

### Step 4: 実際の使用状況を確認

対象ファイル（`30101_player_spec.md`）を読み、実際に使われているIDを確認：

```
REQ-30101-001 ✅ 使用中
REQ-30101-002 ✅ 使用中
REQ-30101-003 ✅ 使用中
...
REQ-30101-045 ✅ 使用中
REQ-30101-046 ～ 050 ⚪ 予約済み（未使用）
```

### Step 5: 次の空き範囲を計算

```
最大使用ID: REQ-30101-045
予約済み範囲: 001-050
↓
次の空き範囲: 051 ～ (051 + count - 1)
→ REQ-30101-051 ～ 100
```

### Step 6: 予約を記録

`.id-reservations.yml` に新規予約を追加：

```yaml
- id: res-003
  type: REQ
  folder: 30101
  folder_path: docs/3_ingame/301_player/
  range_start: 51
  range_end: 100
  session: session-player-20251220
  created_at: 2025-12-20T10:30:00Z
  expires_at: 2025-12-20T18:00:00Z
  status: active
```

### Step 7: 結果を出力

```
✅ ID範囲予約完了
- Type: REQ
- Folder: 30101 (docs/3_ingame/301_player/)
- Range: REQ-30101-051 ～ REQ-30101-100
- Session: session-player-20251220
- Expires: 2025-12-20 18:00

予約ID: res-003

この範囲内でREQ-IDを使用してください。
使用状況は /id-reserve-status で確認できます。
```

## オプション

| オプション | 説明 |
|-----------|------|
| `--auto` | 自動的に次の空き範囲を予約（確認なし） |
| `--extend` | 既存の予約を拡張 |
| `--session <name>` | セッション名を指定 |
| `--expires <hours>` | 有効期限を指定（デフォルト: 8時間） |
| `--force` | 既存の予約を上書き（危険） |

## 予約の有効期限

### デフォルト

- 予約から **8時間** で自動失効
- 失効後は他のセッションが使用可能

### 延長

```bash
/id-reserve-extend res-003 4
# → 4時間延長
```

### 手動解放

```bash
/id-reserve-release res-003
# → 即座に解放
```

## 予約状況の確認

### 全予約の確認

```bash
/id-reserve-status

=== ID Reservations ===

[ACTIVE] res-001
- Type: REQ
- Folder: 30101
- Range: 001-050
- Session: session-player-20251220
- Created: 2 hours ago
- Expires: in 6 hours

[ACTIVE] res-002
- Type: DES
- Folder: 30102
- Range: 001-030
- Session: session-player-20251220
- Created: 1 hour ago
- Expires: in 7 hours

[EXPIRED] res-003
- Type: TST
- Folder: 30105
- Range: 001-100
- Session: session-player-20251219
- Expired: 2 hours ago

Total: 2 active, 1 expired
```

### 特定フォルダの確認

```bash
/id-reserve-status 30101

=== ID Reservations for 30101 ===

REQ: 001-050 (active)
DES: 001-030 (active)
BHV: (no reservation)
TST: 001-100 (expired)
```

## 自動クリーンアップ

### 期限切れ予約の削除

```bash
/id-reserve-cleanup

Cleaning up expired reservations...

✅ Removed res-003 (expired 2 hours ago)
✅ Removed res-005 (expired 1 day ago)

Total cleaned: 2
```

### 未使用予約の検出

```bash
/id-reserve-cleanup --unused

Checking for unused reservations...

⚠️ res-001: Reserved 50 IDs, used only 10
  → Consider releasing unused range

⚠️ res-004: Reserved 100 IDs, used 0
  → No IDs used, auto-releasing
```

## エラーハンドリング

### E001: 範囲が重複

```
ERROR E001: Range overlap detected
Type: REQ
Folder: 30101
Requested: 001-050
Existing: 001-050 (session-player-20251220)

Cannot reserve overlapping range.
Use --auto to get next available range.
```

**解決方法**:
```bash
/id-reserve REQ 30101 50 --auto
```

### E002: 範囲が大きすぎる

```
ERROR E002: Range too large
Type: REQ
Folder: 30101
Requested: 001-1000

Maximum range per reservation: 200
Consider splitting into multiple reservations.
```

**解決方法**:
```bash
/id-reserve REQ 30101 200
/id-reserve REQ 30101 200 --auto
...
```

### E003: フォルダが存在しない

```
ERROR E003: Folder not found
Folder: 30101
Expected path: docs/3_ingame/301_player/

The folder does not exist yet.
Create the folder first or use a valid folder number.
```

## CI/CD 統合

### GitHub Actions での使用

```yaml
name: Reserve IDs

on:
  workflow_dispatch:
    inputs:
      folder:
        description: 'Folder number'
        required: true
      count:
        description: 'Number of IDs'
        default: '50'

jobs:
  reserve:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Reserve IDs
        run: |
          claude-code /id-reserve REQ ${{ github.event.inputs.folder }} ${{ github.event.inputs.count }} --auto
      - name: Commit reservation
        run: |
          git add .id-reservations.yml
          git commit -m "chore: reserve IDs for ${{ github.event.inputs.folder }}"
          git push
```

## ベストプラクティス

### 1. セッション開始時に予約

```bash
# セッション開始
/session-init session-player 301_player

# すぐにID予約
/id-reserve REQ 30101 50
/id-reserve DES 30102 30
```

### 2. 適切な予約数

| ファイルタイプ | 推奨予約数 |
|--------------|-----------|
| spec.md (REQ) | 50-100 |
| design.md (DES) | 20-50 |
| behavior.md (BHV) | 30-80 |
| test.md (TST) | 100-200 |

### 3. 使い切る前に追加予約

```bash
# 残り10個になったら追加予約
/id-reserve REQ 30101 50 --auto
```

### 4. セッション終了時に未使用を解放

```bash
# セッション終了前
/id-reserve-status --unused
/id-reserve-release res-001
```

## セッション管理との連携

### 自動予約（将来実装）

`/session-init` 実行時に自動的に予約：

```bash
/session-init session-player 301_player

✅ セッション初期化完了
✅ ID自動予約完了
- REQ: 001-050
- DES: 001-030
- BHV: 001-050
- TST: 001-100
```

### 自動解放

`/session-merge` 実行時に未使用範囲を自動解放：

```bash
/session-merge session-player

Checking ID usage...
- REQ: used 45/50 → release 046-050
- DES: used 30/30 → all used
- TST: used 0/100 → release all

✅ 未使用ID範囲を解放しました
```

## トラブルシューティング

### Q: 予約したのに他のセッションが使った

**A**: 予約は「推奨」であり、強制ではありません。

実際のID衝突は `/docs-validate` で検出されます。

### Q: 予約を間違えた

**A**: 予約を削除してやり直してください。

```bash
/id-reserve-release res-001
/id-reserve REQ 30101 50
```

### Q: 自動予約が失敗する

**A**: 手動で範囲を指定してください。

```bash
/id-reserve REQ 30101 50 --auto
↓（失敗）
/id-reserve REQ 30101 51 100  # 手動で範囲指定
```

## 関連コマンド

- `/id` - IDの定義箇所を表示
- `/id-list` - ファイル内の全IDを一覧
- `/id-next` - 次の連番を取得
- `/docs-validate` - ID重複チェック
- `/session-init` - セッション初期化
- `/session-status` - セッション状況確認

## 設計意図

このコマンドは、**並列セッションでのID衝突を事前に防ぐ**ためのものです。

### なぜ必要か

1. **複数セッションが同じフォルダで作業**
   - Terminal 1: Player仕様書作成
   - Terminal 2: Playerテスト作成
   - → 同じREQ-IDを使う可能性

2. **ID衝突は後から修正が困難**
   - 仕様書、設計書、テスト、実装すべてに影響
   - 事前予約で回避

3. **自動化の前提**
   - 将来的にセッション管理を完全自動化
   - ID予約も自動化の一部

### 使用タイミング

- **セッション開始時**: 必ず予約
- **ID使用前**: 予約範囲内か確認
- **セッション終了時**: 未使用を解放

---

**重要**: このコマンドは推奨ですが、強制ではありません。
実際のID衝突は `/docs-validate` で最終確認されます。
