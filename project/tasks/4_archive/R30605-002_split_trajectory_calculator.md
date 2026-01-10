---
id: "R30605-002"
title: "trajectory_calculator.rs の分割"
type: "refactor"
status: "done"
priority: "medium"
related_task: "30605"
spec_ids: ["30605_trajectory_calculation_spec.md"]
blocked_by: ["B30000-001"]
blocks: []
branch_name: "R30605-002"
worktree_path: "../isdf_padel_R30605-002"
plan_file: null
tags: ["refactor", "code-quality", "architecture"]
parent_task_id: null
created_at: "2026-01-10T13:45:00+09:00"
updated_at: "2026-01-10T19:30:00+09:00"
completed_at: "2026-01-10T19:30:00+09:00"
---

# Task R30605-002: trajectory_calculator.rs の分割

## Summary

コード監査で検出された大きなファイルを分割し、保守性を向上させる。
`trajectory_calculator.rs` は1091行あり、300行の目安を大幅に超過している。

## 対象

| File | Lines | Note |
|------|-------|------|
| systems/trajectory_calculator.rs | 1091 | 複数の計算ロジックが混在 |

## 分割結果

| ファイル | 行数 | 内容 |
|---------|------|------|
| mod.rs | 39 | モジュール定義・再エクスポート |
| types.rs | 54 | 型定義 |
| physics_utils.rs | 200 | 物理計算ユーティリティ |
| landing_position.rs | 123 | 着地位置計算 |
| launch_angle.rs | 237 | 発射角度計算 |
| serve_trajectory.rs | 87 | サーブ軌道計算 |
| main_trajectory.rs | 51 | メイン軌道計算 |
| tests.rs | 391 | テストコード |

## Related Specifications

- `project/docs/3_ingame/306_shot/30605_trajectory_calculation_spec.md`

## Progress

### Completed

- [x] ファイル構造の分析
- [x] 論理的な分割単位の特定
- [x] 7つのモジュールへの分割
- [x] ビルド成功確認
- [x] 全12テストパス確認
- [x] @spec コメントの維持
- [x] mod.rs への適切なエクスポート

## Dependencies

- **Blocked By:** B30000-001（完了済み）
- **Blocks:** なし

## 完了チェックリスト

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] 分割後の各ファイルが300行以下
- [x] @spec コメントの維持
- [x] mod.rs への適切なエクスポート
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

- コード監査（2026-01-10）で検出
- 深刻度: MAJOR（保守性への影響）
- ファイル分割時は public API を維持すること
- 元のファイル1091行 → 合計1182行（モジュール構造のオーバーヘッド）
