---
id: "F057"
title: "id-nextスクリプト化"
status: "done"
priority: "high"
blocked_by: []
blocks: []
---

# F057: id-nextスクリプト化

## 概要

`/id-next` コマンド用の Python スクリプト `scripts/id-next.py` を新規作成する。

## 背景

- `/id-next` 実行時に `scripts/id-next.py` が見つからずエラー
- `/task-next` と同様にスクリプト化してコンテキスト削減

## 成果物

1. `scripts/id-next.py` - ID採番スクリプト
2. `.claude/commands/id-next.md` 更新 - スクリプト呼び出し追加

## 受け入れ条件

- [x] `/id-next REQ-30101` が正常動作
- [x] `/id-next FXXX` が正常動作
- [x] `/id-next 30XXX` が正常動作
- [x] `/id-next PXXX` が正常動作
- [x] `/id-next B30101` が正常動作
