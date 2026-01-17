---
id: "R30000-041"
title: "main 関数分割"
type: "refactor"
status: "in-review"
priority: "low"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: "refactor/R30000-041_split_main_function"
worktree_path: "../isdf_padel_R30000-041"
plan_file: null
tags: ["long-function", "code-quality", "main"]
parent_task_id: null
created_at: "2026-01-17T00:00:00+09:00"
updated_at: "2026-01-17T00:00:00+09:00"
completed_at: null
---

# Task R30000-041: main 関数分割

## Summary

2026-01-17 コード監査で検出された長い関数を分割し、可読性と保守性を向上させる。

## 対象

| ファイル | 関数 | 現在の行数 | 目標 |
|----------|------|-----------|------|
| `project/src/main.rs` | main | 109行 | 50行以下 |

## 分割方針

1. Bevy App の構築を機能単位でヘルパー関数に分割
2. プラグイン追加、リソース初期化、システム登録を整理
3. 各ヘルパー関数は単一責務（SRP）に従う

## Related Specifications

- 監査レポート: 2026-01-17

## Progress

### DONE

- [x] 現状のコード構造を分析
- [x] 分割ポイントを特定
- [x] ヘルパー関数を抽出（6つの関数に分割）
- [x] テスト実行・動作確認（162テストPASS）
- [x] ビルド・Clippy 確認

### 分割結果

| 関数名 | 行数 | 責務 |
|--------|------|------|
| `main()` | **26行** | アプリ起動フロー |
| `add_default_plugins()` | 23行 | DefaultPlugins・Asset登録 |
| `add_resources()` | 6行 | リソース初期化 |
| `add_game_plugins()` | 15行 | ゲームプラグイン追加 |
| `add_messages()` | 9行 | メッセージ登録 |
| `add_input_systems()` | 15行 | 入力システム追加 |
| `add_game_logic_systems()` | 40行 | ゲームロジックシステム追加 |

**削減効果**: main関数 109行 → 26行（-77%）

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）- 162テスト
- [x] `cargo clippy` で main.rs に警告なし（既存の未使用関数警告は範囲外）
- [x] 分割後の各関数が50行以下
- [x] in-review に移動済み
- [ ] レビュー完了

## メモ

- Effort: S（小規模）
- Bevy App 構築の典型的なパターンを適用
- main 関数は変更頻度が低いため優先度低め
