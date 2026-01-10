---
id: "R30000-009"
title: "デッドコード削除（replay関連・未使用フィールド）"
type: "refactor"
status: "todo"
priority: "low"
related_task: "30000"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["code-quality", "cleanup"]
parent_task_id: null
created_at: "2026-01-10"
updated_at: "2026-01-10"
completed_at: null
---

# Task R30000-009: デッドコード削除（replay関連・未使用フィールド）

## Summary

コンパイル警告で検出された未使用コード・フィールドを削除し、コードベースをクリーンに保つ。

## Related Specifications

- なし（リファクタリングのみ）

## Progress

### Completed

(なし)

## Next Actions

1. 未使用関数の削除:
   - `calculate_recovery_position` in `ai_movement.rs:88`
   - `load_replay`, `load_replay_unchecked` in replay/
   - `replay_input_system`, `replay_finished_check_system`

2. 未使用構造体の削除:
   - `ReplayPlayer` struct
   - `ReplayPlaybackFinished` struct

3. 未使用フィールドの削除（または将来使用予定なら `#[allow(dead_code)]` 付与）:
   - `TrajectoryConfig.min_launch_angle`, `max_landing_deviation`
   - `ShotAttributesConfig.max_direction_error`
   - `ServeConfig.ball_spawn_offset_y`, `serve_angle`, etc.
   - `AiMovementConfig.home_position_x`, `recovery_depth`, etc.

4. 未使用 import の削除（cargo fix で自動修正可能）

5. ビルド・テスト確認（警告ゼロを目標）

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] 警告数が大幅減少
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

- replay関連コードは将来使用予定の可能性あり → 削除前にユーザー確認推奨
- `#[allow(dead_code)]` は最終手段、可能なら削除
- `cargo fix` で未使用 import は自動修正可能
