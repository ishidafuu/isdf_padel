# 並列セッション実行ガイド

複数Claude Codeセッションを同時実行するためのベストプラクティス。

## 前提

- **ユーザー**: 1人
- **Claude Codeセッション**: 複数（2-4セッション推奨）
- **目的**: 異なる機能を並行開発し、開発速度を向上
- **管理**: session-manager-agent が自動的に準備・調整

---

## ⚠️ 重要：並列実行の推奨フェーズ

### 並列実行は実装フェーズで最も効果的

#### フェーズ別推奨度

##### 🟢 強く推奨：実装フェーズ

**理由**:
- 仕様が確定済み
- ID衝突のリスクがない（事前予約）
- 作業が機械的で並列化の効果が高い

**並列パターン**:
```
Session 1: Player実装（REQ-30101-001～020 に対応するコード）
Session 2: Enemy実装（REQ-30201-001～015 に対応するコード）
Session 3: Stage実装（REQ-30301-001～010 に対応するコード）

→ 3倍の速度で実装完了
```

##### 🔴 非推奨：仕様策定・設計フェーズ

**理由**:
- ユーザーが内容を把握する必要がある
- 依存関係の調整コストが高い
- 認知負荷が高すぎる

**推奨アプローチ**:
```
午前: 仕様策定（順次実行）
  1. Player仕様 → 完了・コミット
  2. Enemy仕様 → 完了・コミット
  3. Stage仕様 → 完了・コミット

午後: 実装（並列実行）
  Terminal 1: Player実装
  Terminal 2: Enemy実装
  Terminal 3: Stage実装
```

---

## 基本原則

### 1. 事前準備型アーキテクチャ（worktree方式）

session-manager-agent が事前に全セッションを準備：

```
事前準備:
- worktree作成（独立したワーキングディレクトリ）
- フォルダロック取得
- ブランチ作成
- ID範囲予約

↓ バッティングが原理的に発生しない

各セッション:
- 独立したディレクトリで作業
- ビルド成果物の競合なし
- セッション管理を意識不要
```

### 2. worktreeによる完全な分離

各セッションは **独立したworktree** で作業:

```
メインリポジトリ:
  /Users/user/repo/spec-driven-framework/  (master)

Worktree A:
  /Users/user/repo/spec-driven-framework-player/  (auto-12345-player)
  → docs/3_ingame/301_player/

Worktree B:
  /Users/user/repo/spec-driven-framework-enemy/  (auto-12346-enemy)
  → docs/3_ingame/302_enemy/

Worktree C:
  /Users/user/repo/spec-driven-framework-stage/  (auto-12347-stage)
  → docs/3_ingame/303_stage/
```

**利点**:
- 物理的に別ディレクトリ → 競合ゼロ
- target/ シンボリックリンクでビルドキャッシュ共有 → フルビルド回避
- IDEが混乱しない → 別プロジェクトとして開ける

**禁止**: 同じフォルダに複数セッションを割り当てる

### 3. 共有リソースの調整

共有ファイル（dependencies.md等）を変更するセッションは優先的にマージ

---

## 実行フロー（worktree方式）

### Step 0: 既存worktreeのクリーンアップ（自動確認）

```
Terminal 1で指示:
「Player、Enemy、Stageを並列実装したい」

↓ session-manager-agent が最初に既存worktreeを確認:
⚠️  既存のworktreeが見つかりました:
/Users/user/repo/spec-driven-framework-old-player

これらのworktreeをクリーンアップしますか？ (y/n)
> y

削除中: /Users/user/repo/spec-driven-framework-old-player
✅ クリーンアップ完了
```

### Step 1: 並列セッション開始

