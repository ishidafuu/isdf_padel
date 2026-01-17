---
id: "R30000-042"
title: "event_tracer モジュール分割"
type: "refactor"
status: "todo"
priority: "medium"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: "refactor/R30000-042_split_event_tracer"
worktree_path: "../isdf_padel_R30000-042"
plan_file: null
tags: ["large-module", "code-quality", "simulation"]
parent_task_id: null
created_at: "2026-01-17T00:00:00+09:00"
updated_at: "2026-01-17T00:00:00+09:00"
completed_at: null
---

# Task R30000-042: event_tracer モジュール分割

## Summary

2026-01-17 コード監査で検出された大きなモジュールを分割し、可読性と保守性を向上させる。

## 対象

| ファイル | 現在の行数 | 目標 |
|----------|-----------|------|
| `project/src/simulation/event_tracer.rs` | 669行 | 300行以下 |

## 分割方針

1. 責務ごとにサブモジュールを作成
2. 以下の構成を検討:
   - `event_tracer/mod.rs` - 公開インターフェース
   - `event_tracer/types.rs` - データ型定義
   - `event_tracer/writer.rs` - ファイル出力
   - `event_tracer/formatter.rs` - フォーマット処理
3. 各サブモジュールは300行以下を目標

## Related Specifications

- 監査レポート: 2026-01-17

## Progress

### TODO

- [ ] 現状のコード構造を分析
- [ ] 分割ポイントを特定
- [ ] サブモジュールを作成
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
- [ ] 分割後の各モジュールが300行以下
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

- Effort: M（中規模）
- モジュール分割は影響範囲が広いため慎重に実施
