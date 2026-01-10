# R30602-001: shot_direction_system 関数分割

## 概要

`shot_direction_system` 関数（177行）を適切なサイズに分割し、保守性を向上させる。

## 検出日

2026-01-10

## 検出元

`/code-audit` フル監査

## 問題詳細

**ファイル**: `project/src/systems/shot_direction.rs`
**関数**: `shot_direction_system`
**行数**: 177行（推奨: 50行以下）

## 分割案

1. `handle_normal_shot()` - 通常ショット処理
2. `handle_serve_shot()` - サーブショット処理（既存: 61行）
3. `calculate_shot_trajectory()` - 軌道計算
4. `apply_shot_attributes()` - 属性適用

## 優先度

low（機能に影響なし、保守性改善のみ）

## 工数見積

M（2-4時間）

## 関連仕様書

- project/docs/3_ingame/306_shot_system/30602_shot_direction_spec.md
