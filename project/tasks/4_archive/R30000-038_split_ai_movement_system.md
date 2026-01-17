---
id: "R30000-038"
title: "ai_movement_system 分割"
type: "refactor"
status: "done"
priority: "medium"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: "refactor/R30000-038_split_ai_movement_system"
worktree_path: "../isdf_padel_R30000-038"
plan_file: null
tags: ["long-function", "code-quality", "ai"]
parent_task_id: null
created_at: "2026-01-17T00:00:00+09:00"
updated_at: "2026-01-17T10:15:00+09:00"
completed_at: "2026-01-17T10:20:00+09:00"
---

# Task R30000-038: ai_movement_system 分割

## Summary

2026-01-17 コード監査で検出された長い関数を分割し、可読性と保守性を向上させる。

## 対象

| ファイル | 関数 | 現在の行数 | 目標 |
|----------|------|-----------|------|
| `project/src/systems/ai/movement.rs` | ai_movement_system | 144行 | 50行以下 |

## 分割方針

1. 責務ごとにヘルパー関数を抽出
2. 各ヘルパー関数は単一責務（SRP）に従う
3. 関数名は処理内容を明確に表現

## Related Specifications

- 監査レポート: 2026-01-17

## Progress

### DONE

- [x] 現状のコード構造を分析
- [x] 分割ポイントを特定
- [x] ヘルパー関数を抽出
- [x] テスト実行・動作確認
- [x] ビルド・Clippy 確認

### 抽出したヘルパー関数

| 関数名 | 行数 | 責務 |
|--------|------|------|
| `handle_no_ball_state` | 11 | ボール不在時のAI状態リセット |
| `is_ball_coming_to_ai` | 6 | ボール方向判定 |
| `determine_ai_target` | 25 | 状態遷移・目標位置決定 |
| `log_ai_movement_debug` | 24 | デバッグログ出力 |
| `execute_ai_movement` | 25 | 移動実行（到達判定含む） |

### 結果

- **Before**: 144行
- **After**: 47行（関数本体）、64行（シグネチャ含む）

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] `cargo clippy` で警告ゼロ（既存のdead-code警告除く）
- [x] 分割後の各関数が50行以下
- [x] in-review に移動済み
- [ ] レビュー完了

## メモ

- Effort: S（小規模）
- Bevy システム関数の分割パターンを適用
- 既存の`calculate_shot_attributes`未使用警告は別タスク対象
