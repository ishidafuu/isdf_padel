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
