---
id: "R30000-007"
title: "config.rs の分割"
type: "refactor"
status: "todo"
priority: "medium"
related_task: "30000"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["code-quality", "maintainability"]
parent_task_id: null
created_at: "2026-01-10"
updated_at: "2026-01-10"
completed_at: null
---

# Task R30000-007: config.rs の分割

## Summary

`resource/config.rs` が1457行と肥大化しているため、設定カテゴリ別にファイルを分割して保守性を向上させる。

## Related Specifications

- `project/docs/8_data/80101_game_constants.md`

## Progress

### Completed

(なし)

## Next Actions

1. config.rs の構造を分析し、分割単位を決定
2. 以下のカテゴリ別にファイルを作成:
   - `physics_config.rs` - PhysicsConfig, SpinPhysicsConfig
   - `court_config.rs` - CourtConfig
   - `player_config.rs` - PlayerConfig, PlayerVisualConfig
   - `ball_config.rs` - BallConfig
   - `shot_config.rs` - ShotConfig, ShotAttributesConfig, TrajectoryConfig
   - `scoring_config.rs` - ScoringConfig
   - `input_config.rs` - InputConfig, InputKeysConfig, GamepadButtonsConfig
   - `visual_config.rs` - ShadowConfig, VisualFeedbackConfig
3. mod.rs で re-export して既存の参照を維持
4. ビルド・テスト確認

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

- 現在の警告: 未使用フィールドは R30000-009 で対応
- 分割後も GameConfig は統合エントリポイントとして維持
