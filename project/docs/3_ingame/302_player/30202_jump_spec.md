# Player Jump Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

プレイヤーのジャンプ動作と重力の適用、着地判定を定義します。ジャンプは地上でのみ可能で、重力により自然に落下します。

## Core Requirements (MVP v0.1)

### REQ-30202-001: ジャンプ開始
**WHEN** プレイヤーがジャンプボタン（Space キーまたはXボタン）を押す
**AND** プレイヤーが接地している（IsGrounded == true）
**THE SYSTEM SHALL** プレイヤーのY軸速度を設定する
- Y軸速度 = `config.Player.JumpSpeed` (デフォルト: 8.0 m/s、上向き)
- IsGrounded を false に変更する

**テスト**: TST-30204-007
**イベント**: PlayerJumpEvent

---

### REQ-30202-002: ジャンプ中の重力適用
**WHILE** プレイヤーが空中にいる（IsGrounded == false）
**THE SYSTEM SHALL** Y軸速度に重力を毎フレーム加算する
- `velocityY += config.Physics.Gravity * deltaTime`
- Gravity: デフォルト -9.8 m/s²

**テスト**: TST-30204-008

---

### REQ-30202-003: 着地判定
**WHEN** プレイヤーのY座標が地面（Y = 0）以下になる
**THE SYSTEM SHALL** 着地処理を実行する
- Y座標を 0 に補正する
- Y軸速度を 0 にリセットする
- IsGrounded を true に変更する

**テスト**: TST-30204-009
**イベント**: PlayerLandEvent

---

### REQ-30202-004: 空中ジャンプの禁止
**WHEN** プレイヤーがジャンプボタンを押す
**AND** プレイヤーが空中にいる（IsGrounded == false）
**THE SYSTEM SHALL** ジャンプ入力を無視する

**テスト**: TST-30204-010

---

### REQ-30202-005: 天井衝突時のY軸速度リセット
**WHEN** プレイヤーのY座標が天井に到達する
**AND** Y軸速度が正（上方向）である
**THE SYSTEM SHALL** Y軸速度を 0 にリセットする
- Y座標を `config.Court.CeilingHeight` に補正する

**テスト**: TST-30204-011

---

### REQ-30202-006: ふっとばし中のジャンプ禁止
**WHEN** プレイヤーがふっとばし状態である（Knockback.IsActive == true）
**THE SYSTEM SHALL** ジャンプ入力を無視する

**参照**: [30203_knockback_spec.md](30203_knockback_spec.md)
**テスト**: TST-30204-012

---

### REQ-30202-007: PlayerJumpEvent の発行
**WHEN** プレイヤーがジャンプを開始する
**THE SYSTEM SHALL** PlayerJumpEvent を発行する
- イベントデータ：
  - `PlayerId: int` - プレイヤーID
  - `JumpVelocity: float` - ジャンプ初速度

**参照**: [20005_event_system.md](../../2_architecture/20005_event_system.md)
**テスト**: TST-30204-013

---

### REQ-30202-008: PlayerLandEvent の発行
**WHEN** プレイヤーが着地する
**THE SYSTEM SHALL** PlayerLandEvent を発行する
- イベントデータ：
  - `PlayerId: int` - プレイヤーID
  - `LandPosition: Vector3` - 着地位置

**参照**: [20005_event_system.md](../../2_architecture/20005_event_system.md)
**テスト**: TST-30204-014

---

## Extended Requirements (v0.2)

### REQ-30202-050: 2段ジャンプ
**WHEN** プレイヤーが空中でジャンプボタンを押す
**AND** 2段ジャンプ可能フラグが true である
**THE SYSTEM SHALL** 2段目のジャンプを実行する
- Y軸速度 = `config.Player.SecondJumpSpeed`

**テスト**: TST-30204-050

### REQ-30202-051: 長押しジャンプ
**WHEN** プレイヤーがジャンプボタンを長押しする
**THE SYSTEM SHALL** ジャンプ力を調整する
- 最大長押し時間: `config.Player.MaxJumpHoldTime`
- ジャンプ力: 長押し時間に比例

**テスト**: TST-30204-051

---

## Future Requirements (v0.3+)

### REQ-30202-100: 壁ジャンプ
**WHEN** プレイヤーが壁に接触しながらジャンプする
**THE SYSTEM SHALL** 壁から跳ね返るジャンプを実行する

**テスト**: TST-30204-100

---

## 制約（Design by Contract）

### 事前条件
- PlayerComponent が存在する
- TransformComponent が存在する
- VelocityComponent が存在する
- GameConfig が読み込まれている
- コート境界が定義されている

### 事後条件
- ジャンプ後、Y軸速度が正（上方向）になる
- 着地後、Y座標が 0 になる
- 着地後、IsGrounded が true になる
- ジャンプ・着地時にイベントが発行される

### 不変条件
- `0 <= position.Y <= config.Court.CeilingHeight`
- IsGrounded == true のとき、`position.Y == 0`
- IsGrounded == true のとき、`velocityY == 0`

---

## データ参照

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| ジャンプ初速度 | config.Player.JumpSpeed | 8.0 m/s |
| 重力加速度 | config.Physics.Gravity | -9.8 m/s² |
| 天井高さ | config.Court.CeilingHeight | 5.0 m |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先
- [30501_court_spec.md](../305_court/30501_court_spec.md) - コート座標系を使用
- [30203_knockback_spec.md](30203_knockback_spec.md) - ふっとばし中はジャンプ禁止
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - イベント発行
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - Player, Physics パラメータを参照

---

## 備考

- 空中制御（ジャンプ中の左右移動）は 30201_movement_spec.md で定義
- 2段ジャンプは将来の拡張として検討
- ジャンプ力の変動（長押し調整など）は現時点では実装しない
