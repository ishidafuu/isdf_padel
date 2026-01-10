# R30000-020: presentation/ + character/ 簡素化

## 概要
code-simplifier を使用して `presentation/` と `character/` を簡素化する。

## 対象ファイル
- `project/src/presentation/` (517行)
- `project/src/character/` (900行)

## 現状の課題
- `debug_ui.rs`: 9個のテキストフォーマット関数
- `animation.rs`: 補間ロジックが attributes.rs と重複

## 期待効果
- 共通補間ユーティリティ
- ~100行削減

## 実行方法
code-simplifier エージェントを使用

## 検証
1. `cargo build` - コンパイル確認
2. `cargo test` - テスト通過確認
3. `cargo clippy` - 警告確認

## 優先度
Tier 3（低）
