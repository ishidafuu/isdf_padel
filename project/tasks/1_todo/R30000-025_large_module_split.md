---
id: "R30000-025"
title: "大規模モジュール分割（direction.rs等）"
type: "refactor"
status: "todo"
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
completed_at: null
---

# Task R30000-025: 大規模モジュール分割（direction.rs等）

## Summary

500行を超える大規模モジュール（3件）を責務ごとにサブモジュールに分割する。
可読性・保守性の向上が目的。

## Related Specifications

- コード監査レポート（2026-01-11）

## Progress

### Completed

(なし)

## Next Actions

1. `shot/direction.rs`（651行）の責務分析
2. `direction/` ディレクトリ化とサブモジュール分割
3. `match_control/scoring.rs`（562行）の責務分析
4. `scoring/` ディレクトリ化とサブモジュール分割
5. `ball/trajectory.rs`（536行）の責務分析
6. 必要に応じて分割
7. ビルド・テスト確認

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] 各サブモジュールが300行以下
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

**対象ファイル**:

| ファイル | 行数 | 分割案 |
|----------|------|--------|
| `shot/direction.rs` | 651 | `direction/mod.rs`, `calculation.rs`, `validation.rs` |
| `match_control/scoring.rs` | 562 | `scoring/mod.rs`, `point.rs`, `game.rs`, `set.rs` |
| `ball/trajectory.rs` | 536 | 責務分析後に判断 |

**注意**: 公開API（pub関数）は変更しない。内部構造のみ整理。

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
