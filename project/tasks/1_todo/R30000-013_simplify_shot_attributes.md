---
id: "R30000-013"
title: "systems/shot/attributes.rs 簡素化"
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
tags: ["code-simplifier", "shot", "refactor"]
parent_task_id: null
---

# R30000-013: systems/shot/attributes.rs 簡素化

## 概要
code-simplifier を使用して `systems/shot/attributes.rs` を簡素化する。

## 対象ファイル
- `project/src/systems/shot/attributes.rs` (499行)

## 現状の課題
- 6種の類似補間関数（`get_*_factors`）
- `#[allow(dead_code)]` マーカー（3箇所のバウンス状態システム）
- 汎用補間ユーティリティ抽出可能

## 期待効果
- 重複削減 ~150行

## 実行方法
code-simplifier エージェントを使用

## 検証
1. `cargo build` - コンパイル確認
2. `cargo test` - テスト通過確認
3. `cargo clippy` - 警告確認

## 優先度
Tier 1（高）
