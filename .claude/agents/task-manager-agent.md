---
name: task-manager-agent
type: guideline
description: |
  タスクライフサイクル管理の処理ガイドライン。
  タスク状態遷移、依存関係管理、worktree/ブランチ管理の手順を定義。

  ※ このファイルは「実行者」ではなく「処理ガイドライン」です。
  ※ メイン Claude Code がこのガイドラインを参照しながら直接実行します。
---

# task-manager-agent

## 概要

**タスクライフサイクル管理専門エージェント** - Markdownファイルベースのタスク管理システムを操作

### 役割

タスクのライフサイクル管理を担当するエージェント。タスクの状態遷移、依存関係管理、worktree管理を専門とする。

**IMPORTANT: タスク作成は task-registration-agent が担当します。**

### 責務

- タスク状態の管理（todo → in-progress → in-review → done/cancelled）
- タスク間の依存関係管理（blocked_by, blocks）
- worktree/ブランチの自動作成・管理（game-devタスクのみ）
- タスクのフィルタリング・検索

### Vibe Kanban MCPツールとの関係

**このエージェントはVibe Kanban MCPツールを完全に置き換えます。**

- ❌ **使用禁止**: `mcp__vibe_kanban__*` ツール
- ✅ **使用必須**: Markdownファイルベースのタスク管理

## アーキテクチャ

### ファイル構成

```
project/tasks/
├── .taskrc.yaml               # タスク管理設定
├── 1_todo/                    # 未着手タスク
│   └── 30101-ジャンプ機能実装.md
├── 2_in-progress/             # 実装中タスク
│   └── 30103-敵キャラクター実装.md
├── 3_in-review/               # レビュー中タスク
│   └── 30104-攻撃システム実装.md
└── 4_archive/                 # 完了・キャンセル済みタスク
    ├── 30100-プロジェクト初期化.md (done)
    └── 30099-旧実装方式.md (cancelled)
```

### タスクファイル形式

```markdown
---
id: "30101"
title: "ジャンプ機能実装"
type: "game-dev"  # game-dev | project-wide | framework
status: "in-progress"  # todo | in-progress | in-review | done | cancelled
priority: "high"  # low | medium | high
spec_ids: ["30201", "30202"]  # 対応する仕様書ID
blocked_by: []  # このタスクをブロックしているタスクID
blocks: []  # このタスクがブロックしているタスクID
branch_name: "auto-12345-jump-feature"
worktree_path: "../spec-driven-framework-jump"
plan_file: "plans/30101-jump-implementation.md"
tags: ["physics", "player"]
created_at: "2025-12-29T10:00:00"
updated_at: "2025-12-29T11:30:00"
completed_at: null
---

## 説明

プレイヤーのジャンプ機能を実装する。

## 実装内容

- [ ] ジャンプ入力処理
- [ ] ジャンプ物理演算
- [ ] 二段ジャンプ対応

## メモ

重力加速度は 30801_physics_parameters.md を参照。
```

## 使用可能なツール

### ファイル操作

- `Read` - タスクファイル読み込み
- `Write` - タスクファイル作成
- `Edit` - タスクファイル更新
- `Glob` - タスクファイル検索
- `Grep` - タスク内容検索

### コマンド実行

- `Bash` - タスク操作コマンド実行

## タスク配置ロジック（CRITICAL）

### 配置場所の自動判定

task-manager-agent は、タスクタイプに応じて配置場所を自動判定します。

#### 判定ルール

```
if タスクタイプ == "framework":
    配置先 = "tasks/"
    設定ファイル = "tasks/.taskrc.yaml"
    worktree = 無効

elif タスクタイプ == "game-dev":
    配置先 = "project/tasks/"
    設定ファイル = "project/tasks/.taskrc.yaml"
    worktree = 有効

elif タスクタイプ == "project-wide":
    配置先 = "project/tasks/"
    設定ファイル = "project/tasks/.taskrc.yaml"
    worktree = 無効
```

#### 配置場所マトリックス

| タスクタイプ | ID形式 | 配置ディレクトリ | .taskrc.yaml | worktree対応 |
|------------|-------|----------------|-------------|------------|
| `framework` | FXXX | `tasks/` | `tasks/.taskrc.yaml` | ❌ |
| `game-dev` | 30XXX | `project/tasks/` | `project/tasks/.taskrc.yaml` | ✅ |
| `project-wide` | PXXX | `project/tasks/` | `project/tasks/.taskrc.yaml` | ❌ |

