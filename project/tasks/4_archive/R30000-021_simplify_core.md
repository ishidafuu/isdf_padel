---
id: "R30000-021"
title: "core/ 簡素化"
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
tags: ["code-simplifier", "core", "refactor"]
parent_task_id: null
---

# R30000-021: core/ 簡素化

## 概要
code-simplifier を使用して `core/` を簡素化する。

## 対象ファイル
- `project/src/core/` (1,056行)
- 特に `wall.rs` (378行)、`events.rs` (dead_code多数)

## 現状の課題
- `wall.rs`: 反射計算をユーティリティへ抽出可能
- `events.rs`: 15+ の `#[allow(dead_code)]` マーカー

## 期待効果
- dead_code 整理
- ユーティリティ抽出

## 実行方法
code-simplifier エージェントを使用

## 検証
1. `cargo build` - コンパイル確認
2. `cargo test` - テスト通過確認
3. `cargo clippy` - 警告確認

## 優先度
Tier 3（低）

## 実行結果
**結論**: core/ ディレクトリは既に適切に整理されており、大幅な簡素化は不要

### 分析結果
- **events.rs**: dead_code は意図的（イベントフィールドは将来のログ/デバッグ機能用）
- **wall.rs**: WallReflection 構造体が既にユーティリティとして適切に整理済み
- **テスト用メソッド**: ユーザー指示により保持

### 検証結果
- ✅ cargo build: 成功（core/ 関連警告なし）
- ✅ cargo test: 151 テスト全通過
- ✅ cargo clippy: core/ 関連警告なし
