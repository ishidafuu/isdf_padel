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
