---
id: "P002"
title: "Bevy 0.17 エンジン実装導入"
type: "project-wide"
status: "done"
priority: "high"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/s13219/.claude/plans/inherited-churning-eagle.md"
tags: ["bevy", "rust", "implementation", "infrastructure"]
created_at: "2026-01-06T12:00:00+09:00"
updated_at: "2026-01-06T19:25:00+09:00"
completed_at: "2026-01-06T19:25:00+09:00"
---

# Task P002: Bevy 0.17 エンジン実装導入

## 説明

Rustプロジェクトを初期化し、Bevy 0.17を依存として追加。5層構造のモジュール基盤を構築する。

## 背景

P001（仕様書のBevy対応）が完了し、仕様書は全てBevy/Rust対応済み。
本タスクでは実際のRustコードを作成し、最小限のBevy動作確認を行う。

## 実装内容

### Step 1: Rustプロジェクト初期化

- [x] `cargo init --name padel_game` を `project/` で実行
- [x] `.cargo/config.toml` 作成（高速コンパイル設定）

### Step 2: Cargo.toml 設定

- [x] Bevy 0.17 依存追加
- [x] serde, ron 依存追加
- [x] 開発プロファイル最適化

### Step 3: 5層モジュール構造作成

- [x] `src/core/` - Core層（events, utils）
- [x] `src/resource/` - Resource層（config）
- [x] `src/components/` - Components層（stub）
- [x] `src/systems/` - Systems層（stub）
- [x] `src/presentation/` - Presentation層（stub）

### Step 4: GameConfig実装

- [x] `src/resource/config.rs` - 仕様書 `80101_game_constants.md` 準拠
- [x] PhysicsConfig, CourtConfig, PlayerConfig 等

### Step 5: RONファイル作成

- [x] `assets/config/game_config.ron` - 仕様書定義に従って作成

### Step 6: main.rs 最小実装

- [x] Bevy App 初期化
- [x] ウィンドウ設定
- [x] GameConfig ロード
- [x] Camera2d spawn

### Step 7: 動作確認

- [x] `cargo build` 成功
- [x] `cargo run` でウィンドウ表示

## 成果物

| ファイル | 目的 |
|---------|------|
| `project/Cargo.toml` | Rust プロジェクト設定 |
| `project/.cargo/config.toml` | 高速コンパイル設定 |
| `project/src/main.rs` | エントリーポイント |
| `project/src/core/mod.rs` | Core層 |
| `project/src/resource/mod.rs` | Resource層 |
| `project/src/components/mod.rs` | Components層（stub） |
| `project/src/systems/mod.rs` | Systems層（stub） |
| `project/src/presentation/mod.rs` | Presentation層（stub） |
| `project/assets/config/game_config.ron` | ゲーム設定データ |

## 成功基準

1. ✅ `cargo build` がエラーなく完了
2. ✅ `cargo run` でウィンドウが表示される
3. ✅ ウィンドウタイトル: "Padel Game - MVP v0.1"
4. ✅ GameConfigがRONファイルからロードできる

## 参照仕様書

- `project/docs/2_architecture/20000_overview.md` - アーキテクチャ概要
- `project/docs/2_architecture/20001_layers.md` - 5層構造定義
- `project/docs/2_architecture/20005_event_system.md` - イベント定義
- `project/docs/8_data/80101_game_constants.md` - GameConfig構造

## 依存関係

- **ブロック**: なし（P001は完了済み）
- **ブロックされる**: 30XXX（機能実装タスク群）
- **関連ドキュメント**:
  - プランファイル: `~/.claude/plans/inherited-churning-eagle.md`
