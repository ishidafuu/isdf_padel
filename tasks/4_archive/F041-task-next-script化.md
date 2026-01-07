---
id: "F041"
title: "task-next-script化"
type: "framework"
status: "done"
priority: "medium"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "~/.claude/plans/humble-hugging-canyon.md"
tags: ["optimization", "tooling"]
created_at: "2026-01-07T12:00:00+09:00"
updated_at: "2026-01-07T12:00:00+09:00"
completed_at: "2026-01-07T12:30:00+09:00"
---

# Task F041: task-next-script化

## Summary

`/task-next` コマンドを Python スクリプト化し、コンテキスト消費を削減する。

## 背景

現状の `/task-next` は Claude が複数回ファイルを読み込むため、コンテキストを大量に消費している:
- Glob × 4回 + Read × N回（タスク数分）

Python スクリプト化により:
- Bash 1回の呼び出しで結果のみ取得
- コンテキスト消費: 数千トークン → 数百トークン

## 実装内容

1. `scripts/task-next.py` を新規作成
   - YAML Frontmatter パース
   - 依存関係分析
   - READY タスク判定・ソート
   - 整形済み出力

2. `.claude/commands/task-next.md` を更新
   - スクリプト呼び出し指示を追加

## 完了条件

- [x] `python3 scripts/task-next.py` で同等の出力が得られる
- [x] `--limit N` オプションが動作する
