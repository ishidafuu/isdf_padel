---
id: "F001"
title: "impl-agent コミットメッセージ形式更新"
type: "framework"
status: "todo"
priority: "high"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["agent", "framework", "improvement"]
created_at: "2025-12-29T10:00:00.000000"
updated_at: "2025-12-29T10:00:00.000000"
completed_at: null
parent_task_id: null
---

# impl-agent コミットメッセージ形式更新

## 概要

`impl-agent` のコミットメッセージ形式を更新し、タスクIDを含めるようにする。

**現在の形式:**
```
feat: プレイヤージャンプ機能実装
```

**新しい形式:**
```
[30101] feat: プレイヤージャンプ機能実装
```

**注意:** このタスクは `framework` タイプであり、フレームワーク全体に影響するため、**worktreeは作成されません**。`tasks/` ディレクトリに配置されます。

## 対象範囲

### 影響を受けるファイル

- `.claude/agents/impl-agent.md` - エージェント定義
- `docs/tutorials/quickstart.md` - コミットメッセージ例
- `docs/CHANGELOG.md` - 変更履歴

## 実装計画

### Phase 1: エージェント定義更新（1-2h）

- [ ] `.claude/agents/impl-agent.md` 更新
  - [ ] コミットメッセージ形式の説明を追加
  - [ ] 例を更新

```markdown
## コミットメッセージ形式

**形式:** `[タスクID] prefix: 変更内容`

**prefix:**
- `feat`: 新機能
- `fix`: バグ修正
- `refactor`: リファクタリング
- `test`: テスト追加・修正
- `docs`: ドキュメント更新

**例:**
\```
[30101] feat: プレイヤージャンプ機能実装
[30102] fix: 敵AIの移動ロジック修正
[P001] feat: CI/CD構築
\```

**利点:**
- タスクIDから関連コミットを検索できる
- `git log --grep="30101"` でタスクの全コミットを表示
```

- [ ] サンプルコード更新

### Phase 2: ドキュメント更新（1h）

- [ ] `docs/tutorials/quickstart.md` 更新
  - [ ] Phase 3セクションのコミットメッセージ例を更新

```markdown
**コミットメッセージ:**
\```bash
git commit -m "[30101] feat: プレイヤージャンプ機能実装"
\```
```

```markdown
## コミットメッセージ形式

**標準形式:** `[タスクID] prefix: 変更内容`

### タスクIDの取得

- game-devタスク: タスクファイル名から取得（例: `30101-ジャンプ機能実装.md` → `30101`）
- project-wideタスク: `PXXX`
- frameworkタスク: `FXXX`
```

### Phase 3: CHANGELOG更新（30分）

- [ ] `docs/CHANGELOG.md` に変更記録を追加

```markdown
## [Unreleased]

### Changed
- impl-agent のコミットメッセージ形式を `[タスクID] prefix: 変更内容` に更新
```

### Phase 4: 既存ドキュメントの整合性確認（1h）

- [ ] `docs/` 配下の全ドキュメントをスキャン
- [ ] 古いコミットメッセージ例を新しい形式に更新
- [ ] クロスリファレンスが正しいか確認

## 実装上の注意点

### 1. worktree非対応

このタスクは `framework` タイプのため、**worktreeは作成されません**。
メインワーキングディレクトリで作業してください。

### 2. 配置場所

フレームワークタスクは `tasks/` ディレクトリに配置されます（`project/tasks/` ではない）。

```
tasks/
├── .taskrc.yaml          # フレームワークタスク設定
├── 1_todo/
│   └── F001-impl-agent-コミットメッセージ形式更新.md  ← このファイル
├── 2_in-progress/
├── 3_in-review/
└── 4_archive/
```

### 3. 既存エージェントへの影響

`impl-agent` は全ての実装で使用されるため、変更の影響範囲が広い：
- 全ての開発者が新しいコミットメッセージ形式を使うようになる
- 既存のコミット履歴は変更しない（後方互換性）

## テスト計画

### エージェント動作確認

- [ ] `impl-agent` を起動してコミットメッセージ形式を確認
  - [ ] 新しい形式でコミットされることを確認
  - [ ] タスクIDが正しく含まれていることを確認

- [ ] ドキュメント整合性確認
  - [ ] 全ての例が新しい形式に更新されていることを確認
  - [ ] クロスリファレンスが正しいことを確認

## 完了条件

- [ ] `.claude/agents/impl-agent.md` が更新されている
- [ ] 全てのドキュメントが新しいコミットメッセージ形式に更新されている
- [ ] `docs/CHANGELOG.md` に変更が記録されている
- [ ] エージェント動作テストが完了している
- [ ] ドキュメント整合性チェックが完了している

## メモ

### 設計判断

**なぜタスクIDを含めるのか:**
- タスクから関連コミットを追跡しやすい
- `git log --grep="30101"` でタスクの全変更履歴を表示できる
- PRとタスクの対応が明確になる

**後方互換性:**
- 既存のコミット履歴は変更しない
- 新しいコミットのみ新形式を使用

### 将来の拡張

- コミットメッセージの自動生成（タスクファイルから）
- pre-commit hook でコミットメッセージ形式を検証

### 関連タスク

- F002: タスク管理の人間向けドキュメント作成
- F003: エージェント定義の整合性確認

---

**このテンプレートは `framework` タスクの例です。**
**フレームワーク全体に影響するため、worktreeは使用せず、慎重に実行してください。**
