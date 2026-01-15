---
id: "R30000-034"
title: "未使用関数の削除"
type: "refactor"
status: "done"
priority: "low"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/nifty-jingling-sifakis.md"
tags: ["dead-code", "cleanup"]
parent_task_id: null
created_at: "2026-01-15T00:00:00+09:00"
updated_at: "2026-01-15T00:00:00+09:00"
completed_at: null
---

# Task R30000-034: 未使用関数の削除

## Summary

2026-01-15 コード監査で検出された未使用関数（dead_code 警告）を削除する。

## 対象コード

| File | Line | Function | 状態 |
|------|------|----------|------|
| `project/src/systems/ai/shot.rs` | 18 | `apply_direction_variance` | 削除済み |
| `project/src/systems/ai/shot.rs` | 157 | `distance_xz` | 削除済み |
| `project/src/systems/input/shot.rs` | 142 | `distance_2d` | 削除済み |

## Related Specifications

- 監査レポート: 2026-01-15

## Progress

### Completed

- grep検索で未使用を確認
- TODOコメントなし確認
- 3関数とそのテストを削除
- ビルド成功、テスト150件全PASS

## Next Actions

- レビュー完了後にアーカイブへ移動

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] dead_code 警告が解消されていること
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

- Effort: S（小規模）
- 3関数の削除のみ
