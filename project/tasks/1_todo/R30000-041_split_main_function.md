---
id: "R30000-041"
title: "main 関数分割"
type: "refactor"
status: "todo"
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

### TODO

- [ ] 現状のコード構造を分析
- [ ] 分割ポイントを特定
- [ ] ヘルパー関数を抽出
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
- [ ] 分割後の各関数が50行以下
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

- Effort: S（小規模）
- Bevy App 構築の典型的なパターンを適用
- main 関数は変更頻度が低いため優先度低め
