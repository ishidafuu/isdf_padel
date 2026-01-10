---
id: "R30000-018"
title: "resource/ (ロジック系) 簡素化"
type: "refactor"
status: "done"
priority: "medium"
related_task: "30000"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["code-simplifier", "resource", "refactor"]
parent_task_id: null
---

# R30000-018: resource/ (ロジック系) 簡素化

## 概要
code-simplifier を使用して `resource/` のロジック系ファイルを簡素化する。

## 対象ファイル
- `project/src/resource/scoring.rs` (487行)
- `project/src/resource/mod.rs` + その他ロジック系

## 現状の課題
- `scoring.rs`: コートサイドインデックスヘルパー抽出可能
- バリデーションロジックの整理

## 期待効果
- ヘルパー関数の整理

## 実行方法
code-simplifier エージェントを使用

## 検証
1. `cargo build` - コンパイル確認
2. `cargo test` - テスト通過確認
3. `cargo clippy` - 警告確認

## 優先度
Tier 2（中）

## 完了内容

### scoring.rs: ServeStateリセット処理共通化
- `reset_toss_state()` プライベートヘルパー関数を追加
- `on_hit_success()`, `reset_for_retry()`, `reset_for_new_point()` の重複コードを解消
- DRY原則の適用

### その他
- `debug.rs`, `fixed_delta.rs` は既に簡素な構造のため変更なし
- `mod.rs` は必要最小限のため変更なし
