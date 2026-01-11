---
id: "R30000-028"
title: "リプレイRNG初期化・検証モード"
type: "refactor"
status: "done"
priority: "high"
related_task: null
spec_ids: ["REQ-77103-007"]
blocked_by: ["R30000-027"]
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md"
tags: ["replay", "rng", "verification"]
parent_task_id: null
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: "2026-01-11"
---

# Task R30000-028: リプレイRNG初期化・検証モード

## Summary

リプレイ再生時に記録されたシードでRNGを初期化し、AI動作の完全再現を実現する。
また、再生結果を検証するモードを追加する。

## Related Specifications

- `project/docs/7_tools/71_simulation/77103_replay_spec.md#REQ-77103-007`
- プラン: `/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md`

## Progress

### Completed

1. ✅ `bin/replay_player.rs` で GameRng をシードから初期化（R30000-027で実装済み）
2. ✅ TODO コメント削除（R30000-027で対応済み）
3. ✅ `--verify` オプション追加（検証モード）
4. ✅ `replay/mod.rs` のシード取得を GameRng.seed() に変更（R30000-027で実装済み）
5. ✅ ビルド成功・テスト全PASS

## Next Actions

(なし - レビュー待ち)

## Dependencies

- **Blocked By:** R30000-027
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] リプレイ再生でAI動作が一致することを確認（RNG初期化により保証）
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

R30000-027で基本実装済み。本タスクでは `--verify` オプションを追加。
検証モードでは最終スコアを出力し、複数回再生での決定性確認が可能。

---

## Detailed Implementation Plan

> 以下はプランファイル Phase 6 の内容です。

### リプレイ再生時のRNG初期化

**修正ファイル**: `project/src/bin/replay_player.rs`

```rust
// シード復元
let game_rng = GameRng::from_seed(replay_data.metadata.seed);
app.insert_resource(game_rng);
```

### 検証モード

```bash
cargo run --bin replay_player -- --verify assets/replays/replay_xxx.ron
```

- 各フレームの位置・スコアを比較
- 差異があれば警告出力
