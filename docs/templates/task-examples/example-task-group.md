---
id: "30101"
title: "ジャンプ機能実装"
type: "game-dev"
status: "planning"
priority: "high"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["ジャンプ機能実装", "parent"]
created_at: "2025-12-29T10:00:00.000000"
updated_at: "2025-12-29T10:00:00.000000"
completed_at: null
parent_task_id: null
---

# ジャンプ機能実装（親タスク）

## 概要

このタスクは **親タスク（タスクグループ）** です。
ジャンプ機能の実装を3つの段階に分割して、順次実行します。

**親タスクの役割:**
- 子タスクの進捗管理
- 子タスク間の依存関係管理
- 全体の完了判定

**親タスクは実装作業を行いません。** `status="planning"` で固定され、子タスクの完了を管理します。

## サブタスク

このタスクは以下のサブタスクに分割されています：

- [ ] **30101-1**: ジャンプ機能-仕様書 (spec)
  - タスクタイプ: `game-dev`
  - ステータス: `todo`
  - 依存: なし
  - 説明: ジャンプ機能の仕様書を作成する（spec.md, design.md, behavior.md, test.md）

- [ ] **30101-2**: ジャンプ機能-実装 (impl)
  - タスクタイプ: `game-dev`
  - ステータス: `todo`
  - 依存: `blocked_by: ["30101-1"]`（仕様書完了後に開始）
  - 説明: 仕様書に基づいてジャンプ機能を実装する

- [ ] **30101-3**: ジャンプ機能-テスト (test)
  - タスクタイプ: `game-dev`
  - ステータス: `todo`
  - 依存: `blocked_by: ["30101-2"]`（実装完了後に開始）
  - 説明: ジャンプ機能のテストを実装する

## マージ戦略

**順序:** 30101-1 → 30101-2 → 30101-3

**並列度:** 低（順次実行）

**理由:** 仕様策定完了後に実装開始、実装完了後にテスト作成

### ワークフロー

```
1. 子タスク30101-1（仕様書）を開始
   → worktree作成: ../spec-driven-framework-jump-spec
   → 仕様書作成（spec-agent, design-agent, behavior-agent, test-agent）
   → コミット・PRマージ
   → 子タスク30101-1完了

2. 子タスク30101-2（実装）を開始（30101-1完了後）
   → worktree作成: ../spec-driven-framework-jump-impl
   → 実装（impl-agent）
   → コミット・PRマージ
   → 子タスク30101-2完了

3. 子タスク30101-3（テスト）を開始（30101-2完了後）
   → worktree作成: ../spec-driven-framework-jump-test
   → テスト実装
   → コミット・PRマージ
   → 子タスク30101-3完了

4. 親タスク30101を完了
   → update-parentコマンドで親タスクをdoneに更新
```

## 進捗確認

Claude Code に「タスク30101の子タスクを確認して」と指示すると、以下のような形式で表示されます：

```
Parent Task: 30101 - ジャンプ機能実装 [planning]
Progress: 1/3 completed (33%)

Children:
  ✅ 30101-1: ジャンプ機能-仕様書   [done]
  🔄 30101-2: ジャンプ機能-実装     [in-progress]
  ⏸  30101-3: ジャンプ機能-テスト   [todo] (blocked by 30101-2)
```

### 親タスク更新

全子タスクが完了したら、「タスク30101を完了にして」と指示します。
Claude Code が子タスクの完了状況を確認し、親タスクを完了状態に更新します。

## タスクグループのパターン

このタスクは **spec-impl-test** パターンを使用しています。

### spec-impl-test パターン

**目的:** 仕様策定 → 実装 → テストの順次実行

**使い時:**
- ✅ 1つの機能を段階的に実装したい
- ✅ 仕様が固まっていない（仕様策定から開始）
- ✅ 各段階でレビューを挟みたい

**子タスク:**
1. 仕様書作成（spec.md, design.md, behavior.md, test.md）
2. 実装（コード）
3. テスト（ユニットテスト、統合テスト）

### 他のパターン

#### parallel-impl パターン

**目的:** 複数のサブシステムを並列実装 + 統合テスト

