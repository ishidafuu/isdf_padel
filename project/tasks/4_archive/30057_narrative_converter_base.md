---
id: "30057"
title: "ナラティブ変換基盤"
type: "game-dev"
status: "done"
priority: "medium"
related_task: null
spec_ids:
  - "REQ-77201-001"
  - "REQ-77201-002"
blocked_by:
  - "30056"
blocks:
  - "30058"
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/optimized-strolling-puppy.md"
tags:
  - "telemetry"
  - "llm-qa"
  - "phase2"
parent_task_id: null
created_at: "2026-01-16T16:00:00+09:00"
updated_at: "2026-01-16T17:30:00+09:00"
completed_at: "2026-01-16T19:00:00+09:00"
---

# Task 30057: ナラティブ変換基盤

## Summary

JSONLテレメトリログを読み込み、内部データ構造に変換する基盤を実装する。

## Related Specifications

- `project/docs/7_tools/71_simulation/77201_narrative_spec.md`

## Progress

### Completed

- ✅ `project/src/bin/trace_narrator.rs` 新規作成
- ✅ `project/src/bin/trace_narrator/types.rs` データ構造定義（serde対応）
- ✅ `project/src/bin/trace_narrator/parser.rs` JSONL/JSON配列形式の両対応パーサー
- ✅ `project/src/bin/trace_narrator/mod.rs` モジュール定義
- ✅ Cargo.toml にバイナリ定義追加
- ✅ JSONL読み込み機能を実装（REQ-77201-001）
- ✅ FrameTrace構造体の定義（bin用に別定義、serde::Deserialize対応）
- ✅ イベントの時系列ソート機能を実装（REQ-77201-002）
- ✅ 基本的なCLI引数パース（clap使用）
- ✅ テスト4件追加・全PASS

## Dependencies

- **Blocked By:** 30056 (TraceConfig拡張 - Phase 1完了) ✅
- **Blocks:** 30058

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`） - 154テスト通過
- [x] in-review に移動済み
- [x] レビュー完了

## 実装詳細

### ファイル構成

```
project/src/bin/
├── trace_narrator.rs           # エントリポイント（CLIパース、統計表示）
└── trace_narrator/
    ├── parser.rs               # JSONL/JSON配列形式の両対応パーサー
    └── types.rs                # serde対応データ構造
```

※ モジュールは `#[path]` アトリビュートで直接参照（mod.rs不要）

### 対応フォーマット

1. **JSON配列形式**（headless_simの出力形式）
   ```json
   {"frames": [{"frame": 0, "timestamp": 0.0, ...}, ...]}
   ```

2. **JSONL形式**（仕様書の想定形式）
   ```jsonl
   {"frame": 0, "timestamp": 0.0, ...}
   {"frame": 1, "timestamp": 0.016, ...}
   ```

### CLIオプション

```bash
cargo run --bin trace_narrator -- input.jsonl -o output.md -d normal -a 1.5
```

| オプション | 説明 | デフォルト |
|-----------|------|-----------|
| `input` | 入力ファイル | 必須 |
| `-o, --output` | 出力ファイル | stdout |
| `-d, --detail-level` | 詳細度 | normal |
| `-a, --anomaly-threshold` | 異常検出閾値 | 1.5 |
| `--include-physics` | 物理詳細含める | false |
| `--rally-only` | ラリー要約のみ | false |

## メモ

- simulation/event_tracer.rs の構造体は再利用せず、bin用にserde対応の別定義とした
- JSON配列形式を先に試行し、失敗したらJSONL形式として処理する堅牢な設計
- 次タスク30058でマークダウン出力機能を実装予定
