---
id: "B30000-001"
title: "Cargo.toml edition を 2021 に修正"
type: "bugfix"
status: "done"
priority: "critical"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["build", "critical"]
parent_task_id: null
created_at: "2026-01-10T13:45:00+09:00"
updated_at: "2026-01-10T13:45:00+09:00"
completed_at: "2026-01-10T17:55:00+09:00"
---

# Task B30000-001: Cargo.toml edition を 2021 に修正

## Summary

コード監査で検出されたビルドエラーを修正する。
`Cargo.toml` の `edition = "2024"` は無効な値であり、`replay_player` バイナリのビルドが失敗している。

## 問題

```
Cargo.toml:4
edition = "2024"  # Rust 2024 edition は存在しない
```

### エラーメッセージ

```
error[E0433]: failed to resolve: could not find `replay` in `padel_game`
  --> src/bin/replay_player.rs:25:17
```

## 修正内容

```diff
- edition = "2024"
+ edition = "2021"
```

## Related Specifications

- なし（ビルド基盤の問題）

## Progress

### Completed

1. `Cargo.toml` の `edition` を `"2021"` に修正
2. `cargo check` で全バイナリのビルド確認（成功）
3. `cargo test` でテスト実行（149テスト全PASS）

## Next Actions

- レビュー待ち

## Dependencies

- **Blocked By:** なし
- **Blocks:** 全てのビルド・テスト作業

## 完了チェックリスト

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）- 149テスト全PASS
- [x] 全バイナリ（padel_game, headless_sim, replay_player）のビルド確認
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

- コード監査（2026-01-10）で検出
- 深刻度: CRITICAL（ビルド失敗）
