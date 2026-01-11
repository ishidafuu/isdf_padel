# B30301-002: AI軌道ライン追跡の修正

## 概要

AIのボール追跡ロジックが落下地点または現在位置のみを追跡しており、テニス/パデルの正しい動き「ボール軌道のラインに入る」ができていない。

## 問題

| 問題 | 現在の動作 | 正しい動作 |
|------|-----------|-----------|
| 短いボール追跡 | `ball_pos.z`（現在位置） | 着地地点を計算して移動 |
| 軌道ライン概念なし | 点への移動のみ | 軌道ライン上に早期に入る |

## 実装ステップ

- [ ] Step 1: `calculate_landing_position` 関数を追加
- [ ] Step 2: 短いボール追跡を着地地点に修正
- [ ] Step 3: `calculate_trajectory_line_z` 関数を追加
- [ ] Step 4: `calculate_intercept_z` を `calculate_trajectory_line_z` に置き換え
- [ ] Step 5: 仕様書 `30301_ai_movement_spec.md` を更新

## 修正対象

- `project/src/systems/ai/movement.rs`
- `project/docs/3_ingame/303_ai/30301_ai_movement_spec.md`

## 検証

1. `cargo run --bin bevy_padel`
2. 短いボールをAIに打つ → 着地地点に向かうことを確認
3. 長いボールでAIが軌道ライン上に移動することを確認
