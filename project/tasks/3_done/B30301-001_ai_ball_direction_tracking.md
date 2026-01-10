---
id: "B30301-001"
title: "AI移動 - ボール飛行方向による追跡判定"
type: "bugfix"
status: "done"
priority: "medium"
related_task: "30301"
spec_ids: ["30301"]
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["ai", "movement"]
created_at: "2026-01-10T10:55:00+09:00"
updated_at: "2026-01-10T10:55:00+09:00"
completed_at: "2026-01-10T10:55:00+09:00"
---

# Task B30301-001: AI移動 - ボール飛行方向による追跡判定

## 説明

AIが相手の打球が打たれた瞬間、着地予測点ではなくボールの現在位置に向かって移動してしまうバグを修正。

## 背景

### 現状

`ai_movement.rs` でボールの**位置**で判定しているため、相手が打った直後はボールがまだ相手コート側にあり、着地予測が行われない。

```rust
// 問題のあるコード
let ball_on_my_side = match player.court_side {
    CourtSide::Left => ball_pos.x < 0.0,   // ボール位置で判定
    CourtSide::Right => ball_pos.x > 0.0,
};
```

### 改修理由

ボールの位置ではなく飛行方向（速度のX成分）で判定することで、打球直後から正しく着地予測点へ移動できるようにする。

## 実装内容

- [x] 判定ロジックを `ball_on_my_side`（位置判定）から `ball_coming_to_me`（飛行方向判定）に変更
- [x] 着地予測失敗時のフォールバックを `ball_pos` から待機位置（`idle_pos`）に変更

## 対象ファイル

- `project/src/systems/ai_movement.rs`

## 完了チェックリスト

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] 完了

## メモ

修正後のコード:
```rust
// 修正後: ボールの速度方向で判定
let ball_coming_to_me = match player.court_side {
    CourtSide::Left => ball_vel.x < 0.0,   // 左に向かっている
    CourtSide::Right => ball_vel.x > 0.0,  // 右に向かっている
};
```

## 依存関係

- **ブロック**: なし
- **ブロックされる**: なし
- **関連仕様**: project/docs/3_ingame/303_ai/30301_ai_movement_spec.md
