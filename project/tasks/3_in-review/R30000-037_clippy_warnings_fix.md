---
id: "R30000-037"
title: "Clippy 警告対応"
type: "refactor"
status: "in-review"
priority: "low"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: "refactor/R30000-037_clippy_warnings_fix"
worktree_path: "../isdf_padel_R30000-037"
plan_file: "/Users/ishidafuu/.claude/plans/nifty-jingling-sifakis.md"
tags: ["clippy", "code-quality"]
parent_task_id: null
created_at: "2026-01-15T00:00:00+09:00"
updated_at: "2026-01-16T00:00:00+09:00"
completed_at: null
---

# Task R30000-037: Clippy 警告対応

## Summary

2026-01-15 コード監査で検出された Clippy 警告を解消し、コード品質を向上させる。

## 対象警告

### 引数が多すぎる関数（6件）

| 関数 | 対応 |
|------|------|
| calculate_tracking_target | `#[allow(clippy::too_many_arguments)]` |
| ai_shot_system | `#[allow(clippy::too_many_arguments)]` |
| point_end_to_next_system | `#[allow(clippy::too_many_arguments)]` |
| serve_toss_input_system | `#[allow(clippy::too_many_arguments)]` |
| movement_system | `#[allow(clippy::too_many_arguments)]` |
| net_fault_judgment_system | `#[allow(clippy::too_many_arguments)]` |

### 複雑な型定義（6件）

| 関数 | 対応 |
|------|------|
| double_bounce_judgment_system | `#[allow(clippy::type_complexity)]` |
| own_court_hit_judgment_system | `#[allow(clippy::type_complexity)]` |
| let_judgment_system | `#[allow(clippy::type_complexity)]` |
| net_fault_judgment_system | `#[allow(clippy::type_complexity)]` |
| out_of_bounds_judgment_system | `#[allow(clippy::type_complexity)]` |
| wall_hit_judgment_system | `#[allow(clippy::type_complexity)]` |

### その他の警告

| 警告 | 対応 |
|------|------|
| impl can be derived (2件) | Default derive + #[default] 属性 |
| useless use of vec! (2件) | 配列リテラルに変更 |
| doc list item without indentation | 空行追加 |
| module has same name as containing module | mod tests 削除 |
| to_* method taking &self on Copy type | self に変更 |
| 未使用コード (simulation/) | #![allow(dead_code)] |

## Related Specifications

- 監査レポート: 2026-01-15

## Progress

### Completed

- [x] impl can be derived 警告修正（2件）
- [x] 引数が多すぎる関数の修正（6件）
- [x] 複雑な型定義の修正（6件）
- [x] useless vec! 警告修正（2件）
- [x] 未使用コードの対応
- [x] その他の警告修正
- [x] 最終ビルド・テスト確認

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）- 150件
- [x] `cargo clippy` で警告ゼロ
- [x] in-review に移動済み
- [ ] レビュー完了

## メモ

- Effort: M（中規模）
- Bevy のシステム関数は引数が多くなりがちなので `#[allow]` で対応
- simulation/ モジュールは将来の統合に向けて実装済みだが未使用のため `#![allow(dead_code)]`
