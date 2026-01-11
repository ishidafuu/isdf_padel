---
id: "R30000-032"
title: "AI行動ログ・ゲームイベントログ"
type: "refactor"
status: "done"
priority: "medium"
related_task: null
spec_ids: []
blocked_by: ["R30000-031"]
blocks: []
branch_name: "R30000-032"
worktree_path: "/Users/ishidafuu/Documents/repository/isdf_padel_R30000-032"
plan_file: "/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md"
tags: ["ai", "logging", "events", "scoring"]
parent_task_id: null
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: "2026-01-11"
---

# Task R30000-032: AI行動ログ・ゲームイベントログ

## Summary

AIの意思決定過程と得点理由・フォルト種別を記録するログ機能を実装する。
「なぜその動作をしたか」を追跡可能にする。

## Related Specifications

- プラン: `/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md`

## Progress

### Completed

1. `systems/ai/movement.rs` に AI 移動ログ追加
2. `systems/ai/shot.rs` に AI ショットログ追加
3. `systems/ai/serve.rs` に AI サーブログ追加（timer_init, toss, hit）
4. `point_judgment/bounce_judgment.rs` に得点イベントログ追加（BOUNCE, DoubleBounce, OwnCourtHit）
5. `point_judgment/out_judgment.rs` に得点イベントログ追加（Out, WallHit）
6. `point_judgment/net_judgment.rs` に得点イベントログ追加（Let, NetFault）
7. `main.rs` に `mod simulation` 追加（DebugLogger アクセス用）

### 備考

- PointReason enum は既存の `RallyEndReason` で代替可能なため追加不要
- DebugLogger は Option<ResMut<DebugLogger>> としてシステムに追加し、simulation モードでのみ有効

## Next Actions

レビュー待ち

## Dependencies

- **Blocked By:** R30000-031
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）- 151テストパス
- [x] [AI], [SCORING], [PHYSICS] 形式のログが出力されることを確認
- [x] in-review に移動済み
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
