---
id: "P003-006"
title: "player.id参照箇所の整理"
type: "project-wide"
status: "todo"
priority: "low"
related_task: "P003"
spec_ids: []
blocked_by: ["P003-004"]
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["refactor", "cleanup", "ECS"]
parent_task_id: "P003"
created_at: "2026-01-08T13:52:00"
updated_at: "2026-01-08T13:52:00"
completed_at: null
---

# Task P003-006: player.id参照箇所の整理

## Summary

コードベース内の `player.id` 参照を整理し、CourtSide ベースまたはマーカーコンポーネントベースに統一する。

## Current State

`player.id` を使った分岐が複数箇所に存在：

```rust
// systems/movement.rs:89
let (z_min, z_max) = get_player_z_bounds(player.id, &config);

// systems/shot_input.rs:50
info!("Player {} shot ignored: knockback active", player.id);

// systems/serve.rs:34-35
CourtSide::Player1 => 1,
CourtSide::Player2 => 2,
```

## Classification

### A. ログ出力用（許容）
```rust
info!("Player {} shot ignored", player.id);  // デバッグ用、影響なし
```

### B. ロジック分岐用（要検討）
```rust
let (z_min, z_max) = get_player_z_bounds(player.id, &config);
// → CourtSide ベースに変更可能
```

### C. ID ↔ CourtSide 変換（整理対象）
```rust
match player.id { 1 => CourtSide::Player1, _ => CourtSide::Player2 }
// → Player コンポーネントに court_side があるので直接使用可能
```

## Implementation Plan

1. **Player.id の用途を明確化**
   - ログ表示用の識別子として維持
   - ロジック分岐は `Player.court_side` を使用

2. **ヘルパー関数の統一**
   ```rust
   // Before: get_player_z_bounds(player.id, &config)
   // After:  get_player_z_bounds(player.court_side, &config)
   ```

3. **不要な変換を削除**
   ```rust
   // Before
   let side = match player.id { 1 => CourtSide::Player1, ... };

   // After
   let side = player.court_side;  // 既に持っている
   ```

## Modified Files

| ファイル | 変更内容 |
|---------|---------|
| `project/src/systems/movement.rs` | CourtSide ベースに変更 |
| `project/src/systems/knockback.rs` | CourtSide ベースに変更 |
| `project/src/systems/ai_movement.rs` | CourtSide ベースに変更 |
| `project/src/systems/serve.rs` | 不要な変換削除 |
| `project/src/systems/match_flow.rs` | CourtSide ベースに変更 |

## Acceptance Criteria

- [ ] player.id による分岐を player.court_side に置換
- [ ] ヘルパー関数のシグネチャを CourtSide ベースに変更
- [ ] 不要な id ↔ CourtSide 変換を削除
- [ ] テスト通過

## Dependencies

- **Blocked By:** P003-004（スコアリングECS化）完了後が望ましい
- **Blocks:** なし

## Notes

- P003-004 との競合に注意
