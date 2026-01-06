# Tasks Directory

このディレクトリはMarkdownファイルベースのタスク管理に使用します。

## 構成

```
tasks/
├── .taskrc.yaml      # 設定ファイル
├── 1_todo/           # 未着手タスク
├── 2_in-progress/    # 実装中タスク
└── 3_archive/        # 完了・キャンセル済み
```

## タスクファイル形式

```yaml
---
id: "T001"
title: "タスクタイトル"
status: "todo"
priority: "medium"
tags: []
created_at: "2026-01-06T10:00:00"
updated_at: "2026-01-06T10:00:00"
completed_at: null
---

# Task T001: タスクタイトル

## Summary
...

## Progress
...

## Next Actions
...

## メモ
...
```

## 使い方

詳細は `README.md`（親ディレクトリ）を参照してください。
