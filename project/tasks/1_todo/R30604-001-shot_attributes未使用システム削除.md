---
id: "R30604-001"
title: "shot_attributes未使用システム削除"
type: "refactor"
status: "todo"
priority: "medium"
related_task: "30028"
spec_ids: ["30604"]
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["cleanup", "dead-code", "shot-system"]
parent_task_id: null
audit_source: "2026-01-08"
severity: "minor"
category: "code-quality"
created_at: "2026-01-08T00:00:00+09:00"
updated_at: "2026-01-08T00:00:00+09:00"
completed_at: null
---

# Task R30604-001: shot_attributes未使用システム削除

## Summary

`shot_attributes.rs` 内の未使用システム関数とリソースフィールドを削除する。
これらは仕様策定時に定義されたが、現在の実装では使用されていない。

## 検出された問題

### 未使用システム関数

| 行 | 関数名 | 状態 |
|----|--------|------|
| 84 | `track_shot_button_system()` | 未使用 |
| 340 | `update_bounce_state_timer_system()` | 未使用 |
| 352 | `handle_ground_bounce_event_system()` | 未使用 |
| 367 | `reset_bounce_state_on_shot_system()` | 未使用 |

### 未使用リソースフィールド

| 行 | フィールド | 状態 |
|----|-----------|------|
| 77 | `ShotButtonState::player1_holding` | 未読み取り |
| 77 | `ShotButtonState::player2_holding` | 未読み取り |

**深刻度**: MINOR

## 修正方針

### 判断基準

1. **v0.2以降で使用予定** → `#[allow(dead_code)]` を付与してコメントで理由記載
2. **使用予定なし** → 削除

### 確認事項

- `30604_shot_attributes_spec.md` を確認し、これらの機能が将来実装予定か確認
- v0.2スコープ（`30010_v02_scope.md`）に含まれているか確認

## Related Specifications

- [30604_shot_attributes_spec.md](../../docs/3_ingame/306_shot_system/30604_shot_attributes_spec.md)
- [30010_v02_scope.md](../../docs/3_ingame/30010_v02_scope.md)

## Progress

### Completed

（なし）

## Next Actions

1. 仕様書で将来使用予定を確認
2. 削除対象を決定
3. 削除または `#[allow(dead_code)]` 付与
4. ビルド確認

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] shot_attributes.rs の未使用警告が解消
- [ ] in-review に移動済み
- [ ] レビュー完了
