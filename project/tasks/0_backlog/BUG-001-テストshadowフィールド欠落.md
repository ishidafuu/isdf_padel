---
title: "テストコードにshadowフィールドが欠落"
severity: "minor"
discovered: "2026-01-07"
commit: "727dbbf"
related_feature: "30022"
status: "unreviewed"
---

# バグ報告: テストコードにshadowフィールドが欠落

## 現象

テスト実行時に `missing field shadow` エラーが発生する。

```
error[E0063]: missing field `shadow` in initializer of `GameConfig`
```

## 期待動作

テストが正常に実行される。

## 原因

コミット fa5e0ee（プレイヤー影を追加）で `ShadowConfig` が `GameConfig` に追加されたが、テストコード内で `GameConfig` を手動作成している箇所が更新されなかった。

## 発生箇所

- `project/src/systems/boundary.rs:119` - テスト内の GameConfig 初期化
- `project/src/systems/fault_judgment.rs:264` - テスト内の GameConfig 初期化

## 再現手順

1. `cargo test` を実行

## 発生条件

- 発生頻度: 常に
- 発生環境: 全環境

## 備考

サービスライン描画（30024）とは無関係の既存問題。
