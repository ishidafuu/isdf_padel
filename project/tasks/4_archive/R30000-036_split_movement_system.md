---
id: "R30000-036"
title: "movement_system 分割"
type: "refactor"
status: "done"
priority: "medium"
related_task: null
spec_ids:
  - "REQ-30201"
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/nifty-jingling-sifakis.md"
tags: ["long-function", "refactor", "player"]
parent_task_id: null
created_at: "2026-01-15T00:00:00+09:00"
updated_at: "2026-01-15T00:00:00+09:00"
completed_at: "2026-01-16T00:00:00+09:00"
---

# Task R30000-036: movement_system 分割

## Summary

2026-01-15 コード監査で検出された長大関数 `movement_system`（113行）を論理的な単位に分割し、可読性・保守性を向上させる。

## 対象コード

| File | Function | Lines |
|------|----------|-------|
| `project/src/systems/player/movement.rs` | `movement_system` | **113行** |

## Related Specifications

- `project/docs/3_ingame/302_player/30201_movement_spec.md`

## Progress

### Completed

- [x] 関数の責務を分析
- [x] 論理的な分割ポイントを特定
- [x] ヘルパー関数に分割:
  - `calculate_movement_velocity` (34行): 入力正規化・速度計算・最大速度制限
  - `calculate_serve_position_constraints` (28行): サーブ待機中の位置制約
- [x] `movement_system`: 113行 → 77行に削減
- [x] テストで動作確認（150 passed）

## Next Actions

(レビュー待ち)

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] 分割後のヘルパー関数が50行以下であること（34行、28行）
- [x] @spec コメントが維持されていること
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

- Effort: S（小規模）
- R30000-035 より小さいため工数は少ない
