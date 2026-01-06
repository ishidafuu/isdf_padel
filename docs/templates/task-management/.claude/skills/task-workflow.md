# task-workflow

タスク管理スキル（統合版）

---

## ディレクトリ構造

```
tasks/
├── .taskrc.yaml           # 設定ファイル
├── 1_todo/                # 未着手タスク
├── 2_in-progress/         # 実装中タスク
└── 3_archive/             # 完了・キャンセル済み
```

---

## ファイル名形式

**形式: `{ID}-{タイトル}.md`**

```
T001-ログイン機能実装.md
T002-ダッシュボード作成.md
```

---

## Frontmatter

### 必須フィールド

```yaml
---
id: "T001"                           # タスクID（必須）
title: "ログイン機能実装"              # タイトル（必須）
status: "todo"                       # ステータス（必須）
priority: "medium"                   # 優先度（必須）
tags: []                             # タグ（任意）
created_at: "2026-01-06T10:00:00"    # 作成日時（必須）
updated_at: "2026-01-06T10:00:00"    # 更新日時（必須）
completed_at: null                   # 完了日時（完了時のみ）
---
```

| フィールド | 型 | 必須 | 説明 |
|-----------|-----|-----|------|
| `id` | string | Yes | タスクID（T001形式） |
| `title` | string | Yes | タスクタイトル |
| `status` | string | Yes | todo, in-progress, done, cancelled |
| `priority` | string | Yes | high, medium, low |
| `tags` | array | - | タグ（検索用） |
| `created_at` | string | Yes | ISO 8601形式 |
| `updated_at` | string | Yes | ISO 8601形式 |
| `completed_at` | string | - | ISO 8601形式（完了時のみ） |

---

## 本文構造

```markdown
---
(frontmatter)
---

# Task {ID}: {title}

## Summary

タスクの概要説明

## Progress

### Completed

- [x] 完了した作業1
- [x] 完了した作業2

## Next Actions

1. 次のアクション1
2. 次のアクション2

## メモ

- 重要な決定事項
- 注意点
```

---

## 状態遷移

```
[todo] → [in-progress] → [done]
              ↓
         [cancelled]
```

| 状態 | 説明 | 次の状態 |
|-----|------|---------|
| `todo` | 未着手 | in-progress, cancelled |
| `in-progress` | 実装中 | done, todo（一時停止）, cancelled |
| `done` | 完了 | - |
| `cancelled` | キャンセル | - |

### 状態遷移時のディレクトリ移動

| 状態変更 | ファイル移動 |
|---------|-------------|
| `todo` → `in-progress` | `1_todo/` → `2_in-progress/` |
| `in-progress` → `done` | `2_in-progress/` → `3_archive/` |
| `*` → `cancelled` | 現在のディレクトリ → `3_archive/` |

---

## 基本操作

### タスク作成

1. `/id-next` でID採番
2. タスクファイルを `1_todo/` に作成

### タスク開始

```bash
# 1. ファイル移動
mv tasks/1_todo/T001-*.md tasks/2_in-progress/

# 2. status更新
Edit(status: "todo" -> "in-progress")
Edit(updated_at: "新タイムスタンプ")
```

### タスク完了

```bash
# 1. ファイル移動
mv tasks/2_in-progress/T001-*.md tasks/3_archive/

# 2. status と completed_at 更新
Edit(status: "in-progress" -> "done")
Edit(completed_at: "完了タイムスタンプ")
Edit(updated_at: "新タイムスタンプ")
```

### タスクキャンセル

```bash
# ファイルを archive/ に移動、status を cancelled に
mv tasks/2_in-progress/T001-*.md tasks/3_archive/
Edit(status: "in-progress" -> "cancelled")
```

---

## タスク検索

```bash
# 全アクティブタスク
ls tasks/1_todo/ tasks/2_in-progress/

# 状態別
Glob("tasks/1_todo/*.md")        # 未着手
Glob("tasks/2_in-progress/*.md") # 進行中
Glob("tasks/3_archive/*.md")     # 完了・キャンセル
```

---

## Handover連携

タスクファイルが Single Source of Truth。引き継ぎ情報は以下セクションに記録:

- `## Progress` - 完了した作業の履歴
- `## Next Actions` - 次に実行すべきアクション
- `## メモ` - 重要な決定事項

### /handover

セッション終了前に進捗を記録:

```bash
/handover              # in-progress タスクを自動検出
/handover --task T001  # 指定タスクを更新
```

### /resume-handover

セッション開始時に状態を復元:

```bash
/resume-handover              # in-progress タスクを自動検出
/resume-handover --task T001  # 指定タスクから再開
```

---

## ベストプラクティス

1. **タスク優先原則** - 作業開始前にタスクを作成
2. **即時コミット** - タスクファイル更新後はコミット
3. **updated_at更新** - タスクファイル変更時は必ず更新
4. **completed_at設定** - 完了時は必ず設定