**使い時:**
- ✅ 複数の独立したコンポーネントを並列開発したい
- ✅ 最後に統合テストが必要

**例:**
```
親タスク: 30201 [planning] バトルシステム実装
  ├─ 30201-1 [todo] 入力処理 (impl) → worktree: ../spec-driven-framework-battle-input
  ├─ 30201-2 [todo] 物理演算 (impl) → worktree: ../spec-driven-framework-battle-physics
  ├─ 30201-3 [todo] UI (impl) → worktree: ../spec-driven-framework-battle-ui
  └─ 30201-4 [todo] 統合テスト (test) blocked_by: [30201-1,30201-2,30201-3]
```

**並列度:** 高（30201-1, 30201-2, 30201-3 を同時に3つのターミナルで開発可能）

## 実装上の注意点

### 1. 親タスクは planning で固定

親タスクの `status` は常に `planning` です。実装作業は行いません。

### 2. 子タスクの独立性

各子タスクは独立したworktreeで作業します：
- 30101-1: `../spec-driven-framework-jump-spec`
- 30101-2: `../spec-driven-framework-jump-impl`
- 30101-3: `../spec-driven-framework-jump-test`

### 3. 依存関係の自動管理

`blocked_by` フィールドで依存関係が管理されます：
- 30101-2 は 30101-1 完了まで開始不可
- 30101-3 は 30101-2 完了まで開始不可

## 完了条件

- [ ] 全子タスク（30101-1, 30101-2, 30101-3）が `done` 状態
- [ ] 親タスク30101が `done` 状態に更新されている
- [ ] 全てのworktreeがクリーンアップされている

## メモ

### 設計判断

**なぜタスクグループを使うのか:**
- 大きな機能を段階的に実装できる
- 各段階でレビュー・修正が可能
- 進捗状況が明確になる

**単一タスク vs タスクグループ:**
- 単一タスク: 1-2日で完了する機能、仕様が固まっている
- タスクグループ: 3日以上かかる機能、仕様策定から開始

### 将来の拡張

- タスクグループの自動進捗レポート
- 子タスク完了時の親タスク自動更新

### 関連ドキュメント

- [タスク管理 - Task Groups](../../concepts/tasks.md)
- [task-workflow.md - 親子タスク](../../../.claude/skills/task-workflow.md#親子タスクタスクグループ)

---

**このテンプレートは親タスク（タスクグループ）の例です。**
**実際のタスクグループでは、プロジェクトの開発フローに合わせて子タスクを調整してください。**

---

## 子タスクファイルの例

### 30101-1: ジャンプ機能-仕様書

```yaml
---
id: "30101-1"
title: "ジャンプ機能-仕様書"
type: "game-dev"
status: "todo"
priority: "high"
spec_ids: []
blocked_by: []
blocks: ["30101-2"]
parent_task_id: "30101"  # 親タスクへの参照
tags: ["ジャンプ機能実装", "spec"]
...
---

## 説明

ジャンプ機能の仕様書を作成する。

## 実装内容

- [ ] spec.md（要件定義）
- [ ] design.md（データ構造）
- [ ] behavior.md（ロジック）
- [ ] test.md（テストシナリオ）
```

### 30101-2: ジャンプ機能-実装

```yaml
---
id: "30101-2"
title: "ジャンプ機能-実装"
type: "game-dev"
status: "todo"
priority: "high"
spec_ids: ["30201", "30202"]
blocked_by: ["30101-1"]  # 仕様書完了まで開始不可
blocks: ["30101-3"]
parent_task_id: "30101"  # 親タスクへの参照
tags: ["ジャンプ機能実装", "impl"]
...
---

## 説明

仕様書（30101-1で作成）に基づいてジャンプ機能を実装する。
```

### 30101-3: ジャンプ機能-テスト

```yaml
---
id: "30101-3"
title: "ジャンプ機能-テスト"
type: "game-dev"
status: "todo"
priority: "high"
spec_ids: ["30201"]
blocked_by: ["30101-2"]  # 実装完了まで開始不可
blocks: []
parent_task_id: "30101"  # 親タスクへの参照
tags: ["ジャンプ機能実装", "test"]
...
---

## 説明

ジャンプ機能のテストを実装する。
```
