---
id: "R30000-030"
title: "CSV/JSON出力・interval_frames対応"
type: "refactor"
status: "done"
priority: "medium"
related_task: null
spec_ids: []
blocked_by: ["R30000-029"]
blocks: []
branch_name: "task/R30000-030"
worktree_path: "/Users/ishidafuu/Documents/repository/isdf_padel_R30000-030"
plan_file: "/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md"
tags: ["trace", "output", "csv", "json"]
parent_task_id: null
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: "2026-01-11"
---

# Task R30000-030: CSV/JSON出力・interval_frames対応

## Summary

EventTracer に記録されたデータをCSV/JSON形式でファイル出力する。
interval_frames 設定に基づく記録間隔制御を実装する。

## Related Specifications

- プラン: `/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md`

## Progress

### Completed

- CSV出力関数を `event_tracer.rs` に追加
- JSON出力関数を `event_tracer.rs` に追加
- `write_to_file()` で拡張子に応じた形式選択を実装
- `simulation_runner.rs` で試合終了時にトレースファイル出力
- interval_frames による記録間隔制御（既に R30000-029 で実装済み）

## Next Actions

(レビュー待ち)

## Dependencies

- **Blocked By:** R30000-029
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] debug_trace.csv が生成されることを確認
- [x] in-review に移動済み
- [x] レビュー完了

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
