# R30000-019: resource/config/ 簡素化

## 概要
code-simplifier を使用して `resource/config/` を簡素化する。

## 対象ファイル
- `project/src/resource/config/` (13ファイル、~1,300行)
- 特に `shot_config.rs` (427行)

## 現状の課題
- 設定ファイル間での類似パターン
- 検証ロジックの重複可能性

## 期待効果
- 設定読み込みパターンの統一

## 実行方法
code-simplifier エージェントを使用

## 検証
1. `cargo build` - コンパイル確認
2. `cargo test` - テスト通過確認
3. `cargo clippy` - 警告確認

## 優先度
Tier 2（中）
