# Fault Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-25

## 概要

審判システムにおけるフォールト判定（サーブエリア外、ダブルフォルト、Faultカウンタ管理）の仕様を定義します。

## 依存関係

- `304_ball`: ボール着地位置
- `305_court`: サーブエリア定義
- `306_serve`: サーブ実行イベント
- `307_scoring`: PointEvent発行

## Core Requirements (MVP v0.1)

### REQ-30902-001: サーブエリア外判定
- WHEN サーブがサーブエリア外に着地した
- THE SYSTEM SHALL フォルトと判定する
- AND Faultカウンタをインクリメントする
- WITH 判定基準: サーブエリア境界（ServiceBox定義）
- WITH サーブエリア:
  - デュース側: 左側サービスボックス
  - アド側: 右側サービスボックス
- **テスト**: TST-30904-010
- **データ**: `80101_game_constants.md#serve_config`
- **参考**: [90114_scoring.md](../../9_reference/901_reference_game/mechanics/90114_scoring.md)（★★★★★）

### REQ-30902-002: ダブルフォルト判定
- WHEN Faultカウンタが2に達した
- THE SYSTEM SHALL ダブルフォルトと判定する
- AND PointEvent を発行する（サーバーが失点）
- AND Faultカウンタを0にリセットする
- **テスト**: TST-30904-011
- **データ**: `80101_game_constants.md#serve_config`
- **参考**: [90114_scoring.md](../../9_reference/901_reference_game/mechanics/90114_scoring.md)（★★★★★）

### REQ-30902-003: Faultカウンタ管理
- WHEN ポイントが開始される
- THE SYSTEM SHALL Faultカウンタを0に初期化する
- WHEN フォルトが発生した
- THE SYSTEM SHALL Faultカウンタをインクリメントする（最大2）
- WITH 状態遷移: 0 → 1（1st Fault）→ 2（Double Fault → 失点）
- **テスト**: TST-30904-012
- **データ**: `80101_game_constants.md#serve_config`

---

## Extended Requirements (v0.2)

### REQ-30902-050: フット・フォルト判定
- WHEN サーブ時にサーバーがベースラインを踏んだ
- THE SYSTEM SHALL フット・フォルトと判定する
- AND Faultカウンタをインクリメントする
- WITH 判定基準: サーバーの足の位置がベースラインを超えていない
- **テスト**: TST-30904-060
- **データ**: `80101_game_constants.md#serve_config`

### REQ-30902-051: レット時のFault継続
- WHEN レット（ネットイン）が発生した
- AND すでにフォルトが1回発生していた
- THE SYSTEM SHALL Faultカウントを継続する（リセットしない）
- AND サーバーは再度セカンドサーブを打つ
- **テスト**: TST-30904-061
- **データ**: `80101_game_constants.md#serve_config`

---

## Constraints（Design by Contract）

### Preconditions
- サーブエリア（ServiceBox）が定義済み
- Faultカウンタが初期化済み（0）
- ポイント開始時にカウンタがリセットされる

### Postconditions
- フォルト判定後、Faultカウンタが正しく更新される
- ダブルフォルト時、PointEventが発行される
- ポイント終了後、Faultカウンタが0にリセットされる

### Invariants
- Faultカウンタは常に 0, 1, 2 のいずれか
- ダブルフォルト時は即座にポイント終了
