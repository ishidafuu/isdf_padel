# Trajectory Calculation Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2026-01-09

## 概要

入力から着地地点を決定し、その地点に到達するよう発射角度・初速を逆算するシステムを定義します。

固定角度発射から「着地点逆算型発射」への変更により、狙った場所に打てるテニスゲームを実現します。

---

## コンセプト

### 設計思想

- **着地地点主導**: 入力 → 着地地点決定 → 発射パラメータ逆算
- **角度主体の調整**: テニス的アプローチ（速度でなく角度で距離調整）
- **スピン考慮**: トップスピン/スライスの有効重力変動を反映

### 処理フロー

```
入力 (x, y)
    ↓
着地地点決定 (target_x, target_z)
    ↓
スピン効果計算 (有効重力)
    ↓
弾道逆算 (発射角度)
    ↓
初速調整 (球種・距離係数)
    ↓
ズレ計算 (精度による偏差)
    ↓
発射パラメータ (angle, speed, direction)
```

---

## Core Requirements (v0.4)

### 着地地点決定

#### REQ-30605-010: ニュートラル着地
**WHEN** ショットを実行する
**AND** 方向入力がない（x=0, y=0）
**THE SYSTEM SHALL** デフォルト着地地点を計算する
- X座標（深さ）: `default_landing_depth`（サービスライン付近）
- Z座標（左右）: 0（中央）

**データ**: config.trajectory.default_landing_depth (デフォルト: 4.0m)
**テスト**: TST-30605-001

---

#### REQ-30605-011: 前後入力による深さ調整
**WHEN** ショットを実行する
**AND** Y方向入力がある
**THE SYSTEM SHALL** 着地のX座標（深さ）を調整する

**Player1の場合**:
```
target_x = lerp(net_x + margin, baseline_x - margin, (input.y + 1.0) / 2.0)
```

| 入力Y | 着地位置 | 説明 |
|-------|---------|------|
| -1.0 | ネット際 | ドロップショット |
| 0.0 | サービスライン付近 | ニュートラル |
| +1.0 | ベースライン際 | 深い球 |

**データ**: config.trajectory.landing_margin (デフォルト: 0.5m)
**テスト**: TST-30605-002, TST-30605-003

---

#### REQ-30605-012: 左右入力によるコース調整
**WHEN** ショットを実行する
**AND** X方向入力がある
**THE SYSTEM SHALL** 着地のZ座標（左右）を調整する

```
target_z = input.x * (court_width / 2 - margin)
```

| 入力X | 着地位置 | 説明 |
|-------|---------|------|
| -1.0 | 左サイドライン際 | クロスorストレート |
| 0.0 | 中央 | センター |
| +1.0 | 右サイドライン際 | クロスorストレート |

**データ**: config.court.width, config.trajectory.landing_margin
**テスト**: TST-30605-004

---

#### REQ-30605-013: Player2の着地計算
**WHEN** Player2がショットを実行する
**THE SYSTEM SHALL** Player1の計算結果をZ軸反転する
- target_x = `court_length - player1_target_x`（相手コート側に変換）

---

### 弾道逆算

#### REQ-30605-020: 有効重力の計算
**WHEN** 弾道を計算する
**THE SYSTEM SHALL** スピンを考慮した有効重力を算出する

```
// 飛行時間推定
estimated_flight_time = 2 * initial_height / sqrt(gravity * initial_height)

// 平均スピン効果
avg_spin = spin * (1 - spin_decay_rate * estimated_flight_time / 2)

// 有効重力
g_eff = gravity * (1 + avg_spin * gravity_spin_factor)
```

**データ**:
- config.physics.gravity (デフォルト: 9.8 m/s²)
- config.spin_physics.gravity_spin_factor (デフォルト: 0.3)
- config.spin_physics.spin_decay_rate (デフォルト: 0.5)

**テスト**: TST-30605-011, TST-30605-012

---

#### REQ-30605-021: 発射角度の逆算
**WHEN** 有効重力が確定する
**THE SYSTEM SHALL** 放物線公式から発射角度を逆算する

**放物線着地条件**:
```
0 = h + d * tan(θ) - g_eff * d² / (2 * v² * cos²(θ))

where:
  h   = 打点高さ - 着地高さ
  d   = 水平距離
  θ   = 発射角度（求める値）
  v   = 初速（基準値）
  g_eff = 有効重力
```

**二次方程式の解**:
```
tan(θ) = (v² ± sqrt(v⁴ - g_eff * (g_eff * d² + 2 * h * v²))) / (g_eff * d)
```

- 2つの解のうち、角度が低い方を採用（テニス的な軌道）
- 実数解がない場合は初速を増加（REQ-30605-024）

**テスト**: TST-30605-010

