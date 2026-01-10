---
id: "R30000-010"
title: "systems/ サブディレクトリ整理"
type: "refactor"
status: "done"
priority: "low"
related_task: "30000"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["architecture", "organization"]
parent_task_id: null
created_at: "2026-01-10"
updated_at: "2026-01-10"
completed_at: "2026-01-10"
---

# Task R30000-010: systems/ サブディレクトリ整理

## Summary

`systems/` ディレクトリに20個のファイルが散在しているため、機能カテゴリ別にサブディレクトリを作成して整理する。

## Related Specifications

- `project/docs/2_architecture/20004_ecs_overview.md`

## Progress

### Completed

- [x] ai/ サブディレクトリ作成（movement.rs, shot.rs, serve.rs）
- [x] ball/ サブディレクトリ作成（trajectory.rs, collision.rs）
- [x] input/ サブディレクトリ作成（human.rs, gamepad.rs, shot.rs）
- [x] match_control/ サブディレクトリ作成（scoring.rs, flow.rs, fault.rs, serve.rs）
- [x] player/ サブディレクトリ作成（movement.rs, jump.rs, knockback.rs）
- [x] shot/ サブディレクトリ作成（direction.rs, attributes.rs）
- [x] systems/mod.rs 更新
- [x] インポートパス修正
- [x] ビルド成功確認
- [x] テスト全PASS確認（149テスト）

## Final Structure

```
systems/
├── ai/
│   ├── mod.rs
│   ├── movement.rs
│   ├── shot.rs
│   └── serve.rs
├── ball/
│   ├── mod.rs
│   ├── trajectory.rs
│   └── collision.rs
├── input/
│   ├── mod.rs
│   ├── human.rs
│   ├── gamepad.rs
│   └── shot.rs
├── match_control/
│   ├── mod.rs
│   ├── scoring.rs
│   ├── flow.rs
│   ├── fault.rs
│   └── serve.rs
├── player/
│   ├── mod.rs
│   ├── movement.rs
│   ├── jump.rs
│   └── knockback.rs
├── shot/
│   ├── mod.rs
│   ├── direction.rs
│   └── attributes.rs
├── point_judgment/ (既存)
├── trajectory_calculator/ (既存)
├── boundary.rs
├── court_factory.rs
├── debug_marker.rs
└── mod.rs
```

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

- 全サブディレクトリを一括で移動・作成
- re-exportにより既存の参照を維持
- 一部のインポートパス修正が必要だった（super:: → crate::systems::）
