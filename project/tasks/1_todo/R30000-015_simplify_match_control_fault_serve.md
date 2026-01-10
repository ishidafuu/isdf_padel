# R30000-015: systems/match_control/ (fault + serve) 簡素化

## 概要
code-simplifier を使用して `match_control/fault.rs` と `serve.rs` を簡素化する。

## 対象ファイル
- `project/src/systems/match_control/fault.rs` (450行)
- `project/src/systems/match_control/serve.rs` (393行)

## 現状の課題
- `fault.rs`: 4段ネストのサービスボックス計算
- `serve.rs`: 複数の状態遷移が混在

## 期待効果
- ネスト削減
- 可読性向上

## 実行方法
code-simplifier エージェントを使用

## 検証
1. `cargo build` - コンパイル確認
2. `cargo test` - テスト通過確認
3. `cargo clippy` - 警告確認

## 優先度
Tier 1（高）