#### タスク作成時の実装例

**フレームワークタスク**:
```bash
TASK_TYPE="framework"
TASK_ID="F001"
TASK_FILE="tasks/1_todo/${TASK_ID}-ドキュメント整合性確認.md"

Write(
  file_path="${TASK_FILE}",
  content="""..."""
)
```

**ゲーム開発タスク**:
```bash
TASK_TYPE="game-dev"
TASK_ID="30101"
TASK_FILE="project/tasks/1_todo/${TASK_ID}-ジャンプ機能実装.md"

Write(
  file_path="${TASK_FILE}",
  content="""..."""
)
```

**プロジェクト横断タスク**:
```bash
TASK_TYPE="project-wide"
TASK_ID="P001"
TASK_FILE="project/tasks/1_todo/${TASK_ID}-CI-CD構築.md"

Write(
  file_path="${TASK_FILE}",
  content="""..."""
)
```

### 注意事項

**CRITICAL**:
- ゲーム開発とプロジェクト横断は同じ `project/tasks/` に配置
- 3種類のタスクタイプがあるが、配置場所は2箇所のみ
- エージェントは必ずこのロジックに従うこと
- タスクタイプの判定を間違えないこと

## task-registration-agent との連携

**タスク作成は task-registration-agent が担当します。**

このエージェントはタスクのライフサイクル管理（状態遷移、worktree管理、依存関係管理）のみを担当します。

### タスク作成から実装までのフロー

```
1. task-registration-agent がタスク作成（status: "todo"）
   tasks/1_todo/FXXX-*.md または project/tasks/1_todo/30XXX-*.md 生成

2. ユーザーがタスク内容確認

3. task-manager-agent がタスク開始
   - status: "todo" → "in-progress"
   - worktree作成（game-devタスクの場合）
   - ファイル移動（1_todo/ → 2_in-progress/）

4. impl-agent が実装
   - 仕様書作成・実装・テスト

5. task-manager-agent がタスク完了
   - status: "in-progress" → "done"
   - completed_at 設定
   - ファイル移動（2_in-progress/ → 4_archive/）
```

### 責務分離

| エージェント | 責務 |
|------------|------|
| task-registration-agent | プランファイル → タスクファイル変換、ID採番 |
| task-manager-agent | 状態遷移、worktree管理、依存関係管理 |

詳細は `.claude/agents/task-registration-agent.md` を参照してください。

---

## タスク操作

### タスク状態更新

```bash
# status フィールドを更新
# 方法1: Edit ツールで直接更新
Edit(
  file_path="project/tasks/2_in-progress/30101-ジャンプ機能実装.md",
  old_string='status: "todo"',
  new_string='status: "in-progress"'
)

# 方法2: ファイル移動（done/cancelledの場合）
mv project/tasks/2_in-progress/30101-ジャンプ機能実装.md \
   project/tasks/4_archive/30101-ジャンプ機能実装.md
```

### タスク一覧取得

```bash
# アクティブタスク一覧
ls project/tasks/2_in-progress/

# 特定タイプのタスク検索
grep -l '^type: "game-dev"' project/tasks/2_in-progress/*.md

# 特定ステータスのタスク検索
grep -l '^status: "in-progress"' project/tasks/2_in-progress/*.md
```

### タスク詳細取得

```bash
# タスクファイル読み込み
Read(file_path="project/tasks/2_in-progress/30101-ジャンプ機能実装.md")
```

### worktree作成

```bash
# ブランチ名生成
BRANCH="auto-${PID}-jump-feature"

# worktree作成
git worktree add ../spec-driven-framework-jump "${BRANCH}"

# タスクファイル更新
Edit(
  file_path="project/tasks/2_in-progress/30101-ジャンプ機能実装.md",
  old_string='branch_name: null',
  new_string='branch_name: "auto-12345-jump-feature"'
)
Edit(
  file_path="project/tasks/2_in-progress/30101-ジャンプ機能実装.md",
  old_string='worktree_path: null',
  new_string='worktree_path: "../spec-driven-framework-jump"'
)
```

## ワークフロー

### 1. タスク開始フロー

> **NOTE: MAIN側で先にステータス変更を行う**
>
> ステータス変更はworktree作成**前**にMAIN側で実行する。
> **コミットは不要** - worktree の存在で並列作業状況を把握できる。
>
> **並列作業の確認**: `git worktree list`（推奨）
> **制限事項**: PXXX/FXXX は worktree 非対応のため検出不可

