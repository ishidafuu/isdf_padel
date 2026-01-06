---
id: F033
title: worktree作成時にtarget/シンボリックリンクを自動作成
type: framework
status: done
priority: medium
created_at: 2026-01-07
completed_at: 2026-01-07
---

# F033: worktree作成時にtarget/シンボリックリンクを自動作成

## 概要

worktree方式で並列セッションを作成する際、Rustの`target/`ディレクトリがコピーされず毎回フルビルドが走る問題を解決する。

## 問題

- worktreeは独立したディレクトリを作成するが、`target/`は含まれない
- 各worktreeで`cargo build`すると毎回フルビルドが発生
- ビルド時間が大幅に増加

## 解決策

worktree作成後に、メインリポジトリの`target/`へのシンボリックリンクを自動作成する。

```bash
# worktreeディレクトリで実行
cd <worktree_path>/project
ln -s <main_repo>/project/target target
```

## 変更対象ファイル

1. `.claude/agents/session-manager-agent.md`
   - フローA Step 2の後にシンボリックリンク作成ステップを追加

2. `.claude/skills/parallel-sessions.md`
   - Step 1に説明を追加

## 実装詳細

### session-manager-agent.md への追加

「2. worktree作成とブランチ準備」の後に以下を追加:

```bash
# 3. ビルドキャッシュの共有設定（Rust project用）
for feature in player enemy stage; do
  WORKTREE_PATH="${PARENT_DIR}/${PROJECT_NAME}-${feature}"
  if [ -d "${PROJECT_ROOT}/project/target" ]; then
    # 既存のtargetがあれば削除（空の場合のみ）
    rm -rf "${WORKTREE_PATH}/project/target" 2>/dev/null
    # シンボリックリンク作成
    ln -s "${PROJECT_ROOT}/project/target" "${WORKTREE_PATH}/project/target"
    echo "✅ target/ symlink created for ${feature}"
  fi
done
```

## 完了条件

- [x] session-manager-agent.mdにシンボリックリンク作成ステップを追加
- [x] parallel-sessions.mdに説明を追加
- [x] 既存のworktreeがある場合のクリーンアップ処理も考慮（既存のクリーンアップ処理で対応済み）

## 関連

- session-manager-agent.md
- parallel-sessions.md
