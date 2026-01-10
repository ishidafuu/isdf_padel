---
id: "B30700-001"
title: "scoring.rs ブール式ロジック修正"
type: "bugfix"
status: "in-review"
priority: "critical"
related_task: null
spec_ids:
  - "30701_point_spec.md"
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/tender-strolling-whale.md"
tags:
  - "clippy"
  - "audit"
parent_task_id: null
created_at: "2026-01-10"
updated_at: "2026-01-10"
completed_at: null
---

# Task B30700-001: scoring.rs ブール式ロジック修正

## Summary

Clippy が検出した CRITICAL レベルのブール式ロジックバグを修正する。
冗長な条件式を簡略化。

## 問題

**ファイル**: `project/src/resource/scoring.rs:176-177`

```rust
scorer_index >= win_index && opponent_index < win_index - 1
    || scorer_index >= win_index
```

**Clippy 指摘**: 後半の `scorer_index >= win_index` が前半の条件を完全に包含しているため、式全体が `scorer_index >= win_index` と等価。

## 修正内容

```rust
// MVP v0.1: デュースなし（40から得点で即勝利）
// win_index = 4 の場合、index 3 (40) から得点で勝利
scorer_index >= win_index
```

## Related Specifications

- `project/docs/3_ingame/307_scoring/30701_point_spec.md`

## Progress

### Completed

- [x] 冗長なブール式を簡略化（176-177行 → 174行）
- [x] 未使用変数 `opponent_index` を削除

## Next Actions

1. `scoring.rs:176-177` の冗長なブール式を簡略化
2. `cargo clippy` でエラー解消を確認

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）- 149 passed
- [x] Clippy エラー解消（`cargo clippy`）
- [x] in-review に移動済み
- [ ] レビュー完了

## メモ

監査（/code-audit）で検出された CRITICAL 問題。
