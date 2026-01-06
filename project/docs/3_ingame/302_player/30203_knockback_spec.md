# Player Knockback Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

プレイヤーがボールに被弾した際のふっとばし動作、無敵時間、操作不能時間を定義します。ふっとばし中はプレイヤーの入力を受け付けず、一定時間経過後に操作可能になります。

## Core Requirements (MVP v0.1)

### REQ-30203-001: ふっとばし開始
**WHEN** プレイヤーがボールに被弾する（BallHitEvent を受信）
**THE SYSTEM SHALL** ふっとばし処理を開始する
- ふっとばし方向：ボール→プレイヤーの単位ベクトル
- ふっとばし速度：`ボール速度 * config.Knockback.SpeedMultiplier` (デフォルト: 0.5)
- Knockback.IsActive を true に設定する
- Knockback.Duration を `config.Knockback.Duration` に設定する (デフォルト: 0.5秒)
- Knockback.InvincibilityTime を `config.Knockback.InvincibilityTime` に設定する (デフォルト: 1.0秒)

**テスト**: TST-30204-015
**イベント**: PlayerKnockbackEvent

---

### REQ-30203-002: ふっとばし移動
**WHILE** Knockback.IsActive == true
**THE SYSTEM SHALL** プレイヤーをふっとばしベクトル方向に移動させる
- `position += knockbackVelocity * deltaTime`

**テスト**: TST-30204-016

---

### REQ-30203-003: ふっとばし中の境界制限
**WHEN** ふっとばし移動がコート境界を超える
**THE SYSTEM SHALL** 境界位置で停止する
**AND** ふっとばしベクトルの該当成分を 0 にリセットする

**参照**: [30503_boundary_behavior.md](../305_court/30503_boundary_behavior.md)
**テスト**: TST-30204-017

---

### REQ-30203-004: ふっとばし終了
**WHEN** Knockback.Duration が 0 以下になる
**THE SYSTEM SHALL** ふっとばし状態を終了する
- Knockback.IsActive を false に設定する
- ふっとばしベクトルを (0, 0, 0) にリセットする

**テスト**: TST-30204-018

---

### REQ-30203-005: 無敵時間の管理
**WHILE** Knockback.InvincibilityTime > 0
**THE SYSTEM SHALL** 無敵時間を毎フレーム減算する
- `InvincibilityTime -= deltaTime`
- ボールの衝突判定を無視する

**テスト**: TST-30204-019

---

### REQ-30203-006: 操作不能時間の管理
**WHILE** Knockback.Duration > 0
**THE SYSTEM SHALL** プレイヤーの入力を無視する

**テスト**: TST-30204-020

---

### REQ-30203-007: PlayerKnockbackEvent の発行
**WHEN** ふっとばしが開始される
**THE SYSTEM SHALL** PlayerKnockbackEvent を発行する
- イベントデータ：
  - `PlayerId: int` - プレイヤーID
  - `KnockbackVelocity: Vector3` - ふっとばしベクトル
  - `Duration: float` - ふっとばし時間
  - `InvincibilityTime: float` - 無敵時間

**参照**: [20005_event_system.md](../../2_architecture/20005_event_system.md)
**テスト**: TST-30204-021

---

### REQ-30203-008: ふっとばし中の重力適用
**WHILE** Knockback.IsActive == true
**AND** プレイヤーが空中にいる（IsGrounded == false）
**THE SYSTEM SHALL** Y軸速度に重力を毎フレーム加算する
- `velocityY += config.Physics.Gravity * deltaTime`

**参照**: [30202_jump_spec.md](30202_jump_spec.md)
**テスト**: TST-30204-022

---

## Extended Requirements (v0.2)

### REQ-30203-050: ふっとばし方向の調整
**WHEN** ふっとばしが発生する
**THE SYSTEM SHALL** ボールの角度に応じてふっとばし方向を調整する
- 垂直方向成分を追加

**テスト**: TST-30204-050

### REQ-30203-051: ダメージ蓄積
**WHEN** プレイヤーが被弾する
**THE SYSTEM SHALL** ダメージ量を蓄積する
**AND** ダメージ量に応じてふっとばし距離を増加させる

**テスト**: TST-30204-051

---

## Future Requirements (v0.3+)

### REQ-30203-100: ふっとばしアニメーション
**WHEN** ふっとばしが発生する
**THE SYSTEM SHALL** ふっとばしモーションのアニメーションを再生する

**テスト**: TST-30204-100

---

## 制約（Design by Contract）

### 事前条件
- PlayerComponent が存在する
- TransformComponent が存在する
- KnockbackComponent が存在する
- ボールとの衝突が検出されている
- GameConfig が読み込まれている

### 事後条件
- ふっとばし開始時、Knockback.IsActive が true になる
- ふっとばし終了時、Knockback.IsActive が false になる
- 無敵時間中、ボールの衝突判定が無効になる
- 操作不能時間中、プレイヤーの入力が無視される
- PlayerKnockbackEvent が発行される

### 不変条件
- `Knockback.InvincibilityTime >= 0`
- `Knockback.Duration >= 0`
- プレイヤーは常にコート境界内に留まる

---

## データ参照

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| ふっとばし時間 | config.Knockback.Duration | 0.5秒 |
| 速度倍率 | config.Knockback.SpeedMultiplier | 0.5 |
| 無敵時間 | config.Knockback.InvincibilityTime | 1.0秒 |
| 重力加速度 | config.Physics.Gravity | -9.8 m/s² |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先
- ボール衝突システム（304_ball）- ボール衝突イベントを受信
- [30501_court_spec.md](../305_court/30501_court_spec.md) - コート座標系を使用
- [30503_boundary_behavior.md](../305_court/30503_boundary_behavior.md) - 境界判定に従う
- [30202_jump_spec.md](30202_jump_spec.md) - ふっとばし中も重力適用
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - イベント発行
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - Knockback, Physics パラメータを参照

---

## 備考

- ふっとばし中もコート境界を越えない
- ふっとばし中も重力が適用される（空中での被弾も考慮）
- 無敵時間中は視覚的なフィードバック（点滅など）が必要（UI担当）
- 操作不能時間は無敵時間より短い（無敵時間終了前に操作可能になる）
