---
id: "R30000-039"
title: "analyze_rallies 分割"
type: "refactor"
status: "in-review"
priority: "low"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: "refactor/R30000-039_split_analyze_rallies"
worktree_path: "../isdf_padel_R30000-039"
plan_file: null
tags: ["long-function", "code-quality", "trace-narrator"]
parent_task_id: null
created_at: "2026-01-17T00:00:00+09:00"
updated_at: "2026-01-17T00:00:00+09:00"
completed_at: null
---

# Task R30000-039: analyze_rallies 分割

## Summary

2026-01-17 コード監査で検出された長い関数を分割し、可読性と保守性を向上させる。

## 対象

| ファイル | 関数 | 現在の行数 | 目標 |
|----------|------|-----------|------|
| `project/src/bin/trace_narrator/analyzer.rs` | analyze_rallies | 115行 | 50行以下 |

## 分割方針

1. 責務ごとにヘルパー関数を抽出
2. 各ヘルパー関数は単一責務（SRP）に従う
3. 関数名は処理内容を明確に表現

## Related Specifications

- 監査レポート: 2026-01-17

## Progress

### DONE

- [x] 現状のコード構造を分析
- [x] 分割ポイントを特定
- [x] ヘルパー関数を抽出（CurrentRallyState構造体導入）
- [x] テスト実行・動作確認（12テスト全PASS）
- [x] ビルド・Clippy 確認

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] `cargo clippy` で対象ファイル警告ゼロ（既存警告は別ファイル）
- [x] 分割後の各関数が50行以下（analyze_rallies: 115行→37行）
- [x] in-review に移動済み
- [ ] レビュー完了

## メモ

- Effort: S（小規模）
- trace_narrator はデバッグツールなので優先度低め
