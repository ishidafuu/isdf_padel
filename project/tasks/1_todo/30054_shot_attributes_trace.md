---
id: "30054"
title: "ショット属性トレース実装"
type: "game-dev"
status: "todo"
priority: "high"
related_task: null
spec_ids:
  - "REQ-77200-001"
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

# Task 30054: ショット属性トレース実装

## Summary

`calculate_shot_attributes` 関数にトレース呼び出しを追加し、ショット計算の中間係数と最終結果をEventTracerに記録する。

## Related Specifications

- `project/docs/7_tools/71_simulation/77200_telemetry_spec.md`
- `project/docs/3_ingame/306_shot/30604_shot_attributes_spec.md`

## Progress

### Completed

(なし)

## Next Actions

1. `project/src/systems/shot/attributes.rs` を開く
2. `calculate_shot_attributes` 関数の末尾にトレース呼び出しを追加
3. EventTracerリソースへのアクセス方法を検討（システムパラメータ経由）
4. 設定フラグ `trace.shot_attributes` による制御を実装
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

`calculate_shot_attributes` は純粋関数なので、EventTracerへの記録は呼び出し側（システム）で行う必要があるかもしれない。設計を検討。

---

## Detailed Implementation Plan

### 実装方針

Option A: 関数を拡張してEventTracerを受け取る
```rust
pub fn calculate_shot_attributes(
    context: &ShotContext,
    config: &ShotAttributesConfig,
    tracer: Option<&mut EventTracer>,  // 追加
) -> ShotAttributes
```

Option B: 呼び出し側システムでトレース
```rust
// systems/shot/direction.rs 等
let attrs = calculate_shot_attributes(&context, &config);
if let Some(tracer) = tracer.as_mut() {
    tracer.record_event(GameEvent::ShotAttributesCalculated { ... });
}
```

推奨: Option B（純粋関数を維持）

### 記録タイミング

- ショット実行時（`shot_direction_system` または同等システム）
- サーブ実行時（`serve_shot_system` または同等システム）
