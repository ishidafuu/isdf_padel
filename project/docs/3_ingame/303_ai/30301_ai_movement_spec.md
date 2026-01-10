# AI Movement Spec

**Version**: 1.2.0
**Last Updated**: 2026-01-11
**Status**: Active

---

## Overview

AIキャラクターの移動とポジショニングを定義します。

ボールを追跡して打てる位置に移動する、対戦相手として機能するための基本行動です。

---

## Core Requirements (MVP v0.1)

### REQ-30301-001: ボール追跡移動

- WHEN ボールがプレイ中である
- THE SYSTEM SHALL AIをボールのXZ平面位置に向かって移動させる
- WITH 移動速度 `config.ai.move_speed`（デフォルト: 5.0 m/s）

### REQ-30301-002: 移動方向計算

- WHEN AIがボールを追跡する
- THE SYSTEM SHALL ボール位置とAI位置の差分ベクトルを正規化して移動方向とする
- WITH XZ平面のみ（Y軸は無視）

### REQ-30301-003: 到達判定

- WHEN AIとボールのXZ平面距離が打球可能距離以内になった
- THE SYSTEM SHALL AIの追跡移動を停止する
- WITH 打球可能距離 `config.shot.max_distance`（デフォルト: 1.5 m）

### REQ-30301-004: 境界制限

- WHILE AIが移動中
- THE SYSTEM SHALL AIをコート境界内に制限する
- WITH 境界 `config.court.*` を参照

### REQ-30301-005: 待機位置復帰

- WHEN ボールが相手コート側にある
- AND ボールがAIのショット範囲外にある
- THE SYSTEM SHALL AIをホームポジションに向かって移動させる
- WITH ホームポジション `config.ai.home_position`（デフォルト: 自コート中央）

---

## v0.5 Requirements

### REQ-30301-v05-001: 着地点予測移動

- WHEN ボールが自分のコート側に向かっている
- THE SYSTEM SHALL ボールの軌道から着地点を予測し、その位置に先回り移動する
- WITH 放物線運動の二次方程式を解いて着地点を計算

### REQ-30301-v05-002: 動的待機位置

- WHEN ボールが相手コート側にある
- THE SYSTEM SHALL 相手の返球範囲をカバーする動的な待機位置に移動する
- WITH
  - X軸: `config.ai.optimal_depth`（デフォルト: 5.0m）
  - Z軸: ボール位置 × `config.ai.coverage_bias_factor`（デフォルト: 0.3）
  - Z軸最大: `config.ai.max_z_offset`（デフォルト: 3.0m）

### REQ-30301-v05-003: リカバリーポジショニング

- WHEN AIがショットを打った直後
- THE SYSTEM SHALL 最適なリカバリー位置へ戻る
- WITH
  - X軸: `config.ai.recovery_depth`（デフォルト: 4.0m）
  - Z軸: 打球方向の逆サイド × `config.ai.recovery_bias_factor`（デフォルト: 0.5）
  - Z軸最大: `config.ai.max_recovery_z`（デフォルト: 2.5m）

### REQ-30301-v05-004: AI移動状態管理

- THE SYSTEM SHALL AIの移動状態を以下の3つで管理する
- WITH
  - `Idle`: 待機中（ボールが相手側）
  - `Tracking`: 追跡中（ボールが自分側）
  - `Recovering`: リカバリー中（ショット後）

---

## Extended Requirements (v0.2)

### REQ-30301-050: 軌道予測移動

- WHEN ボールが打たれた
- THE SYSTEM SHALL ボールの落下予測地点を計算し、その地点に向かって移動する
- WITH 予測精度 `config.ai.prediction_accuracy`（0.0〜1.0）

### REQ-30301-051: 落下地点計算

- WHEN ボールが空中にある
- THE SYSTEM SHALL 放物線軌道に基づき着地点を予測する
- WITH 計算式: 二次方程式による時間計算 → XZ位置算出

### REQ-30301-052: 予測誤差

- WHEN 軌道予測を行う
- THE SYSTEM SHALL 難易度に応じた誤差を加える
- WITH 誤差範囲 `config.ai.prediction_error`（メートル）

### REQ-30301-053: 反応遅延

- WHEN ボール状態が変化した（打球、反射）
- THE SYSTEM SHALL 反応遅延時間の後に追跡を開始する
- WITH 遅延時間 `config.ai.reaction_delay`（デフォルト: 100ms）

### REQ-30301-054: 戦略的ポジショニング

- WHILE ボールが自分側に向かっていない
- THE SYSTEM SHALL 戦略的な待機位置に移動する
- WITH 待機位置 = コート中央やや後方

### REQ-30301-055: 先読み移動

