---
id: "R30000-024"
title: "Dead Code 削除（未使用フィールド/メソッド）"
type: "refactor"
status: "todo"
priority: "low"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/snoopy-crafting-knuth.md"
tags: ["audit", "dead-code", "cleanup"]
parent_task_id: null
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: null
---

# Task R30000-024: Dead Code 削除（未使用フィールド/メソッド）

## Summary

ビルド警告で検出された未使用コード（4件）を整理する。
将来使用予定のものは `#[allow(dead_code)]` で明示、不要なものは削除。

## Related Specifications

- コード監査レポート（2026-01-11）

## Progress

### Completed

(なし)

## Next Actions

1. `VolleyFactors` の未使用インポートを削除（mod.rs:30）
2. `min_launch_angle` の使用状況を確認し判断
3. `max_landing_deviation` の使用状況を確認し判断
4. `max_direction_error` の使用状況を確認し判断
5. `record_fault` メソッドの使用予定を確認し判断
6. ビルド・テスト確認

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] ビルド警告が0件（dead_code関連）
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

**対象一覧**:

| 場所 | 内容 | 判断基準 |
|------|------|----------|
| `config/mod.rs:30` | `VolleyFactors` インポート | 削除 |
| `shot_config.rs:64` | `min_launch_angle` | 仕様書にあれば `#[allow]` |
| `shot_config.rs:92` | `max_landing_deviation` | 仕様書にあれば `#[allow]` |
| `shot_config.rs:183` | `max_direction_error` | 仕様書にあれば `#[allow]` |
| `scoring.rs:285` | `record_fault` | 仕様書にあれば `#[allow]` |

---

## Detailed Implementation Plan

> 以下はプランファイル `~/.claude/plans/snoopy-crafting-knuth.md` より抜粋

### 修正方針

- 未使用インポート削除
- 未使用フィールドに `#[allow(dead_code)]` または削除
- 未使用メソッドは将来使用予定か確認後に判断

### 変更ファイル

- `src/resource/config/mod.rs`
- `src/resource/config/shot_config.rs`
- `src/resource/scoring.rs`

### 工数

S（1時間）
