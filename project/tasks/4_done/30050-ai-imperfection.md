---
id: "30050"
title: "AI不完全性導入"
type: "game-dev"
status: "completed"
priority: "high"
related_task: null
spec_ids:
  - "REQ-30301-052"
  - "REQ-30301-053"
  - "REQ-30302-055"
blocked_by: []
blocks: []
branch_name: "task/30050-ai-imperfection"
worktree_path: "/Users/ishidafuu/Documents/repository/isdf_padel-30050"
plan_file: "/Users/ishidafuu/.claude/plans/mutable-moseying-zebra.md"
tags:
  - "ai"
  - "balance"
parent_task_id: null
created_at: "2026-01-10"
updated_at: "2026-01-10"
completed_at: "2026-01-10"
---

# Task 30050: AI不完全性導入

## Summary

オートモードでAIが完璧すぎてラリーが終わらない問題を解決する。
AIの思考能力（予測・反応・判断）に不完全性を導入し、ポイントが進行するようにする。

## Related Specifications

- `project/docs/3_ingame/303_ai/30301_ai_movement_spec.md` - AI移動仕様（REQ-30301-052, 053）
- `project/docs/3_ingame/303_ai/30302_ai_shot_spec.md` - AIショット仕様（REQ-30302-055）

## Progress

### Completed

1. config.rs に AiConfig 新フィールド追加（prediction_accuracy, prediction_error, direction_variance, reaction_delay）
2. game_config.ron に対応するパラメータ追加（Normal難易度相当）
3. components/mod.rs の AiController に reaction_timer フィールド追加
4. ai_movement.rs に予測誤差（apply_prediction_error）と反応遅延を実装
5. ai_shot.rs に方向ランダム化（apply_direction_variance）を実装
6. cargo build 成功
7. cargo test 全テスト PASS（149 passed; 0 failed）

## Next Actions

1. ~~config.rs/game_config.ron にパラメータ追加~~ ✅
2. ~~ai_movement.rs に予測誤差実装~~ ✅
3. ~~ai_shot.rs に方向ランダム化実装~~ ✅
4. ~~ai_movement.rs に反応遅延実装~~ ✅
5. ~~ビルド・動作検証~~ ✅
6. **レビュー待ち**: オートモードで60秒以内にポイント決着を確認

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [ ] オートモードで60秒以内にポイント決着を確認
- [x] in-review に移動済み
- [ ] レビュー完了

## メモ

- 既存仕様書のExtended Requirements v0.2として定義済み
- 新規仕様作成は不要
- Normal難易度相当のパラメータで実装

---

## Detailed Implementation Plan

> 以下はプランファイル `~/.claude/plans/mutable-moseying-zebra.md` の全内容です。

# AI不完全性導入プラン

## 概要
オートモードでAIが完璧すぎてラリーが終わらない問題を解決する。

## 問題
- AIは着地点を100%正確に予測し、必ずボールに追いつく
- ショット計算は完全決定的（ランダム性なし）
- 結果、ポイントが永遠に決まらない

## 解決策（3つ実装）
1. **打球方向ランダム化**: コート端への打ち分けを可能に（REQ-30302-055）
2. **予測誤差の導入**: 着地点予測に誤差を加える（REQ-30301-052）
3. **反応遅延の導入**: ボール状態変化後に遅延追加（REQ-30301-053）

## 既存仕様の状況
仕様書に全て定義済み（Extended Requirements v0.2）。新規仕様作成は不要。
- `30301_ai_movement_spec.md`: REQ-30301-052（予測誤差）、REQ-30301-053（反応遅延）
- `30302_ai_shot_spec.md`: REQ-30302-055（方向ランダム化）、REQ-30302-056（ミス率）
- 難易度別パラメータ表（Easy/Normal/Hard）も完備

---

## 実装ステップ

### Step 1: 設定パラメータ追加
**対象ファイル**:
- `project/src/resource/config.rs` - AiConfig構造体に新フィールド追加
- `project/assets/config/game_config.ron` - 値の外部定義

**追加パラメータ（Normal難易度相当）**:
```ron
ai: AiConfig(
    // 既存パラメータはそのまま
    prediction_accuracy: 0.7,  // 予測精度（仕様書Normal: 0.7）
    prediction_error: 0.5,     // 最大予測誤差（仕様書: 0.5m）
    direction_variance: 10.0,  // 打球方向ブレ（仕様書Normal: 10°）
    reaction_delay: 0.15,      // 反応遅延（仕様書Normal: 150ms）
)
```

### Step 2: AI移動の予測誤差実装
**対象ファイル**: `project/src/systems/ai_movement.rs`

**変更内容**:
```rust
// 着地点予測に誤差を加える
fn apply_prediction_error(landing_pos: Vec3, config: &AiConfig, rng: &mut impl Rng) -> Vec3 {
    let error_range = (1.0 - config.prediction_accuracy) * config.prediction_error;
    let error_x = rng.gen_range(-error_range..=error_range);
    let error_z = rng.gen_range(-error_range..=error_range);
    Vec3::new(landing_pos.x + error_x, landing_pos.y, landing_pos.z + error_z)
}
```

### Step 3: AIショット方向ランダム化
**対象ファイル**: `project/src/systems/ai_shot.rs`

**変更内容**:
```rust
// 打球方向にランダムブレを追加
fn apply_direction_variance(base_direction: Vec2, variance_deg: f32, rng: &mut impl Rng) -> Vec2 {
    let variance_rad = variance_deg.to_radians();
    let offset = rng.gen_range(-variance_rad..=variance_rad);
    // 2D回転
    let cos = offset.cos();
    let sin = offset.sin();
    Vec2::new(
        base_direction.x * cos - base_direction.y * sin,
        base_direction.x * sin + base_direction.y * cos,
    )
}
```

### Step 4: 反応遅延の実装
**対象ファイル**: `project/src/systems/ai_movement.rs`

**変更内容**:
- ボール状態変化（打球、反射）検出時に反応タイマーを設定
- タイマー中はAIが追跡を開始しない
- AiControllerに `reaction_timer: f32` フィールド追加

```rust
// ボール状態変化を検出したら反応遅延を設定
if ball_state_changed {
    ai_controller.reaction_timer = config.ai.reaction_delay;
}

// タイマー中は移動しない
if ai_controller.reaction_timer > 0.0 {
    ai_controller.reaction_timer -= time.delta_secs();
    return; // 追跡を開始しない
}
```

---

## 検証方法
1. `cargo build` でコンパイル確認
2. `cargo run` でオートモード実行
3. 60秒以内にポイントが決まることを確認
4. ログで `RallyEndEvent` が発行されることを確認

---

## Critical Files
| ファイル | 変更内容 |
|---------|---------|
| `project/src/resource/config.rs:AiConfig` | 新フィールド追加 |
| `project/assets/config/game_config.ron` | パラメータ外部化 |
| `project/src/systems/ai_movement.rs` | 予測誤差の追加 |
| `project/src/systems/ai_shot.rs` | 方向ランダム化 |
