# Ball Reflection Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

ボールの地面バウンド、壁・天井反射を定義します。反射時にバウンド係数を適用し、エネルギーを減衰させます。

## Core Requirements (MVP v0.1)

### REQ-30402-001: 地面バウンド
**WHEN** ボールが地面（Y = 0）に接触した
**THE SYSTEM SHALL** ボールの垂直速度（Velocity.Y）を反転させる
- `Velocity.Y = -Velocity.Y * config.Ball.BounceFactor`
- BounceFactor: デフォルト 0.8

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#ball-config)
**テスト**: TST-30404-007

---

### REQ-30402-002: バウンドイベント発行
**WHEN** ボールが地面にバウンドした
**THE SYSTEM SHALL** `BallBounceEvent` を発行する
- イベントデータ：Position（バウンド位置）, Velocity（バウンド後速度）

**テスト**: TST-30404-008

---

### REQ-30402-003: 壁反射（X軸）
**WHEN** ボールが壁（X = ±Court.Width/2）に到達した
**THE SYSTEM SHALL** ボールの水平速度（Velocity.X）を反転させる
- `Velocity.X = -Velocity.X * config.Ball.BounceFactor`

**参照**: [30502_wall_design.md](../305_court/30502_wall_design.md)
**テスト**: TST-30404-009

---

### REQ-30402-004: 壁反射イベント発行
**WHEN** ボールが壁に反射した
**THE SYSTEM SHALL** `WallReflectionEvent` を発行する
- イベントデータ：Position（反射位置）, WallType（壁種類）, Velocity（反射後速度）

**テスト**: TST-30404-010

---

### REQ-30402-005: 奥壁反射（Z軸）
**WHEN** ボールが奥壁（Z = ±Court.Depth/2）に到達した
**THE SYSTEM SHALL** ボールの前後速度（Velocity.Z）を反転させる
- `Velocity.Z = -Velocity.Z * config.Ball.BounceFactor`

**テスト**: TST-30404-011

---

### REQ-30402-006: 天井反射
**WHEN** ボールが天井（Y = Court.CeilingHeight）に到達した
**THE SYSTEM SHALL** ボールの垂直速度（Velocity.Y）を反転させる
- `Velocity.Y = -Velocity.Y * config.Ball.BounceFactor`

**参照**: [30501_court_spec.md](../305_court/30501_court_spec.md#req-30501-004)
**テスト**: TST-30404-012

---

### REQ-30402-007: 壁反射時のめり込み防止
**WHEN** ボールが壁に反射する
**THE SYSTEM SHALL** ボールの位置を壁境界内に補正する
- X軸: `Position.X = Clamp(Position.X, -Court.Width/2, +Court.Width/2)`
- Z軸: `Position.Z = Clamp(Position.Z, -Court.Depth/2, +Court.Depth/2)`

**参照**: [30502_wall_design.md](../305_court/30502_wall_design.md)
**テスト**: TST-30404-013

---

## Extended Requirements (v0.2)

### REQ-30402-050: 回転による反射変化
**WHEN** ボールがスピン状態で壁に反射する
**THE SYSTEM SHALL** スピンに応じて反射角度を変化させる

**テスト**: TST-30404-050

### REQ-30402-051: 摩擦効果
**WHEN** ボールが地面にバウンドする
**THE SYSTEM SHALL** 摩擦により水平速度を減衰させる
- 摩擦係数: `config.Ball.FrictionFactor`

**テスト**: TST-30404-051

---

## v0.3 Requirements (Spin Bounce Effects)

### REQ-30402-100: スピンによるバウンド挙動変化
**WHEN** ボールがスピン状態（BallSpin.value ≠ 0）で地面にバウンドする
**THE SYSTEM SHALL** スピン値に応じてバウンド挙動を変化させる
- 水平方向（X, Z）: `velocity *= base_bounce * (1.0 + spin * bounce_spin_horizontal_factor)`
- 垂直方向（Y）: `velocity.y = -velocity.y * base_bounce * (1.0 - spin * bounce_spin_vertical_factor)`
- bounce_spin_horizontal_factor: `config.spin_physics.bounce_spin_horizontal_factor` (デフォルト: 0.3)
- bounce_spin_vertical_factor: `config.spin_physics.bounce_spin_vertical_factor` (デフォルト: 0.2)

**計算例**（base_bounce = 0.7, h_factor = 0.3, v_factor = 0.2）:
| spin値 | 水平維持率 | 垂直維持率 | 挙動 |
|--------|----------|----------|------|
| +1.0（トップ）| 91% (0.7 × 1.3) | 56% (0.7 × 0.8) | 低く伸びる |
| 0（ニュートラル）| 70% (0.7 × 1.0) | 70% (0.7 × 1.0) | 通常 |
| -1.0（スライス）| 49% (0.7 × 0.7) | 84% (0.7 × 1.2) | 高く止まる |

**挙動説明**:
- **トップスピン（spin > 0）**: 水平方向維持率上昇、垂直方向低下 → 低く伸びるバウンド
- **スライス（spin < 0）**: 水平方向維持率低下、垂直方向上昇 → 高く止まるバウンド

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#spin-physics-config)
**テスト**: TST-30404-103

---

## Future Requirements (v0.4+)

### REQ-30402-150: 反射エフェクト
**WHEN** ボールが壁に反射する
**THE SYSTEM SHALL** 反射エフェクトを表示する

**テスト**: TST-30404-150

---

## 制約（Design by Contract）

### 事前条件
- バウンド係数は 0.0 ～ 1.0 の範囲
- 壁反射前、ボールは境界付近にある
- 天井反射前、ボールは天井高度付近にある

### 事後条件
- 反射後、速度は減衰（エネルギー保存則違反なし）
- 反射後、ボールは境界内に位置補正される

### 不変条件
- バウンド係数は常に一定（途中変更禁止）
- 反射イベントは1回のバウンドにつき1回のみ発行

---

## データ参照

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| バウンド係数 | config.Ball.BounceFactor | 0.8 |
| コート幅 | config.Court.Width | 10.0 m |
| コート奥行き | config.Court.Depth | 6.0 m |
| 天井高さ | config.Court.CeilingHeight | 5.0 m |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - Ball, Court パラメータ
- [30502_wall_design.md](../305_court/30502_wall_design.md) - 壁反射設計
- [30503_boundary_behavior.md](../305_court/30503_boundary_behavior.md) - 境界判定
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - イベント定義

---

## 備考

- 反射係数は物理基本原則に基づく独自定義
- 壁反射設計（30502）に従い、めり込み防止を実装
