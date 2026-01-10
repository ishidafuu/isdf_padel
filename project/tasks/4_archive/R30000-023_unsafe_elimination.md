---
id: "R30000-023"
title: "unsafe ブロック排除（simulation_runner.rs）"
type: "refactor"
status: "completed"
priority: "high"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/snoopy-crafting-knuth.md"
tags: ["audit", "unsafe", "thread-safety"]
parent_task_id: null
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: "2026-01-11"
---

# Task R30000-023: unsafe ブロック排除（simulation_runner.rs）

## Summary

`src/simulation/simulation_runner.rs` で使用されている `static mut` + `unsafe` パターンを安全な実装に置換する。
現状はデバッグ用途だが、スレッドセーフでないため未定義動作のリスクがある。

## Related Specifications

- コード監査レポート（2026-01-11）

## Progress

### Completed

- `static mut LAST_LOG_TIME` → `Local<f32>` に置換
- `static mut LAST_FLOW_STATE` → `Local<Option<MatchFlowState>>` に置換
- `static mut LAST_PHASE` → `Local<Option<RallyPhase>>` に置換
- 全ての `unsafe` ブロック除去完了

## Next Actions

(レビュー待ち)

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] `unsafe` が simulation_runner.rs から除去されていること
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

**変更対象箇所**（4箇所）:
- L321-327: `LAST_LOG_TIME`
- L366-372: `LAST_FLOW_STATE`
- L376-382: `LAST_PHASE`

**推奨パターン**: Bevy の Local リソースとして実装するのが最もクリーン

---

## Detailed Implementation Plan

> 以下はプランファイル `~/.claude/plans/snoopy-crafting-knuth.md` より抜粋

### 修正方針

- `static mut` を `std::sync::OnceLock` または `RefCell` に置換
- または Local リソースとして Bevy の Resource に移行

### 変更ファイル

- `src/simulation/simulation_runner.rs`

### 工数

S（1-2時間）
