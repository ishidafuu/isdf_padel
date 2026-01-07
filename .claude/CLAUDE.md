# CLAUDE.md

## プロジェクト概要

**仕様書駆動開発フレームワーク** - Claude Code による2Dアクションゲーム開発向けフレームワーク

## ディレクトリ構成（IMPORTANT）

このリポジトリは **フレームワーク** と **ゲームプロジェクト** の2層構造です。

```
spec-driven-framework/          (リポジトリルート)
├── .claude/                    # フレームワーク定義（共有）
│   ├── agents/                 # エージェント定義（19種）
│   ├── commands/               # スラッシュコマンド
│   └── skills/                 # スキルファイル
├── docs/                       # フレームワークドキュメント
│   ├── tutorials/              # クイックスタート
│   ├── concepts/               # 概念説明（エージェント、タスク等）
│   ├── reference/              # 仕様・ツールリファレンス
│   ├── guides/                 # 実践ガイド（レガシーコード解析等）
│   └── index.md                # ★ ドキュメント索引（困った時はここ）
├── tasks/                      # ★ フレームワーク開発タスク（FXXX）
│   ├── .taskrc.yaml            # フレームワークタスク設定
│   ├── 1_todo/                 # 未着手タスク
│   ├── 2_in-progress/          # 実装中タスク
│   ├── 3_in-review/            # レビュー中タスク
│   └── 4_archive/              # 完了・キャンセル済みタスク
│
└── project/                    # ★★ ゲーム開発エリア
    ├── docs/                   # ゲーム仕様書（番号体系適用）
    │   ├── 1_project/          # プロジェクト定義
    │   ├── 2_architecture/     # アーキテクチャ定義
    │   ├── 3_ingame/           # インゲーム機能
    │   ├── 4_outgame/          # アウトゲーム機能
    │   ├── 8_data/             # データ定義
    │   └── 9_reference/        # 参照資料（レガシーコード解析等）
    ├── src/                    # ゲームコード
    ├── tests/                  # テストコード
    └── tasks/                  # ★ ゲーム開発・プロジェクト横断タスク（30XXX/PXXX）
        ├── .taskrc.yaml        # プロジェクトタスク設定
        ├── 1_todo/             # 未着手タスク
        ├── 2_in-progress/      # 実装中タスク
        ├── 3_in-review/        # レビュー中タスク
        └── 4_archive/          # 完了・キャンセル済みタスク
```

**作業対象の区別：**
- **ゲーム開発** → `project/` 配下を操作
- **フレームワーク推敲** → `docs/`, `.claude/agents/`, `.claude/commands/`, `.claude/skills/` を操作

## 必ず守るルール（CRITICAL）

### 0. タスク優先原則（CRITICAL）

**すべての変更は必ずタスクを先に作成してから行う。**

```
作業依頼 → タスク作成 → 仕様書作成/変更 → 実装 → コミット
```

**適用範囲:**
- ✅ 新機能開発
- ✅ バグ修正
- ✅ 仕様変更
- ✅ リファクタリング
- ✅ ドキュメント更新（仕様書）

**理由:**
- タスク駆動により作業の追跡可能性を確保
- worktree/ブランチの自動管理による競合回避
- 作業の開始・終了が明確になる
- 依存関係の事前管理が可能

#### タスク作成フロー

**フロー（2段階）:**

```
【ステップ1: プラン作成】
ユーザーがプランモードでプラン作成
  ↓
プランファイル保存（~/.claude/plans/xxx.md）

【ステップ2: タスク登録】
ユーザー: 「プランからタスクを作成して」
  ↓
メイン Claude Code がガイドライン（.claude/agents/task-registration-agent.md）を参照
  ↓
メイン Claude Code が直接実行:
  1. プランファイル検出（Bash）
  2. プランファイル読み込み（Read）
  3. タスクタイプ判定
  4. ID採番（Skill: id-next）
  5. タスクファイル作成（Write）
  ↓
タスクファイル作成完了（tasks/1_todo/ または project/tasks/1_todo/）
プランファイル保持（削除しない）
```

**メイン Claude Code の責務:**
- ✅ `.claude/agents/task-registration-agent.md` を参照してフローを確認
- ✅ ガイドラインに従ってツールを直接実行（Read, Write, Bash, Skill）
- ✅ タスクファイルを直接作成する
- ❌ プランファイルを削除しない（保持する）

**エージェント定義の位置づけ:**
```
エージェント定義（.claude/agents/*.md）= 処理ガイドライン

- 専門知識・処理フローを定義したドキュメント
- メイン Claude Code が参照しながら直接実行する
- Task tool で起動する「実行者」ではない
```

