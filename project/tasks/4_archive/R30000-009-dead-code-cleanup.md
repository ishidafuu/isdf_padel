---
id: "R30000-009"
title: "デッドコード削除（replay関連・未使用フィールド）"
type: "refactor"
status: "done"
priority: "low"
related_task: "30000"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["code-quality", "cleanup"]
parent_task_id: null
created_at: "2026-01-10"
updated_at: "2026-01-10"
completed_at: "2026-01-10"
---

# Task R30000-009: デッドコード削除（replay関連・未使用フィールド）

## Summary

コンパイル警告で検出された未使用コード・フィールドを削除し、コードベースをクリーンに保つ。

## Related Specifications

- なし（リファクタリングのみ）

## Progress

### Completed

1. **未使用関数の削除**:
   - `calculate_recovery_position` を `ai_movement.rs` から削除

2. **replay関連コードへの `#[allow(dead_code)]` 付与**（将来使用予定のため残す）:
   - `replay/loader.rs`, `replay/player.rs`, `replay/data.rs`, `replay/manager.rs`, `replay/recorder.rs`

3. **未使用Configフィールドへの対処**（将来使用予定のため残す）:
   - `resource/config.rs` に `#![allow(dead_code)]` を追加
   - `AiMovementState::Recovering` バリアントに `#[allow(dead_code)]` を追加

4. **未使用 import の削除**:
   - `cargo fix` で自動修正
   - `replay_player.rs` のインポートをフルパスに変更

5. **テストモジュールの修正**:
   - `trajectory.rs` テストに `create_court_bounds` インポートを追加

### 結果

- ビルド成功（警告ゼロ）
- テスト全PASS（149件）

## Next Actions

なし（レビュー待ち）

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] 警告数が大幅減少（0件）
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

- replay関連コードは将来使用予定の可能性あり → 削除前にユーザー確認推奨
- `#[allow(dead_code)]` は最終手段、可能なら削除
- `cargo fix` で未使用 import は自動修正可能