- WHEN 相手がショットを構えている（ホールド中）
- THE SYSTEM SHALL 打球方向を予測して先行移動する
- WITH 予測精度 `config.ai.anticipation_accuracy`

---

## Data References

### v0.5 Parameters

| パラメータ | 設定パス | デフォルト値 | 説明 |
|-----------|---------|-------------|------|
| 移動速度 | `config.ai.move_speed` | 5.0 m/s | AI の最大移動速度 |
| 待機深さ | `config.ai.optimal_depth` | 5.0 m | 待機時のX軸深さ |
| Z軸調整係数 | `config.ai.coverage_bias_factor` | 0.3 | ボール位置に応じたZ軸調整 |
| Z軸最大オフセット | `config.ai.max_z_offset` | 3.0 m | Z軸移動の最大値 |
| リカバリー深さ | `config.ai.recovery_depth` | 4.0 m | リカバリー時のX軸深さ |
| リカバリーバイアス | `config.ai.recovery_bias_factor` | 0.5 | 打球逆サイドへの寄り係数 |
| リカバリーZ軸最大 | `config.ai.max_recovery_z` | 2.5 m | リカバリーZ軸の最大値 |

### Legacy Parameters

| パラメータ | 設定パス | デフォルト値 | 説明 |
|-----------|---------|-------------|------|
| ホームポジションX | `config.ai.home_position.x` | 0.0 m | 待機位置X座標 |
| ホームポジションZ | `config.ai.home_position.z` | 5.0 m | 待機位置Z座標（自コート後方） |
| 反応遅延 | `config.ai.reaction_delay` | 0.1 s | ボール認識の遅れ |
| 予測精度 | `config.ai.prediction_accuracy` | 0.8 | 軌道予測の正確さ（0.0〜1.0） |
| 予測誤差 | `config.ai.prediction_error` | 0.5 m | 予測位置の最大誤差 |
| 先読み精度 | `config.ai.anticipation_accuracy` | 0.6 | 相手ショット予測精度 |

---

## Constraints

### Preconditions

- AIエンティティが存在し、Position/Velocity コンポーネントを持つ
- ボールエンティティが存在する
- コート境界が定義されている

### Postconditions

- AIは常にコート境界内に位置する
- AIの速度は最大移動速度を超えない

### Invariants

- AIの移動速度 ≤ `config.ai.move_speed`
- AIのY座標 ≥ 0（地面以上）

---

## v0.7 Requirements

### REQ-30301-v07-001: インターセプト方式移動

- WHEN ボールが自分のコート側に向かっている
- THE SYSTEM SHALL AIは自身のX座標を維持し、ボールが通過するZ座標を予測してそこに移動する
- WITH
  - X座標: 現在のAI X座標を維持（固定）
  - Z座標: ボールがAIのX座標を通過する時点でのZ座標を予測
  - 予測式: `intercept_z = ball_z + ball_vel_z * (ai_x - ball_x) / ball_vel_x`

### REQ-30301-v07-002: 短いボール判定

- WHEN ボールがAIのX座標に到達する前に着地する
- THE SYSTEM SHALL AIはボールの現在位置を追跡する
- WITH 判定条件: `time_to_landing < time_to_intercept`

### REQ-30301-v07-003: 目標ロック機構（振動防止）

- WHEN ボールの移動状態が変化した（方向転換、新規ショット）
- THE SYSTEM SHALL 新しい目標Z座標を計算し、誤差を1回だけ適用してロックする
- WITH
  - ロック条件: ボール速度X成分の符号変化を検知
  - ロック解除: ボール速度X成分の符号が再度変化したとき
  - 振動防止: ロック中は毎フレームの再計算を行わない

---

## Related Specifications

- [30300_overview.md](30300_overview.md) - AI概要
- [30302_ai_shot_spec.md](30302_ai_shot_spec.md) - AIショット仕様
- [30401_trajectory_spec.md](../304_ball/30401_trajectory_spec.md) - ボール軌道
- [30501_court_spec.md](../305_court/30501_court_spec.md) - コート境界

---

## Change Log

### 2026-01-11 - v1.2.0

- v0.7: インターセプト方式移動（REQ-30301-v07-001）
- v0.7: 短いボール判定（REQ-30301-v07-002）
- v0.7: 目標ロック機構（REQ-30301-v07-003）

### 2026-01-10 - v1.1.0

- v0.5: 着地点予測移動（REQ-30301-v05-001）
- v0.5: 動的待機位置（REQ-30301-v05-002）
- v0.5: リカバリーポジショニング（REQ-30301-v05-003）
- v0.5: AI移動状態管理（REQ-30301-v05-004）

### 2026-01-08 - v1.0.0（初版）

- MVP v0.1: シンプルなボール追跡（REQ-30301-001〜005）
- v0.2: 軌道予測、ポジショニング（REQ-30301-050〜055）