**注意**: Task tool のサブエージェントはツール実行ができないため、ファイル操作を伴うタスクはメインが直接実行します。

詳細: [クイックスタート](docs/tutorials/quickstart.md)、[タスク管理ガイド](docs/concepts/tasks.md)

### 1. 仕様書駆動原則

```
仕様書（spec.md） → 実装（src/） → テスト（tests/）
```

- **全ての実装は仕様書に基づく** - 仕様書にない機能は絶対に実装しない
- **仕様書を先に更新** - 実装前に必ず仕様書を更新する
- **EARS記法を使用** - 要件記述は `.claude/skills/ears.md` を参照
- **対応コメント必須** - 実装時は `@spec`/`@data` コメントを必ず付与

### 2. test.md は作成しない（CRITICAL）

**理由**: 仕様書が唯一の真実の源（Single Source of Truth）

- ✅ **仕様書（spec.md）**: 要件を記述（EARS記法）
- ✅ **実装テスト（tests/）**: 実装が仕様を満たすか検証、テスト名に REQ-ID を含める
- ❌ **test.md**: 作成しない（冗長・メンテナンスコスト増）

### 3. ハードコーディング禁止原則（CRITICAL）

**NEVER hardcode parameter values. ALL adjustable values MUST be externalized to data files.**

```rust
// ❌ 絶対に禁止
velocity.y += -9.8 * time.delta_secs();  // 重力値をハードコーディング

// ✅ 必須: 外部データ化
velocity.y += config.physics.gravity * time.delta_secs();
```

**対象**: 物理パラメータ、移動パラメータ、サイズ、時間、ゲームバランス値など
**配置**: `project/docs/8_data/` に定義 → RON ファイル（`.ron`）

### 4. 1タスク=1コミット原則

**1つのタスクは1つのコミットにまとめる**

- **game-dev（30XXX）**: worktree でスカッシュマージ → 1コミット
- **framework/project-wide（FXXX/PXXX）**: 実装 → git add（ステージングのみ）→ タスクDONE処理 → git add → まとめて1コミット

**フロー（framework/project-wide）:**
```
実装作業 → git add（ステージングのみ、コミットしない）
   ↓
タスクDONE処理（status更新、archive移動）
   ↓
git add（タスクファイルも追加）
   ↓
まとめて1コミット
```

### 5. フェーズ管理（MVP/バージョン管理）

**仕様書には全バージョンの要件を記載し、実装スコープは別途管理する**

- 仕様書は「Core Requirements (MVP v0.1)」「Extended Requirements (v0.2)」でセクション分離
- 実装時は `30009_mvp_scope.md` で対象 REQ-ID を確認
- **impl-agent は MVP 範囲外の要件を実装しない**

