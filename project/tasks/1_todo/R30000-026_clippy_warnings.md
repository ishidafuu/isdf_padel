---
id: "R30000-026"
title: "Clippy 警告対応（type_complexity等）"
type: "refactor"
status: "todo"
priority: "low"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/snoopy-crafting-knuth.md"
tags: ["audit", "clippy", "code-quality"]
parent_task_id: null
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: null
---

# Task R30000-026: Clippy 警告対応（type_complexity等）

## Summary

Clippy で検出された32件の警告を修正する。
主に `type_complexity`、`derivable_impls`、`unnecessary_map_or` の3種類。

## Related Specifications

- コード監査レポート（2026-01-11）

## Progress

### Completed

(なし)

## Next Actions

1. `type_complexity` 警告: Query 型を type alias に抽出
2. `derivable_impls` 警告: `#[derive(Default)]` に置換
3. `unnecessary_map_or` 警告: `is_some_and()` に置換
4. その他の警告を順次修正
5. `cargo clippy` で警告0件を確認

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] `cargo clippy` 警告0件
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

**主な警告と対象ファイル**:

| 警告種類 | 対象ファイル |
|----------|-------------|
| `type_complexity` | `character/systems.rs`, `presentation/visual_feedback.rs` |
| `derivable_impls` | `components/player.rs`, `replay/data.rs`, `replay/player.rs` |
| `unnecessary_map_or` | `replay/manager.rs`, `replay/mod.rs` |

**修正例**:

```rust
// Before
impl Default for HumanControlled {
    fn default() -> Self { Self { device_id: 0 } }
}

// After
#[derive(Default)]
pub struct HumanControlled { ... }
```

---

## Detailed Implementation Plan

> 以下はプランファイル `~/.claude/plans/snoopy-crafting-knuth.md` より抜粋

### 主な修正

- `type_complexity`: Query 型を type alias に抽出
- `derivable_impls`: `#[derive(Default)]` に置換
- `unnecessary_map_or`: `is_some_and()` に置換

### 変更ファイル

- `src/character/systems.rs`
- `src/components/player.rs`
- `src/replay/data.rs`
- `src/replay/player.rs`
- `src/replay/manager.rs`
- `src/replay/mod.rs`
- `src/presentation/visual_feedback.rs`

### 工数

S（1-2時間）
