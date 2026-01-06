---
description: セッションマージ（競合検出、推奨順序提案） (project)
argument-hint: [--auto] [--cleanup]
---

# /session-merge コマンド

並列セッションをマージするための競合検出と推奨順序を提案します。

**オプション**: $ARGUMENTS

## 使用者

**自動実行用** - session-manager-agent ガイドラインに従って Claude Code が実行

## オプション

| オプション | 説明 |
|-----------|------|
| `--auto` | 競合がない場合、自動的にマージを実行 |
| `--cleanup` | マージ完了後、worktreeを自動削除 |

## 処理フロー

### 1. 各セッションの変更分析

```bash
# 各worktreeの変更ファイルを収集
git -C ../spec-driven-framework-player diff --name-only master..HEAD
git -C ../spec-driven-framework-enemy diff --name-only master..HEAD
git -C ../spec-driven-framework-stage diff --name-only master..HEAD
```

### 2. 競合検出

```bash
# ファイルの重複をチェック
# 共有リソース（dependencies.md, architecture/）の変更を特定
```

### 3. マージ順序決定

優先度ルール:
1. 共有リソースを変更したセッション → 最初
2. 依存関係の上流 → 先
3. コミット数が多い → 先（競合解決の手間を最小化）

### 4. マージ手順生成

```bash
# 推奨されるマージ手順を出力
```

## 出力例

### 競合なしの場合

```
ユーザー: 「セッションをマージしたい」

Claude:
競合分析結果: 競合なし

推奨マージ順序:
1. auto-12345-player (4 commits)
2. auto-12346-enemy (3 commits)
3. auto-12347-stage (2 commits)

各worktreeで以下を実行してください:

# 1. auto-12345-player
cd ../spec-driven-framework-player
git push origin auto-12345-player
gh pr create --title "[Player] 実装完了" --body "REQ-30101対応"

# 2. auto-12346-enemy
cd ../spec-driven-framework-enemy
git push origin auto-12346-enemy
gh pr create --title "[Enemy] 実装完了" --body "REQ-30201対応"

# 3. auto-12347-stage
cd ../spec-driven-framework-stage
git push origin auto-12347-stage
gh pr create --title "[Stage] 実装完了" --body "REQ-30301対応"

マージ完了後、worktreeをクリーンアップ:
git worktree remove ../spec-driven-framework-player
git worktree remove ../spec-driven-framework-enemy
git worktree remove ../spec-driven-framework-stage
```

### 競合ありの場合

```
ユーザー: 「セッションをマージしたい」

Claude:
競合分析結果:
⚠️  dependencies.md を複数セッションが変更

  auto-12346-enemy: +5行（Enemy依存追加）
  auto-12347-stage: +3行（Stage依存追加）

推奨マージ順序:
1. auto-12346-enemy ← 先にマージ（dependencies.md変更あり）
2. auto-12345-player
3. auto-12347-stage ← 最新をpull後、dependencies.mdをマージ

手順:

# Step 1: enemy を先にマージ
cd ../spec-driven-framework-enemy
git push origin auto-12346-enemy
gh pr create --title "[Enemy] 実装完了" --body "dependencies.md変更あり"
# PRをマージ

# Step 2: player をマージ
cd ../spec-driven-framework-player
git push origin auto-12345-player
gh pr create --title "[Player] 実装完了" --body "REQ-30101対応"
# PRをマージ

# Step 3: stage は最新をpull後マージ
cd ../spec-driven-framework-stage
git fetch origin master
git rebase origin/master  # dependencies.mdの競合を解決
git push origin auto-12347-stage
gh pr create --title "[Stage] 実装完了" --body "REQ-30301対応"
```

### --auto オプション

```
ユーザー: /session-merge --auto

Claude:
競合分析結果: 競合なし

自動マージを実行中...

✅ auto-12345-player: PR #123 作成・マージ完了
✅ auto-12346-enemy: PR #124 作成・マージ完了
✅ auto-12347-stage: PR #125 作成・マージ完了

全セッションのマージが完了しました。
```

### --cleanup オプション

```
ユーザー: /session-merge --cleanup

Claude:
...（マージ処理）...

worktreeをクリーンアップ中...
✅ 削除: ../spec-driven-framework-player
✅ 削除: ../spec-driven-framework-enemy
✅ 削除: ../spec-driven-framework-stage

ローカルブランチを削除中...
✅ 削除: auto-12345-player
✅ 削除: auto-12346-enemy
✅ 削除: auto-12347-stage

クリーンアップ完了
```

## エラーハンドリング

| 条件 | メッセージ |
|------|-----------|
| セッションなし | `No active sessions found. Nothing to merge.` |
| 未コミット変更あり | `Uncommitted changes in {worktree}. Commit or stash first.` |
| push失敗 | `Failed to push {branch}. Check remote access.` |
| PR作成失敗 | `Failed to create PR for {branch}. Check gh auth.` |

## ID予約の解放

マージ完了後、ID予約は自動的に解放されます:

```bash
/id-reserve-release auto-12345-player
/id-reserve-release auto-12346-enemy
/id-reserve-release auto-12347-stage
```

## 注意事項

- `--auto` は競合がない場合のみ使用可能
- マージ前に各セッションのテストがパスしていることを確認
- 共有リソースの競合は手動解決が必要な場合あり

## 関連ドキュメント

- `/session-init` - セッション初期化
- `/session-status` - セッション状態確認
- `skills/parallel-sessions.md` - 並列セッション実行ガイド
- `agents/session-manager-agent.md` - セッション管理ガイドライン
