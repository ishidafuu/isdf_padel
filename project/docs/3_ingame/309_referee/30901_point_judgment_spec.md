# Point Judgment Specification

**Version**: 2.0.0
**Status**: Draft
**Last Updated**: 2026-01-08

## 概要

審判システムにおける得点判定（アウト、ツーバウンド、ネットイン）の仕様を定義します。
テニスルールに基づき、アウト判定が主要な失点条件となります。

## 依存関係

- `304_ball`: ボール状態（位置、バウンド回数、ネット接触）
- `305_court`: コート境界、サーブエリア定義
- `307_scoring`: PointEvent発行

## 失点条件の優先順位

テニスルールに基づく失点条件の優先順位：

1. **アウト**: ボールがコート外に着地（主要失点条件）
2. **ツーバウンド**: 自コートで2回バウンド
3. **ネットイン**: ボールがネットを越えなかった

## Core Requirements (MVP v0.1)

### REQ-30901-001: アウト判定
- WHEN ボールがコート外に着地した
- THE SYSTEM SHALL アウトと判定する
- AND PointEvent を発行する（打った側が失点）
- WITH 判定座標: ボールの着地位置
- WITH 判定基準: CourtBounds定義のコート境界
  - サイドアウト: |X| > `config.Court.Width / 2`
  - ベースラインアウト: |Z| > `config.Court.Depth / 2`
- **テスト**: TST-30904-001
- **データ**: `80101_game_constants.md#court_config`
- **参照**: [30501_court_spec.md#REQ-30501-007](../305_court/30501_court_spec.md#req-30501-007-アウト境界)

### REQ-30901-002: ツーバウンド判定
- WHEN ボールがコート内で2回バウンドした
- AND 該当プレイヤーが打ち返していない
- THE SYSTEM SHALL ツーバウンドと判定する
- AND PointEvent を発行する（該当プレイヤーが失点）
- WITH 判定条件: Ball.BounceCount >= 2
- **テスト**: TST-30904-002
- **データ**: `80101_game_constants.md#ball_config`

### REQ-30901-003: ネットイン判定（サーブ時）
- WHEN サーブがネットに触れて相手コートに入った
- THE SYSTEM SHALL レットと判定する
- AND レットイベントを発行する（再サーブ）
- WITH 判定条件: Ball.TouchedNet == true AND Ball.InOpponentCourt == true
- **テスト**: TST-30904-003
- **データ**: `80101_game_constants.md#serve_config`

### REQ-30901-004: コート境界取得
- WHEN システムが初期化される
- THE SYSTEM SHALL CourtBoundsからコート境界座標を取得する
- AND 境界座標をキャッシュする
- WITH 取得対象:
  - シングルスラインX座標（サイドライン）
  - ベースラインZ座標
  - ネット位置
- **テスト**: TST-30904-004
- **データ**: `80101_game_constants.md#court_config`

### REQ-30901-005: ネットフォルト判定
- WHEN ボールがネットに当たり、相手コートに入らなかった
- THE SYSTEM SHALL ネットフォルトと判定する
- AND PointEvent を発行する（打った側が失点）
- WITH 判定条件: Ball.TouchedNet == true AND Ball.CrossedNet == false
- **テスト**: TST-30904-005
- **データ**: `80101_game_constants.md#court_config`

### REQ-30901-006: 壁（フェンス）ヒット判定
- WHEN ボールが壁（フェンス）に当たった
- THE SYSTEM SHALL アウトと判定する
- AND RallyEndEvent を発行する（打った側が失点）
- WITH 判定条件: WallReflectionEvent 受信時
- WITH 得点者: LastShooter の相手側
- **テスト**: TST-30036-001, TST-30036-002
- **備考**: テニスルールでは壁に当たった時点でインプレー終了

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

---

## Change Log

### 2026-01-08 - v2.0.0（テニスへ変更）

- **アウト判定強調**: 主要失点条件としてREQ-30901-001を強調
- **REQ-30901-005追加**: ネットフォルト判定
- **失点条件の優先順位**: テニスルールに基づく優先順位を明記
- **参照更新**: 30501_court_spec.md の新しいREQ-30501-007を参照

### 2025-12-25 - v1.0.0（初版）

- 初版作成
