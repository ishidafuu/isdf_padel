---
id: "30057"
title: "ナラティブ変換基盤"
type: "game-dev"
status: "todo"
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
updated_at: "2026-01-16T16:00:00+09:00"
completed_at: null
---

# Task 30057: ナラティブ変換基盤

## Summary

JSONLテレメトリログを読み込み、内部データ構造に変換する基盤を実装する。

## Related Specifications

- `project/docs/7_tools/71_simulation/77201_narrative_spec.md`

## Progress

### Completed

(なし)

## Next Actions

1. `project/src/bin/trace_narrator.rs` を新規作成
2. JSONL読み込み機能を実装
3. FrameTrace構造体の定義（または既存を再利用）
4. イベントの時系列ソート機能を実装
5. 基本的なCLI引数パース（clap使用）

## Dependencies

- **Blocked By:** 30056 (TraceConfig拡張 - Phase 1完了)
- **Blocks:** 30058

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

simulation/event_tracer.rs の構造体を再利用できるか検討。bin用に別定義が必要かもしれない。

---

## Detailed Implementation Plan

### ファイル構成

```
project/src/bin/
├── trace_narrator.rs           # エントリポイント
└── trace_narrator/
    ├── mod.rs
    ├── parser.rs              # JSONL解析
    ├── types.rs               # データ構造
    └── ...
```

### 基本構造

```rust
// bin/trace_narrator.rs
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// 入力JSONLファイル
    input: PathBuf,

    /// 出力ファイル（省略時はstdout）
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let frames = parse_jsonl(&args.input)?;
    // ...
}
```
