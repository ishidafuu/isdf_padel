# AI Shot Spec

**Version**: 1.0.0
**Last Updated**: 2026-01-08
**Status**: Draft

---

## Overview

AIキャラクターのショット判断と実行を定義します。

いつ打つか、どこに打つか、どのような属性で打つかを決定し、対戦相手として機能するための攻撃行動です。

---

## Core Requirements (MVP v0.1)

### REQ-30302-001: ショット可能判定

- WHEN AIとボールの距離が打球可能距離以内になった
- AND ボールの高さがAIの打球可能高さ範囲内にある
- THE SYSTEM SHALL AIをショット可能状態にする
- WITH 打球可能距離 `config.shot.max_distance`
- WITH 打球可能高さ差 `config.shot.max_height_diff`

### REQ-30302-002: ショット実行

- WHEN AIがショット可能状態である
- AND ショットクールダウンが0以下である
- THE SYSTEM SHALL ShotEventを発行する
- WITH クールダウン `config.ai.shot_cooldown`（デフォルト: 0.5s）

### REQ-30302-003: 打球方向（シンプル）

- WHEN AIがショットを実行する
- THE SYSTEM SHALL 相手コート中央に向かう方向を打球方向とする
- WITH 方向 = (相手コート中央 - AI位置).normalize()

### REQ-30302-004: クールダウン管理

- WHEN AIがショットを実行した
- THE SYSTEM SHALL ショットクールダウンを設定する
- WITH クールダウン時間 `config.ai.shot_cooldown`

### REQ-30302-005: ジャンプショット禁止（MVP）

- WHILE MVP v0.1
- THE SYSTEM SHALL AIはジャンプショットを行わない
- WITH jumpHeight = 0

---

## Extended Requirements (v0.2)

### REQ-30302-050: ショット属性システム対応

- WHEN AIがショットを実行する
- THE SYSTEM SHALL 5要素に基づいたShotContextを生成する
- WITH 入力方式、打点高さ、バウンド経過時間、入り方、距離

### REQ-30302-051: AI入力方式シミュレーション

- WHEN AIがショットを実行する
- THE SYSTEM SHALL 難易度に応じたプッシュ/ホールドをシミュレートする
- WITH プッシュ精度 `config.ai.push_timing_accuracy`
- WITH ホールド時間 `config.ai.hold_duration`

### REQ-30302-052: 打点調整

- WHEN AIがショットを準備する
- THE SYSTEM SHALL 最適な打点高さでショットを実行するよう移動を調整する
- WITH 最適打点 `config.shot_attributes.optimal_height`

### REQ-30302-053: タイミング調整

- WHEN ボールがバウンドした
- THE SYSTEM SHALL 最適なタイミングでショットを実行する
- WITH タイミング精度 `config.ai.timing_accuracy`

### REQ-30302-054: 戦略的打球方向

- WHEN AIがショットを実行する
- THE SYSTEM SHALL 相手の位置を考慮した打球方向を選択する
- WITH 相手がいない方向を優先

### REQ-30302-055: 打球方向ランダム化

- WHEN AIがショットを実行する
- THE SYSTEM SHALL 難易度に応じたブレを打球方向に加える
- WITH ブレ角度 `config.ai.direction_variance`（度）

### REQ-30302-056: ミスショット発生

- WHEN AIがショットを実行する
- AND 乱数がミス率を下回った
- THE SYSTEM SHALL 意図的にミスショットを発生させる
- WITH ミス率 `config.ai.miss_rate`

### REQ-30302-057: 難易度別パラメータ

- WHERE 難易度が設定されている
- THE SYSTEM SHALL 難易度に応じたAIパラメータを適用する
- WITH Easy/Normal/Hard の各パラメータセット

---

## Difficulty Settings (v0.2)

### Easy

| パラメータ | 値 | 説明 |
|-----------|-----|------|
| move_speed | 3.0 m/s | 遅い移動 |
| reaction_delay | 300 ms | 遅い反応 |
| miss_rate | 0.20 | 20%ミス |
| direction_variance | 20° | 大きいブレ |
| prediction_accuracy | 0.5 | 低い予測精度 |

### Normal

| パラメータ | 値 | 説明 |
|-----------|-----|------|
| move_speed | 5.0 m/s | 標準移動 |
| reaction_delay | 150 ms | 標準反応 |
| miss_rate | 0.10 | 10%ミス |
| direction_variance | 10° | 中程度ブレ |
| prediction_accuracy | 0.7 | 標準予測精度 |

### Hard

| パラメータ | 値 | 説明 |
|-----------|-----|------|
| move_speed | 6.5 m/s | 速い移動 |
| reaction_delay | 50 ms | 速い反応 |
| miss_rate | 0.03 | 3%ミス |
| direction_variance | 5° | 小さいブレ |
| prediction_accuracy | 0.9 | 高い予測精度 |

---

## Constraints

### Preconditions

- AIエンティティが存在する
- ボールエンティティが存在し、プレイ中である
- ショットシステムが初期化されている

### Postconditions

- ShotEvent発行後、クールダウンが設定される
- ボールの所有者がAIに変更される（ショット成功時）

### Invariants

- ショットクールダウン中はショット不可
- 打球可能距離外ではショット不可

---

## Data References

| パラメータ | 設定パス | デフォルト値 | 説明 |
|-----------|---------|-------------|------|
| ショットクールダウン | `config.ai.shot_cooldown` | 0.5 s | ショット間の待機時間 |
| ミス率 | `config.ai.miss_rate` | 0.10 | ショット失敗確率 |
| 方向ブレ | `config.ai.direction_variance` | 10° | 打球方向の誤差 |
| タイミング精度 | `config.ai.timing_accuracy` | 0.7 | タイミング合わせの正確さ |
| プッシュ精度 | `config.ai.push_timing_accuracy` | 0.6 | プッシュタイミング精度 |
| ホールド時間 | `config.ai.hold_duration` | 200 ms | ホールド時のチャージ時間 |

---

## Related Specifications

- [30300_overview.md](30300_overview.md) - AI概要
- [30301_ai_movement_spec.md](30301_ai_movement_spec.md) - AI移動仕様
- [30601_shot_input_spec.md](../306_shot_system/30601_shot_input_spec.md) - ショット入力
- [30604_shot_attributes_spec.md](../306_shot_system/30604_shot_attributes_spec.md) - ショット属性システム

---

## Change Log

### 2026-01-08 - v1.0.0（初版）

- MVP v0.1: 基本的な返球（REQ-30302-001〜005）
- v0.2: ショット属性対応、難易度調整（REQ-30302-050〜057）
- 難易度別パラメータ定義（Easy/Normal/Hard）
