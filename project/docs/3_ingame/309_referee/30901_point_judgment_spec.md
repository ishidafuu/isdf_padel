# Point Judgment Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-25

## 概要

審判システムにおける得点判定（アウト、ツーバウンド、ネットイン）の仕様を定義します。

## 依存関係

- `304_ball`: ボール状態（位置、バウンド回数、ネット接触）
- `305_court`: コート境界、サーブエリア定義
- `307_scoring`: PointEvent発行

## Core Requirements (MVP v0.1)

### REQ-30901-001: アウト判定
- WHEN ボールがコート外に着地した
- THE SYSTEM SHALL アウトと判定する
- AND PointEvent を発行する（打った側が失点）
- WITH 判定座標: ボールの着地位置
- WITH 判定基準: CourtBounds定義のコート境界
- **テスト**: TST-30904-001
- **データ**: `80101_game_constants.md#court_config`
- **参考**: [90114_scoring.md](../../9_reference/901_reference_game/mechanics/90114_scoring.md)（★★★★★）

### REQ-30901-002: ツーバウンド判定
- WHEN ボールがコート内で2回バウンドした
- AND 該当プレイヤーが打ち返していない
- THE SYSTEM SHALL ツーバウンドと判定する
- AND PointEvent を発行する（該当プレイヤーが失点）
- WITH 判定条件: Ball.BounceCount >= 2
- **テスト**: TST-30904-002
- **データ**: `80101_game_constants.md#ball_config`
- **参考**: [90114_scoring.md](../../9_reference/901_reference_game/mechanics/90114_scoring.md)（★★★★★）

### REQ-30901-003: ネットイン判定（サーブ時）
- WHEN サーブがネットに触れて相手コートに入った
- THE SYSTEM SHALL レットと判定する
- AND レットイベントを発行する（再サーブ）
- WITH 判定条件: Ball.TouchedNet == true AND Ball.InOpponentCourt == true
- **テスト**: TST-30904-003
- **データ**: `80101_game_constants.md#serve_config`
- **参考**: [90114_scoring.md](../../9_reference/901_reference_game/mechanics/90114_scoring.md)（★★★★★）

### REQ-30901-004: コート境界取得
- WHEN システムが初期化される
- THE SYSTEM SHALL CourtBoundsからコート境界座標を取得する
- AND 境界座標をキャッシュする
- WITH 取得対象:
  - シングルスラインX座標
  - ベースラインZ座標
  - ネット位置
- **テスト**: TST-30904-004
- **データ**: `80101_game_constants.md#court_config`

---

## Extended Requirements (v0.2)

### REQ-30901-050: レット判定（詳細）
- WHEN サーブがネットに触れて相手サービスボックスに入った
- THE SYSTEM SHALL レットと判定する
- AND サーバーは同じサーブを再度打つ
- WITH Faultカウントは継続（フォルト後のレットは再フォルト）
- **テスト**: TST-30904-050
- **データ**: `80101_game_constants.md#serve_config`

### REQ-30901-051: タッチネット判定
- WHEN プレイヤーがネットに触れた
- THE SYSTEM SHALL タッチネットと判定する
- AND PointEvent を発行する（該当プレイヤーが失点）
- WITH 判定条件: Player.TouchedNet == true
- **テスト**: TST-30904-051

### REQ-30901-052: キャラクター当たり判定
- WHEN ボールがプレイヤーに当たった
- AND ボールが自陣に着地していない
- THE SYSTEM SHALL プレイヤーの失点と判定する
- AND PointEvent を発行する
- **テスト**: TST-30904-052

---

## Constraints（Design by Contract）

### Preconditions
- CourtBounds が初期化済み
- Ball の状態（位置、バウンド回数、ネット接触）が取得可能
- Scoring System が PointEvent を受信可能

### Postconditions
- 判定結果に基づいて PointEvent が発行される
- レット判定時は再サーブが可能な状態になる

### Invariants
- コート境界は試合中変更されない
- 判定は即座に行われる（遅延なし）