```
↓ session-manager-agent が自動実行:
- 機能フォルダの特定（301_player, 302_enemy, 303_stage）
- worktree作成（独立したディレクトリ）
- ブランチ作成（auto-{PID}-{feature}）
- **target/ シンボリックリンク作成（ビルドキャッシュ共有）**
- フォルダロック取得
- ID範囲予約（各 001-050）

↓ 結果:
準備されたworktree:
- ../spec-driven-framework-player (Branch: auto-12345-player)
- ../spec-driven-framework-enemy (Branch: auto-12346-enemy)
- ../spec-driven-framework-stage (Branch: auto-12347-stage)

次のコマンドを各Terminalで実行してください:

# Terminal 2:
cd ../spec-driven-framework-enemy && claude

# Terminal 3:
cd ../spec-driven-framework-stage && claude
```

### Step 2: 各ターミナルで作業

```bash
# Terminal 1（worktree: ../spec-driven-framework-player）
> 「30101_spec.md に基づいてPlayerを実装して」
impl-agent: （通常通り実装、独立したディレクトリで作業）

# Terminal 2（worktree: ../spec-driven-framework-enemy）
> 「30201_spec.md に基づいてEnemyを実装して」
impl-agent: （通常通り実装、独立したディレクトリで作業）

# Terminal 3（worktree: ../spec-driven-framework-stage）
> 「30301_spec.md に基づいてStageを実装して」
impl-agent: （通常通り実装、独立したディレクトリで作業）
```

**重要**:
- 各worktreeは完全に独立したディレクトリ
- ビルド成果物の競合なし
- 各エージェントは通常通り作業するだけ

### Step 3: セッション状態確認（任意）

```
いずれかのターミナルで:
「セッション状態を確認して」

↓ session-manager-agent が表示:
アクティブセッション: 3
- auto-12345 (4 commits) - Player
- auto-12346 (3 commits) - Enemy
- auto-12347 (2 commits) - Stage

競合: なし
```

### Step 4: マージ調整（worktree方式）

```
いずれかのターミナルで:
「セッションをマージしたい」

↓ session-manager-agent が分析:
- 各worktreeの変更ファイル確認
- 共有リソース競合の検出
- 推奨マージ順序の提案

推奨マージ順序:
1. auto-12346-enemy (dependencies.md 変更あり - 先にマージ)
2. auto-12345-player
3. auto-12347-stage
```

#### 方法A: ローカルスカッシュマージ（推奨）

mainリポジトリで各ブランチをスカッシュマージ:

```bash
# 1. mainリポジトリに移動、mainを最新化
cd /path/to/main/repository
git checkout main
git pull origin main

# 2. 各ブランチを推奨順序でスカッシュマージ
# --- Enemy ---
git merge --squash auto-12346-enemy
git commit -m "feat(30201): Enemy実装

REQ-30201対応

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"

# --- Player ---
git merge --squash auto-12345-player
git commit -m "feat(30101): Player実装

REQ-30101対応

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"

# --- Stage ---
git merge --squash auto-12347-stage
git commit -m "feat(30301): Stage実装

REQ-30301対応

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"

# 3. mainをプッシュ
git push origin main
```

#### 方法B: GitHub PR経由（チームレビューが必要な場合）

```bash
# 各worktreeで:
git push origin <branch>
gh pr create --title "..." --body "..."
```

PR作成後、GitHub上で **Squash and merge** を選択してマージ。

#### クリーンアップ（共通）

マージ完了後、worktreeとブランチをクリーンアップ:

```bash
# worktree削除
git worktree remove ../spec-driven-framework-player
git worktree remove ../spec-driven-framework-enemy
git worktree remove ../spec-driven-framework-stage

# ブランチ削除（-D: スカッシュマージ後は強制削除が必要）
git branch -D auto-12345-player
git branch -D auto-12346-enemy
git branch -D auto-12347-stage
```

---

## シナリオ別ガイド

### シナリオ1: 完全独立な機能（推奨パターン）

**状況**: Player と Enemy で相互依存なし

```
Session A: 301_player
Session B: 302_enemy

共有リソース変更: なし
競合リスク: ゼロ
```

