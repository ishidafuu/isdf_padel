---
id: "R30000-012"
title: "systems/shot/direction.rs 簡素化"
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

# R30000-012: systems/shot/direction.rs 簡素化

## 概要
code-simplifier を使用して `systems/shot/direction.rs` を簡素化する。

## 対象ファイル
- `project/src/systems/shot/direction.rs` (651行)

## 現状の課題
- 複数責務混在（ハンドラ、軌道計算、デバッグ更新）
- `handle_normal_shot`, `handle_serve_shot` が大きい
- 7個のヘルパー関数 + 13個のテスト関数

## 期待効果
- 責務分離
- 可読性向上

## 実行方法
code-simplifier エージェントを使用

## 検証
1. `cargo build` - コンパイル確認
2. `cargo test` - テスト通過確認
3. `cargo clippy` - 警告確認

## 優先度
Tier 1（高）
