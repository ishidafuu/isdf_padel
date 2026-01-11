# B30301-001: AI短いボール時のX座標追跡

## 概要

ラリー中、AIがボールを追跡せずガタつく現象を修正。

## 問題

2つのバグが存在：

### 1. 反応遅延の無限リセット（根本原因）
`project/src/systems/ai/movement.rs` 228-230行：
```rust
if ball_coming_to_me && ai.movement_state == AiMovementState::Idle {
    ai.reaction_timer = config.ai.reaction_delay;  // 毎フレームリセット！
}
```
→ **永遠にTracking状態に移行できない**

### 2. 短いボール時のX座標固定
288-289行：
```rust
let target = Vec3::new(ai_pos.x, 0.0, target_z);
```
→ 短いボールでもX座標が固定されネット側に進入できない

## 修正内容

### 1. 反応遅延の修正（228-230行）
```rust
// Before
if ball_coming_to_me && ai.movement_state == AiMovementState::Idle {

// After - 初回のみ設定
if ball_coming_to_me && ai.movement_state == AiMovementState::Idle && ai.reaction_timer <= 0.0 {
```

### 2. 短いボール時のX座標追跡（266-295行）
```rust
let is_short = is_short_ball(ai_pos.x, ball_pos, ball_vel, gravity);
let (target_x, target_z) = if is_short {
    (ball_pos.x, ball_pos.z)  // X座標もボール位置を追跡
} else {
    (ai_pos.x, z)  // インターセプト方式
};
let target = Vec3::new(target_x, 0.0, target_z);
```

## 検証結果

ヘッドレスシミュレーションで確認：
- `state=Tracking` に正常遷移
- 短いボール時に `target.x` がボール位置を追跡

## ステータス

- [x] 実装完了
- [x] シミュレーション確認