---

#### REQ-30605-022: 発射角度の範囲制限
**WHEN** 発射角度が計算される
**THE SYSTEM SHALL** 角度を有効範囲内に制限する

```
angle = clamp(calculated_angle, min_launch_angle, max_launch_angle)
```

**データ**:
- config.trajectory.min_launch_angle (デフォルト: 5°)
- config.trajectory.max_launch_angle (デフォルト: 60°)

---

#### REQ-30605-023: 方向ベクトルの計算
**WHEN** 着地地点と発射角度が確定する
**THE SYSTEM SHALL** 3D方向ベクトルを計算する

```
horizontal_dir = normalize(target_pos.xz - start_pos.xz)
direction = Vec3(
    horizontal_dir.x * cos(angle),
    sin(angle),
    horizontal_dir.z * cos(angle)
)
```

---

#### REQ-30605-024: 初速の自動調整
**WHEN** 放物線公式で解が得られない（届かない距離）
**THE SYSTEM SHALL** 初速を増加させて再計算する

```
if no_real_solution:
    adjusted_speed = base_speed * 1.1  // 10%増加
    // 再計算（最大3回まで）
```

**テスト**: TST-30605-010

---

### 初速調整

#### REQ-30605-030: 基準初速の取得
**WHEN** ショットを実行する
**THE SYSTEM SHALL** ショット属性から基準初速を取得する

```
base_speed = shot_attributes.power
```

