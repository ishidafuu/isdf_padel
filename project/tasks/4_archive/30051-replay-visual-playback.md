---
id: "30051"
title: "リプレイ表示付き再生機能"
type: "game-dev"
status: "done"
priority: "medium"
related_task: null
spec_ids:
  - "REQ-77103-050"
  - "REQ-77103-051"
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags:
  - "replay"
  - "extended"
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: "2026-01-11"
---

# Task 30051: リプレイ表示付き再生機能

## Summary

ヘッドレスモードで記録されたリプレイを、ウィンドウ表示付きで等倍速再生する機能を実装する。現在の `replay_player.rs` は MinimalPlugins を使用しているため画面表示がない。DefaultPlugins に切り替え、描画系システムを追加することで視覚的な再生を可能にする。

## Related Specifications

- `project/docs/7_tools/71_simulation/77103_replay_spec.md`
  - REQ-77103-050: リプレイモード状態（Extended）
  - REQ-77103-051: リプレイ選択UI（Extended、本タスクでは対象外）

## Progress

### Completed

1. `replay_viewer.rs` 作成（`src/bin/replay_viewer.rs`）
   - DefaultPlugins（ウィンドウ表示付き）を使用
   - 1280x720 ウィンドウで視覚的再生
   - ESCキーで終了
   - **AI再シミュレーション方式**: 同一シードでAI動作を再現（入力再生ではない）
2. 描画系システム追加
   - `sync_transform_system`: 論理座標→表示座標変換
   - `spawn_ball_shadow_system`, `spawn_player_shadow_system`: 影スポーン
   - `sync_shadow_system`, `despawn_ball_shadow_system`: 影同期・削除
   - 視覚フィードバック（`player_hold_visual_system`, `ball_spin_color_system`等）
3. setup関数追加（カメラ、コート、プレイヤーのスポーン）
   - 両プレイヤーに`AiController`を付与（ヘッドレスシミュレータと同様）
4. Cargo.toml に `replay_viewer` バイナリ追加
5. **HeadlessPlugins にリプレイ記録機能追加**
   - `headless_plugins.rs` に `ReplayRecordPlugin` 追加
   - `simulation_runner.rs` にタイムアウト時の手動リプレイ保存を追加
   - `assets/replays/` にリプレイファイルが自動生成される

## Next Actions

(レビュー待ち)

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）- 152 passed; 0 failed
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

### 技術的アプローチ

**重要: AI再シミュレーション方式**

AIは `ai_movement_system` で直接 `LogicalPosition` を操作するため、`InputState` は使用しない。
したがって、記録されたリプレイには実質的な入力データがない（空の入力のみ）。

解決策として、リプレイファイルからシード値を取得し、同一シードでAI再シミュレーションを行う：

```rust
// リプレイのシード値でGameRngを初期化
let game_rng = GameRng::from_seed(replay_data.metadata.seed);

// 両プレイヤーにAiControllerを付与
commands.entity(player1_entity).insert(AiController { ... });
commands.entity(player2_entity).insert(AiController { ... });

// AIシステムで再シミュレーション
app.add_systems(Update, (
    ai_movement_system,
    ai_shot_system.run_if(in_state(MatchFlowState::Rally)),
    // ...描画系システム
));
```

### CLI使用例

```bash
# 1. ヘッドレスシミュレータでリプレイ生成
cargo run --bin headless_sim

# 2. 表示付きリプレイ再生（AI再シミュレーション）
cargo run --bin replay_viewer -- assets/replays/replay_20260111_183157.replay

# 詳細出力付き
cargo run --bin replay_viewer -- -v assets/replays/replay_XXXXXX.replay
```

---

## Detailed Implementation Plan

### 1. ファイル作成

`project/src/bin/replay_viewer.rs` を作成

### 2. 主要な変更点

| 項目 | replay_player.rs | replay_viewer.rs |
|------|------------------|------------------|
| プラグイン | MinimalPlugins | DefaultPlugins |
| 描画 | なし | sync_transform, shadow系 |
| setup | なし | カメラ、コート、プレイヤー |
| ウィンドウ | なし | 1280x720 |

### 3. コードフロー

1. CLIでリプレイファイル指定
2. リプレイデータをロード
3. DefaultPlugins でウィンドウ作成
4. setup でゲームオブジェクトをスポーン
5. replay_input_system で入力を注入
6. 描画システムで視覚化
7. 試合終了または再生完了で終了
