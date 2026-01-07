---
description: 次に着手可能なタスクを提案 (project)
argument-hint: [--limit <N>]
---

# /task-next

## 概要

`project/tasks/` 内の着手可能なタスクを一覧表示し、推奨タスクを提案する。

## 対象範囲

- `project/tasks/` のみ（30XXX/B30XXX/R30XXX/PXXX）
- フレームワーク開発タスク（FXXX）は対象外

## 処理フロー

```
1. Glob で project/tasks/1_todo/*.md を取得
   → todoタスク一覧

2. Glob で project/tasks/4_archive/*.md を取得
   → 完了済みタスクのIDを収集（status: "done" のみ）

3. Glob で project/tasks/2_in-progress/*.md, 3_in-review/*.md を取得
   → 進行中タスク一覧

4. 各todoタスクについて:
   a. Frontmatter の blocked_by を読み取り
   b. blocked_by の全IDが完了済み(done)なら READY
   c. blocked_by が空なら READY

5. READY タスクについて:
   a. 進行中タスクと相互依存がないか確認
   b. 相互依存なしなら「並列可能」

6. ソート:
   a. priority: high > medium > low
   b. blocks.length: 多い順（解除インパクト大）
   c. id: 小さい順

7. --limit N オプションで表示件数を制限（デフォルト: 全件）
```

## 出力形式

```
次に着手可能なタスク (N件):

🔴 ⬜ [30013] ポイント進行
   └─ Blocks: 30014, 30016, 30018, 30021 (4件解除)
   └─ 並列: ✅ 可能

🟡 ⬜ [30012] ジャンプショット
   └─ Blocks: なし
   └─ 並列: ✅ 可能

🟢 ⬜ [P002] ドキュメント整備
   └─ Blocks: なし
   └─ 並列: ⚠️ 不可（30011 と相互依存）

---
推奨: 30013（ポイント進行）を先に実装すると4タスクが着手可能になります
```

## アイコン定義

### 優先度（task-status.md 準拠）

- 🔴 high
- 🟡 medium
- 🟢 low

### ステータス

- ⬜ todo
- 🔄 in-progress
- 🔍 in-review

### 並列可能

- ✅ 可能
- ⚠️ 不可（理由）

## 着手可能タスクがない場合

```
着手可能なタスクはありません。

現在の状況:
- 進行中: 2件（30011, 30012）
- 待機中: 3件（依存関係で blocked）
```

## オプション

| オプション | 説明 | デフォルト |
|-----------|------|-----------|
| `--limit <N>` | 表示件数を制限 | 全件 |

## 使用タイミング

### 自動実行

- タスク完了時（done 遷移後）に自動実行
- task-lifecycle.md の「タスク完了時の次タスク提案」参照

### 手動実行

```bash
/task-next           # 着手可能な全タスクを表示
/task-next --limit 3 # 上位3件のみ表示
```

## 関連ドキュメント

- `skills/task-lifecycle.md` - タスク完了時の自動表示
- `skills/task-operations.md` - 判定ロジック詳細
- `skills/task-status.md` - アイコン定義
