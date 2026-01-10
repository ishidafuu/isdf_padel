---
id: "B30802-001"
title: "visual_feedback.rs デッドコード削除"
type: "bugfix"
status: "completed"
priority: "critical"
related_task: null
spec_ids:
  - "30802_visual_feedback_spec.md"
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
completed_at: "2026-01-11"
---

# Task B30802-001: visual_feedback.rs デッドコード削除

## Summary

Clippy が検出した CRITICAL レベルのデッドコード（無意味な代入）を削除する。

## 問題

**ファイル**: `project/src/presentation/visual_feedback.rs:30-33`

```rust
let color = sprite.color;
// Entity ID を取得するために再度クエリが必要だが、
// ここでは commands を使って後から追加する
sprite.color = color; // 現在の色を維持
color
```

**Clippy 指摘**: `sprite.color` を読んで同じ値を再代入している（swap のように見える無意味なコード）。

## 修正内容

```rust
// OriginalColor がない場合は現在の色を使用
sprite.color
```

## Related Specifications

- `project/docs/3_ingame/308_presentation/30802_visual_feedback_spec.md`

## Progress

### Completed

1. デッドコード削除（visual_feedback.rs:30-35）

## Next Actions

1. `visual_feedback.rs:30-33` の無意味な代入を削除
2. `cargo clippy` でエラー解消を確認

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] Clippy エラー解消（`cargo clippy`）
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

監査（/code-audit）で検出された CRITICAL 問題。
