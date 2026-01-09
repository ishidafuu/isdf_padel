# Shot Attributes Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2026-01-08

## 概要

ショットの5要素判定と属性計算システムを定義します。

プレイヤーの状況（入力方式、打点、タイミング、移動、距離）から、ショットの属性（威力、安定性、角度、スピン、精度）を無段階で算出します。

---

## コンセプト

### 設計思想

- **ボタンでの打ち分けではなく、状況による自然な打ち分け**
- 5つの入力要素が独立して属性に影響
- 各要素は連続値（無段階）で変化
- テニス/パデルの本質的なショット感覚を再現

### 入力要素 → 出力属性

```
┌─────────────────────────────────────────────────────┐
│                    入力要素（5つ）                   │
├─────────────────────────────────────────────────────┤
│ 1. 入力方式（プッシュ精度 / ホールド時間）          │
│ 2. 打点の高さ（Y座標）                              │
│ 3. バウンド経過時間                                 │
│ 4. ボールへの入り方（移動ベクトル内積）             │
│ 5. ボールとの距離                                   │
└─────────────────────────────────────────────────────┘
                         ↓ 係数計算（リニア）
┌─────────────────────────────────────────────────────┐
│                    出力属性（5つ）                   │
├─────────────────────────────────────────────────────┤
│ 威力: ボール初速度に影響                            │
│ 安定性: ミス確率に影響                              │
│ 角度: 発射角度に影響                                │
│ スピン: 軌道カーブに影響                            │
│ 精度: コースブレに影響                              │
└─────────────────────────────────────────────────────┘
```

---

## Core Requirements (v0.2)

### 入力方式判定

#### REQ-30604-050: プッシュ判定
**WHEN** プレイヤーがショットボタンを押した瞬間にボールがヒット範囲内にある
**THE SYSTEM SHALL** プッシュショットとして判定する
- プッシュ精度 = |ボタン押下時刻 - ボールがヒット範囲に入った時刻|
- 精度が良いほど威力UP、安定性DOWN

**データ**: config.shot_attributes.push_perfect_window (デフォルト: 50ms)

---

#### REQ-30604-051: ホールド判定
**WHEN** プレイヤーがショットボタンを押し続けた状態でボールがヒット範囲に入る
**THE SYSTEM SHALL** ホールドショットとして判定する
- ホールド時間 = ボタン押下継続時間
- 時間が長いほど安定性UP（頭打ちあり）
- 威力は常に低め

**データ**:
- config.shot_attributes.hold_stable_time (デフォルト: 200ms)
- config.shot_attributes.hold_power_factor (デフォルト: 0.6)

---

#### REQ-30604-052: ホールド時間による安定性変化
**WHEN** ホールドショットを実行する
**THE SYSTEM SHALL** ホールド時間に応じて安定性係数を計算する
- 0ms: 安定性係数 0.5（不安定）
- 100ms: 安定性係数 0.75
- 200ms以上: 安定性係数 1.0（頭打ち）

```
stability_factor = min(1.0, 0.5 + (hold_time / hold_stable_time) * 0.5)
```

---

#### REQ-30604-053: プッシュ精度による威力変化
**WHEN** プッシュショットを実行する
**THE SYSTEM SHALL** タイミング精度に応じて威力係数を計算する
- 0ms（完璧）: 威力係数 1.0
- 50ms: 威力係数 0.85
- 100ms: 威力係数 0.7
- 150ms以上: ホールド扱いに移行

```
if timing_diff >= push_to_hold_threshold:
    # ホールド扱い
else:
    power_factor = 1.0 - (timing_diff / push_to_hold_threshold) * 0.3
    stability_factor = 0.8 - (1.0 - power_factor) * 0.5  // 威力が高いほど安定性が下がる
```

**データ**: config.shot_attributes.push_to_hold_threshold (デフォルト: 150ms)

---

### 打点の高さ

#### REQ-30604-054: 打点高さの取得
**WHEN** ショットを実行する
**THE SYSTEM SHALL** ボールのY座標を打点高さとして取得する

---

#### REQ-30604-055: 打点高さによる属性変化
**WHEN** 打点高さが確定する
**THE SYSTEM SHALL** 高さに応じて係数を計算する

| Z座標 | 威力ボーナス | 安定性係数 | 角度補正 |
|-------|-------------|-----------|---------|
| 0.0m | -3.0 | 0.5 | +30° |
| 0.5m | -2.0 | 0.7 | +20° |
| 1.0m | -1.0 | 1.0 | +10° |
| 1.5m | 0.0 | 0.9 | 0° |
| 2.0m | +2.0 | 0.8 | -15° |
| 2.5m+ | +3.0 | 0.7 | -30° |

線形補間で中間値を算出する。

**データ**: config.shot_attributes.height_curve

---

### バウンド経過時間

