---
title: "80102_character_parts.md データ仕様書が不存在"
severity: "minor"
discovered: "2026-01-09"
commit: "e0b3fa1"
related_feature: "30042"
status: "done"
---

# バグ報告: 80102_character_parts.md データ仕様書が不存在

## 現象

実装コードで `@data 80102_character_parts.md` を参照しているが、該当ファイルが `project/docs/8_data/` に存在しない。

**参照箇所:**
- `project/src/character/bundle.rs:16`
- `project/src/character/bundle.rs:26`

## 期待動作

`@data` コメントで参照しているデータ仕様書は `8_data/` ディレクトリに存在すべき。

## 原因

30042（パーツ分離キャラクターシステム）実装時にデータ仕様書の作成が漏れた。

## 対応方針

以下のいずれかを選択:

1. **データ仕様書を作成**: `project/docs/8_data/80102_character_parts.md` を作成
2. **参照を修正**: 既存の仕様書（31001_parts_spec.md など）を参照するように修正

## 発生条件

- 発生頻度: 常に（静的コード検証）
- 発生環境: 監査実行時

## 関連ファイル

- `project/src/character/bundle.rs`
- `project/docs/3_ingame/310_character/31001_parts_spec.md`

## 備考

監査（2026-01-09）で検出。