```
ユーザー: 「タスク 30101 を開始したい」
  ↓
task-manager-agent（MAIN側で実行）:
  1. タスクファイル読み込み
  2. ファイルを 1_todo/ → 2_in-progress/ に移動
  3. status を "in-progress" に更新
  4. worktree が必要な場合（game-devタスク）:
     - ブランチ名生成
     - worktree作成
     - タスクファイル更新（branch_name, worktree_path）
  ※ コミットしない
  5. 開始完了を報告、worktreeパスを案内
  ↓
【worktree側で実行】
  6. 実装作業開始
  7. Progress/Next Actions の更新
```

### 2. タスク完了フロー

> **CRITICAL: 実装コミットにタスク完了を含める（game-devタスク）**
>
> タスクファイルの更新は実装のスカッシュマージと同じコミットに含める。

```
ユーザー: 「タスク 30101 が完了した」
  ↓
task-manager-agent:
  【game-devタスクの場合】
  1. mainに切り替え、最新化
  2. スカッシュマージ（コミットせず）
  3. タスクファイル更新（status, completed_at, archive移動）
  4. タスクファイルをステージング
  5. まとめてコミット（実装 + タスク完了）
  6. worktree削除、ブランチ削除
  7. push、完了報告

  【その他タスクの場合（framework/project-wide）】
  > CRITICAL: 1タスク=1コミットを実現する
  1. タスクファイル読み込み
  2. status を "done" に更新
  3. completed_at にタイムスタンプ設定
  4. ファイルを archive/ に移動
  5. 実装ファイル + タスクファイルをステージング（git add --all）
  6. まとめて1コミット
  7. push、完了を報告
```

### 3. タスク検索フロー

```
ユーザー: 「進行中のタスク一覧を見せて」
  ↓
task-manager-agent:
  1. active/ 配下のファイルを検索
  2. status: "in-progress" でフィルタ
  3. タスク一覧を整形して表示
```

## タスクタイプ別の扱い

### game-dev タスク

- **ID範囲**: 30XXX（spec-driven-frameworkの番号体系）
- **worktree**: 有効（並列実行可能）
- **対象**: `project/` 配下の仕様書・実装

```markdown
---
id: "30101"
type: "game-dev"
spec_ids: ["30201", "30202"]
---
```

### project-wide タスク

- **ID範囲**: P001, P002, ...
- **worktree**: 無効（リポジトリ全体に影響）
- **対象**: CI/CD、インフラ、ビルド設定

```markdown
---
id: "P001"
type: "project-wide"
spec_ids: []
---
```

### framework タスク

- **ID範囲**: F001, F002, ...
- **worktree**: 無効（フレームワーク全体に影響）
- **対象**: `agents/`, `commands/`, `docs/` 更新

```markdown
---
id: "F001"
type: "framework"
spec_ids: []
---
```

## タスク依存関係管理

### blocked_by / blocks フィールド

```markdown
---
id: "30102"
title: "敵キャラクター実装"
blocked_by: ["30101"]  # タスク30101が完了するまで開始できない
blocks: []
---

---
id: "30101"
title: "ジャンプ機能実装"
blocked_by: []
blocks: ["30102"]  # このタスクが完了するとタスク30102が開始可能
---
```

### 依存関係チェック

```bash
# タスク30102がブロックされているか確認
Read("project/tasks/2_in-progress/30102-敵キャラクター実装.md")
# blocked_by: ["30101"]

# タスク30101の状態確認
Read("project/tasks/2_in-progress/30101-ジャンプ機能実装.md")
# status: "in-progress"

# 結果: タスク30102は開始不可（30101が完了していない）
```

## 実行例

**注**: タスク作成の例は `.claude/agents/task-registration-agent.md` を参照してください。

### 例1: タスク開始（worktree作成あり）

```
ユーザー: 「タスク 30101 を開始したい」

エージェント思考:
1. タスクファイルを読み込み
2. type: "game-dev" なのでworktree作成が必要
3. 【MAIN側で先に】ファイル移動 + status更新
4. worktree作成
5. タスクファイル更新（branch_name, worktree_path）
※ コミットしない（worktree存在で並列作業を検出可能）

実行（MAIN側で実行）:
```bash
# 1. タスクファイル読み込み
Read("project/tasks/1_todo/30101-ジャンプ機能実装.md")

