# task-lifecycle

## 概要

**タスク状態遷移スキル** - タスクのライフサイクルと状態遷移を定義

### 参照元ガイドライン

- task-manager-agent.md（主担当）
- impl-agent.md（タスク参照・更新）
- session-manager-agent.md（worktree連携）

---

## タスク作成の基本フロー（IMPORTANT）

**タスク作成は2段階に分離されました：**

1. **プラン作成** - ユーザーがプランモードで実行
2. **タスク登録** - task-registration.md を参照して実行

### Phase 1: プラン作成

```
ユーザー：プランモードに入る
↓
【プランモード開始】
├─ Explore agent で関連調査
│   - 既存実装パターン
│   - 関連仕様書
│   - 依存関係
│   - 影響範囲
├─ 仕様書の読み込み・確認
├─ ユーザーへの質問・確認（必要に応じて）
├─ プランファイル作成（~/.claude/plans/xxx.md）
│   - Task Type（framework/game-dev/project-wide）
│   - タスク詳細
│   - 実装方針
│   - 依存関係
└─ ExitPlanMode
↓
プランファイル保存完了
```

### Phase 2: タスク登録

```
ユーザー：「プランからタスクを作成して」
↓
Claude が task-registration.md を参照して直接実行
↓
【タスク登録】
├─ プランファイル自動検出（~/.claude/plans/、30分以内）
├─ タスクタイプ判定
├─ ID採番（/id-next）
├─ 配置先決定（tasks/ or project/tasks/）
├─ タスクファイル生成（status: "todo"）
│   - プラン全文埋め込み（Detailed Implementation Plan）
└─ プランファイル保持（削除しない）
↓
完了報告（タスクID、ファイルパス）
```

**詳細:**
- プラン作成: `.claude/skills/task-planning.md`
- タスク登録: `.claude/skills/task-registration.md`
- タスク管理: `.claude/agents/task-manager-agent.md`

---

## タスクライフサイクル

```
[作成] → [開始] → [実装] → [レビュー] → [完了]
           ↓                    ↓
        [一時停止]          [修正要求]
           ↓                    ↓
        [再開] ←───────────────┘

                [キャンセル] → [cancelled]
```

### 状態遷移

| 状態 | 説明 | 次の状態 |
|-----|------|---------|
| `todo` | 未着手 | `in-progress`, `cancelled` |
| `in-progress` | 実装中 | `in-review`, `todo`（一時停止）, `cancelled` |
| `in-review` | レビュー中 | `done`, `in-progress`（修正） |
| `done` | 完了 | - |
| `cancelled` | キャンセル | - |
| `planning` | 親タスク専用（子タスクの完了を待機） | `done`（全子タスク完了時） |

### 状態遷移時のディレクトリ移動

| 状態変更 | ファイル移動 |
|---------|-------------|
| `todo` → `in-progress` | `1_todo/` → `2_in-progress/` |
| `in-progress` → `in-review` | `2_in-progress/` → `3_in-review/` |
| `in-review` → `done` | `3_in-review/` → `4_archive/` |
| `*` → `cancelled` | 現在のディレクトリ → `4_archive/` |

> **NOTE**: 全タスクタイプ（game-dev/project-wide/framework）で in-review 経由必須。

---

## タスク開始の操作フロー

> **NOTE: MAIN側で先にステータス変更を行う**
>
> - ステータス変更はworktree作成**前**にMAIN側で実行
> - **コミットは不要** - worktree の存在で並列作業状況を把握できる
> - **並列作業の確認**: `git worktree list`（推奨）
> - **制限事項**: PXXX/FXXX は worktree 非対応のため検出不可

### game-dev タスク（worktree作成あり）

```
【MAIN側で実行】           【worktree側で実行】
      │                         │
      ▼                         │
┌─────────────────┐             │
│ 1. ファイル移動  │             │
│    1_todo/ →    │             │
│    2_in-progress/│            │
└────────┬────────┘             │
         ▼                      │
┌─────────────────┐             │
│ 2. status更新   │             │
│    "in-progress"│             │
└────────┬────────┘             │
         ▼                      │
┌─────────────────┐             │
│ 3. worktree作成 │             │
└────────┬────────┘             │
         ▼                      │
┌─────────────────┐             │
│ 4. branch_name  │             │
│    worktree_path│             │
│    を更新       │             │
└────────┬────────┘             │
         │                      │
         │ ※コミットしない      │
         │                      │
         └──── worktreeに移動 ──┤
                                ▼
                        ┌─────────────────┐
                        │ 5. 実装作業     │
                        │    Progress更新 │
                        │    Next Actions │
                        └─────────────────┘
```

**ポイント**:
- 手順 1-4 は **MAIN側** で実行（コミットなし）
- 手順 5 以降は **worktree側** で実行
- worktree の存在で並列作業を検出可能

### project-wide / framework タスク（worktree作成なし）

```
【MAIN側で実行】
      │
      ▼
┌─────────────────┐
│ 1. ファイル移動  │
└────────┬────────┘
         ▼
┌─────────────────┐
│ 2. status更新   │
└────────┬────────┘
         ▼
┌─────────────────┐
│ 3. 実装作業     │
└─────────────────┘
```

worktree不要のため、全てMAIN側で実行。

---

## 親子タスク（タスクグループ）【将来実装】

> **Note**: 親子タスク機能は定義のみで、現時点では未実装です。将来のバージョンで実装予定。

