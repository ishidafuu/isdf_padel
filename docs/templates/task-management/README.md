---
name: task-agent
description: タスク管理エージェント。プランからタスク作成、タスク状態管理（todo/in-progress/done）、タスク検索を担当。「プランからタスクを作成」「タスクを開始」「タスクを完了」などの指示で自動発火。
tools: Read, Write, Edit, Glob, Bash, Skill
model: inherit
---

# task-agent

タスク管理統合ガイドライン

---
# task-management

タスク管理システム for Claude Code

## 概要

Markdownファイルベースのタスク管理システムです。既存プロジェクトに簡単に導入できます。

### 特徴

- **シンプルな構成**: 5つのコアファイルのみ
- **Markdownベース**: 特別なツール不要、Git管理可能
- **Handover/Resume対応**: セッション引き継ぎ機能付き
- **カスタマイズ可能**: ID形式、状態、フィールドを柔軟に変更可能

## クイックスタート

### 自動セットアップ

```bash
# 1. task-management/ をプロジェクトルートにコピー
cp -r /path/to/task-management /path/to/your-project/

# 2. セットアップ実行（どちらでもOK）
./task-management/setup.sh          # プロジェクトルートから
cd task-management && ./setup.sh    # フォルダ内から

# 3. テンプレートフォルダを削除
rm -rf ./task-management

# 4. CLAUDE.md にテンプレート内容を追記
```

### 手動セットアップ

```bash
# 1. ディレクトリ作成
mkdir -p .claude/skills .claude/agents .claude/commands
mkdir -p tasks/1_todo tasks/2_in-progress tasks/3_archive

# 2. ファイルコピー（task-management/から）
cp .claude/skills/task-workflow.md .claude/skills/
cp .claude/agents/task-agent.md .claude/agents/
cp .claude/commands/*.md .claude/commands/
cp tasks/.taskrc.yaml tasks/

# 3. CLAUDE.md に追記（CLAUDE.md.templateの内容を参照）
```

## ディレクトリ構成

```
your-project/
├── .claude/
│   ├── CLAUDE.md              # ← テンプレート内容を追記
│   ├── skills/
│   │   └── task-workflow.md
│   ├── agents/
│   │   └── task-agent.md
│   └── commands/
│       ├── id-next.md
│       ├── handover.md
│       └── resume-handover.md
└── tasks/
    ├── .taskrc.yaml
    ├── 1_todo/
    ├── 2_in-progress/
    └── 3_archive/
```

## 使い方

### タスク作成

```
「〇〇機能を実装するタスクを作成して」
→ Claude がプランモードで計画を立て、タスクファイルを作成
```

### タスク開始

```
「タスクT001を開始して」
→ status が in-progress に変更、1_todo/ → 2_in-progress/ へ移動
```

### タスク完了

```
「タスクT001を完了にして」
→ status が done に変更、2_in-progress/ → 3_archive/ へ移動
```

### セッション引き継ぎ

```
# セッション終了前
/handover

# 次のセッション開始時
/resume-handover
```

## タスクファイル形式

```yaml
---
id: "T001"
title: "ログイン機能実装"
status: "todo"
priority: "medium"
tags: ["auth", "feature"]
created_at: "2026-01-06T10:00:00"
updated_at: "2026-01-06T10:00:00"
completed_at: null
---

# Task T001: ログイン機能実装

## Summary

ユーザーログイン機能を実装する

## Progress

### Completed
- [x] 初期設計完了

## Next Actions

1. データベーススキーマ設計
2. APIエンドポイント実装
3. フロントエンド実装

## メモ

- OAuth対応は次フェーズで検討
```

## カスタマイズ

### ID形式の変更

`.taskrc.yaml` を編集:

```yaml
id_numbering:
  prefix: "ISSUE"        # T → ISSUE に変更
  format: "ISSUE-{:04d}" # ISSUE-0001 形式
```

### 状態の追加

`task-workflow.md` を編集して `in-review` などを追加:

```markdown
## 状態遷移

todo → in-progress → in-review → done
```

### フィールドの追加

タスクファイルに独自フィールドを追加可能:

```yaml
jira_ticket: "PROJ-123"
assignee: "alice"
```

## 元フレームワークとの違い

| 機能 | 元フレームワーク | 本テンプレート |
|------|-----------------|--------|
| ID形式 | FXXX, 30XXX, PXXX | TXXX（統一） |
| タスク配置 | tasks/, project/tasks/ | tasks/ のみ |
| 状態 | 4状態 + planning | 3状態 |
| worktree管理 | あり | なし |
| ID予約システム | あり | なし |
| 並列セッション | あり | なし |
| ファイル数 | 13+ | 5 |

## ライセンス

MIT License
