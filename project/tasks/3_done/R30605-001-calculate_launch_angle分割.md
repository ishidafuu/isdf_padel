# R30605-001: calculate_launch_angle 関数分割

## 概要

`calculate_launch_angle` 関数（159行）を適切なサイズに分割し、保守性を向上させる。

## 検出日

2026-01-10

## 検出元

`/code-audit` フル監査

## 問題詳細

**ファイル**: `project/src/systems/trajectory_calculator.rs`
**関数**: `calculate_launch_angle`
**行数**: 159行（推奨: 50行以下）

## 分割案

1. `validate_trajectory_input()` - 入力検証
2. `calculate_min_angle()` - 最小角度計算
3. `calculate_max_angle()` - 最大角度計算
4. `interpolate_angle()` - 角度補間
5. `apply_spin_effect()` - スピン効果適用

## 優先度

low（機能に影響なし、保守性改善のみ）

## 工数見積

M（2-4時間）

## 関連仕様書

- project/docs/3_ingame/306_shot_system/30605_trajectory_calculation_spec.md
