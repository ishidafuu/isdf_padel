# Spec-Driven Framework Template

Claude Code を使用した仕様書駆動開発のためのフレームワークテンプレートです。

## 特徴

- **仕様書駆動開発**: 実装の前に仕様書を書く「Spec → Implementation → Test」のワークフロー
- **EARS記法**: 曖昧さのない要件定義のための標準記法
- **タスク管理**: Markdown ベースのタスク管理システム（worktree対応）
- **16種のエージェント**: 要件定義から実装まで各フェーズに特化したAIエージェント
- **バリデーションhook**: タスクディレクトリ構成、フロントマター検証の自動チェック

## クイックスタート

### 1. セットアップ

```bash
# テンプレートから新規プロジェクト作成
./setup.sh ~/my-new-project

# または引数なしで対話的に作成
./setup.sh
```

### 2. プロジェクト設定

セットアップ時に以下の情報を入力します：

- **Project name**: プロジェクト名
- **Project description**: プロジェクトの説明
- **Rust hook**: Rust プロジェクトの場合は有効化
- **Game commands**: ゲーム開発用コマンドの追加（オプション）

### 3. 初期設定

```bash
cd ~/my-new-project
git init
claude  # Claude Code を起動
```

## ディレクトリ構成

```
my-project/
├── .claude/
│   ├── agents/         # 16種のエージェント定義
│   ├── commands/       # スラッシュコマンド
│   ├── skills/         # スキルファイル
│   ├── hooks/          # バリデーションhook
│   ├── CLAUDE.md       # プロジェクト指示
│   └── settings.json   # Claude Code設定
├── docs/               # フレームワークドキュメント
├── tasks/              # フレームワークタスク (FXXX)
│   ├── 0_backlog/
│   ├── 1_todo/
│   ├── 2_in-progress/
│   ├── 3_in-review/
│   └── 4_archive/
├── project/
│   ├── docs/           # プロジェクト仕様書
│   │   ├── 1_project/      # プロジェクト概要
│   │   ├── 2_architecture/ # アーキテクチャ
│   │   ├── 3_ingame/       # コア機能仕様
│   │   ├── 4_outgame/      # 周辺機能仕様
│   │   ├── 7_tools/        # ツール仕様
│   │   ├── 8_data/         # データ定義
│   │   └── 9_reference/    # 参考資料
│   ├── src/            # ソースコード
│   ├── tests/          # テストコード
│   └── tasks/          # プロジェクトタスク (30XXX)
├── plans/              # プランファイル
└── .claudeignore       # AI探索除外設定
```

## エージェント一覧

| エージェント | 用途 |
|-------------|------|
| requirements-agent | 要件対話・深堀り |
| spec-agent | 仕様書（spec.md）作成 |
| design-agent | データ構造定義（design.md）作成 |
| behavior-agent | ロジック定義（behavior.md）作成 |
| critic-agent | 仕様書批評・検証 |
| review-agent | 仕様書間の整合性検証 |
| test-agent | テストコード設計支援 |
| impl-agent | 仕様書に基づく実装 |
| refactor-agent | リファクタリング・機能廃止 |
| deps-agent | 依存関係管理・可視化 |
| data-agent | マスタデータテーブル管理 |
| module-design-agent | モジュール構成・設計パターン |
| architecture-agent | アーキテクチャ設計 |
| task-manager-agent | タスクライフサイクル管理 |
| session-manager-agent | 並列セッション管理 |
| audit-agent | プロジェクト全体の健康診断 |

## 主要コマンド

```bash
# タスク管理
/task-next          # 次に着手可能なタスクを提案
/handover           # セッション引き継ぎ情報を生成
/resume-handover    # セッション再開

# ID管理
/id <ID>            # IDの定義箇所を表示
/id-next <prefix>   # 次の連番IDを取得
/id-list <file>     # ファイル内の全IDを一覧表示
/id-refs <ID>       # IDの参照箇所を検索

# 検証
/docs-validate      # 仕様書の整合性チェック
/impl-validate      # 実装と仕様書の対応検証
/deps-check         # 参照リンク切れと禁止依存を検出
/ears-validate      # EARS記法の正確性を検証
/code-audit         # プロジェクト全体の健康診断
```

## カスタマイズ

### CLAUDE.md の編集

`.claude/CLAUDE.md` を編集してプロジェクト固有のルールを追加：

```markdown
### プロジェクト固有ルール

- ルール1
- ルール2
```

### Rust hook の手動追加

セットアップ時にスキップした場合、後から追加できます：

```json
// .claude/settings.json の hooks.PostToolUse に追加
{
  "matcher": "Edit|Write",
  "hooks": [
    {
      "type": "command",
      "command": "python3 \"$CLAUDE_PROJECT_DIR/.claude/hooks/rust-check.py\""
    }
  ]
}
```

### ゲーム固有コマンドの追加

`optional/game-commands/` 配下のファイルを `.claude/commands/` にコピー：

```bash
cp optional/game-commands/*.md .claude/commands/
```

## ドキュメント

- [フレームワーク概要](docs/concepts/overview.md)
- [クイックスタート](docs/tutorials/quickstart.md)
- [エージェント詳細](docs/concepts/agents.md)
- [タスク管理](docs/concepts/tasks.md)
- [仕様書の書き方](docs/reference/spec-writing-guide.md)

## ライセンス

MIT License
