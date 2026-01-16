---
id: "30056"
title: "TraceConfig拡張"
type: "game-dev"
status: "todo"
priority: "medium"
related_task: null
spec_ids:
  - "REQ-77200-002"
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

# Task 30056: TraceConfig拡張

## Summary

TraceConfigに新イベントカテゴリのフラグを追加し、テレメトリ出力を細かく制御可能にする。

## Related Specifications

- `project/docs/7_tools/71_simulation/77200_telemetry_spec.md`

## Progress

### Completed

(なし)

## Next Actions

1. `project/src/simulation/config.rs` を開く
2. `TraceConfig` struct に以下のフィールドを追加:
   - `shot_attributes: bool`
   - `ai_decisions: bool`
   - `physics_anomalies: bool`
3. デフォルト値を設定（全て `true`）
4. RON設定ファイル例を更新
5. ビルド・テスト確認

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

設定変更だけなので比較的軽いタスク。

---

## Detailed Implementation Plan

### TraceConfig拡張

```rust
pub struct TraceConfig {
    // 既存フィールド
    pub enabled: bool,
    pub position: bool,
    pub velocity: bool,
    pub events: bool,
    pub interval_frames: u32,

    // 新規フィールド
    pub shot_attributes: bool,    // ショット属性記録
    pub ai_decisions: bool,       // AI決定記録
    pub physics_anomalies: bool,  // 物理異常記録
}
```

### 設定ファイル例

```ron
trace: (
    enabled: true,
    position: true,
    velocity: true,
    events: true,
    interval_frames: 1,
    // 新規
    shot_attributes: true,
    ai_decisions: true,
    physics_anomalies: true,
)
```
