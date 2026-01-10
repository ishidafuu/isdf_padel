# R30000-017: systems/ai/ 簡素化

## 概要
code-simplifier を使用して `systems/ai/` を簡素化する。

## 対象ファイル
- `project/src/systems/ai/movement.rs` (330行)
- `project/src/systems/ai/serve.rs` (301行)
- `project/src/systems/ai/shot.rs` (190行)

## 現状の課題
- `movement.rs`: 着地位置計算（2次方程式）と移動ロジック混在
- `serve.rs`: ターゲット選択と準備ロジック混在

## 期待効果
- 計算ロジック分離
- 複雑度 30% 削減

## 実行方法
code-simplifier エージェントを使用

## 検証
1. `cargo build` - コンパイル確認
2. `cargo test` - テスト通過確認
3. `cargo clippy` - 警告確認

## 優先度
Tier 2（中）
