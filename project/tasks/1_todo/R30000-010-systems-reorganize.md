---
id: "R30000-010"
title: "systems/ サブディレクトリ整理"
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
tags: ["architecture", "organization"]
parent_task_id: null
created_at: "2026-01-10"
updated_at: "2026-01-10"
completed_at: null
---

# Task R30000-010: systems/ サブディレクトリ整理

## Summary

`systems/` ディレクトリに20個のファイルが散在しているため、機能カテゴリ別にサブディレクトリを作成して整理する。

## Related Specifications

- `project/docs/2_architecture/20004_ecs_overview.md`

## Progress

### Completed

(なし)

## Next Actions

1. 現在の構造を確認:
   ```
   systems/
   ├── point_judgment/ (3 files)
   ├── trajectory_calculator/ (7 files)
   └── 20 loose files
   ```

2. 以下のサブディレクトリ構成に再編成:
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
   └── debug_marker.rs
   ```

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

- 大規模な移動のため、影響範囲が広い
- 段階的に実施することも検討（1サブディレクトリずつ）
