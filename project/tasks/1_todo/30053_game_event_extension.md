---
id: "30053"
title: "GameEvent新イベント定義"
type: "game-dev"
status: "todo"
priority: "high"
related_task: null
spec_ids:
  - "REQ-77200-001"
  - "REQ-77200-003"
  - "REQ-77200-005"
blocked_by: []
blocks:
  - "30054"
  - "30055"
  - "30056"
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/optimized-strolling-puppy.md"
tags:
  - "telemetry"
  - "llm-qa"
  - "phase1"
parent_task_id: null
created_at: "2026-01-16T16:00:00+09:00"
updated_at: "2026-01-16T16:00:00+09:00"
completed_at: null
---

# Task 30053: GameEvent新イベント定義

## Summary

LLMベースQAシステムのPhase 1基盤として、既存のGameEvent enumに3つの新イベントタイプを追加する。

## Related Specifications

- `project/docs/7_tools/71_simulation/77200_telemetry_spec.md`

## Progress

### Completed

(なし)

## Next Actions

1. `project/src/simulation/event_tracer.rs` を開く
2. GameEvent enum に以下を追加:
   - `ShotAttributesCalculated` - ショット属性計算詳細
   - `AiMovementDecision` - AI移動決定詳細
   - `PhysicsAnomaly` - 物理異常マーカー
3. `to_json()` メソッドを拡張
4. `to_csv_detail()` メソッドを拡張
5. `type_name()` メソッドを拡張

## Dependencies

- **Blocked By:** なし
- **Blocks:** 30054, 30055, 30056

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

Phase 1 の基盤タスク。後続タスク（30054-30056）はこのタスク完了後に並列実行可能。

---

## Detailed Implementation Plan

### 新規イベント定義

```rust
/// ショット属性計算詳細
ShotAttributesCalculated {
    player_id: u8,
    input_mode: String,           // "Push" / "Hold"
    hit_height: f32,
    bounce_elapsed: Option<f32>,
    approach_dot: f32,
    ball_distance: f32,
    // 中間係数
    height_factors: (f32, f32, f32),   // (power, stability, angle)
    timing_factors: (f32, f32, f32),
    approach_factors: (f32, f32),       // (power, angle)
    distance_factors: (f32, f32, f32), // (power, stability, accuracy)
    // 最終結果
    final_power: f32,
    final_stability: f32,
    final_angle: f32,
    final_spin: f32,
    final_accuracy: f32,
},

/// AI移動決定詳細
AiMovementDecision {
    player_id: u8,
    movement_state: String,
    ball_coming_to_me: bool,
    reaction_timer: f32,
    landing_time: Option<f32>,
    landing_position: Option<Vec3>,
    trajectory_line_z: f32,
    arrival_distance: f32,
    target_position: Vec3,
},

/// 物理異常マーカー
PhysicsAnomaly {
    anomaly_type: String,
    position: Vec3,
    velocity: Vec3,
    expected_value: f32,
    actual_value: f32,
    severity: String,
},
```

### 修正対象ファイル

- `project/src/simulation/event_tracer.rs`
