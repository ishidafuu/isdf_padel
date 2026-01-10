---
id: "R30000-011"
title: "長関数のリファクタリング"
type: "refactor"
status: "completed"
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
completed_at: "2026-01-11"
---

# Task R30000-011: 長関数のリファクタリング

## Summary

50行を超える長関数を分割し、可読性と保守性を向上させる。

## Related Specifications

- なし（リファクタリングのみ）

## Progress

### Completed

1. **spawn_court (114行→35行)** - `main.rs`
   - `spawn_rect` ヘルパー関数を追加
   - 繰り返しパターンを統一し可読性向上

2. **main (108行)** - 現状維持
   - Bevy のビルダーパターンに従った構造
   - 分割するとイディオムから外れ可読性低下のため維持

3. **handle_normal_shot (94行)** - 現状維持
   - 既に早期リターン、構造体構築、関数呼び出しで構造化済み
   - 過度な分割は避け現状維持

4. **update_debug_ui (92行→20行)** - `debug_ui.rs`
   - `format_score_text`, `format_phase_info`, `format_bounce_info`, `format_player_states` ヘルパー関数を追加
   - セクションごとに責務を分離

5. ビルド・テスト確認 ✅
   - `cargo build` 成功
   - `cargo test` 149件 PASS

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

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] in-review に移動済み
- [x] レビュー完了

## メモ

- テスト用の `test_config()` 関数（77行）はテストコードのためスコープ外
- 関数分割時は単一責任の原則に従う
- 過度な分割は避け、論理的なまとまりを維持
