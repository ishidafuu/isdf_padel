---
id: "R30000-014"
title: "systems/match_control/ (scoring + flow) 簡素化"
type: "refactor"
status: "done"
priority: "high"
related_task: "30000"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["code-simplifier", "match_control", "refactor"]
parent_task_id: null
---

# R30000-014: systems/match_control/ (scoring + flow) 簡素化

## 概要
code-simplifier を使用して `match_control/scoring.rs` と `flow.rs` を簡素化する。

## 対象ファイル
- `project/src/systems/match_control/scoring.rs` (562行)
- `project/src/systems/match_control/flow.rs` (325行)

## 現状の課題
- `scoring.rs`: ラリー終了・ゲーム勝利・セット勝利ロジック混在
- `flow.rs`: 状態遷移ロジックが複雑

## 期待効果
- 関心の分離
- テスタビリティ向上

## 実行方法
code-simplifier エージェントを使用

## 検証
1. `cargo build` - コンパイル確認
2. `cargo test` - テスト通過確認
3. `cargo clippy` - 警告確認

## 優先度
Tier 1（高）

## 完了内容

### flow.rs の簡素化
- プレイヤー位置リセットロジックの重複排除
- `reset_player_positions` ヘルパー関数を追加
- `match_start_system` と `point_end_to_next_system` で共通化

### scoring.rs の分析結果
- 既に良好な構造（関心分離済み）
- `rally_end_system`, `handle_game_win`, `handle_set_win`, `handle_point_scored` の分離が適切
- Bevy ECS の DI パターンにより、これ以上の分離は不適切と判断

### 検証結果
- `cargo build`: 成功
- `cargo test`: 151テスト全て成功
