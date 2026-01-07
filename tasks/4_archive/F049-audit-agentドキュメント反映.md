---
id: F049
title: audit-agentドキュメント反映
type: framework
status: done
priority: medium
created: 2026-01-07
branch: null
worktree: null
blocked_by: []
blocks: []
---

# F049: audit-agentドキュメント反映

## 概要

コミット `deee3d2` で追加された audit-agent（定期コード監査機能）を、未反映のドキュメントに追加する。

## 背景

F048 で audit-agent、code-audit スキル、/code-audit コマンドが追加されたが、以下のドキュメントには未反映:
- docs/concepts/agents.md
- docs/reference/tools-reference.md
- docs/reference/framework-spec.md

## 作業内容

### 1. docs/concepts/agents.md（3箇所）
- [x] クイックリファレンス表に `| 定期監査を実行 | 🔍 audit-agent |` 追加
- [x] Q5フローチャートに「プロジェクト全体の定期監査 → 🔍 audit-agent」追加
- [x] 全エージェント詳細セクションに audit-agent 説明を追加

### 2. docs/reference/tools-reference.md（3箇所）
- [x] 配置場所リストに `audit-agent.md` 追加
- [x] エージェント一覧表に audit-agent 行追加
- [x] Skills一覧表に `code-audit.md` 追加

### 3. docs/reference/framework-spec.md（3箇所）
- [x] エージェント一覧表に audit-agent 行追加
- [x] コマンド一覧に `/code-audit` 追加
- [x] Skills一覧表に `code-audit.md` 追加

## 完了条件

- [x] 3ファイル、計9箇所の編集完了
- [x] コミット作成: `docs: audit-agent関連ドキュメントを更新`

## 参照

- プランファイル: `~/.claude/plans/iridescent-swimming-puffin.md`
- `.claude/agents/audit-agent.md`
- `.claude/skills/code-audit.md`
- `.claude/commands/code-audit.md`

## Progress

- 2026-01-07: 全3ファイル、9箇所の編集完了

## Handover

（なし - タスク完了）
