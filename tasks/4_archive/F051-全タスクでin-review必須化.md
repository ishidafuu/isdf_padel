---
id: F051
title: 全タスクで in-review 必須化
type: framework
status: done
priority: high
created: 2026-01-08
completed_at: 2026-01-08
---

# F051: 全タスクで in-review 必須化

## 背景・問題

P003-001（project-wide タスク）完了時にレビューがスキップされた。
調査の結果、**ドキュメントが「PXXX/FXXX は in-review 不要」と定義していた**ことが原因。

Claude はルール通りに動作しており、ルール違反ではなく**ルール自体の問題**だった。

## 目的

全タスクタイプで in-review を必須化し、レビュー漏れを防止する。

## 修正対象

| ファイル | 変更内容 |
|---------|---------|
| `.claude/agents/task-manager-agent.md` | 前提条件チェックを全タスクに適用 |
| `.claude/agents/impl-agent.md` | FXXX/PXXX のレビュー不要ルールを削除 |
| `.claude/skills/task-lifecycle.md` | 注釈追加 |

## 詳細な修正内容

### 1. task-manager-agent.md

**変更箇所1: 前提条件チェックの適用範囲拡大（行322-327）**

```diff
- > **MANDATORY: game-dev タスクは in-review 経由必須**
- >
- > game-dev タスク（30XXX/B30XXX/R30XXX）の完了は **必ず** `3_in-review/` からのみ可能。
+ > **MANDATORY: 全タスクで in-review 経由必須**
+ >
+ > 全タスク（30XXX/B30XXX/R30XXX/PXXX/FXXX）の完了は **必ず** `3_in-review/` からのみ可能。
```

**変更箇所2: タスク完了フロー統一（行329-356）**

- `【game-devタスクの場合】` と `【その他タスクの場合（framework/project-wide）】` を統合
- 全タスクで `3_in-review/` からの完了を必須化

**変更箇所3: セクションタイトル修正（行369-372）**

```diff
- ### 4. タスク状態遷移フロー（game-dev のみ）（CRITICAL / MANDATORY）
-
- **game-dev タスク（30XXX/B30XXX/R30XXX）は必ず in-review を経由する**
+ ### 4. タスク状態遷移フロー（全タスク共通）（CRITICAL / MANDATORY）
+
+ **全タスク（30XXX/B30XXX/R30XXX/PXXX/FXXX）は必ず in-review を経由する**
```

### 2. impl-agent.md

**変更箇所: FXXX/PXXX のレビュー不要ルール削除（行559-563）**

```diff
- ### FXXX/PXXX タスク: in-review 経由不要
-
- > フレームワーク開発（FXXX）・プロジェクト横断（PXXX）タスクは in-review を経由せず、直接 done に遷移可能。
- > task-manager-agent でタスク完了処理を直接実行する。
+ ### 全タスク共通: in-review 経由必須（MANDATORY）
+
+ > 全タスクタイプ（30XXX/B30XXX/R30XXX/PXXX/FXXX）で in-review を経由必須。
+ > レビュー後に task-manager-agent でタスク完了処理を実行する。
```

### 3. task-lifecycle.md

**変更箇所: 注釈追加（行98-101）**

```diff
 | `in-progress` → `in-review` | `2_in-progress/` → `3_in-review/` |
 | `in-review` → `done` | `3_in-review/` → `4_archive/` |
+
+ > **NOTE**: 全タスクタイプ（game-dev/project-wide/framework）で in-review 経由必須。
```

## 検証項目

- [ ] task-manager-agent.md の前提条件チェックが全タスクに適用
- [ ] impl-agent.md のハンドオフフローが全タスク共通
- [ ] task-lifecycle.md の状態遷移説明が正確

## 参照

- プランファイル: `~/.claude/plans/expressive-orbiting-quill.md`