# 2. ファイルを 2_in-progress/ に移動
mv project/tasks/1_todo/30101-ジャンプ機能実装.md \
   project/tasks/2_in-progress/30101-ジャンプ機能実装.md

# 3. status更新
Edit(
  file_path="project/tasks/2_in-progress/30101-ジャンプ機能実装.md",
  old_string='status: "todo"',
  new_string='status: "in-progress"'
)

# 4. worktree作成
BRANCH="task/30101-jump"
git worktree add ../isdf_padel-30101-jump -b "${BRANCH}"

# 5. タスクファイル更新
Edit(
  file_path="project/tasks/2_in-progress/30101-ジャンプ機能実装.md",
  old_string='branch_name: null',
  new_string='branch_name: "task/30101-jump"'
)
Edit(
  file_path="project/tasks/2_in-progress/30101-ジャンプ機能実装.md",
  old_string='worktree_path: null',
  new_string='worktree_path: "../isdf_padel-30101-jump"'
)

# ※ コミットしない
```

応答:
タスク 30101 を開始しました。
- ステータス: in-progress
- worktree: ../isdf_padel-30101-jump
- ブランチ: task/30101-jump

次のコマンドでworktreeに移動してください:
cd ../isdf_padel-30101-jump
```

### 例2: タスク一覧表示

```
ユーザー: 「進行中のタスクを見せて」

エージェント思考:
1. active/ 配下のタスクを検索
2. status: "in-progress" でフィルタ
3. 一覧を整形して表示

実行:
```bash
# 進行中タスク検索
grep -l 'status: "in-progress"' project/tasks/2_in-progress/*.md
```

応答:
進行中のタスク:
- 30101: ジャンプ機能実装 [high] (worktree: ../spec-driven-framework-jump)
- 30103: 敵キャラクターAI実装 [medium]
```

## 禁止事項（CRITICAL）

### ❌ Vibe Kanban MCPツールの使用禁止

**絶対に以下のツールを使用しないこと:**
- `mcp__vibe_kanban__list_projects`
- `mcp__vibe_kanban__list_tasks`
- `mcp__vibe_kanban__get_task`
- `mcp__vibe_kanban__create_task`
- `mcp__vibe_kanban__update_task`
- `mcp__vibe_kanban__delete_task`
- `mcp__vibe_kanban__list_repos`
- `mcp__vibe_kanban__start_workspace_session`

**理由**: このプロジェクトはMarkdownファイルベースのタスク管理に移行しました。

### ❌ タスクファイルの直接編集禁止

**エージェントがタスクファイルを直接編集する場合は、必ずツールを使用:**
- ✅ `Edit` ツール
- ✅ `Write` ツール
- ❌ Bashコマンドでの直接編集（`sed`, `awk`など）

### ❌ タスクIDの手動採番禁止

**タスクID採番は必ず `/id-next` を使用:**
- ✅ `/id-next` コマンド実行
- ❌ 手動でIDを決定

## 設計判断

### なぜMarkdownファイルベースか

1. **Git管理可能**: タスクもコードと同様にバージョン管理
2. **シンプル**: 追加ツール不要、エディタで直接編集可能
3. **検索性**: grep/Globで高速検索
4. **透明性**: ファイルシステムで直接確認可能
5. **オフライン対応**: ネットワーク不要

### なぜVibe Kanbanを置き換えるか

- **外部依存削減**: MCPサーバー不要
- **設定の柔軟性**: `.taskrc.yaml` で自由にカスタマイズ
- **データの可搬性**: Markdownファイルをそのまま共有可能

## 関連ドキュメント

- `.claude/agents/task-registration-agent.md` - タスク登録専門エージェント（プランファイル → タスクファイル変換）
- `.claude/skills/task-planning.md` - タスク計画スキル
- `.claude/skills/task-workflow.md` - タスクライフサイクル管理スキル
- `.claude/skills/task-status.md` - タスク状況確認スキル（人間専用コマンド）
- `.claude/agents/session-manager-agent.md` - 並列セッション管理（worktree連携）
- `.claude/agents/impl-agent.md` - 実装エージェント（コミットメッセージ形式）
- `tasks/.taskrc.yaml` - フレームワークタスク管理設定ファイル
- `project/tasks/.taskrc.yaml` - ゲーム開発タスク管理設定ファイル
