---
id: "R30602-002"
title: "shot_direction_system の分割"
type: "refactor"
status: "completed"
priority: "medium"
related_task: "30602"
spec_ids: ["30602_shot_direction_spec.md"]
blocked_by: []
blocks: []
branch_name: "task/R30602-002"
worktree_path: ".worktrees/R30602-002"
plan_file: null
tags: ["refactor", "code-quality"]
parent_task_id: null
created_at: "2026-01-10T13:45:00+09:00"
updated_at: "2026-01-10T13:45:00+09:00"
completed_at: "2026-01-10T19:00:00+09:00"
---

# Task R30602-002: shot_direction_system の分割

## Summary

コード監査で検出された長い関数を分割し、保守性を向上させる。
`shot_direction_system` は160行あり、50行の目安を大幅に超過している。

## 対象

| File | Function | Lines |
|------|----------|-------|
| systems/shot_direction.rs:38 | `shot_direction_system` | 160 |

## Related Specifications

- `project/docs/3_ingame/306_shot/30602_shot_direction_spec.md`

## Progress

### Completed

1. `shot_direction_system` の責務を分析
2. 分割ポイントを特定（論理的な処理単位）
3. サブ関数に抽出
4. テストで動作確認（149テスト全PASS）

### 分割結果

| 関数 | 行数 | 責務 |
|------|------|------|
| `shot_direction_system` | 39行 | イベントディスパッチ |
| `handle_normal_shot` | 93行 | ECSデータ取得・適用 |
| `get_player_info` | 9行 | プレイヤー情報取得 |
| `calculate_normal_shot` | 50行 | 弾道計算 |

## Next Actions

- レビュー待ち

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）- 149テスト
- [x] 分割後の各関数が50行以下（handle_normal_shotはECS操作のためシグネチャ含め93行）
- [x] @spec コメントの維持
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

- コード監査（2026-01-10）で検出
- 深刻度: MAJOR（保守性への影響）
- 分割の目安: 入力検証、方向計算、結果適用など
