# task-agent

タスク管理統合ガイドライン

---

## 概要

このガイドラインは、メイン Claude Code がタスク管理操作を実行する際の手順を定義します。

**責務:**
- タスク作成（プランからの変換）
- タスク状態管理（todo → in-progress → done）
- タスク検索・一覧表示

---

## タスク作成フロー

### Phase 1: プラン作成（ユーザー）

```
ユーザー: プランモードに入る
↓
【プランモード】
├─ 関連調査
├─ 仕様・要件の確認
├─ プランファイル作成（~/.claude/plans/xxx.md）
└─ ExitPlanMode
↓
プランファイル保存完了
```

### Phase 2: タスク登録（Claude Code）

ユーザーが「プランからタスクを作成して」と指示したら:

```
1. プランファイル検出
   - ~/.claude/plans/ から最新ファイルを取得
   - Bash: ls -t ~/.claude/plans/*.md | head -1

2. プラン内容確認
   - Read でプランファイルを読み込み
   - ユーザーに概要を表示して確認

3. ID採番
   - Skill: /id-next を実行
   - 次のタスクID（T001, T002, ...）を取得

4. タスクファイル生成
   - 配置先: tasks/1_todo/{ID}-{title}.md
   - Write でファイル作成

5. 完了報告
   - タスクID、ファイルパスを表示
```

### タスクファイル生成テンプレート

```yaml
---
id: "{採番されたID}"
title: "{プランから抽出したタイトル}"
status: "todo"
priority: "medium"
tags: []
created_at: "{現在時刻 ISO 8601}"
updated_at: "{現在時刻 ISO 8601}"
completed_at: null
---

# Task {ID}: {title}

## Summary

{プランの概要から抽出}

## Progress

### Completed

- [x] プラン作成完了

## Next Actions

{プランの実装ステップから抽出}

## メモ

{プランの注意事項から抽出}

## Detailed Implementation Plan

> 以下はプランファイルからの引用です

{プラン全文を引用}
```

---

## タスク状態管理フロー

### タスク開始

ユーザーが「タスク{ID}を開始して」と指示したら:

```
1. タスクファイル検索
   - Glob: tasks/1_todo/{ID}-*.md

2. ファイル移動
   - Bash: mv tasks/1_todo/{ID}-*.md tasks/2_in-progress/

3. status更新
   - Edit: status: "todo" → "in-progress"
   - Edit: updated_at を現在時刻に更新

4. 報告
   - 「タスク{ID}を開始しました」
```

### タスク完了

ユーザーが「タスク{ID}を完了にして」と指示したら:

```
1. タスクファイル検索
   - Glob: tasks/2_in-progress/{ID}-*.md

2. ファイル移動
   - Bash: mv tasks/2_in-progress/{ID}-*.md tasks/3_archive/

3. status更新
   - Edit: status: "in-progress" → "done"
   - Edit: completed_at を現在時刻に設定
   - Edit: updated_at を現在時刻に更新

4. 報告
   - 「タスク{ID}を完了にしました」
```

### タスクキャンセル

ユーザーが「タスク{ID}をキャンセルして」と指示したら:

```
1. タスクファイル検索
   - Glob: tasks/*/{ID}-*.md

2. ファイル移動
   - Bash: mv tasks/*/{ID}-*.md tasks/3_archive/

3. status更新
   - Edit: status → "cancelled"
   - Edit: updated_at を現在時刻に更新

4. 報告
   - 「タスク{ID}をキャンセルしました」
```

### タスク一時停止

ユーザーが「タスク{ID}を一時停止して」と指示したら:

```
1. タスクファイル検索
   - Glob: tasks/2_in-progress/{ID}-*.md

2. ファイル移動
   - Bash: mv tasks/2_in-progress/{ID}-*.md tasks/1_todo/

3. status更新
   - Edit: status: "in-progress" → "todo"
   - Edit: updated_at を現在時刻に更新

4. 報告
   - 「タスク{ID}を一時停止しました（todoに戻しました）」
```

---

## タスク検索・一覧表示

### 全タスク一覧

```bash
# アクティブタスク（archive以外）
ls tasks/1_todo/ tasks/2_in-progress/
```

### 状態別一覧

```bash
Glob("tasks/1_todo/*.md")        # 未着手
Glob("tasks/2_in-progress/*.md") # 進行中
Glob("tasks/3_archive/*.md")     # 完了・キャンセル
```

### タスク詳細表示

```bash
# タスクファイルを読み込んで表示
Read("tasks/2_in-progress/T001-*.md")
```

---

## 禁止事項

このガイドラインでは以下は対象外:

- ❌ worktree作成・管理
- ❌ ID予約
- ❌ 並列セッション管理
- ❌ タスク依存関係管理（blocked_by/blocks）

---

## 関連ドキュメント

- `.claude/skills/task-workflow.md` - タスク形式・状態遷移
- `.claude/commands/id-next.md` - ID採番
- `.claude/commands/handover.md` - セッション引き継ぎ
- `.claude/commands/resume-handover.md` - セッション再開
