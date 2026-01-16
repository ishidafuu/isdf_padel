---
id: "30054"
title: "ショット属性トレース実装"
type: "game-dev"
status: "completed"
priority: "high"
related_task: null
spec_ids:
  - "REQ-77200-001"
  - "REQ-77200-002"
blocked_by:
  - "30053"
blocks: []
branch_name: "task/30054-shot-attributes-trace"
worktree_path: "/Users/ishidafuu/Documents/repository/isdf_padel_30054"
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

- [x] TraceConfigに`shot_attributes`フィールド追加
- [x] `ShotAttributesDetail`構造体と`calculate_shot_attributes_detail`関数追加
- [x] `ShotAttributesCalculatedEvent`追加
- [x] `handle_normal_shot`でイベント発行
- [x] `trace_shot_attributes_system`追加
- [x] ビルド成功
- [x] 全テストPASS（150 passed）

## 実装内容

### 修正ファイル

| ファイル | 変更内容 |
|---------|---------|
| `simulation/config.rs` | TraceConfigに`shot_attributes`フィールド追加 |
| `systems/shot/attributes.rs` | `ShotAttributesDetail`と`calculate_shot_attributes_detail`追加 |
| `core/events.rs` | `ShotAttributesCalculatedEvent`追加 |
| `systems/shot/direction/mod.rs` | システムパラメータ拡張、イベント発行追加 |
| `systems/shot/direction/normal_shot.rs` | 詳細計算・イベント発行実装 |
| `simulation/trace_system.rs` | `trace_shot_attributes_system`追加、プラグイン登録 |

### 設計方針

Option B（呼び出し側システムでトレース）を採用：
- 純粋関数`calculate_shot_attributes`は維持
- `calculate_shot_attributes_detail`で中間係数を含む詳細を取得
- `ShotAttributesCalculatedEvent`を介してシステム間で情報を伝達
- `trace_shot_attributes_system`でEventTracerに記録

## Dependencies

- **Blocked By:** 30053 (GameEvent拡張) ✅ 完了
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] in-review に移動済み
- [ ] レビュー完了