#### REQ-30604-056: バウンド経過時間の取得
**WHEN** ショットを実行する
**THE SYSTEM SHALL** ボールの最後のバウンドからの経過時間を取得する
- ノーバウンド（空中）の場合は特別扱い

---

#### REQ-30604-057: ノーバウンド（ボレー）の属性
**WHEN** ボールがバウンドしていない状態で打つ
**THE SYSTEM SHALL** ボレー補正を適用する
- 威力ボーナス: -1.0
- 安定性係数: 0.7
- 角度補正: 0°

**データ**: config.shot_attributes.volley_factors

---

#### REQ-30604-058: バウンド後のタイミング属性
**WHEN** ボールがワンバウンド後の状態で打つ
**THE SYSTEM SHALL** 経過時間に応じて係数を計算する

| 経過時間 | 威力ボーナス | 安定性係数 | 角度補正 | 説明 |
|---------|-------------|-----------|---------|------|
| 0.0s | +2.0 | 0.6 | -5° | ライジング |
| 0.3s | +1.0 | 0.8 | 0° | 上がり中 |
| 0.5s | 0.0 | 1.0 | 0° | 頂点 |
| 0.8s | -1.0 | 0.9 | +10° | 落ち際 |
| 1.0s+ | -2.0 | 0.7 | +20° | 低くなる |

線形補間で中間値を算出する。

**データ**: config.shot_attributes.timing_curve

---

### ボールへの入り方

#### REQ-30604-059: 移動ベクトルの取得
**WHEN** ショットを実行する
**THE SYSTEM SHALL** プレイヤーの移動ベクトルとボール方向の内積を計算する

```
approach_dot = normalize(player_velocity) · normalize(ball_position - player_position)
// 範囲: -1.0（後退）〜 +1.0（前進）
```

---

#### REQ-30604-060: 入り方による属性変化
**WHEN** 移動ベクトル内積が確定する
**THE SYSTEM SHALL** 内積値に応じて係数を計算する

| 内積 | 威力ボーナス | 角度補正 | 説明 |
|------|-------------|---------|------|
| +1.0 | +3.0 | -10° | 前進（攻撃的） |
| 0.0 | 0.0 | 0° | 静止/横移動 |
| -1.0 | -2.0 | +20° | 後退（ロブ気味） |

線形補間で中間値を算出する。

**データ**: config.shot_attributes.approach_curve

---

### ボールとの距離

#### REQ-30604-061: ボール距離の取得
**WHEN** ショットを実行する
**THE SYSTEM SHALL** XY平面でのプレイヤーとボールの距離を計算する

```
distance = sqrt((player.x - ball.x)^2 + (player.y - ball.y)^2)
```

---

#### REQ-30604-062: 距離による属性変化
**WHEN** ボール距離が確定する
**THE SYSTEM SHALL** 距離に応じて係数を計算する

| 距離 | 威力ボーナス | 安定性係数 | 精度係数 |
|------|-------------|-----------|---------|
| 0.5m | +1.0 | 1.1 | 1.1 |
| 1.0m | 0.0 | 1.0 | 1.0 |
| 1.5m | -1.5 | 0.7 | 0.7 |
| 2.0m | -3.0 | 0.4 | 0.4 |

線形補間で中間値を算出する。

**データ**:
- config.shot_attributes.optimal_distance (デフォルト: 1.0m)
- config.shot_attributes.distance_curve

---

### 属性計算

#### REQ-30604-063: 威力の最終計算
**WHEN** 全ての入力要素が確定する
**THE SYSTEM SHALL** 威力を**加算方式**で計算する

```
power = base_power
      + input_power_bonus       // 入力方式ボーナス
      + height_power_bonus      // 打点高さボーナス
      + timing_power_bonus      // タイミングボーナス
      + approach_power_bonus    // 入り方ボーナス
      + distance_power_bonus    // 距離ボーナス
```

**設計方針**:
- 悪い条件: ボーナス -2.0〜-3.0、角度補正 +20°〜+30° → 山なりで返すだけの球
- 通常条件: ボーナス 0、角度補正 0° → それなりの打球
- 良い条件: ボーナス +2.0〜+5.0、角度補正 -10°〜-15° → 強打（低い弾道）

**データ**: config.shot_attributes.base_power (デフォルト: 15.0 m/s)

---

#### REQ-30604-064: 安定性の最終計算
**WHEN** 全ての入力要素が確定する
**THE SYSTEM SHALL** 安定性を計算する

```
stability = base_stability
          × input_stability_factor
          × height_stability_factor
          × timing_stability_factor
          × distance_stability_factor
```

**データ**: config.shot.base_stability (デフォルト: 1.0)

---

#### REQ-30604-065: 角度の最終計算
**WHEN** 全ての入力要素が確定する
**THE SYSTEM SHALL** 発射角度を計算する

```
angle = base_angle
      + height_angle_offset
      + timing_angle_offset
      + approach_angle_offset
```

**データ**: config.shot.base_angle (デフォルト: 15°)

