---
id: "R30000-016"
title: "systems/ball/ 簡素化"
type: "refactor"
status: "completed"
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

## 完了報告

### 変更内容
1. `BallSpinExt` トレイトを追加 (components/ball.rs)
   - `Option<&BallSpin>` から値を取得する `value_or_default()` メソッド提供
2. `trajectory.rs` のスピン取得パターンを統一 (3箇所)
   - `ball_spin.map_or(0.0, |s| s.value)` → `ball_spin.value_or_default()`

### 効果
- スピンアクセスパターンの一貫性向上
- 保守性向上（変更箇所が1箇所に集約）

### 検証
- cargo build: 成功
- cargo test: 151テスト全通過
- cargo clippy: 新規警告なし