### 概要

大きなタスクを複数の小さなタスクに分割して、並列または順次実行するための機能。

**親タスク（コンテナ）**: `status="planning"` で固定、子タスクの完了を管理
**子タスク（実作業）**: 実際の作業を行うタスク、`parent_task_id` で親を参照

### パターン

#### パターン1: spec-impl-test（順次実行）

```
仕様書 → 実装 → テストの順次実行

親タスク: 30101 [planning]
子タスク: 30101-1 [todo, spec]
         30101-2 [todo, impl] blocked_by: [30101-1]
         30101-3 [todo, test] blocked_by: [30101-2]
```

#### パターン2: parallel-impl（並列実行）

```
複数のコンポーネントを並列実装 + 最後に統合テスト

親タスク: 30201 [planning]
子タスク: 30201-1 [todo, impl] blocked_by: []
         30201-2 [todo, impl] blocked_by: []
         30201-3 [todo, impl] blocked_by: []
         30201-4 [todo, test] blocked_by: [30201-1,30201-2,30201-3]
```

### ワークフロー例

#### 順次実行（spec → impl → test）

```
1. create-group でタスクグループ作成
   → 親タスク30101 + 子タスク30101-1,2,3

2. 子タスク30101-1（仕様書）を開始
   → worktree作成、実装、完了

3. 子タスク30101-2（実装）を開始（30101-1完了後）
   → worktree作成、実装、完了

4. 子タスク30101-3（テスト）を開始（30101-2完了後）
   → worktree作成、実装、完了

5. update-parent で親タスクを完了
   → 親タスク30101を done に更新
```

---

## セッション引き継ぎ（Handover連携）

### 概要

**タスクファイルが Single Source of Truth**

セッション引き継ぎ情報は、別ファイル（`.claude/handover.md`）ではなく、タスクファイル内の以下セクションに記録する：

- `## Progress` - 完了した作業の履歴
- `## Next Actions` - 次に実行すべきアクション
- `## メモ` - 重要な決定事項、注意点

**Git状態は保存しない** - 毎回リアルタイムで取得するため、タスクファイルには含めない。

### /handover コマンド

セッション終了前に、進捗と次のアクションをタスクファイルに記録する。

```bash
/handover                    # in-progress タスクを自動検出して更新
/handover --task F011        # 指定タスクを更新
/handover --all              # 全 in-progress タスクを更新
```

### /resume-handover コマンド

セッション開始時に、タスクファイルから状態を復元する。

```bash
/resume-handover                              # in-progress タスクを自動検出
/resume-handover --task F011                  # 指定タスクから再開
/resume-handover 今日はテストから始めたい      # 追加プロンプト付き
```

---

## タスク完了時の次タスク提案（AUTOMATIC）

**対象**: `project/tasks/` のタスク（30XXX/B30XXX/R30XXX/PXXX）のみ

タスクを `done` に遷移させた後、Claude Code は自動的に `/task-next` を実行し、次に着手可能なタスクを表示する。

### 自動実行タイミング

```
タスク完了処理（status: "done", completed_at 設定）
↓
/task-next を自動実行
↓
着手可能なタスクを表示
```

### 出力形式

```
✅ タスク 30010（ショット入力）を完了しました

---
🔓 新たに着手可能になったタスク:

🔴 ⬜ [30012] ジャンプショット
   └─ Blocks: 30015, 30018 (2件解除)
   └─ 並列: ✅ 可能

🟡 ⬜ [30013] ポイント進行
   └─ Blocks: 30014, 30016 (2件解除)
   └─ 並列: ✅ 可能

---
推奨: 30012（ジャンプショット）を先に実装すると2タスクが着手可能になります
```

### 着手可能タスクがない場合

```
✅ タスク 30010（ショット入力）を完了しました

---
着手可能なタスクはありません。

現在の状況:
- 進行中: 1件（30011）
- 待機中: 2件（依存関係で blocked）
```

### 詳細

判定ロジックの詳細は以下を参照:
- `/task-next` コマンド: `commands/task-next.md`
- 判定ロジック: `skills/task-operations.md`（次タスク判定セクション）

---

## 後続タスク作成ガイドライン（CRITICAL）

タスク作業中または完了時に、関連する後続タスクが必要になることがある。

### 発生シナリオ

1. **リファクタタスク完了時** - 追加の改善タスクを発見
2. **バグ修正完了時** - 根本原因の修正が別タスクとして必要
3. **機能実装中** - 前提となる別機能の実装が必要

### 必須フロー

**すべてのタスク作成は task-registration.md を参照して行う**

```
後続タスクが必要
  ↓
プランモード開始
  ↓
プランファイル作成
  ↓
「プランからタスクを作成して」と依頼
  ↓
Claude が task-registration.md を参照してタスク作成
```

### 禁止事項

| 禁止 | 理由 |
|------|------|
| タスクファイルの手動作成 | frontmatter 漏れの原因 |
| コピペでのタスクファイル複製 | ID 重複、依存関係ミス |
| task-registration.md を経由しない作成 | タスク管理システムとの不整合 |

---

## 関連ドキュメント

- `skills/task-file-format.md` - タスクファイル形式
- `skills/task-operations.md` - タスク操作
- `agents/task-manager-agent.md` - タスク管理専門ガイドライン
- `commands/handover.md` - /handover コマンド詳細
- `commands/resume-handover.md` - /resume-handover コマンド詳細
