---
id: "P003-005"
title: "PlayerBundle色決定の外部化"
type: "project-wide"
status: "todo"
priority: "medium"
related_task: "P003"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["refactor", "config", "externalize"]
parent_task_id: "P003"
created_at: "2026-01-08T13:52:00"
updated_at: "2026-01-08T13:52:00"
completed_at: null
---

# Task P003-005: PlayerBundle色決定の外部化

## Summary

PlayerBundle 内でハードコーディングされているプレイヤー色を外部データ化する。

## Current Problem

```rust
// components/mod.rs:370-375
// Player1: 青、Player2: 赤
let color = if id == 1 {
    Color::srgb(0.2, 0.4, 0.8)  // ❌ ハードコーディング
} else {
    Color::srgb(0.8, 0.2, 0.2)  // ❌ ハードコーディング
};
```

**違反**: CLAUDE.md ルール3「ハードコーディング禁止原則」

## Implementation Plan

### 1. データ定義（RON ファイル）

```ron
// project/assets/config/player_visual.ron
PlayerVisualConfig(
    colors: {
        Player1: (r: 0.2, g: 0.4, b: 0.8),
        Player2: (r: 0.8, g: 0.2, b: 0.2),
    },
    size: (width: 40.0, height: 60.0),
)
```

### 2. コード修正

```rust
impl PlayerBundle {
    pub fn new(id: u8, position: Vec3, config: &PlayerVisualConfig) -> Self {
        let court_side = CourtSide::from_id(id);
        let color = config.get_color(court_side);
        // ...
    }
}
```

## Modified Files

| ファイル | 変更内容 |
|---------|---------|
| `project/src/components/mod.rs` | PlayerBundle |
| `project/src/main.rs` | プレイヤー生成箇所 |
| `project/assets/config/` | 新規データファイル |

## Acceptance Criteria

- [ ] PlayerVisualConfig データ定義作成
- [ ] RON ファイル作成
- [ ] PlayerBundle から色のハードコーディング削除
- [ ] テスト通過

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## Notes

- 並列実行可
