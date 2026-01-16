---
id: "R30000-035"
title: "ai_movement_system 分割"
type: "refactor"
status: "done"
priority: "medium"
related_task: null
spec_ids:
  - "REQ-30301-v05"
  - "REQ-30301-v07"
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/nifty-jingling-sifakis.md"
tags: ["long-function", "refactor", "ai"]
parent_task_id: null
created_at: "2026-01-15T00:00:00+09:00"
updated_at: "2026-01-16T00:00:00+09:00"
completed_at: "2026-01-16T00:00:00+09:00"
---

# Task R30000-035: ai_movement_system 分割

## Summary

2026-01-15 コード監査で検出された長大関数 `ai_movement_system`（205行）を論理的な単位に分割し、可読性・保守性を向上させる。

## 対象コード

| File | Function | Lines |
|------|----------|-------|
| `project/src/systems/ai/movement.rs` | `ai_movement_system` | **205行 → 145行** |

## Related Specifications

- `project/docs/3_ingame/303_ai/30301_ai_movement_spec.md`

## Progress

### Completed

- [x] 関数の責務を分析
- [x] 論理的な分割ポイントを特定
- [x] 4つのヘルパー関数を追加
  - `update_reaction_timer` (21行): 反応遅延タイマー更新
  - `detect_lock_state_change` (7行): 目標ロック状態変化検出
  - `calculate_tracking_target` (47行): 追跡目標位置計算
  - `calculate_arrival_distance` (18行): 到達距離計算
- [x] ビルド成功
- [x] テスト全PASS (6 tests)
- [x] mainにマージ完了

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] 分割後の各ヘルパー関数が50行以下であること
- [x] @spec コメントが維持されていること
- [x] mainにマージ完了

## メモ

- Effort: M（中規模）
- メインのシステム関数は145行（ECSのforループ構造上、これ以上の分割は過度の抽象化）
- 全ヘルパー関数は50行以下
- 動作に影響なくリファクタリング完了