詳細: [docs/reference/framework-spec.md#フェーズ管理](docs/reference/framework-spec.md#フェーズ管理mvpバージョン管理)

## クイックリファレンス

### 困った時の参照先

| 目的 | ドキュメント |
|------|-------------|
| **今すぐ開発を始めたい** | [クイックスタート](docs/tutorials/quickstart.md)（20分） |
| **どのエージェントを使うか迷っている** | [エージェントガイド](docs/concepts/agents.md)（10分） |
| **完全な仕様を見たい** | [フレームワーク仕様書](docs/reference/framework-spec.md)（60分） |
| **仕様書の書き方を知りたい** | [仕様書執筆ガイド](docs/reference/spec-writing-guide.md)（40分） |
| **コマンド・Skills一覧** | [ツールリファレンス](docs/reference/tools-reference.md)（20分） |
| **番号体系の計算方法** | [番号体系](docs/reference/framework-spec.md#番号体系) |
| **設計判断の理由を知りたい** | [設計判断集](docs/reference/design-decisions.md)（30分） |
| **レガシーコードを解析したい** | [レガシーコード解析ガイド](docs/guides/legacy-code-analysis.md)（30分） |

**迷ったら**: [docs/index.md](docs/index.md) を参照（ドキュメント索引）

### 処理ガイドライン（旧エージェント）

`.claude/agents/` にある各ファイルは **処理ガイドライン** です。
メイン Claude Code がこれらを参照しながら直接ツールを実行します。

詳細は [エージェントガイド](docs/concepts/agents.md) を参照。

| フェーズ | ガイドライン |
|---------|-------------|
| 初期化 | 🔧 setup-agent |
| 要件策定 | 💬 requirements-agent → 📋 spec-agent → 🔍 critic-agent |
| モジュール設計 | 🧩 module-design-agent |
| 詳細設計 | 🏗️ design-agent → ⚙️ behavior-agent → 🧪 test-agent |
| タスク管理 | 📝 task-registration-agent, 🗂️ task-manager-agent |
| 実装 | 💻 impl-agent → ✅ review-agent |
| 横断 | 🏛️ architecture-agent, 🔗 deps-agent, 📊 data-agent, ♻️ refactor-agent |
| 並列セッション管理 | 🎯 session-manager-agent |
| 参照資料 | 🔬 legacy-analyzer-agent, 🎮 game-reference-agent |

**注意**: これらは Task tool で起動する「実行者」ではなく、メインが参照する「ガイドライン」です。

### コマンド設計思想

**原則**: 人間はコマンドを直接使わない。メインがガイドラインに従って自動的に使う。

```
人間: 「プレイヤーの仕様を作って」← 意図を伝えるだけ
  ↓
メイン Claude Code: ガイドラインを参照してツールを実行
  • /id-next で自動採番
  • /docs-validate で自動検証
  • /deps-check で自動チェック
  ↓
結果: 整合性が保たれた仕様書
```

詳細: [コマンドREADME](.claude/commands/README.md)、[ツールリファレンス](docs/reference/tools-reference.md)

### 人間専用コマンド（2個）

| コマンド | 説明 | 使用タイミング |
|---------|------|--------------|
| `/handover [--task <id>]` | タスクファイルに引き継ぎ情報を記録 | セッション終了前 |
| `/resume-handover [--task <id>]` | タスクファイルからセッションを再開 | セッション開始時 |

## 並列セッション実行（IMPORTANT）

**このプロジェクトは複数 Claude Code セッションの同時実行を想定しています。**
**session-manager-agent が事前準備とマージ調整を行います。**

### 並列実行ポリシー（CRITICAL）

**並列実行は実装フェーズでのみ推奨。仕様策定・設計フェーズは順次実行を推奨。**

| フェーズ | 並列実行 | 理由 |
|---------|---------|------|
| 要件定義・仕様策定・設計 | ❌ **非推奨** | ユーザー把握、依存関係調整、一貫性が必要 |
| **実装** | ✅ **推奨** | 仕様確定後、機械的な変換作業 |
| レビュー | ✅ 推奨 | 検証作業、影響範囲が限定的 |

**推奨ワークフロー**:
```
午前: 仕様策定（順次実行）
  1. Player仕様策定 → 完了・コミット
  2. Enemy仕様策定 → 完了・コミット

午後: 実装（並列実行）
  Terminal 1: Player実装
  Terminal 2: Enemy実装
```

### 基本フロー（worktree方式）

```
Terminal 1で指示:
「Player、Enemy、Stageを並列実装したい」

↓ session-manager-agent が自動実行:
- worktree作成（独立したワーキングディレクトリ）
- ブランチ作成（auto-{PID}-{feature}）
- フォルダロック・ID範囲の予約

↓ 各Terminalで作業:
Terminal 1: Player実装（worktree: ../spec-driven-framework-player）
Terminal 2: Enemy実装（worktree: ../spec-driven-framework-enemy）

↓ マージ調整:
「セッションをマージしたい」
session-manager-agent が競合検出・推奨順序を提案
```

詳細: [.claude/skills/parallel-sessions.md](.claude/skills/parallel-sessions.md)

## タスク管理（Markdownファイルベース）

- タスクは **Markdownファイル** で管理する
  - フレームワーク開発タスク（FXXX）: `/tasks/`
  - ゲーム開発・プロジェクト横断タスク（30XXX/PXXX）: `project/tasks/`
- 詳細は [.claude/skills/task-workflow.md](.claude/skills/task-workflow.md) を参照

### タスク種別

Markdownタスクシステムは3種類のタスクを一元管理：

| タスク種別 | ID形式 | タイトル形式 | 配置場所 | 対象範囲 | worktree対応 |
|-----------|-------|-------------|---------|---------|------------|
| **ゲーム開発** | `30XXX` | `30101-ジャンプ機能実装.md` | `project/tasks/` | `project/` 配下の仕様書・実装 | ✅ 有効 |
| **プロジェクト横断** | `PXXX` | `P001-CI-CD構築.md` | `project/tasks/` | リポジトリ全体（CI/CD、インフラ） | ❌ 無効 |
| **フレームワーク開発** | `FXXX` | `F001-エージェント更新.md` | `tasks/` | `.claude/agents/`, `docs/`, `.claude/commands/`, `.claude/skills/` | ❌ 無効 |

**worktree対応**:
- ゲーム開発タスクのみworktreeによる並列実行が可能
- プロジェクト横断・フレームワーク開発タスクは、リポジトリ全体に影響するためworktree非対応

**タスク配置ルール（CRITICAL）**:

タスクタイプは3種類だが、配置場所は2箇所のみ：

1. **フレームワーク開発タスク（FXXX）**: `/tasks/` に配置
   - 対象: `.claude/agents/`, `docs/`, `.claude/commands/`, `.claude/skills/`
   - worktree: 無効（フレームワーク全体に影響）

2. **ゲーム開発・プロジェクト横断タスク（30XXX/PXXX）**: `project/tasks/` に配置
   - ゲーム開発（30XXX）: `project/` 配下の仕様書・実装、worktree有効
   - プロジェクト横断（PXXX）: リポジトリ全体（CI/CD、インフラ）、worktree無効

**重要**: ゲーム開発とプロジェクト横断は同じ `project/tasks/` に共存。

### タスクファイル構造

```
## フレームワーク開発タスク（FXXX）
tasks/
├── .taskrc.yaml               # フレームワークタスク設定
├── 1_todo/                    # 未着手タスク
│   └── F001-ドキュメント整合性確認.md
├── 2_in-progress/             # 実装中タスク
├── 3_in-review/               # レビュー中タスク
└── 4_archive/                 # 完了・キャンセル済みタスク

## ゲーム開発・プロジェクト横断タスク（30XXX/PXXX）
project/tasks/
├── .taskrc.yaml               # タスク管理設定
├── 1_todo/                    # 未着手タスク
│   └── 30101-ジャンプ機能実装.md
├── 2_in-progress/             # 実装中タスク
│   └── P001-CI-CD構築.md
├── 3_in-review/               # レビュー中タスク
│   └── 30103-攻撃機能実装.md
└── 4_archive/                 # 完了・キャンセル済みタスク
    ├── 30100-テストタスク.md (done)
    └── 30099-廃止機能.md (cancelled)
```

### ガイドラインに基づく処理

**タスク登録（task-registration-agent.md を参照）:**
- プランファイル → タスクファイル変換
- ID採番（Skill: id-next）
- 配置先決定（tasks/ or project/tasks/）
- 初期状態設定（status: "todo"）

**タスク管理（task-manager-agent.md を参照）:**
- タスク状態遷移（todo → in-progress → in-review → done）
- worktree作成・管理（game-devタスクのみ）
- タスク依存関係管理（blocked_by, blocks）
- タスク検索・フィルタリング

**人間の操作:**
- タスク登録・開始・完了の指示（メインがガイドラインに従って実行）

## 除外設定（.claudeignore）

プロジェクトルートに `.claudeignore` を作成し、以下を記載:

```
docs/_deprecated/
```

**理由**: 廃止された仕様書をAIの自律探索対象から除外し、古いID参照を防ぐ

## スキル一覧

| スキル | 用途 | 利用エージェント |
|-------|------|-----------------|
| `.claude/skills/ears.md` | EARS記法による要件記述 | spec-agent, requirements-agent, critic-agent |
| `.claude/skills/task-planning.md` | プランモード → タスク登録フロー | task-manager-agent |
| `.claude/skills/task-workflow.md` | タスク管理スキル索引 | task-manager-agent, impl-agent, review-agent |
| `.claude/skills/task-lifecycle.md` | タスク状態遷移、親子タスク【将来実装】 | task-manager-agent |
| `.claude/skills/task-file-format.md` | タスクファイル形式、Frontmatter | task-registration-agent |
| `.claude/skills/task-operations.md` | タスク操作、検索、worktree管理 | task-manager-agent, impl-agent |
| `.claude/skills/task-status.md` | 状況表示形式、アイコン定義 | 全エージェント |
| `.claude/skills/impl-comments.md` | 実装コメント規約 | impl-agent |
| `.claude/skills/extraction-schema.md` | 仕様抽出チェックリスト | game-reference-agent, legacy-analyzer-agent |
| `.claude/skills/design-patterns.md` | 設計パターンガイド | module-design-agent, design-agent, behavior-agent |
| `.claude/skills/parallel-sessions.md` | 並列セッション実行ガイド | 全エージェント |
| `.claude/skills/deep-investigation.md` | 技術質問への詳細回答 | 全エージェント（調査時） |
| `.claude/skills/ntfy-notification.md` | ntfy経由リモート通知 | 全エージェント（フック実行） |

**人間向け詳細**: `docs/concepts/tasks.md`（出力例、参考実装、FAQ）

## 思考バジェット（複雑なタスク用）

複雑な問題には以下のキーワードで思考時間を調整：
- `think` - 基本的な計画
- `think hard` - より深い分析
- `ultrathink` - 最大限の思考時間
