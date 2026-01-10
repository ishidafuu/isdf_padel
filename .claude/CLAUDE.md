# CLAUDE.md

## プロジェクト概要

**仕様書駆動開発フレームワーク** - Claude Code による2Dアクションゲーム開発向けフレームワーク

## ディレクトリ構成（IMPORTANT）

このリポジトリは **フレームワーク** と **ゲームプロジェクト** の2層構造です。

```
spec-driven-framework/          (リポジトリルート)
├── .claude/                    # フレームワーク定義（共有）
│   ├── agents/                 # エージェント定義（20種）
│   ├── commands/               # スラッシュコマンド
│   └── skills/                 # スキルファイル
├── docs/                       # フレームワークドキュメント（詳細は docs/index.md）
├── tasks/                      # フレームワーク開発タスク（FXXX）
└── project/                    # ゲーム開発エリア
    ├── docs/                   # ゲーム仕様書（番号体系適用）
    ├── src/                    # ゲームコード
    ├── tests/                  # テストコード
    └── tasks/                  # ゲーム開発タスク（30XXX/PXXX）
```

**作業対象**: ゲーム開発 → `project/` 配下 / フレームワーク → `docs/`, `.claude/`

## 必ず守るルール（CRITICAL）

### 0. タスクタイプ別ワークフロー（CRITICAL）

| タスクタイプ | ワークフロー | 理由 |
|-------------|-------------|------|
| **ゲーム開発（30XXX）** | 仕様書作成 → タスク作成 → 実装 | 仕様書駆動が原則 |
| **バグ修正（B30XXX）** | プランモード → タスク作成 → 実装 | 既存コードの修正 |
| **リファクタ（R30XXX）** | プランモード → タスク作成 → 実装 | 既存コードの改善 |
| **フレームワーク（FXXX）** | プランモード → タスク作成 → 実装 | 設計変更 |
| **プロジェクト横断（PXXX）** | プランモード → タスク作成 → 実装 | インフラ等 |

**共通原則**: タスク駆動で作業追跡、worktree/ブランチ自動管理

**エージェント定義**: `.claude/agents/*.md` = 処理ガイドライン（メインが参照して直接実行）

### 1. 仕様書駆動原則

```
仕様書（spec.md） → 実装（src/） → テスト（tests/）
```

- **全ての実装は仕様書に基づく** - 仕様書にない機能は絶対に実装しない
- **仕様書を先に更新** - 実装前に必ず仕様書を更新する
- **EARS記法を使用** - 要件記述は `.claude/skills/ears.md` を参照
- **対応コメント必須** - 実装時は `@spec`/`@data` コメントを必ず付与

### 2. test.md は作成しない（CRITICAL）

仕様書が唯一の真実の源（Single Source of Truth）。test.md は冗長なため作成禁止。

### 3. ハードコーディング禁止原則（CRITICAL）

**NEVER hardcode parameter values. ALL adjustable values MUST be externalized to data files.**

```rust
// ❌ 絶対に禁止
velocity.y += -9.8 * time.delta_secs();  // 重力値をハードコーディング

// ✅ 必須: 外部データ化
velocity.y += config.physics.gravity * time.delta_secs();
```

**対象**: 物理/移動パラメータ、サイズ、時間、ゲームバランス値など → `project/docs/8_data/` → `.ron`

### 4. ECS設計原則（CRITICAL）

**ゲームオブジェクトの状態は必ずエンティティ/コンポーネントで管理。固定識別子は絶対禁止。**

```rust
// ❌ 禁止: 固定識別子をフィールド名に埋め込む
pub struct ShotButtonState { pub player1_holding: bool, pub player2_holding: bool }

// ❌ 禁止: 固定識別子で分岐
match player.id { 1 => ..., 2 => ... }

// ❌ 禁止: ゲームオブジェクトの状態をリソースで管理
#[derive(Resource)] pub struct PlayerStates { ... }
```

**正解**: 状態は Component、振る舞いの違いは Marker Component、グローバル1つのみ Resource

### 5. 1タスク=1コミット原則

1つのタスクは1つのコミットにまとめる（worktree: スカッシュマージ / FXXX: ステージング → DONE → 1コミット）

### 6. フェーズ管理（MVP/バージョン管理）

仕様書は「Core Requirements (MVP)」「Extended Requirements」でセクション分離。実装時は `30009_mvp_scope.md` で対象 REQ-ID を確認。

## 並列セッション実行（IMPORTANT）

**並列実行は実装フェーズでのみ推奨。仕様策定・設計は順次実行。**

| フェーズ | 並列実行 | 理由 |
|---------|---------|------|
| 要件定義・仕様策定・設計 | ❌ 非推奨 | ユーザー把握、依存関係調整 |
| **実装** | ✅ 推奨 | 仕様確定後の機械的変換 |
| レビュー | ✅ 推奨 | 検証作業 |

詳細: `.claude/skills/parallel-sessions.md`

## タスク管理（Markdownファイルベース）

| タスク種別 | ID形式 | 配置場所 | worktree |
|-----------|-------|---------|----------|
| ゲーム開発 | `30XXX` | `project/tasks/` | ✅ 有効 |
| バグ修正 | `B30XXX-NNN` | `project/tasks/` | ✅ 有効 |
| リファクタ | `R30XXX-NNN` | `project/tasks/` | ✅ 有効 |
| プロジェクト横断 | `PXXX` | `project/tasks/` | ❌ 無効 |
| フレームワーク開発 | `FXXX` | `tasks/` | ❌ 無効 |

詳細: `.claude/skills/task-workflow.md`

### タスク登録ルール（CRITICAL）

**タスク登録時は必ず `.claude/skills/task-registration.md` を参照。**

- **配置先は `1_todo/` のみ** - 他ディレクトリへの配置禁止
- **プラン作成後は必ずタスク登録** - プランから直接実装は禁止
- **順序厳守**: プラン作成 → タスク登録 → タスク開始 → 実装

## 人間専用コマンド

- `/handover [--task <id>]` - セッション終了前に引き継ぎ情報を記録
- `/resume-handover [--task <id>]` - セッション開始時に再開

## 除外設定

`.claudeignore` に `docs/_deprecated/` を記載し、廃止仕様書をAI探索から除外。

## 思考バジェット

複雑な問題には `think` / `think hard` / `ultrathink` で思考時間を調整。

## クイックリファレンス

迷ったら [docs/index.md](docs/index.md) を参照（ドキュメント索引）