**フロー**:
1. 両セッション同時開始
2. 並行作業
3. 完了した順にマージ（順序自由）

### シナリオ2: 共有リソース変更あり

**状況**: Player が dependencies.md を変更

```
Session A: 301_player (dependencies.md 変更)
Session B: 302_enemy
Session C: 303_stage
```

**フロー**:
1. 全セッション同時開始・並行作業
2. Session A を最初にマージ
3. Session B, C が最新を pull
4. Session B, C を順次マージ

---

## ベストプラクティス

### ✅ 推奨

1. **セッション数は 2-4 に抑える**
   - 多すぎると管理コストが増加

2. **1日の終わりに全セッションをマージ**
   - 翌日に持ち越さない

3. **完全に独立した機能を選ぶ**
   - 301_player, 302_enemy, 303_stage のような明確な分離

4. **仕様書は事前に完成させる**
   - 午前に順次作成 → 午後に並列実装

### ❌ 避けるべき

1. **同じフォルダに複数セッションを割り当て**
   - 確実にファイル競合

2. **仕様策定フェーズでの並列実行**
   - 認知負荷が高すぎる

3. **相互依存する機能の並列実装**
   - Player と Enemy が互いに参照する場合は順次実装

---

## パフォーマンス最適化

### 開発速度の最大化

```
理想的な並列パターン:

午前（順次実行）:
  1人で順番に仕様策定
    1. Player 仕様策定 → コミット
    2. Enemy 仕様策定 → コミット
    3. Stage 仕様策定 → コミット

午後（並列実行）:
  - Session 1: Player 実装
  - Session 2: Enemy 実装
  - Session 3: Stage 実装

夕方（並列実行）:
  - Session 1: Player PR作成・マージ
  - Session 2: Enemy PR作成・マージ
  - Session 3: Stage PR作成・マージ
```

**重要**: 仕様策定フェーズは1セッションで順次実行。実装フェーズから並列化。

### 効果

| 項目 | 1セッション | 3セッション並列 |
|------|-----------|---------------|
| 1機能の実装時間 | 2h | 2h |
| 1日で完了する機能数 | 2-3機能 | 6-8機能 |
| 待ち時間 | なし | ほぼなし（マージ調整時のみ） |

---

## トラブルシューティング

### Q1: 共有リソースを複数セッションが変更してしまった

**対応**:
```
「セッションをマージしたい」で確認

session-manager-agent が検出:
⚠️  dependencies.md を Session A, B が両方変更

推奨マージ順序:
1. Session A を先にマージ
2. Session B は最新を pull してから再マージ
```

### Q2: ID範囲が足りなくなった

**状況**: 予約した 50個 の REQ-ID を使い切った

**対応**:
```
spec.md 冒頭のコメントを手動編集:
<!-- ID Range: REQ-30101-001～050 → 001～100 に拡張 -->
```

### Q3: 古いセッションが残っている

**対応**:
```bash
# 1. worktreeを削除
git worktree remove <worktree-path> --force

# 2. ブランチを削除（-D: 強制削除）
git branch -D <古いブランチ名>

# 3. .session-locks.yml を編集して該当エントリを削除
Edit(.session-locks.yml から該当セッションを削除)
```

---

## まとめ

### 成功の鍵

1. **計画**: 午前に仕様策定、午後に並列実装
2. **分離**: 独立した機能フォルダを選ぶ
3. **準備**: session-manager-agent に任せる
4. **シンプル**: 各エージェントは通常通り作業

### 並列セッションの効果

- **3倍の開発速度**: 3つの機能を同時に実装
- **バッティングなし**: 事前準備で競合を回避
- **シンプルな操作**: ユーザーは機能名を指示するだけ

---

## 関連ドキュメント

- `agents/session-manager-agent.md` - エージェント詳細
- `.claude/CLAUDE.md` - プロジェクト全体設定
