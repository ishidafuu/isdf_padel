# task-registration

**タスク登録スキル** - プランから1_todo/にタスクファイルを生成

## 呼び出し方式

**自動呼び出し**（人間が `/task-register` と打つのではない）

- 「プランからタスクを作成して」等の指示で Claude が自動参照
- CLAUDE.md で強制: 「タスク登録時は必ずこのスキルを参照」

---

## 配置ルール（CRITICAL）

### 唯一の配置先

| タスクタイプ | 配置先 |
|-------------|-------|
| FXXX | `tasks/1_todo/` |
| 30XXX, B30XXX, R30XXX, PXXX | `project/tasks/1_todo/` |

### 禁止ディレクトリ

- ❌ `0_backlog/` - バグバックログ専用
- ❌ `2_in-progress/` - 開始後にのみ移動
- ❌ `3_in-review/` - レビュー時にのみ移動
- ❌ `4_archive/` - 完了時にのみ移動
- ❌ その他カスタムディレクトリ - 作成禁止

---

## フロー選択

```
ユーザーの依頼内容を確認
  ↓
┌─────────────────────────────────────────┐
│ 「仕様書から実装タスクを作成して」       │
│  → ゲーム開発フロー（Flow A）           │
├─────────────────────────────────────────┤
│ 「プランからタスクを作成して」           │
│  → プランファイル経由フロー（Flow B）   │
└─────────────────────────────────────────┘
```

---

## Flow A: ゲーム開発（仕様書から直接）

### A-1: 仕様書確認

```
ユーザー: 「ジャンプ機能の実装タスクを作成して」
  ↓
仕様書を確認（Read）:
  project/docs/3_ingame/30X_*/spec.md
  ↓
表示:
  - タイトル
  - 概要（Summary）
  - 主要な要件（REQ-ID）
```

### A-2: ID採番

```
Skill(skill="id-next", args="30XXX")
```

### A-3: タスクファイル生成

配置先: `project/tasks/1_todo/30XXX-機能名.md`

### A-4: 完了報告

```
✅ タスク登録完了

Task ID: 30XXX
Type: game-dev
Status: todo
File: project/tasks/1_todo/30XXX-機能名.md
Spec: project/docs/3_ingame/.../spec.md

次のステップ:
- タスクを開始: 「30XXXを開始」と依頼してください
```

---

## Flow B: プランファイル経由

### B-1: 最新プランファイル取得（CRITICAL）

```bash
ls -t "$HOME/.claude/plans"/*.md 2>/dev/null | head -1
```

### B-2: プラン概要表示 → ユーザー確認（CRITICAL）

**必須**: タスク作成前に概要を表示して確認を得る

```
📋 プラン確認

ファイル: ~/.claude/plans/xxx.md
更新日時: YYYY-MM-DD HH:MM

## タイトル
[プランの見出し]

## 概要
[Summary セクションの内容]

## タスクタイプ
[framework / game-dev / bugfix / refactor / project-wide]

---
このプランからタスクを作成します。よろしいですか？
```

### B-3: タスクタイプ判定

**優先順位**:

1. **明示的マーカー**: `## Task Type` セクション
2. **キーワード検出**:
   - "バグ修正", "不具合", "修正" → bugfix
   - "リファクタ", "最適化", "改善" → refactor
3. **パス解析**:
   - `project/` → game-dev
   - `.claude/`, `docs/` → framework
   - `.github/workflows/` → project-wide
4. **ユーザー確認**: AskUserQuestion

### B-4: ID採番

```
Skill(skill="id-next", args="FXXX")    # framework
Skill(skill="id-next", args="30XXX")   # game-dev
Skill(skill="id-next", args="B30101")  # bugfix（元タスクID指定）
Skill(skill="id-next", args="R30101")  # refactor（元タスクID指定）
Skill(skill="id-next", args="PXXX")    # project-wide
```

### B-5: タスクファイル生成

配置先は「配置ルール」参照

### B-6: 完了報告

```
✅ タスク登録完了

Task ID: F005
Type: framework
Status: todo
File: tasks/1_todo/F005-xxx.md
Plan: ~/.claude/plans/xxx.md（保持）

次のステップ:
- タスクを開始: 「F005を開始」と依頼してください
```

---

## Frontmatter 必須フィールド

```yaml
---
id: "F005"
title: "[タイトル]"
type: "framework"  # game-dev / bugfix / refactor / project-wide
status: "todo"     # 初期値は必ず "todo"
priority: "medium"
related_task: null  # bugfix/refactor のみ元タスクID必須
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/.../plans/xxx.md"  # プラン参照
tags: []
parent_task_id: null
created_at: "..."
updated_at: "..."
completed_at: null
---
```

---

## 本文構造

```markdown
# Task [ID]: [タイトル]

## Summary

[プラン/仕様書の概要から抽出]

## Related Specifications

- [関連ドキュメントへのパス]

## Progress

### Completed

(なし)

## Next Actions

1. [最初のアクション]

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

[タスクタイプに応じて選択]

### game-dev / bugfix / refactor:
> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] in-review に移動済み
- [ ] レビュー完了

### framework / project-wide:
- [ ] 変更内容の検証完了
- [ ] ドキュメント整合性確認

## メモ

(なし)

---

## Detailed Implementation Plan

> 以下はプランファイル `~/.claude/plans/xxx.md` の全内容です。

[プランファイル全文]
```

---

## バリデーションチェックリスト

### 作成前チェック

- [ ] プラン/仕様書ファイルが存在
- [ ] Summary/概要セクションが存在し空でない
- [ ] タスクタイプが確定
- [ ] ユーザー確認が完了

### 作成時チェック

- [ ] `/id-next` で ID 採番済み
- [ ] 配置先が `1_todo/` である
- [ ] status が "todo"
- [ ] Frontmatter 必須フィールド設定済み

---

## 禁止事項（CRITICAL）

- ❌ タスク状態の変更（todo → in-progress 等）
- ❌ worktreeの作成・管理
- ❌ プランファイルの削除（保持する）
- ❌ 依存関係の設定（blocked_by, blocks）
- ❌ `1_todo/` 以外への配置
- ❌ **プラン作成後の直接実装**（タスク登録必須）

---

## 注意

- **プラン作成後は必ずタスク登録 → タスク開始の順序を守る**
- プランから直接実装を開始しない
- タスクファイルを作成してから実装を開始する

---

## 関連ドキュメント

- `.claude/skills/task-workflow.md` - タスクライフサイクル
- `.claude/skills/task-planning.md` - プランモードフロー
- `.claude/agents/task-manager-agent.md` - タスク管理
