---
id: "F047"
title: "task-nextにバグバックログ表示機能を追加"
status: "done"
priority: "medium"
blocked_by: []
blocks: []
created: "2026-01-07"
branch: ""
worktree: ""
---

# F047: task-nextにバグバックログ表示機能を追加

## 概要

`/task-next` コマンド実行時に、精査済み（reviewed）のバグバックログも表示し、対応忘れを防ぐ。

## 背景

- ccaf03a でバグバックログ機能（F046）を追加した
- しかし `/task-next` はバックログを読み込んでおらず、バグが表示されない
- 対応忘れ防止のため、task-next にバグセクションを追加する

## 修正ファイル

- `scripts/task-next.py`

## 実装手順

### 1. BugInfo TypedDict 追加（L17付近）
```python
class BugInfo(TypedDict):
    id: str
    title: str
    severity: str  # critical / major / minor
    discovered: str
    related_feature: str
    status: str
```

### 2. parse_bug_frontmatter 関数追加（L73付近）
- バグ用 Frontmatter パース（severity, discovered, related_feature を取得）
- テンプレートファイル（_bug_template.md）は除外

### 3. get_severity_icon 関数追加
```python
def get_severity_icon(severity: str) -> str:
    return {"critical": "🔴", "major": "🟠", "minor": "🟡"}.get(severity, "🟡")
```

### 4. バックログ読み込み追加（L96付近）
```python
backlog_files = list(tasks_dir.glob("0_backlog/*.md"))
# テンプレートを除外
backlog_files = [f for f in backlog_files if not f.name.startswith("_")]
```

### 5. reviewed バグ抽出（L107付近）
```python
reviewed_bugs: list[BugInfo] = []
for f in backlog_files:
    bug = parse_bug_frontmatter(f)
    if bug and bug["status"] == "reviewed":
        reviewed_bugs.append(bug)
```

### 6. 出力にバグセクション追加（L156付近、タスク出力の前）
```python
if reviewed_bugs:
    print("🐛 精査済みバグ（タスク化待ち）:")
    print()
    for bug in reviewed_bugs:
        icon = get_severity_icon(bug["severity"])
        print(f"{icon} [{bug['id']}] {bug['title']}")
        print(f"   └─ 深刻度: {bug['severity']} | 関連: {bug['related_feature']} | 発見: {bug['discovered']}")
        print()
    print("---")
    print()
```

## 出力例

```
🐛 精査済みバグ（タスク化待ち）:

🟡 [BUG-001] テストshadowフィールド欠落
   └─ 深刻度: minor | 関連: 30022 | 発見: 2026-01-07

---

次に着手可能なタスク (3件):

🔴 ⬜ [30013] ポイント進行
   └─ Blocks: 30014, 30016, 30018, 30021 (4件解除)
   └─ 並列: ✅ 可能
```

## 完了条件

- [x] BugInfo TypedDict 追加
- [x] parse_bug_frontmatter 関数追加
- [x] get_severity_icon 関数追加
- [x] バックログ読み込みロジック追加
- [x] reviewed バグの抽出ロジック追加
- [x] 出力にバグセクション追加
- [x] 動作確認（`/task-next` でバグが表示される）

## Progress

### 2026-01-07
- タスク作成
- 実装完了: BugInfo TypedDict, parse_bug_frontmatter, get_severity_icon 関数追加
- バグIDはファイル名から抽出する方式に修正（Frontmatterにid無し）
- 動作確認完了
