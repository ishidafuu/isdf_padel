# docs/ ドキュメント索引

仕様書駆動開発フレームワークのドキュメント一覧。

> **重要**: このディレクトリは**フレームワーク自体**のドキュメントです。
> ゲームプロジェクトの仕様書は `project/docs/` に配置されます。

---

## 目的から探す

### 今すぐ始めたい

- [クイックスタート](./tutorials/quickstart.md) - フレームワークの使い方（20分）

### 深く理解したい

- [フレームワーク概要](./concepts/overview.md) - 設計思想と基本概念
- [エージェントガイド](./concepts/agents.md) - エージェントの選択と詳細
- [タスク管理](./concepts/tasks.md) - タスク管理システムの使い方

### 逆引きしたい

- [フレームワーク仕様書](./reference/framework-spec.md) - 番号体系・層責務・依存関係
- [仕様書の書き方](./reference/spec-writing-guide.md) - spec/design/behavior/test
- [ツールリファレンス](./reference/tools-reference.md) - コマンド・Skills一覧
- [設計判断集](./reference/design-decisions.md) - 設計の理由・トレードオフ

### 特定シナリオ

- [レガシーコード解析](./guides/legacy-code-analysis.md) - 既存コードからの仕様抽出
- [レガシーコード解析（制作者向け）](./guides/legacy-code-creator.md) - 自作ゲームの再構築
- [ntfy 通知](../.claude/skills/ntfy-notification.md) - 承認待ち・実行完了のリモート通知

---

## 読み順ガイド

### 初めて使う方

1. [クイックスタート](./tutorials/quickstart.md) - 全体フロー把握（20分）
2. [エージェントガイド](./concepts/agents.md) の選択フローチャート（10分）
3. 必要に応じて concepts/ や reference/ を参照

### フレームワーク開発者

1. [フレームワーク開発背景](./framework-development/philosophy.md)（15分）
2. [フレームワーク開発ガイド](./framework-development/guide.md)（20分）
3. [コントリビューションガイド](./framework-development/contributing.md)（10分）

---

## ディレクトリ構成

```
docs/
├── index.md                      ← このファイル（目的別ナビゲーション）
├── CHANGELOG.md                  # 変更履歴
│
├── tutorials/                    # クイックスタート
│   └── quickstart.md             # フレームワーク入門（20分）
│
├── concepts/                     # 概念説明
│   ├── overview.md               # フレームワーク概要
│   ├── agents.md                 # エージェントガイド
│   └── tasks.md                  # タスク管理ガイド
│
├── reference/                    # リファレンス（逆引き）
│   ├── framework-spec.md         # フレームワーク仕様書
│   ├── spec-writing-guide.md     # 仕様書執筆ガイド
│   ├── tools-reference.md        # ツールリファレンス
│   ├── design-decisions.md       # 設計判断集
│   ├── validation-tools-spec.md  # 検証ツール仕様
│   └── ...
│
├── guides/                       # 実践ガイド
│   ├── legacy-code-analysis.md   # レガシーコード解析
│   ├── legacy-code-creator.md    # 制作者向けワークフロー
│   └── ...                       # 実践ガイド
│
├── templates/                    # テンプレート集
│   └── legacy-analysis/          # レガシーコード解析用
│
├── framework-development/        # フレームワーク開発者向け
│   ├── guide.md                  # 開発ガイド
│   ├── contributing.md           # 推敲の進め方
│   └── philosophy.md             # 開発背景
│
└── _archive/                     # アーカイブ
    └── planning/                 # 古い計画書
```

---

## トピック別インデックス

### 番号体系・ID管理

- [番号体系](./reference/framework-spec.md#番号体系) - 5桁番号の構成と計算
- [ID管理コマンド](./reference/tools-reference.md#コマンド一覧) - /id, /id-list, /id-next

### 仕様書執筆

- [仕様書執筆ガイド](./reference/spec-writing-guide.md) - spec/design/behavior/test の書き方
- [EARS記法](../.claude/skills/ears.md) - 要件の構造化記法

### エージェント・ツール

- [エージェントガイド](./concepts/agents.md) - 選択フローチャートと詳細
- [ツールリファレンス](./reference/tools-reference.md) - コマンド・Skills一覧

### アーキテクチャ

- [層の責務](./reference/framework-spec.md#各層の責務) - 1_project, 2_architecture, 3_ingame, 4_outgame, 8_data
- [依存関係](./reference/framework-spec.md#依存関係) - 層間ルール、禁止依存
- [設計判断集](./reference/design-decisions.md) - 設計の理由・トレードオフ

---

## 関連リソース

### リポジトリルート

- [../README.md](../README.md) - プロジェクト全体の概要
- [../.claude/agents/](../.claude/agents/) - エージェント定義
- [../.claude/commands/](../.claude/commands/) - コマンド定義
- [../.claude/skills/](../.claude/skills/) - スキル定義

### ゲームプロジェクト

- [../project/](../project/) - ゲーム開発エリア
- [../project/docs/](../project/docs/) - ゲーム仕様書（番号体系適用）