**参照**: [30604_shot_attributes_spec.md](30604_shot_attributes_spec.md#req-30604-063)

---

#### REQ-30605-031: 球種による初速係数
**WHEN** 基準初速が確定する
**THE SYSTEM SHALL** スピン値に応じた初速係数を適用する

```
spin_factor = if spin > 0.1 {
    spin_speed_topspin  // トップスピン
} else if spin < -0.1 {
    spin_speed_slice    // スライス
} else {
    spin_speed_flat     // フラット
}
```

| 球種 | spin範囲 | 係数 | 説明 |
|------|---------|------|------|
| フラット | -0.1〜+0.1 | 1.0 | 最速 |
| トップスピン | > +0.1 | 0.92 | やや遅い |
| スライス | < -0.1 | 0.88 | さらに遅い |

**データ**:
- config.trajectory.spin_speed_flat (デフォルト: 1.0)
- config.trajectory.spin_speed_topspin (デフォルト: 0.92)
- config.trajectory.spin_speed_slice (デフォルト: 0.88)

**テスト**: TST-30605-020, TST-30605-021

---

#### REQ-30605-032: 距離による初速係数
**WHEN** 水平距離が確定する
**THE SYSTEM SHALL** 距離に応じた初速係数を適用する

```
distance_ratio = horizontal_distance / max_court_distance
distance_factor = lerp(distance_speed_min, distance_speed_max, distance_ratio)
```

| 距離 | 係数 | 説明 |
|------|------|------|
| 近距離（ネット際） | 1.0 | 基準速度 |
| 遠距離（ベースライン） | 1.15 | 15%増加 |

**データ**:
- config.trajectory.distance_speed_min (デフォルト: 1.0)
- config.trajectory.distance_speed_max (デフォルト: 1.15)

**テスト**: TST-30605-022

---

#### REQ-30605-033: 最終初速の計算
**WHEN** 全ての係数が確定する
**THE SYSTEM SHALL** 最終初速を計算する

```
final_speed = base_speed * spin_factor * distance_factor
```

---

### ズレ計算

#### REQ-30605-040: 着地ズレの適用
**WHEN** 着地地点が確定する
**THE SYSTEM SHALL** 精度に応じたズレを追加する

```
deviation = (1.0 - accuracy) * max_landing_deviation
offset_x = random(-1, 1) * deviation
offset_z = random(-1, 1) * deviation
final_target = target + Vec3(offset_x, 0, offset_z)
```

**データ**: config.trajectory.max_landing_deviation (デフォルト: 1.0m)
**参照**: [30604_shot_attributes_spec.md](30604_shot_attributes_spec.md#req-30604-067)
**テスト**: TST-30605-030

---

## Extended Requirements (v0.4+)

### サーブ用着地点計算

#### REQ-30605-050: サーブ着地点計算
**WHEN** サーブを実行する（ShotEvent.is_serve == true）
**THE SYSTEM SHALL** サービスボックス内に制限された着地点を計算する
- 入力方向に応じてサービスボックス内の着地点を決定
- サービスボックス境界からマージンを考慮

**テスト**: TST-30605-050

---

#### REQ-30605-051: サーブの深さ調整
**WHEN** サーブを実行する
**AND** Y方向入力がある
**THE SYSTEM SHALL** サービスボックス内の深さを調整する

| 入力Y | 着地位置 | 説明 |
|-------|---------|------|
| -1.0 | ネット際（サービスボックス前端） | 浅いサーブ |
| 0.0 | サービスボックス中央 | ニュートラル |
| +1.0 | サービスライン際（サービスボックス後端） | 深いサーブ |

**テスト**: TST-30605-051

---

#### REQ-30605-052: サーブのコース調整
**WHEN** サーブを実行する
**AND** X方向入力がある
**THE SYSTEM SHALL** サービスボックス内の左右位置を調整する

| 入力X | 着地位置 | 説明 |
|-------|---------|------|
| -1.0 | サービスボックス左端 | ワイドサーブ/センターサーブ |
| 0.0 | サービスボックス中央 | センター |
| +1.0 | サービスボックス右端 | センターサーブ/ワイドサーブ |

**テスト**: TST-30605-052

---

#### REQ-30605-053: サーブの打点高さ使用
**WHEN** サーブを実行する
**THE SYSTEM SHALL** ShotEvent.hit_position の高さを打点として使用する
- トスボールの位置（1.8m〜2.7m）が打点高さとなる
- 高打点からの下向き発射を自然に表現

**テスト**: TST-30605-053

---

#### REQ-30605-054: サーブ用発射角度の計算
**WHEN** サーブの着地点と打点が確定する
**THE SYSTEM SHALL** 放物線公式から発射角度を逆算する
- 高い打点から下向きの軌道を計算
- オーバーハンドサーブ特性を自然に表現

**テスト**: TST-30605-054

---

## 制約（Design by Contract）

### 事前条件

- ShotEvent が発行されている
- ボールが存在し、位置情報を持つ
- プレイヤーが存在し、方向入力を持つ
- ShotAttributes が計算済み（power, spin, accuracy）
- GameConfig, TrajectoryConfig が読み込まれている

### 事後条件

- 発射角度が 5°〜60° の範囲内
- 最終初速が正の値
- 着地地点がコート内に収まる（マージン考慮）
- 方向ベクトルが正規化されている

### 不変条件

- 弾道計算は決定的（同じ入力 → 同じ出力、ズレ計算を除く）
- 全てのパラメータは外部データから読み込む
- スピンがゼロの場合、有効重力は通常重力と一致

---

## データ参照

### TrajectoryConfig パラメータ

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| 着地マージン | config.trajectory.landing_margin | 0.5m |
| デフォルト着地深さ | config.trajectory.default_landing_depth | 4.0m |
| 最小発射角度 | config.trajectory.min_launch_angle | 5° |
| 最大発射角度 | config.trajectory.max_launch_angle | 60° |
| フラット初速係数 | config.trajectory.spin_speed_flat | 1.0 |
| トップスピン初速係数 | config.trajectory.spin_speed_topspin | 0.92 |
| スライス初速係数 | config.trajectory.spin_speed_slice | 0.88 |
| 近距離初速係数 | config.trajectory.distance_speed_min | 1.0 |
| 遠距離初速係数 | config.trajectory.distance_speed_max | 1.15 |
| 最大着地ズレ | config.trajectory.max_landing_deviation | 1.0m |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先

- [80101_game_constants.md](../../8_data/80101_game_constants.md) - TrajectoryConfig パラメータ
- [30604_shot_attributes_spec.md](30604_shot_attributes_spec.md) - power, spin, accuracy 取得
- [30401_trajectory_spec.md](../304_ball/30401_trajectory_spec.md) - 物理定数（gravity等）
- [30501_court_spec.md](../305_court/30501_court_spec.md) - コート寸法

### 依存元

- [30602_shot_direction_spec.md](30602_shot_direction_spec.md) - 発射パラメータを受け取る

---

## 備考

### 技術的考慮

- **放物線公式の解**: 二次方程式の解なので、計算コストは低い
- **イテレーション不要**: スピン効果の平均化で1パス計算
- **届かないケース**: 角度上限（60°）と初速微調整で対応

### 将来の拡張（v0.5+）

- カーブショット（横方向のスピン影響）
- ロブの自動判定（高角度時の特別処理）
- 風の影響
- キャラ別パラメータ

### 調整ポイント

- 角度範囲（5°〜60°）はプレイ感に大きく影響
- 初速係数のバランスは実際にプレイしながら調整
- ズレ量は精度の重要性を左右する

---

## Change Log

### 2026-01-09 - v1.0.0（初版）

- 弾道計算仕様の初期バージョン
- 着地地点決定ロジック（REQ-30605-010〜013）
- 弾道逆算アルゴリズム（REQ-30605-020〜024）
- 初速調整ロジック（REQ-30605-030〜033）
- ズレ計算（REQ-30605-040）
