# R31000-001: Character仕様書リンク切れ修正

## 概要

監査で検出されたcharacterモジュール仕様書内のリンク切れを修正する。

## 検出日

2026-01-10

## 検出元

`/code-audit` フル監査

## 問題詳細

### 1. 20001_ecs_architecture.md（存在しない）

**参照元**:
- `project/docs/3_ingame/310_character/31002_animation_spec.md:258`
- `project/docs/3_ingame/310_character/31001_parts_spec.md:218`

**正しいパス**: `20004_ecs_overview.md`

### 2. 80102_character_parts.md（存在しない）

**参照元**:
- `project/docs/3_ingame/310_character/31001_parts_spec.md:211`
- `project/docs/3_ingame/310_character/31001_parts_spec.md:219`

**対応**: リンク削除（データは80101_game_constants.mdに統合済み）

## 対応内容

1. `31002_animation_spec.md` のリンク修正
   - `20001_ecs_architecture.md` → `20004_ecs_overview.md`

2. `31001_parts_spec.md` のリンク修正
   - `20001_ecs_architecture.md` → `20004_ecs_overview.md`
   - `80102_character_parts.md` への参照を削除または `80101_game_constants.md` に変更

## 優先度

medium

## 工数見積

S（30分以内）

## 関連仕様書

- project/docs/3_ingame/310_character/31001_parts_spec.md
- project/docs/3_ingame/310_character/31002_animation_spec.md
