---
id: "R30000-011"
title: "長関数のリファクタリング"
type: "refactor"
status: "todo"
priority: "low"
related_task: "30000"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["code-quality", "readability"]
parent_task_id: null
created_at: "2026-01-10"
updated_at: "2026-01-10"
completed_at: null
---

# Task R30000-011: 長関数のリファクタリング

## Summary

50行を超える長関数を分割し、可読性と保守性を向上させる。

## Related Specifications

- なし（リファクタリングのみ）

## Progress

### Completed

(なし)

## Next Actions

1. **spawn_court (114行)** - `main.rs:196`
   - コートエンティティ生成をヘルパー関数に分割
   - 壁生成、ネット生成、境界生成を個別関数化

2. **main (108行)** - `main.rs:37`
   - プラグイン設定を別関数に抽出
   - システム登録を別関数に抽出

3. **handle_normal_shot (93行)** - `shot_direction.rs:97`
   - 方向計算ロジックを分割
   - 早期リターンでネスト削減

4. **update_debug_ui (78行)** - `presentation/debug_ui.rs:53`
   - UI セクションごとに関数分割

5. ビルド・テスト確認

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

- テスト用の `test_config()` 関数（77行）はテストコードのためスコープ外
- 関数分割時は単一責任の原則に従う
- 過度な分割は避け、論理的なまとまりを維持
