# バグバックログ

## 概要

バグ発見時に素早く記録し、後で精査・タスク化するための一時保管場所。

## ファイル命名規則

`BUG-{連番3桁}-{簡潔な説明}.md`

例:
- `BUG-001-着地判定バグ.md`
- `BUG-002-音が鳴らない.md`

**連番管理**: `ls BUG-*.md | tail -1` で最新番号を確認し、+1 する

## 使い方

1. `_bug_template.md` をコピー
2. ファイル名を `BUG-XXX-説明.md` に変更
3. 必須項目を入力:
   - title, severity, discovered, commit, 現象

## ステータス

| status | 説明 |
|--------|------|
| unreviewed | 未精査（発見直後） |
| reviewed | 精査済み（詳細追記完了） |
| tasked | タスク化済み（B30XXX 作成済み） |

## 詳細

スキル参照: `.claude/skills/bug-backlog.md`
