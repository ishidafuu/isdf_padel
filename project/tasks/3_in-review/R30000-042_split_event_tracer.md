---
id: "R30000-042"
title: "event_tracer モジュール分割"
type: "refactor"
status: "in_review"
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

### DONE

- [x] 現状のコード構造を分析
- [x] 分割ポイントを特定
- [x] サブモジュールを作成
- [x] テスト実行・動作確認
- [x] ビルド・Clippy 確認

### 分割結果

| ファイル | 行数 | 責務 |
|---------|------|------|
| `types.rs` | 64行 | EntityType, EntityTrace, FrameTrace |
| `events.rs` | 101行 | GameEvent enum |
| `mod.rs` | 120行 | EventTracer + コアロジック |
| `writer.rs` | 128行 | ファイル出力処理 |
| `formatter.rs` | 289行 | CSV/JSONフォーマット |

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [x] `cargo clippy` で警告ゼロ（event_tracer関連）
- [x] 分割後の各モジュールが300行以下
- [x] in-review に移動済み
- [ ] レビュー完了

## メモ

- Effort: M（中規模）
- モジュール分割は影響範囲が広いため慎重に実施
