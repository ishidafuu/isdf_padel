# Changelog

このファイルは、フレームワーク推敲における変更履歴を記録します。

フォーマットは [Keep a Changelog](https://keepachangelog.com/ja/1.0.0/) に基づいています。

---

## [Unreleased]

### Added
- タスク管理の人間向けドキュメント追加
  - `docs/getting-started/task-management-guide.md` - 詳細ガイド（30分、約900行）
    - Overview and Motivation（なぜMarkdownベースなのか）
    - Task Types（3つのタスクタイプ、決定木Mermaid図）
    - Task Lifecycle（5状態、ライフサイクルMermaid状態図）
    - Common Operations（4シナリオ、並列ワークフローMermaidシーケンス図）
    - Worktree Integration（自動管理の仕組み）
    - Troubleshooting（よくあるエラーと対処法）
  - `docs/reference/task-management-faq.md` - FAQ集（約500行、38個のQ&A）
    - General Questions（10個）
    - Workflow Questions（10個）
    - Task Groups（6個）
    - Troubleshooting（6個）
    - Best Practices（6個）
  - `docs/templates/task-examples/` - テンプレート4種（約870行）
    - `example-game-dev-task.md` - ゲーム開発タスクの例（約220行）
    - `example-project-wide-task.md` - プロジェクト横断タスクの例（約180行）
    - `example-framework-task.md` - フレームワークタスクの例（約170行）
    - `example-task-group.md` - 親子タスクグループの例（約300行）

### Changed
- `docs/getting-started/users-guide.md` - Phase 3タスク管理セクション拡張（約90行追加）
  - タスク管理の概要（Markdownベースの理由、3つのタスクタイプ、worktree統合）
  - クイックスタート: 単一機能実装
  - クイックスタート: 並列開発
  - タスク状況確認コマンド
- `docs/getting-started/agent-selection-guide.md` - タスク管理決定木追加（約110行追加）
  - task-manager-agentの使い時
  - タスク管理の決定木（Mermaid図）
  - タスクタイプ決定表
  - 基本操作コマンド例
  - 開発ワークフローとの統合
- `docs/index.md` - クイックリファレンス更新
  - タスク管理ガイドへのリンク追加
- `docs/reference/tools-reference.md` - タスク管理例追加（約90行追加）
  - `/task-status` コマンドの出力例（基本、タイプ別、状態別、優先度別）

### 推敲中
- エージェント定義の精緻化
- ワークフローの検証
- ドキュメントの整備

---

## [v2.16-draft] - 2024-12-16

### Added
- 初期ディレクトリ構成を整備
  - `agents/` - 14種のエージェント定義
  - `docs/` - ユーザーズガイド、エージェント一覧
  - `templates/` - 展開用テンプレート（準備中）
- README.md を作成（推敲中ステータスを明示）
- CHANGELOG.md を作成
- CONTRIBUTING.md を作成（推敲の進め方）

### エージェント（v2.16時点）
- 💬 requirements-agent - 対話で要件を深掘り
- 📋 spec-agent - EARS記法で文書化
- 🔍 critic-agent - 仕様の批評・検証
- 🏗️ design-agent - データ構造設計
- ⚙️ behavior-agent - ロジック設計
- 🧪 test-agent - BDDテストシナリオ
- 🏛️ architecture-agent - アーキテクチャ設計
- 💻 impl-agent - コード実装
- ✅ review-agent - 整合性検証
- 🔧 setup-agent - プロジェクト初期化
- 🔗 deps-agent - 依存関係管理
- ♻️ refactor-agent - リファクタリング
- 📊 data-agent - マスタデータ管理

---

## 今後の予定

### v2.16-rc1（レビュー完了後）
- [ ] 全エージェントのレビュー完了
- [ ] 仕様書体系の確定
- [ ] ワークフローの確定

### v2.16（正式版）
- [ ] 実プロジェクトでの検証完了
- [ ] templates/ の整備完了
- [ ] 導入手順の確定
