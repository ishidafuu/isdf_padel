---
title: "@spec 30401_ball_spec.md 参照ファイル名不一致"
severity: "minor"
discovered: "2026-01-09"
commit: "e0b3fa1"
related_feature: "30401"
status: "done"
---

# バグ報告: @spec 30401_ball_spec.md 参照ファイル名不一致

## 現象

実装コードで `@spec 30401_ball_spec.md` を参照しているが、該当ファイルが存在しない。

**参照箇所:**
- `project/src/components/mod.rs:89`

**実際に存在するファイル:**
- `30400_overview.md`
- `30401_trajectory_spec.md`
- `30402_reflection_spec.md`
- `30403_collision_spec.md`

## 期待動作

`@spec` コメントは実際に存在する仕様書ファイル名を参照すべき。

## 原因

ファイル命名規則の変更、または記述時のミス。

## 対応方針

コードの内容を確認し、適切な仕様書を参照するように修正:

```rust
// 修正前
/// @spec 30401_ball_spec.md

// 修正後（内容に応じて選択）
/// @spec 30400_overview.md
// または
/// @spec 30401_trajectory_spec.md
```

## 発生条件

- 発生頻度: 常に（静的コード検証）
- 発生環境: 監査実行時

## 関連ファイル

- `project/src/components/mod.rs`
- `project/docs/3_ingame/304_ball/30400_overview.md`
- `project/docs/3_ingame/304_ball/30401_trajectory_spec.md`

## 備考

監査（2026-01-09）で検出。
