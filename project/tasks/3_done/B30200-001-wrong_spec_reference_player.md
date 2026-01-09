---
title: "@spec 30200_player_overview.md 参照ファイル名不一致"
severity: "minor"
discovered: "2026-01-09"
commit: "e0b3fa1"
related_feature: "30200"
status: "done"
---

# バグ報告: @spec 30200_player_overview.md 参照ファイル名不一致

## 現象

実装コードで `@spec 30200_player_overview.md` を参照しているが、実際のファイル名は `30200_overview.md`。

**参照箇所:**
- `project/src/components/mod.rs:39`
- `project/src/components/mod.rs:355`
- `project/src/main.rs:122`

## 期待動作

`@spec` コメントは実際に存在する仕様書ファイル名を参照すべき。

## 原因

ファイル命名規則の変更、または記述時のミス。

## 対応方針

以下の `@spec` コメントを修正:

```rust
// 修正前
/// @spec 30200_player_overview.md

// 修正後
/// @spec 30200_overview.md
```

## 発生条件

- 発生頻度: 常に（静的コード検証）
- 発生環境: 監査実行時

## 関連ファイル

- `project/src/components/mod.rs`
- `project/src/main.rs`
- `project/docs/3_ingame/302_player/30200_overview.md`

## 備考

監査（2026-01-09）で検出。
