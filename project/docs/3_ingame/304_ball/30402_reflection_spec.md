# Ball Reflection Specification

**Version**: 2.0.0
**Status**: Draft
**Last Updated**: 2026-01-09

## 概要

ボールの地面バウンドおよびアウト境界動作を定義します。テニスはオープンコート（壁・天井なし）であり、コートライン外に落ちたボールはアウトとなります。

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

### REQ-30402-003: コートライン外アウト（X軸方向）
**WHEN** ボールが地面に着地し、位置が |X| > Court.Depth/2
**THE SYSTEM SHALL** アウト判定を行う
- ベースライン外（打ち合い方向）へのボールはアウト
- 壁反射は発生しない（オープンコート）

**参照**: [30501_court_spec.md](../305_court/30501_court_spec.md#req-30501-007)
**テスト**: TST-30404-009

---

### REQ-30402-004: コートライン外アウト（Z軸方向）
**WHEN** ボールが地面に着地し、位置が |Z| > Court.Width/2
**THE SYSTEM SHALL** アウト判定を行う
- サイドライン外（コート幅方向）へのボールはアウト
- 壁反射は発生しない（オープンコート）

**参照**: [30501_court_spec.md](../305_court/30501_court_spec.md#req-30501-007)
**テスト**: TST-30404-010

---

### REQ-30402-005: アウトイベント発行
**WHEN** ボールがアウト判定された
**THE SYSTEM SHALL** `BallOutEvent` を発行する
- イベントデータ：Position（着地位置）, OutType（サイドアウト/ベースラインアウト）

**参照**: [30901_point_judgment_spec.md](../309_referee/30901_point_judgment_spec.md)
**テスト**: TST-30404-011

---

## Extended Requirements (v0.2)

### REQ-30402-050: 摩擦効果
**WHEN** ボールが地面にバウンドする
**THE SYSTEM SHALL** 摩擦により水平速度を減衰させる
- 摩擦係数: `config.Ball.FrictionFactor`

**テスト**: TST-30404-050

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

### REQ-30402-150: バウンドエフェクト
**WHEN** ボールが地面にバウンドする
**THE SYSTEM SHALL** バウンドエフェクトを表示する

**テスト**: TST-30404-150

---

## 制約（Design by Contract）

### 事前条件
- バウンド係数は 0.0 ～ 1.0 の範囲
- アウト判定は着地位置で判定（空中は対象外）

### 事後条件
- バウンド後、速度は減衰（エネルギー保存則違反なし）
- アウト判定後、ポイント処理が開始される

### 不変条件
- バウンド係数は常に一定（途中変更禁止）
- バウンドイベントは1回のバウンドにつき1回のみ発行

---

## データ参照

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| バウンド係数 | config.Ball.BounceFactor | 0.8 |
| コート幅（Z方向） | config.Court.Width | 10.0 m |
| コート奥行き（X方向） | config.Court.Depth | 6.0 m |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - Ball, Court パラメータ
- [30501_court_spec.md](../305_court/30501_court_spec.md) - コート座標系・アウト境界定義
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - イベント定義

### 依存元
- [30901_point_judgment_spec.md](../309_referee/30901_point_judgment_spec.md) - アウト判定によるポイント処理

---

## 備考

### テニスの特徴
- オープンコート（壁・天井なし）
- 壁反射は発生しない（パデルとの違い）
- コートライン外に着地したボールはアウト（打った側の失点）

### 削除されたパデル仕様
- REQ-30402-003（旧）: 側壁反射 → アウト境界に変更
- REQ-30402-005（旧）: 奥壁反射 → アウト境界に変更
- REQ-30402-006（旧）: 天井反射 → 削除（オープンコート）
- REQ-30402-007（旧）: めり込み防止 → 壁反射不要のため削除

---

## Change Log

### 2026-01-09 - v2.0.0（テニスへ変更）

- **概要**: 「壁・天井反射」→「地面バウンドおよびアウト境界動作」
- **REQ-30402-003**: 側壁反射 → コートライン外アウト（X軸方向）に変更
- **REQ-30402-004**: 壁反射イベント → コートライン外アウト（Z軸方向）に変更
- **REQ-30402-005**: 奥壁反射 → アウトイベント発行に変更
- **REQ-30402-006**: 天井反射 → 削除
- **REQ-30402-007**: めり込み防止 → 削除
- **REQ-30402-050**: 回転による反射変化 → 削除（壁反射不要）
- **REQ-30402-150**: 反射エフェクト → バウンドエフェクトに変更

### 2025-12-23 - v1.0.0（初版）

- 初版作成（パデルベース）
