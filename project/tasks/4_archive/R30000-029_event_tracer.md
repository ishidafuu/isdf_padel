---
id: "R30000-029"
title: "EventTracer + TraceConfig 実装"
type: "refactor"
status: "done"
priority: "medium"
related_task: null
spec_ids: []
blocked_by: []
blocks: ["R30000-030"]
branch_name: "refactor/R30000-029_event_tracer"
worktree_path: "/Users/ishidafuu/Documents/repository/isdf_padel_R30000-029"
plan_file: "/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md"
tags: ["trace", "debug", "simulation"]
parent_task_id: null
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: "2026-01-11"
---

# Task R30000-029: EventTracer + TraceConfig 実装

## Summary

フレーム単位の詳細データ記録機能を実装する。
TraceConfig（既存の設定構造）を活用し、位置・速度・イベントを記録する。

## Related Specifications

- `project/docs/7_tools/71_simulation/77100_headless_sim.md`
- プラン: `/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md`

## Progress

### Completed

1. `event_tracer.rs` を新規作成（EventTracer, FrameTrace, EntityTrace, GameEvent 型）
2. `trace_system.rs` を新規作成（TraceSystemPlugin、位置記録・イベント記録システム）
3. `mod.rs` に公開追加
4. `simulation_runner.rs` で TraceConfig 有効時に EventTracer を挿入

## Next Actions

1. `project/src/simulation/event_tracer.rs` を新規作成
2. `project/src/simulation/trace_system.rs` を新規作成
3. `project/src/simulation/mod.rs` に公開追加
4. `simulation/simulation_runner.rs` で TraceConfig 有効時に EventTracer を挿入

## Dependencies

- **Blocked By:** なし
- **Blocks:** R30000-030

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] trace.enabled: true で EventTracer にデータが記録されることを確認（frames=120, events=132）
- [x] in-review に移動済み
- [x] レビュー完了（イベントのみ記録時の問題を修正）

## メモ

TraceConfig は既に定義済み（simulation/config.rs）だが機能未実装。

---

## Detailed Implementation Plan

> 以下はプランファイル Phase 2 の内容です。

### EventTracer リソース

**ファイル**: `project/src/simulation/event_tracer.rs` (新規)

```rust
#[derive(Resource, Default)]
pub struct EventTracer {
    pub enabled: bool,
    pub frames: Vec<FrameTrace>,
    pub config: TraceConfig,
}

pub struct FrameTrace {
    pub frame: u64,
    pub timestamp: f32,
    pub entities: Vec<EntityTrace>,
    pub events: Vec<GameEvent>,
}

pub struct EntityTrace {
    pub entity_type: EntityType,
    pub position: Vec3,
    pub velocity: Vec3,
}

pub enum GameEvent {
    BallHit { player: u8, shot_type: String },
    Bounce { position: Vec3, court_side: CourtSide },
    WallReflect { position: Vec3, wall_type: String },
    Point { winner: u8, reason: String },
    Fault { fault_type: String },
    StateChange { from: String, to: String },
}
```

### トレース記録システム

- `trace_positions_system`: interval_frames ごとに位置・速度を記録
- `trace_events_system`: ゲームイベント発生時に記録