---

#### REQ-30604-066: スピンの計算
**WHEN** 全ての入力要素が確定する
**THE SYSTEM SHALL** スピン量を計算する
- 正の値: トップスピン（落ちる軌道）
- 負の値: スライス（滑る軌道）

**係数の導出:**
- height_spin_factor: 打点高さから導出
  - 高い打点（2.0m+）: +0.5（トップスピン傾向）
  - 最適打点（1.0m）: 0.0
  - 低い打点（0.5m以下）: -0.5（スライス傾向）
- timing_spin_factor: バウンド経過時間から導出
  - ライジング（0.0-0.3s）: +0.3（トップスピン傾向）
  - 頂点（0.5s）: 0.0
  - 落ち際（0.8s+）: -0.3（スライス傾向）

```
spin = height_spin_factor + timing_spin_factor
// 範囲: -1.0 〜 +1.0
```

**データ**: config.shot_attributes.spin_curve

---

#### REQ-30604-067: 精度の最終計算
**WHEN** 全ての入力要素が確定する
**THE SYSTEM SHALL** 精度を計算する

```
accuracy = base_accuracy × distance_accuracy_factor
```

**データ**: config.shot.base_accuracy (デフォルト: 1.0)

---

### ショット実行への反映

#### REQ-30604-068: 威力のボール速度反映
**WHEN** ショットを実行する
**THE SYSTEM SHALL** 威力をボール初速度に反映する

```
ball_speed = power  // power は既に m/s 単位
```

---

#### REQ-30604-069: 安定性による威力減衰
**WHEN** ショットを実行する
**AND** 安定性が閾値未満
**THE SYSTEM SHALL** 安定性に応じて威力を減衰させる

```
if stability < stability_threshold:
    power_reduction = (stability_threshold - stability) / stability_threshold
    final_power = power × (1.0 - power_reduction × 0.5)
```

**データ**: config.shot_attributes.stability_threshold (デフォルト: 0.3)

**注意**: ランダム性は導入しない（同じ入力 → 同じ出力の原則）

---

#### REQ-30604-070: 精度による着地位置の収束
**WHEN** ショットを実行する
**THE SYSTEM SHALL** 精度に応じて着地位置をコート中央寄りに収束させる

```
// 精度が低いほどコート中央に寄る（狙った位置に打てない）
convergence = 1.0 - accuracy
final_landing = lerp(target_landing, court_center, convergence × 0.3)
```

**注意**: ランダム性は導入しない（同じ入力 → 同じ出力の原則）

---

## 制約（Design by Contract）

### 事前条件

- ShotEvent が発行されている
- ボールが存在し、位置情報を持つ
- プレイヤーが存在し、位置・速度情報を持つ
- GameConfig が読み込まれている

### 事後条件

- ShotAttributes が計算される
- 全ての属性値が有効な範囲内（0.0〜2.0程度）

### 不変条件

- 係数計算は決定的（同じ入力 → 同じ出力）
- 全てのパラメータは外部データから読み込む

---

## データ参照

### 入力方式パラメータ

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| プッシュ完璧判定 | config.shot_attributes.push_perfect_window | 50ms |
| プッシュ→ホールド閾値 | config.shot_attributes.push_to_hold_threshold | 150ms |
| ホールド安定化時間 | config.shot_attributes.hold_stable_time | 200ms |
| ホールド威力係数 | config.shot_attributes.hold_power_factor | 0.6 |

### 距離パラメータ

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| 最適距離 | config.shot_attributes.optimal_distance | 1.0m |
| 最大打球距離 | config.shot.max_distance | 2.0m |

### 安定性パラメータ

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| 安定性閾値 | config.shot_attributes.stability_threshold | 0.3 |
| 最大方向ブレ | config.shot_attributes.max_direction_error | 15° |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先

- [80101_game_constants.md](../../8_data/80101_game_constants.md) - パラメータ定義
- [30601_shot_input_spec.md](30601_shot_input_spec.md) - ショット入力判定
- [30401_trajectory_spec.md](../304_ball/30401_trajectory_spec.md) - ボール軌道

### 依存元

- [30302_ai_shot_spec.md](../303_ai/30302_ai_shot_spec.md) - AIのショット実行

---

## 備考

### 将来の拡張（v0.3+）

- キャラクター別カーブ係数
- イージングカーブ対応
- スピンの視覚エフェクト
- 必殺ショット判定との連携

### 調整ポイント

- 各係数のバランスは実際にプレイしながら調整
- ホールドの「構え」表示（アニメーション/エフェクト）は別途検討
- AIの構え読みシステムは v0.3 以降

---

## Change Log

### 2026-01-07 - v1.0.0（初版）

- 5要素ショット属性システムの初期仕様
- 入力要素の定義（入力方式、打点、タイミング、入り方、距離）
- 出力属性の定義（威力、安定性、角度、スピン、精度）
- 係数計算ルールの定義
