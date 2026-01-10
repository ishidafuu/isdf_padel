---
id: "R30000-032"
title: "AI行動ログ・ゲームイベントログ"
type: "refactor"
status: "todo"
priority: "medium"
related_task: null
spec_ids: []
blocked_by: ["R30000-031"]
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md"
tags: ["ai", "logging", "events", "scoring"]
parent_task_id: null
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: null
---

# Task R30000-032: AI行動ログ・ゲームイベントログ

## Summary

AIの意思決定過程と得点理由・フォルト種別を記録するログ機能を実装する。
「なぜその動作をしたか」を追跡可能にする。

## Related Specifications

- プラン: `/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md`

## Progress

### Completed

(なし)

## Next Actions

1. `systems/ai/movement.rs` に AI 移動ログ追加
2. `systems/ai/shot.rs` に AI ショットログ追加
3. `systems/ai/serve.rs` に AI サーブログ追加
4. `point_judgment/*.rs` に得点イベントログ追加
5. `core/events.rs` に PointReason enum 追加

## Dependencies

- **Blocked By:** R30000-031
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] [AI], [SCORING], [PHYSICS] 形式のログが出力されることを確認
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

ログ出力例:
```
[AI] P1 target=(5.00,0.00,1.20) reason=Intercept ball_pred=(3.50,0.00,1.20)
[AI] P1 shot decision: SHOOT (distance=0.45, cooldown=0.00)
[PHYSICS] BOUNCE: pos=(2.10,0.00,-0.50), court_side=Left, count=1
[SCORING] POINT: P2 wins (reason=SecondBounce)
```

---

## Detailed Implementation Plan

### AI行動ログ

**修正ファイル**: `systems/ai/movement.rs`

```rust
logger.log_ai(&format!(
    "P{} target=({:.2},{:.2},{:.2}) reason={:?} ball_pred=({:.2},{:.2},{:.2})",
    player_number, target.x, target.y, target.z,
    decision_reason, predicted.x, predicted.y, predicted.z
));
```

### ゲームイベントログ

**修正ファイル**: `systems/point_judgment/*.rs`

```rust
logger.log_scoring(&format!(
    "OUT: ball at ({:.2},{:.2},{:.2}), court_side={:?}",
    pos.x, pos.y, pos.z, court_side
));
```

### PointReason enum

```rust
pub enum PointReason {
    Ace,
    DoubleFault,
    OutOfBounds { position: Vec3 },
    NetFault,
    SecondBounce,
    BodyHit,
    ServiceFault,
}
```
