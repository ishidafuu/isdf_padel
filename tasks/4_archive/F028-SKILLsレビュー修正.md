---
id: "F028"
title: "SKILLsレビュー修正"
type: "framework"
status: "done"
priority: "medium"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["skills", "docs", "refactor"]
created_at: "2026-01-04T21:00:00+09:00"
updated_at: "2026-01-04T21:00:00+09:00"
completed_at: "2026-01-04T21:30:00+09:00"
---

# Task F028: SKILLsレビュー修正

## Summary

全10個のSKILLファイルをレビュー・修正。CLAUDE.mdとの矛盾解消（プランファイル削除→保持）、重複削除、表記統一を実施。

## Related Specifications

- `.claude/CLAUDE.md` - 整合性確認用

## Progress

- **Current Phase:** Done
- **Completed Steps:**
  - [x] task-planning.md 修正
  - [x] task-workflow.md 修正
  - [x] task-status.md 修正
  - [x] deep-investigation.md 修正
  - [x] ears.md 表記統一
  - [x] extraction-schema.md 表記統一
  - [x] impl-comments.md 表記統一
  - [x] コミット完了 (8d62b59)

## Next Actions

1. task-planning.md を読み込み
2. プランファイル削除セクションを削除
3. 「委譲」表記を修正
4. 表記統一（利用エージェント→参照元ガイドライン）
5. 次のファイルへ

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

---

## Detailed Implementation Plan

> このセクションは、タスク作成時のプランモードで生成されたプランファイルの全内容です。

### 修正対象（7ファイル）

| ファイル | 優先度 | 主な修正内容 |
|---------|-------|-------------|
| `.claude/skills/task-planning.md` | 高 | プランファイル削除記述削除、委譲表記修正 |
| `.claude/skills/task-workflow.md` | 高 | プランファイル削除記述削除、重複削除 |
| `.claude/skills/task-status.md` | 中 | シェルスクリプト例に注記 |
| `.claude/skills/deep-investigation.md` | 中 | ツール呼び出し例更新 |
| `.claude/skills/ears.md` | 低 | 表記統一のみ |
| `.claude/skills/extraction-schema.md` | 低 | 表記統一のみ |
| `.claude/skills/impl-comments.md` | 低 | 表記統一のみ |

### 修正不要（3ファイル）

- `.claude/skills/design-patterns.md` - 完成度が高い
- `.claude/skills/parallel-sessions.md` - 参照パス確認済み
- `.claude/skills/ntfy-notification.md` - hookパス確認済み

### Implementation Approach

1. **整合性修正（高優先度）**
   - task-planning.md: プランファイル削除記述を削除、「委譲」表記を修正
   - task-workflow.md: プランファイル削除記述を削除、重複部分を参照リンクに

2. **実装整合性（中優先度）**
   - task-status.md: シェルスクリプト例に注記追加
   - deep-investigation.md: ツール呼び出し例を現行形式に更新

3. **表記統一（低優先度）**
   - 全対象ファイル: 「利用エージェント」→「参照元ガイドライン」

### Progress Steps

- [ ] task-planning.md 修正（プランファイル削除→保持、委譲表記修正）
- [ ] task-workflow.md 修正（プランファイル削除→保持、重複削除）
- [ ] task-status.md 修正（シェルスクリプト例に注記）
- [ ] deep-investigation.md 修正（ツール呼び出し例更新）
- [ ] ears.md 表記統一
- [ ] extraction-schema.md 表記統一
- [ ] impl-comments.md 表記統一
- [ ] 最終確認・コミット

### Technical Notes

- プランファイルはCLAUDE.mdで「保持する」と明記されている
- 「エージェント」は「処理ガイドライン」に変更されている
- task-workflow.mdは1343行と長いが、重複削除のみで大幅な圧縮はしない

### Verification Checklist

- [ ] CLAUDE.mdとの整合性確認（プランファイル扱い）
- [ ] 全ファイルの「利用エージェント」が「参照元ガイドライン」に変更
- [ ] 「委譲」表記がすべて修正されている
- [ ] シェルスクリプト例に適切な注記がある
