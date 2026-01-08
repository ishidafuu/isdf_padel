---
id: "F052"
title: "audit-agentにunity-ai-reviewer視点を統合"
type: "framework"
status: "done"
priority: "medium"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "~/.claude/plans/structured-leaping-rainbow.md"
tags: ["audit", "code-quality", "review"]
parent_task_id: null
created_at: "2026-01-08T10:00:00+09:00"
updated_at: "2026-01-08T12:00:00+09:00"
completed_at: "2026-01-08T12:00:00+09:00"
---

# Task F052: audit-agentにunity-ai-reviewer視点を統合

## Summary

[unity-ai-reviewer](https://github.com/2RiniaR/unity-ai-reviewer) の5つのレビュー視点を audit-agent に追加し、Rust/Bevy ECS環境に適応させる。

## 追加する視点（5つ）

| 視点 | 元ネタ | 主なチェック項目 |
|------|--------|------------------|
| メモリアロケーション | gc_allocation | clone(), Vec::new(), to_string() |
| 実行時エラーリスク | runtime_error | unwrap(), panic!, 直接インデックス |
| 車輪の再発明 | wheel_reinvention | Bevy標準機能の再実装、重複ユーティリティ |
| パフォーマンス | efficiency | O(n²)、毎フレーム重処理、非効率クエリ |
| セキュリティ | security | unsafe, 入力バリデーション, パストラバーサル |

## 修正対象ファイル

| ファイル | 変更内容 |
|---------|---------|
| `.claude/agents/audit-agent.md` | Phase 3 に5カテゴリ追加、深刻度基準追加 |
| `.claude/skills/code-audit.md` | 5カテゴリの詳細チェック項目と検出方法追加 |

## Progress

### Completed

- [x] audit-agent.md の Phase 3「独自分析」セクションに5カテゴリ追加
- [x] 深刻度の判断基準セクションに新カテゴリの基準追加
- [x] code-audit.md に5カテゴリの詳細（チェック項目、検出方法）追加
- [x] 診断レポートのサマリーテーブル更新（両ファイル）
- [x] タスク化テンプレートに新カテゴリ追加

## Next Actions

（完了）

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## Detailed Implementation Plan

詳細は `~/.claude/plans/structured-leaping-rainbow.md` を参照

### Step 1: audit-agent.md 更新

1. Phase 3「独自分析」セクションに5カテゴリ追加
2. 深刻度の判断基準セクションに新カテゴリの基準追加
3. 監査チェックリストに新項目追加

### Step 2: code-audit.md 更新

1. 監査カテゴリセクションに5カテゴリ追加（カテゴリ5〜9）
2. 各カテゴリに以下を記載：
   - チェック項目
   - 深刻度
   - 検出方法（grep/検索パターン）
3. 出力形式（診断レポート）のサマリーテーブル更新
4. タスク化テンプレートに新カテゴリ追加

## 完了チェックリスト

- [x] 変更内容の検証完了
- [x] ドキュメント整合性確認

## メモ

- ゲーム開発特有の許容事項を考慮
  - 初期化時のみ実行されるコードは除外
  - パフォーマンスより可読性を優先するケースの許容
- 既存の4カテゴリとの整合性を維持
- タスク化時の category 値を新設（`memory`, `runtime-error`, `wheel-reinvention`, `performance`, `security`）
