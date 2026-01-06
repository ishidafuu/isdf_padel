---
id: "F026"
title: "session-managerインタラクティブ修正"
type: "framework"
status: "done"
priority: "medium"
spec_ids: []
blocked_by: ["F019"]
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/s13219/.claude/plans/sharded-stargazing-zephyr.md"
tags: ["agents", "session-manager", "fix"]
created_at: "2026-01-04T16:00:00+09:00"
updated_at: "2026-01-04T17:15:00+09:00"
completed_at: "2026-01-04T17:15:00+09:00"
---

# Task F026: session-managerインタラクティブ修正

## 説明

session-manager-agent のインタラクティブシェル使用（`read -r`）を AskUserQuestion ツールに置き換える。

## 背景

### 現状

L139-155 で `read -r CLEANUP_RESPONSE` を使用:
```bash
echo "既存のworktreeが見つかりました:"
...
read -r CLEANUP_RESPONSE
```

### 改修理由

- Claude Code はインタラクティブシェルを前提としない
- `read -r` は動作しない
- AskUserQuestion ツールを使用すべき

## 実装内容

### 修正対象

**該当箇所**: L139-155

**現在のコード**:
```bash
echo "既存のworktreeが見つかりました:"
git worktree list
echo ""
echo "これらをクリーンアップしますか？"
echo "1) 全てクリーンアップして続行"
echo "2) キャンセル"
read -r CLEANUP_RESPONSE
```

**修正後**:
```markdown
既存worktreeが見つかった場合:
1. AskUserQuestion ツールでユーザーに確認
2. ユーザーの回答に基づいて処理を分岐

使用するツール: AskUserQuestion
- question: "既存のworktreeが見つかりました。クリーンアップしますか？"
- options: ["全てクリーンアップして続行", "キャンセル"]
```

### 追加修正

**SM-02: ブランチ命名の重複リスク（LOW）**
- `$$` (プロセスID) ベースのセッションID
- 同一PIDで複数worktreeを作成すると重複リスク
- → タイムスタンプまたはUUIDベースに変更検討

**SM-03: クリーンアップ手順の変数未定義**
- `SESSION_ID_*` 変数がフロー D で参照できない
- → .session-locks.yml から読み取る手順に変更

## 対象ファイル

| ファイル | 操作 |
|---------|------|
| `.claude/agents/session-manager-agent.md` | インタラクティブ処理の修正 |

## 依存関係

- **ブロック**: なし
- **ブロックされる**: F019
- **関連レビュー**:
  - tasks/1_todo/agent-review-g6.md（SM-01, SM-02, SM-03）

## メモ

- 1ファイルのみの修正
- 約20行の修正
- SM-02, SM-03 は優先度 LOW だが、同時に修正可能なら対応
