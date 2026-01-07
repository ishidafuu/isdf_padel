---
id: "F042"
title: "worktree作業フロー改善"
type: "framework"
status: "done"
priority: "medium"
related_task: null
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/sharded-roaming-lark.md"
tags: ["parallel-sessions", "worktree", "documentation"]
created_at: "2026-01-07T16:00:00+09:00"
updated_at: "2026-01-07T16:00:00+09:00"
completed_at: "2026-01-07T16:30:00+09:00"
---

# Task F042: worktree作業フロー改善

## 説明

worktree並列セッションの作業フローを改善する。絶対パス方式の明文化とtargetシンボリックリンク作成の確実化を行う。

## 背景

### 現状

- タスク30012の作業で問題が発覚
- parallel-sessions.md では絶対パス方式が明示的に言及されていない
- session-manager-agent.md にtargetリンク記載はあるが、手動実行で漏れやすい

### 改修理由

1. 絶対パス方式はcwdを維持でき、混乱を防ぐ
2. targetリンク漏れでフルビルド（3分）が発生する問題を防ぐ

## 実装内容

- [x] `.claude/skills/parallel-sessions.md` に絶対パス方式の推奨を追加
- [x] `.claude/skills/parallel-sessions.md` にコマンド例を追加
- [x] `.claude/commands/session-init.md` にtargetシンボリックリンク作成を必須チェック項目として追加
- [x] `.claude/commands/session-init.md` に作成漏れ時の影響を注記

## メモ

- cargo: `-C` / `--manifest-path` オプションで絶対パス完全対応
- git: `-C` オプションで絶対パス対応
- 相対パス参照: worktree内で完結（問題なし）

## 依存関係

- **ブロック**: なし
- **ブロックされる**: なし
- **関連ドキュメント**: `.claude/skills/parallel-sessions.md`, `.claude/commands/session-init.md`

---

## Detailed Implementation Plan

以下は、プランファイル `~/.claude/plans/sharded-roaming-lark.md` の全内容です。

# worktree作業フロー改善プラン

## 背景

タスク30012の作業で以下が判明：
1. **絶対パス方式が有効**: cdせずに絶対パスでworktree側を操作する方式が簡潔
2. **targetシンボリックリンク漏れ**: worktree作成時にtargetリンクを忘れ、フルビルド（3分）が発生

## 改善内容

### 1. 絶対パス方式の明文化

**現状**: parallel-sessions.md では明示的に言及なし

**改善**:
- worktree作成後、メインセッションは移動せず絶対パスで操作することを推奨として明記
- コマンド例を追加

```bash
# 推奨: 絶対パスで操作（cwdを維持）
cargo build --manifest-path /path/to/worktree/project/Cargo.toml
git -C /path/to/worktree status

# 非推奨: cd による移動
cd /path/to/worktree && cargo build
```

### 2. targetシンボリックリンク作成の確実化

**現状**: session-manager-agent.md に記載はあるが、手動実行で漏れやすい

**改善**:
- session-init.md にチェックリスト形式で明示
- worktree作成直後に必ず実行する手順として強調

```bash
# worktree作成直後に必ず実行
ln -s $(pwd)/project/target ${WORKTREE_PATH}/project/target
```

## 修正対象ファイル

1. `.claude/skills/parallel-sessions.md`
   - 絶対パス方式の推奨を追加
   - コマンド例を更新

2. `.claude/commands/session-init.md`
   - targetシンボリックリンク作成を必須チェック項目として追加
   - 作成漏れ時の影響（フルビルド3分）を注記

## 検証済み事項

- cargo: `-C` / `--manifest-path` オプションで絶対パス完全対応
- git: `-C` オプションで絶対パス対応
- 相対パス参照: worktree内で完結（問題なし）
