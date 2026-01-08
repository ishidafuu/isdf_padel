# F056: session-merge スキルに --squash 警告追加

## 概要
session-merge コマンド実行時に `git merge` と `git merge --squash` を間違えるミスを防ぐため、スキルファイルに警告を追加する。

## 背景
- commit bd84eae で通常のマージコミット（親2つ）が作成された
- 原因: `git merge --squash` ではなく `git merge` を実行してしまった
- スカッシュマージなら親は1つで、ブランチの全変更が1コミットに圧縮される

## 対応内容
`.claude/commands/session-merge.md` に警告セクションを追加

```markdown
## ⚠️ 重要: --squash を絶対に忘れないこと

❌ git merge <branch>        ← 親2つのマージコミットになる
✅ git merge --squash <branch>  ← 1コミットに圧縮
```

## 完了条件
- [x] session-merge.md に警告が追加されている
- [x] 警告がマージ手順の直前に配置されている
