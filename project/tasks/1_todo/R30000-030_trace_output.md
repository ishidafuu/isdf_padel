---
id: "R30000-030"
title: "CSV/JSON出力・interval_frames対応"
type: "refactor"
status: "todo"
priority: "medium"
related_task: null
spec_ids: []
blocked_by: ["R30000-029"]
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md"
tags: ["trace", "output", "csv", "json"]
parent_task_id: null
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: null
---

# Task R30000-030: CSV/JSON出力・interval_frames対応

## Summary

EventTracer に記録されたデータをCSV/JSON形式でファイル出力する。
interval_frames 設定に基づく記録間隔制御を実装する。

## Related Specifications

- プラン: `/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md`

## Progress

### Completed

(なし)

## Next Actions

1. `trace_system.rs` に CSV 出力関数追加
2. `trace_system.rs` に JSON 出力関数追加
3. `trace_output_system` で試合終了時に出力
4. interval_frames による記録間隔制御
5. `simulation_debug.ron` 更新

## Dependencies

- **Blocked By:** R30000-029
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] debug_trace.csv と debug_trace.json が生成されることを確認
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

出力形式:
- CSV: Excel/スプレッドシート向け
- JSON: プログラム解析向け
- 拡張子なし指定時は両方出力

---

## Detailed Implementation Plan

### CSV出力形式

```csv
frame,timestamp,entity,pos_x,pos_y,pos_z,vel_x,vel_y,vel_z
0,0.000,Ball,0.00,1.50,0.00,5.00,-2.00,0.50
0,0.000,Player1,-7.00,0.00,0.00,0.00,0.00,0.00
```

### JSON出力形式

```json
{
  "frames": [
    {
      "frame": 0,
      "timestamp": 0.0,
      "entities": [
        {"type": "Ball", "position": [0, 1.5, 0], "velocity": [5, -2, 0.5]}
      ],
      "events": []
    }
  ]
}
```

### interval_frames 制御

```rust
fn trace_positions_system(
    mut tracer: ResMut<EventTracer>,
    mut frame_counter: Local<u64>,
) {
    *frame_counter += 1;
    if *frame_counter % tracer.config.interval_frames as u64 != 0 {
        return;
    }
    // 記録処理
}
```
