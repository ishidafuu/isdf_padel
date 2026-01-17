---
id: "R30000-043"
title: "未使用コード削除"
type: "refactor"
status: "todo"
priority: "low"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: "refactor/R30000-043_remove_unused_code"
worktree_path: "../isdf_padel_R30000-043"
plan_file: null
tags: ["dead-code", "code-quality"]
parent_task_id: null
created_at: "2026-01-17T00:00:00+09:00"
updated_at: "2026-01-17T00:00:00+09:00"
completed_at: null
---

# Task R30000-043: 未使用コード削除

## Summary

2026-01-17 コード監査で検出された未使用コードを削除し、コードベースをクリーンに保つ。

## 対象

| ファイル | 関数 | 状態 |
|----------|------|------|
| `project/src/systems/shot/attributes.rs` | calculate_shot_attributes | 未使用 |

## 削除方針

1. 参照箇所がないことを確認
2. 関連するテストも削除
3. 関連する型・定数も不要なら削除

## Related Specifications

- 監査レポート: 2026-01-17

## Progress

### TODO

- [ ] 参照箇所の確認
- [ ] 関数の削除
- [ ] 関連コードの削除（必要に応じて）
- [ ] テスト実行・動作確認
- [ ] ビルド・Clippy 確認

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] `cargo clippy` で警告ゼロ
- [ ] 未使用コード警告が解消
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

- Effort: XS（極小規模）
- 削除前に参照箇所を十分に確認
