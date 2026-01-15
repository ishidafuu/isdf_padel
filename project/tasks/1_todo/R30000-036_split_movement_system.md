---
id: "R30000-036"
title: "movement_system 分割"
type: "refactor"
status: "todo"
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
completed_at: null
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

(なし)

## Next Actions

1. 関数の責務を分析
2. 論理的な分割ポイントを特定:
   - 入力処理
   - 速度計算
   - 位置更新
3. ヘルパー関数に分割
4. テストで動作確認

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] 分割後の各関数が50行以下であること
- [ ] @spec コメントが維持されていること
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

- Effort: S（小規模）
- R30000-035 より小さいため工数は少ない
