---
id: "R30000-016"
title: "systems/ball/ 簡素化"
type: "refactor"
status: "todo"
priority: "medium"
related_task: "30000"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["code-simplifier", "ball", "refactor"]
parent_task_id: null
---

# R30000-016: systems/ball/ 簡素化

## 概要
code-simplifier を使用して `systems/ball/` を簡素化する。

## 対象ファイル
- `project/src/systems/ball/trajectory.rs` (536行)
- `project/src/systems/ball/collision.rs` (302行)

## 現状の課題
- `trajectory.rs`: 6システム混在（重力、位置更新、バウンス、壁反射、エアドラッグ、スピン減衰）
- スピン関連ロジックが各所に分散

## 期待効果
- 物理システムの分離
- 複雑度 25% 削減

## 実行方法
code-simplifier エージェントを使用

## 検証
1. `cargo build` - コンパイル確認
2. `cargo test` - テスト通過確認
3. `cargo clippy` - 警告確認

## 優先度
Tier 2（中）
