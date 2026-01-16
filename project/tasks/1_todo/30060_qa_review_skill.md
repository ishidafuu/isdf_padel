---
id: "30060"
title: "QAレビュースキル設計"
type: "game-dev"
status: "todo"
priority: "medium"
related_task: null
spec_ids:
  - "REQ-77202-001"
  - "REQ-77202-002"
  - "REQ-77202-003"
  - "REQ-77202-007"
blocked_by:
  - "30059"
blocks:
  - "30061"
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/optimized-strolling-puppy.md"
tags:
  - "telemetry"
  - "llm-qa"
  - "phase3"
parent_task_id: null
created_at: "2026-01-16T16:00:00+09:00"
updated_at: "2026-01-16T16:00:00+09:00"
completed_at: null
---

# Task 30060: QAレビュースキル設計

## Summary

Claude Codeスキルとして `/qa-review` コマンドを実装。基本的なスキル構造と入力処理を設計。

## Related Specifications

- `project/docs/7_tools/71_simulation/77202_qa_review_spec.md`

## Progress

### Completed

(なし)

## Next Actions

1. `.claude/commands/qa-review.md` を新規作成
2. スキルの基本構造を定義（引数、説明）
3. 入力処理ロジックを設計
4. 基本的なプロンプトテンプレートを作成
5. 出力形式（マークダウンレポート）を定義

## Dependencies

- **Blocked By:** 30059 (ナラティブCLIツール完成)
- **Blocks:** 30061

## 完了チェックリスト

- [ ] スキルファイル作成完了
- [ ] 基本動作確認
- [ ] ドキュメント整合性確認

## メモ

Phase 3 の基盤タスク。スキルの骨格を作成し、30061でプロンプトを詳細化。

---

## Detailed Implementation Plan

### スキルファイル構造

```markdown
# /qa-review コマンド

ゲームプレイログをLLMでレビューし、違和感を検出します。

## 使用方法

```bash
/qa-review <narrative.md>
/qa-review --trace <trace.jsonl>
/qa-review <file> --focus physics|ai|ux
```

## 引数

- `file`: ナラティブマークダウンファイル
- `--trace`: JSONLファイル直接指定
- `--focus`: レビュー観点（physics/ai/ux/all）
- `--threshold`: 最低重要度（critical/major/minor）

## 処理フロー

1. 入力ファイル読み込み
2. （--trace時）ナラティブ変換
3. レビュープロンプト構築
4. LLM呼び出し
5. レポート出力

## プロンプトテンプレート

[詳細は30061で設計]
```
