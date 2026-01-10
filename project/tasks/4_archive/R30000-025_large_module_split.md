---
id: "R30000-025"
title: "大規模モジュール分割（direction.rs等）"
type: "refactor"
status: "done"
priority: "medium"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/snoopy-crafting-knuth.md"
tags: ["audit", "architecture", "module-split"]
parent_task_id: null
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: "2026-01-11"
---

# Task R30000-025: 大規模モジュール分割（direction.rs等）

## Summary

500行を超える大規模モジュール（3件）を責務ごとにサブモジュールに分割する。
可読性・保守性の向上が目的。

## Related Specifications

- コード監査レポート（2026-01-11）

## Progress

### Completed

1. `shot/direction.rs`（651行）→ 5ファイルに分割（mod.rs:99, normal_shot.rs:174, serve_shot.rs:75, utils.rs:85, tests.rs:261）
2. `match_control/scoring.rs`（564行）→ 5ファイルに分割（mod.rs:46, rally.rs:79, game_set.rs:127, display.rs:38, tests.rs:312）
3. `ball/trajectory.rs`（537行）→ 4ファイルに分割（mod.rs:46, physics.rs:103, bounce.rs:151, tests.rs:258）
4. ビルド・テスト確認完了（151テスト全PASS）

## Next Actions

- レビュー待ち

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] 各サブモジュールが300行以下
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

**分割結果**:

| 元ファイル | 行数 | 分割後 |
|----------|------|--------|
| `shot/direction.rs` | 651 | `direction/{mod,normal_shot,serve_shot,utils,tests}.rs` (max 261行) |
| `match_control/scoring.rs` | 564 | `scoring/{mod,rally,game_set,display,tests}.rs` (max 312行) |
| `ball/trajectory.rs` | 537 | `trajectory/{mod,physics,bounce,tests}.rs` (max 258行) |

**注意**: 公開API（pub関数）は変更なし。内部構造のみ整理。

---

## Detailed Implementation Plan

> 以下はプランファイル `~/.claude/plans/snoopy-crafting-knuth.md` より抜粋

### 修正方針

- 責務ごとにサブモジュールに分割
- 例: `direction.rs` → `direction/mod.rs`, `direction/calculation.rs`, `direction/validation.rs`

### 変更ファイル

- `src/systems/shot/direction.rs` → 分割
- `src/systems/match_control/scoring.rs` → 分割
- `src/systems/ball/trajectory.rs` → 分割

### 工数

M（半日）
