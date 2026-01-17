# bug-backlog

## 概要

バグ発見時の一時記録から B30XXX タスク化までのワークフローを定義する。

### 参照元ガイドライン
- impl-agent（実装中にバグ発見時）
- review-agent（レビュー中にバグ発見時）
- task-manager-agent（バックログ管理時）

---

## バグバックログとは

**目的**: バグ発見時に素早く記録し、後で精査・タスク化する

**配置**: `project/tasks/0_backlog/`

**ワークフロー**:
1. 発見時 → 0_backlog/ に記録（status: unreviewed）
2. 精査時 → 詳細追記（status: reviewed）
3. タスク化 → B30XXX-NNN として 1_todo/ へ（status: tasked）

---

## バグ報告作成手順

### 1. コミットハッシュ取得
```bash
git rev-parse --short HEAD
```

### 2. ファイル作成
ファイル名: `BUG-{連番3桁}-{簡潔な説明}.md`
例: `BUG-001-着地判定バグ.md`

### 3. テンプレート適用
`project/tasks/0_backlog/_bug_template.md` を使用

### 4. 必須項目入力
- title: バグの簡潔な説明
- severity: critical / major / minor
- discovered: 発見日（YYYY-MM-DD）
- commit: 発見時のコミットハッシュ
- 現象: 何が起きたか

---

## 深刻度（severity）の基準

| レベル | 説明 | 例 |
|--------|------|-----|
| critical | ゲーム進行不可 | クラッシュ、無限ループ |
| major | 機能不全 | 操作が効かない、表示崩れ |
| minor | 軽微な問題 | 見た目の違和感、微調整 |

---

## B30XXX タスク化

バグを正式タスクにする際：

1. バックログファイルの status を `tasked` に変更
2. `project/tasks/1_todo/` に B30XXX-NNN ファイル作成
3. ID形式: `B{関連機能ID}-{連番3桁}`
   - 例: `B30101-001`（機能30101のバグ1号）
4. バックログファイルは削除または保持（参照用）

---

## 関連ドキュメント

- [task-file-format.md](task-file-format.md) - タスクファイル形式
- [task-operations.md](task-operations.md) - タスク操作
- [task-workflow.md](task-workflow.md) - タスク管理索引
