---
id: "F020"
title: "test-agent責務再定義"
type: "framework"
status: "done"
priority: "high"
spec_ids: []
blocked_by: ["F019"]
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/s13219/.claude/plans/sharded-stargazing-zephyr.md"
tags: ["agents", "test-agent", "critical", "design-change"]
created_at: "2026-01-04T16:00:00+09:00"
updated_at: "2026-01-04T22:00:00+09:00"
completed_at: "2026-01-04T22:00:00+09:00"
---

# Task F020: test-agent責務再定義

## 説明

CLAUDE.md の「test.md は作成しない」ルールに合わせて、test-agent の責務を「test.md 作成」から「tests/ テストコード設計支援」に変更する。

## 背景

### 現状

- CLAUDE.md で「test.md は作成しない」が CRITICAL ルール
- しかし test-agent の主目的は test.md 作成
- フレームワーク内で矛盾が発生

### 改修理由

- Single Source of Truth 原則の維持
- 仕様書（spec.md）とテストコード（tests/）の二重管理を防ぐ
- CLAUDE.md との整合性確保

## 実装内容

### 1. test-agent.md 全面改訂

**新責務**:
- tests/ 配下のテストファイル構造設計
- BDD形式でのテストシナリオ設計支援
- REQ-ID とテスト名の対応設計
- カバレッジ漏れの検出
- テスト戦略の提案

**削除する内容**:
- test.md 作成フロー
- test.md テンプレート
- test.md 関連のチェックリスト

### 2. 参照元ファイルの修正

| ファイル | 該当行 | 修正内容 |
|---------|-------|---------|
| spec-agent.md | L164-166, L230-232 | test-agent への言及を修正 |
| critic-agent.md | L231 | test-agent への言及を修正 |
| design-agent.md | L377-379 | ハンドオフフロー修正 |
| behavior-agent.md | L330-337, L258-259 | ハンドオフフロー修正 |
| impl-agent.md | L239-246, L281, L290, L357, L435 | test.md チェック削除 |
| review-agent.md | L438-439 | test.md への言及削除 |
| architecture-agent.md | L288-290 | test-agent 責務説明修正 |

### 3. ハンドオフフローの統一修正

```markdown
Before:
behavior-agent → test-agent(test.md) → impl-agent

After:
behavior-agent → impl-agent（テストコード + プロダクトコード）
  └── test-agent に相談: テスト設計支援
```

## 対象ファイル

| ファイル | 操作 |
|---------|------|
| `.claude/agents/test-agent.md` | 全面改訂 |
| `.claude/agents/spec-agent.md` | 一部修正 |
| `.claude/agents/critic-agent.md` | 一部修正 |
| `.claude/agents/design-agent.md` | 一部修正 |
| `.claude/agents/behavior-agent.md` | 一部修正 |
| `.claude/agents/impl-agent.md` | 一部修正 |
| `.claude/agents/review-agent.md` | 一部修正 |
| `.claude/agents/architecture-agent.md` | 一部修正 |

## 依存関係

- **ブロック**: なし
- **ブロックされる**: F019
- **関連レビュー**: tasks/1_todo/agent-review-g2.md（T-01, T-02）

## メモ

- test-agent は最も大きな設計変更
- 参照元7ファイルの修正が必要
- 修正後はハンドオフフローの一貫性を確認
