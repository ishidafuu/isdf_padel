---
id: "P001"
title: "CI/CD構築（GitHub Actions）"
type: "project-wide"
status: "todo"
priority: "medium"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["ci", "infrastructure", "github-actions"]
created_at: "2025-12-29T10:00:00.000000"
updated_at: "2025-12-29T10:00:00.000000"
completed_at: null
parent_task_id: null
---

# CI/CD構築（GitHub Actions）

## 概要

GitHub Actionsを使用して、自動テスト・ビルド・デプロイのCI/CDパイプラインを構築する。

**注意:** このタスクは `project-wide` タイプであり、リポジトリ全体に影響するため、**worktreeは作成されません**。メインワーキングディレクトリで順次実行します。

## 対象範囲

### 影響を受けるファイル

- `.github/workflows/ci.yml` - 自動テスト・ビルドワークフロー
- `.github/workflows/cd.yml` - デプロイワークフロー（オプション）
- `README.md` - ビルドバッジ追加
- `CONTRIBUTING.md` - CI/CD実行方法の説明追加

## 実装計画

### Phase 1: 自動テストワークフロー（2-3h）

- [ ] `.github/workflows/ci.yml` 作成
  - [ ] トリガー設定（push, pull_request）
  - [ ] ビルド環境設定（Godot Engine, .NET）
  - [ ] 依存関係インストール
  - [ ] テスト実行
  - [ ] テストレポート生成

```yaml
name: CI

on:
  push:
    branches: [master]
  pull_request:
    branches: [master]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Godot
        uses: lihop/setup-godot@v2
        with:
          version: '4.2.0'
      - name: Run Tests
        run: godot --headless --path project --script res://tests/run_tests.gd
      - name: Upload Test Results
        uses: actions/upload-artifact@v3
        with:
          name: test-results
          path: project/tests/results/
```

### Phase 2: ビルドワークフロー（2-3h）

- [ ] ビルド設定追加
  - [ ] Linux ビルド
  - [ ] Windows ビルド
  - [ ] macOS ビルド（オプション）
  - [ ] アーティファクトアップロード

```yaml
  build:
    needs: test
    runs-on: ubuntu-latest
    strategy:
      matrix:
        platform: [linux, windows]
    steps:
      - uses: actions/checkout@v3
      - name: Build ${{ matrix.platform }}
        run: |
          godot --headless --export ${{ matrix.platform }} \
            build/${{ matrix.platform }}/game
      - name: Upload Build
        uses: actions/upload-artifact@v3
        with:
          name: build-${{ matrix.platform }}
          path: build/${{ matrix.platform }}/
```

### Phase 3: ドキュメント更新（1h）

- [ ] `README.md` 更新
  - [ ] ビルドステータスバッジ追加
  - [ ] CI/CD説明追加

```markdown
# My Game Project

![CI Status](https://github.com/username/repo/workflows/CI/badge.svg)

## CI/CD

このプロジェクトはGitHub Actionsで自動テスト・ビルドを実行しています。
```

- [ ] `CONTRIBUTING.md` 更新
  - [ ] PRを出す前のテスト実行方法
  - [ ] CI失敗時の対処法

### Phase 4: デプロイワークフロー（オプション、2-3h）

- [ ] `.github/workflows/cd.yml` 作成（必要に応じて）
  - [ ] タグpush時にデプロイ
  - [ ] itch.io へのアップロード
  - [ ] GitHub Releases 作成

## 設定項目

### Secrets

以下のシークレットをGitHub リポジトリ設定に追加：

- `GODOT_LICENSE` - Godot Engine ライセンスキー（必要な場合）
- `ITCH_API_KEY` - itch.io API Key（デプロイ時）

### Branch Protection Rules

- `master` ブランチにBranch Protection設定
  - ✅ Require status checks to pass before merging
  - ✅ Require CI to pass
  - ✅ Require pull request reviews before merging

## 実装上の注意点

### 1. worktree非対応

このタスクは `project-wide` タイプのため、**worktreeは作成されません**。
メインワーキングディレクトリで作業してください。

### 2. 他の開発者への影響

CI/CDパイプラインはリポジトリ全体に影響します：
- 他の開発者のPRもこのCIを通過する必要がある
- CI設定変更時は事前に通知する

### 3. コスト管理

GitHub Actionsの無料枠を超えないように注意：
- Public repositoryは無料
- Private repositoryは月2000分まで無料
- ビルド時間を最適化する（キャッシュ活用）

## テスト計画

### CI/CDパイプラインのテスト

- [ ] テストケース
  - [ ] PRを作成してCIが自動実行されることを確認
  - [ ] テスト失敗時にCIが失敗することを確認
  - [ ] ビルド成功時にアーティファクトがアップロードされることを確認

- [ ] エラーケース
  - [ ] ビルドエラー時の通知確認
  - [ ] テストタイムアウト時の挙動確認

## 完了条件

- [ ] CI/CDワークフローが正常に動作している
- [ ] テストが全て通過している
- [ ] ビルドアーティファクトが生成されている
- [ ] README.mdにビルドバッジが表示されている
- [ ] ドキュメントが更新されている
- [ ] Branch Protection Rulesが設定されている

## メモ

### 参考資料

- [GitHub Actions公式ドキュメント](https://docs.github.com/ja/actions)
- [Godot CI/CD Examples](https://github.com/abarichello/godot-ci)

### 将来の拡張

- CD: itch.io への自動デプロイ
- CD: GitHub Releases 自動作成
- CD: Steam への自動アップロード（Steamworks SDK連携）

### 関連タスク

- P002: Dockerコンテナ化（ローカルCI実行環境）
- P003: ドキュメント自動生成（APIドキュメント）

---

**このテンプレートは `project-wide` タスクの例です。**
**リポジトリ全体に影響するため、worktreeは使用せず、順次実行してください。**
