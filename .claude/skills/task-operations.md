# task-operations

## 概要

**タスク操作スキル** - タスクの作成・更新・検索・管理の操作手順

### 参照元ガイドライン

- task-manager-agent.md（主担当）
- impl-agent.md（タスク参照・更新）
- review-agent.md（タスク完了）

---

## 基本操作

### 1. タスク作成

**task-registration-agent が担当**。詳細: `skills/task-lifecycle.md`

### 2. タスク開始

#### game-dev タスク（worktree作成あり）

```bash
# 1. タスクファイルを 1_todo/ から 2_in-progress/ に移動
mv project/tasks/1_todo/30101-*.md project/tasks/2_in-progress/

# 2. status を in-progress に更新
Edit(status: "todo" -> "in-progress")

# 3. worktree作成（game-devタスクのみ）
git worktree add ../spec-driven-framework-jump auto-$$-jump

# 4. タスクファイル更新
Edit(branch_name: null -> "auto-12345-jump")
Edit(worktree_path: null -> "../spec-driven-framework-jump")
```

#### project-wide / framework タスク（worktree作成なし）

```bash
# 1. ファイル移動
mv tasks/1_todo/F001-*.md tasks/2_in-progress/

# 2. status更新のみ（worktree不要）
Edit(status: "todo" -> "in-progress")
```

### 3. タスク更新

```bash
# status更新
Edit(status: "in-progress" -> "in-review")
Edit(updated_at: "旧タイムスタンプ" -> "新タイムスタンプ")

# spec_ids追加
Edit(spec_ids: ["30201"] -> ["30201", "30202"])
```

### 4. タスク完了

```bash
# 1. ファイルを archive/ に移動
mv project/tasks/3_in-review/30101-*.md project/tasks/4_archive/

# 2. status と completed_at を更新
Edit(status: "in-review" -> "done")
Edit(completed_at: null -> "2025-12-29T16:00:00+09:00")

# 3. worktree削除（オプション、game-devタスクのみ）
git worktree remove ../spec-driven-framework-jump
```

### 5. タスクキャンセル

```bash
# ファイルを archive/ に移動、status を cancelled に
mv project/tasks/2_in-progress/30101-*.md project/tasks/4_archive/
Edit(status: "in-progress" -> "cancelled")
```

---

## Progress/Next Actions管理

### Progress追加

タスクファイルの `## Progress` セクションに追記:

```markdown
## Progress

- **Current Phase:** Implementation
- **Completed Steps:**
  - [x] 仕様書更新（REQ-30101-01）
  - [x] データ定義作成
```

### Next Actions更新

タスクファイルの `## Next Actions` セクションを更新:

```markdown
## Next Actions

1. Player.cs にジャンプロジック実装
2. テスト作成
```

---

## タスク検索

### アクティブタスク一覧

```bash
# 全アクティブタスク（archive以外）
ls project/tasks/1_todo/ project/tasks/2_in-progress/ project/tasks/3_in-review/

# Glob使用
Glob("project/tasks/2_in-progress/*.md")
```

### 状態別フィルタ

```bash
Glob("project/tasks/1_todo/*.md")        # 未着手
Glob("project/tasks/2_in-progress/*.md") # 進行中
Glob("project/tasks/3_in-review/*.md")   # レビュー中
```

### タイプ別フィルタ

```bash
# game-dev: 30XXX
Grep(pattern="type: \"game-dev\"", path="project/tasks")

# framework: FXXX
Grep(pattern="type: \"framework\"", path="tasks")
```

---

## タスク依存関係管理

### blocked_by / blocks の使い方

```yaml
# タスク30101（先行タスク）
blocks: ["30102"]  # 30102をブロックしている

# タスク30102（後続タスク）
blocked_by: ["30101"]  # 30101完了まで開始不可
```

### 依存関係チェック

```bash
# タスク30102の依存確認
Read("project/tasks/1_todo/30102-敵キャラクター実装.md")
# blocked_by: ["30101"]

# タスク30101の状態確認
Glob("project/tasks/*/30101-*.md")
# 2_in-progress/ にあれば未完了
```

---

## worktree管理

### worktree作成条件

| タイプ | worktree |
|--------|----------|
| game-dev | ✅ 作成 |
| project-wide | ❌ なし |
| framework | ❌ なし |

### worktree作成

```bash
BRANCH="auto-$$-feature-name"
WORKTREE="../spec-driven-framework-feature"
git worktree add "${WORKTREE}" "${BRANCH}"
```

### worktree削除

```bash
git worktree remove ../spec-driven-framework-feature
```

### worktree一覧

```bash
git worktree list
```

---

## 親子タスク操作（将来実装）

親子タスク機能の定義は `skills/task-lifecycle.md` を参照。

現時点では以下の方針:
- 親タスク: status="planning" を使用
- 子タスク: parent_id で親を参照
- 親タスクは全子タスク完了で done に遷移

---

## ベストプラクティス

1. **タスクファイル更新時は必ず updated_at を更新**
2. **worktree作成前にタスクタイプを確認**（game-devのみ）
3. **タスク完了時は必ず completed_at を設定**
4. **依存関係は双方向で管理**（blocks/blocked_by）

---

## 関連ドキュメント

- `skills/task-lifecycle.md` - タスク状態遷移、親子タスク
- `skills/task-file-format.md` - タスクファイル形式
- `agents/task-manager-agent.md` - タスク管理ガイドライン
