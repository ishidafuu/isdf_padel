---
id: "30058"
title: "ラリー要約・異常フラグ"
type: "game-dev"
status: "todo"
priority: "medium"
related_task: null
spec_ids:
  - "REQ-77201-003"
  - "REQ-77201-004"
  - "REQ-77201-005"
  - "REQ-77201-006"
blocked_by:
  - "30057"
blocks:
  - "30059"
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/optimized-strolling-puppy.md"
tags:
  - "telemetry"
  - "llm-qa"
  - "phase2"
parent_task_id: null
created_at: "2026-01-16T16:00:00+09:00"
updated_at: "2026-01-16T16:00:00+09:00"
completed_at: null
---

# Task 30058: ラリー要約・異常フラグ

## Summary

Pointイベントでラリーを分割し、各ラリーの統計を計算。異常値を検出してフラグ付け。

## Related Specifications

- `project/docs/7_tools/71_simulation/77201_narrative_spec.md`

## Progress

### Completed

(なし)

## Next Actions

1. ラリー境界検出ロジックを実装
2. ラリー統計計算（ショット数、平均パワー等）
3. PhysicsAnomalyイベントのハイライト処理
4. 統計的異常検出（平均±1.5σ）
5. 異常リストの生成

## Dependencies

- **Blocked By:** 30057 (ナラティブ変換基盤)
- **Blocks:** 30059

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

(なし)

---

## Detailed Implementation Plan

### ラリー構造

```rust
struct Rally {
    start_frame: u64,
    end_frame: u64,
    duration_secs: f32,
    winner: u8,
    end_reason: String,
    shots: Vec<ShotEvent>,
    bounces: Vec<BounceEvent>,
    anomalies: Vec<Anomaly>,
    stats: RallyStats,
}

struct RallyStats {
    shot_count: u32,
    p1_avg_power: f32,
    p2_avg_power: f32,
    p1_avg_accuracy: f32,
    p2_avg_accuracy: f32,
}

struct Anomaly {
    frame: u64,
    severity: Severity,  // Warning / Error
    description: String,
    expected: f32,
    actual: f32,
}
```

### 異常検出ロジック

```rust
fn detect_statistical_anomalies(shots: &[ShotEvent], threshold: f32) -> Vec<Anomaly> {
    let powers: Vec<f32> = shots.iter().map(|s| s.power).collect();
    let mean = statistical_mean(&powers);
    let std = statistical_std(&powers);

    shots.iter()
        .filter(|s| (s.power - mean).abs() > threshold * std)
        .map(|s| Anomaly { ... })
        .collect()
}
```
