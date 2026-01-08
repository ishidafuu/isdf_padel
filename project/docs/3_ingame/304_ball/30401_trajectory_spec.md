# Ball Trajectory Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

ボールの3D軌道を物理演算で制御します。放物線運動（重力の影響）を含みます。

## Core Requirements (MVP v0.1)

### REQ-30401-001: 放物線運動
**WHEN** ボールが空中にある
**THE SYSTEM SHALL** ボールに重力加速度を適用する
- `velocityY += config.Physics.Gravity * deltaTime`
- Gravity: デフォルト -9.8 m/s²

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#physics-config)
**テスト**: TST-30404-001

---

### REQ-30401-002: 初期速度設定
**WHEN** ボールがプレイヤーから発射される
**THE SYSTEM SHALL** ボールの初期速度を設定する
- 通常ショット: `config.Ball.NormalShotSpeed` (デフォルト: 10.0 m/s)
- 強打: `config.Ball.PowerShotSpeed` (デフォルト: 15.0 m/s)

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#ball-config)
**テスト**: TST-30404-002

---

### REQ-30401-003: 位置更新
**WHEN** 物理演算フレーム（`_PhysicsProcess`）が実行される
**THE SYSTEM SHALL** ボール位置を更新する
- `Position += Velocity * deltaTime`

**テスト**: TST-30404-003

---

### REQ-30401-004: 速度更新（重力適用）
**WHEN** 物理演算フレーム（`_PhysicsProcess`）が実行される
**AND** ボールが空中にある
**THE SYSTEM SHALL** 速度を更新する
- `Velocity.Y += config.Physics.Gravity * deltaTime`

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#physics-config)
**テスト**: TST-30404-004

---

### REQ-30401-005: コート範囲チェック
**WHEN** ボールが移動する
**THE SYSTEM SHALL** ボールがコート境界内にあるか確認する
- X範囲: `-config.Court.Width/2` ～ `+config.Court.Width/2`
- Z範囲: `-config.Court.Depth/2` ～ `+config.Court.Depth/2`

**参照**: [30501_court_spec.md](../305_court/30501_court_spec.md)
**テスト**: TST-30404-005

---

### REQ-30401-006: アウトオブバウンズ検出
**WHEN** ボールが地面（Y < 0）に落下した
**THE SYSTEM SHALL** `BallOutOfBoundsEvent` を発行する
- イベントデータ：Position（最終位置）

**テスト**: TST-30404-006

---

## Extended Requirements (v0.2)

### REQ-30401-050: スピン効果
**WHEN** ボールにスピンが設定されている
**THE SYSTEM SHALL** 軌道にマグヌス効果を適用する
- スピン係数: `config.Ball.SpinFactor`

**テスト**: TST-30404-050

### REQ-30401-051: 風の影響
**WHEN** 風が設定されている
**THE SYSTEM SHALL** ボールに風の力を適用する
- 風ベクトル: `config.Environment.WindVector`

**テスト**: TST-30404-051

---

## v0.3 Requirements (Spin Trajectory Effects)

### REQ-30401-100: スピンによる重力変動
**WHEN** ボールにスピンが設定されている（BallSpin.value ≠ 0）
**THE SYSTEM SHALL** スピン値に応じて実効重力を変動させる
- 計算式: `effective_gravity = base_gravity * (1.0 + spin * gravity_spin_factor)`
- トップスピン（spin > 0）: 重力増加 → 早く落ちる
- スライス（spin < 0）: 重力減少 → 浮く
- gravity_spin_factor: `config.spin_physics.gravity_spin_factor` (デフォルト: 0.3)

**計算例**（base_gravity = -4.0, factor = 0.3）:
| spin値 | 実効重力 | 効果 |
|--------|---------|------|
| +1.0（最大トップ）| -5.2 m/s² | 早く落ちる |
| 0（ニュートラル）| -4.0 m/s² | 変化なし |
| -1.0（最大スライス）| -2.8 m/s² | 浮く |

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#spin-physics-config)
**テスト**: TST-30404-100

---

### REQ-30401-101: スピン時間減衰
**WHEN** ボールが飛行中
**THE SYSTEM SHALL** スピン効果を時間経過で減衰させる
- 計算式: `ball_spin.value *= (1.0 - spin_decay_rate * deltaTime).max(0.0)`
- spin_decay_rate: `config.spin_physics.spin_decay_rate` (デフォルト: 0.5)
- 1秒あたり50%減衰（2秒後に元の25%）

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#spin-physics-config)
**テスト**: TST-30404-101

---

### REQ-30401-102: スピンによる空気抵抗
**WHEN** ボールにスピンが設定されている
**THE SYSTEM SHALL** スピン絶対値に応じて空気抵抗を増加させる
- 計算式: `drag = base_air_drag + spin.abs() * spin_drag_factor`
- 速度減衰: `velocity *= (1.0 - drag * deltaTime).max(0.9)`
- base_air_drag: `config.spin_physics.base_air_drag` (デフォルト: 0.0)
- spin_drag_factor: `config.spin_physics.spin_drag_factor` (デフォルト: 0.3)

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#spin-physics-config)
**テスト**: TST-30404-102

---

## Future Requirements (v0.4+)

### REQ-30401-150: 軌道エフェクト
**WHEN** ボールが飛んでいる
**THE SYSTEM SHALL** 軌跡エフェクトを表示する

**テスト**: TST-30404-150

---

## 制約（Design by Contract）

### 事前条件
- ボール初期化時、Position.Y ≥ 0
- Gravity値は負の値（下向き加速度）

### 事後条件
- 毎フレーム後、Position.Y ≥ 0（地面貫通禁止）

### 不変条件
- ボールは常に1つのみ存在（複数ボール禁止）
- ベース重力は一定（スピンによる変動は許容）

---

## データ参照

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| 重力 | config.Physics.Gravity | -9.8 m/s² |
| 通常ショット速度 | config.Ball.NormalShotSpeed | 10.0 m/s |
| 強打速度 | config.Ball.PowerShotSpeed | 15.0 m/s |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - Ball, Physics パラメータ
- [30501_court_spec.md](../305_court/30501_court_spec.md) - コート座標系
- [30402_reflection_spec.md](30402_reflection_spec.md) - バウンド処理（地面到達時）
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - イベント定義

---

## 備考

- 放物線運動は物理基本原則に基づく独自定義
- 最大速度制限は現時点では実装しない（将来の拡張として検討）
