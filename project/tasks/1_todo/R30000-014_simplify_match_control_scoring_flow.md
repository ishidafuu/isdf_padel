---
id: "R30000-014"
title: "systems/match_control/ (scoring + flow) 簡素化"
type: "refactor"
status: "todo"
priority: "high"
related_task: "30000"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["code-simplifier", "match_control", "refactor"]
parent_task_id: null
---

# R30000-014: systems/match_control/ (scoring + flow) 簡素化

## 概要
code-simplifier を使用して `match_control/scoring.rs` と `flow.rs` を簡素化する。

## 対象ファイル
- `project/src/systems/match_control/scoring.rs` (562行)
- `project/src/systems/match_control/flow.rs` (325行)

## 現状の課題
- `scoring.rs`: ラリー終了・ゲーム勝利・セット勝利ロジック混在
- `flow.rs`: 状態遷移ロジックが複雑

## 期待効果
- 関心の分離
- テスタビリティ向上

## 実行方法
code-simplifier エージェントを使用

## 検証
1. `cargo build` - コンパイル確認
2. `cargo test` - テスト通過確認
3. `cargo clippy` - 警告確認

## 優先度
Tier 1（高）
