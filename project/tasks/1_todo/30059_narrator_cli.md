---
id: "30059"
title: "ナラティブCLIツール完成"
type: "game-dev"
status: "todo"
priority: "medium"
related_task: null
spec_ids:
  - "REQ-77201-007"
  - "REQ-77201-008"
blocked_by:
  - "30058"
blocks:
  - "30060"
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

# Task 30059: ナラティブCLIツール完成

## Summary

マークダウン出力機能を実装し、CLIツールとして完成させる。詳細度オプションを実装。

## Related Specifications

- `project/docs/7_tools/71_simulation/77201_narrative_spec.md`

## Progress

### Completed

(なし)

## Next Actions

1. マークダウンフォーマッタを実装
2. テーブル生成ヘルパー関数を実装
3. `--detail-level` オプション処理
4. ファイル出力 / stdout 切り替え
5. エラーハンドリング改善
6. ヘルプメッセージ整備

## Dependencies

- **Blocked By:** 30058 (ラリー要約・異常フラグ)
- **Blocks:** 30060 (QAレビュースキル)

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

Phase 2 の最終タスク。これが完了したらPhase 3に進める。

---

## Detailed Implementation Plan

### CLI完成形

```bash
# 基本使用
cargo run --bin trace_narrator -- trace.jsonl -o report.md

# オプション
cargo run --bin trace_narrator -- trace.jsonl \
    --output report.md \
    --detail-level full \
    --anomaly-threshold 2.0
```

### マークダウン生成

```rust
fn format_markdown(match_data: &MatchData, options: &FormatOptions) -> String {
    let mut output = String::new();

    // ヘッダー
    output.push_str(&format_header(match_data));

    // サマリー
    output.push_str(&format_summary(match_data));

    // ラリー詳細
    for (i, rally) in match_data.rallies.iter().enumerate() {
        output.push_str(&format_rally(rally, i + 1, options));
    }

    output
}

fn format_rally(rally: &Rally, index: usize, options: &FormatOptions) -> String {
    let mut output = format!("## Rally {} (Frame {}-{})\n\n",
        index, rally.start_frame, rally.end_frame);

    // 結果
    output.push_str(&format!("**Result**: P{} wins ({})\n",
        rally.winner, rally.end_reason));

    // プレイバイプレイ（detail_level に応じて）
    if options.detail_level != DetailLevel::Summary {
        output.push_str(&format_play_by_play(rally));
    }

    // 異常
    if !rally.anomalies.is_empty() {
        output.push_str(&format_anomalies(&rally.anomalies));
    }

    output
}
```
