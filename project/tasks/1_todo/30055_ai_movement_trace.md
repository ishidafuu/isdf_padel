---
id: "30055"
title: "AI移動決定トレース実装"
type: "game-dev"
status: "todo"
priority: "high"
related_task: null
spec_ids:
  - "REQ-77200-003"
  - "REQ-77200-004"
  - "REQ-77200-007"
blocked_by:
  - "30053"
blocks: []
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

# Task 30055: AI移動決定トレース実装

## Summary

AI移動システムにトレース呼び出しを追加し、移動決定の理由（目標位置、到達距離、状態）をEventTracerに記録する。

## Related Specifications

- `project/docs/7_tools/71_simulation/77200_telemetry_spec.md`

## Progress

### Completed

(なし)

## Next Actions

1. `project/src/systems/ai/movement.rs` の構造を確認
2. AI移動目標更新箇所を特定
3. `AiMovementDecision` イベントの記録呼び出しを追加
4. 設定フラグ `trace.ai_decisions` による制御を実装
5. テスト実行で動作確認

## Dependencies

- **Blocked By:** 30053 (GameEvent拡張)
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

AI移動システムは最近分割されたので、正確なファイル構成を確認する必要あり。

---

## Detailed Implementation Plan

### 記録対象情報

```rust
AiMovementDecision {
    player_id: u8,
    movement_state: String,       // "Tracking" / "Idle" / "Recovering"
    ball_coming_to_me: bool,
    reaction_timer: f32,
    landing_time: Option<f32>,
    landing_position: Option<Vec3>,
    trajectory_line_z: f32,
    arrival_distance: f32,
    target_position: Vec3,
}
```

### 記録タイミング

- AI移動目標が更新されるたびに記録
- ただし、間引き設定（interval_frames）に従う
