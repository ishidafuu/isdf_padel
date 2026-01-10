---
id: "R30000-008"
title: "components/mod.rs の分割"
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

# Task R30000-008: components/mod.rs の分割

## Summary

`components/mod.rs` が520行と肥大化しているため、コンポーネントカテゴリ別にファイルを分割して保守性を向上させる。

## Related Specifications

- `project/docs/2_architecture/209_components/`

## Progress

### Completed

(なし)

## Next Actions

1. components/mod.rs の構造を分析し、分割単位を決定
2. 以下のカテゴリ別にファイルを作成:
   - `player.rs` - Player関連コンポーネント
   - `ball.rs` - Ball関連コンポーネント
   - `court.rs` - Court関連コンポーネント
   - `input.rs` - 入力関連コンポーネント
   - `physics.rs` - 物理関連コンポーネント（Position, Velocity, Height等）
   - `game_state.rs` - ゲーム状態関連コンポーネント
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

- ECS設計原則に従い、コンポーネントは適切な粒度で分類
- 既存の use 文への影響を最小化するため re-export を活用
