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

## Future Requirements (v0.3+)

### REQ-30401-100: 軌道エフェクト
**WHEN** ボールが飛んでいる
**THE SYSTEM SHALL** 軌跡エフェクトを表示する

**テスト**: TST-30404-100

---

## 制約（Design by Contract）

### 事前条件
- ボール初期化時、Position.Y ≥ 0
- Gravity値は負の値（下向き加速度）

### 事後条件
- 毎フレーム後、Position.Y ≥ 0（地面貫通禁止）

### 不変条件
- ボールは常に1つのみ存在（複数ボール禁止）
- 重力は常に一定値（途中変更禁止）

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
