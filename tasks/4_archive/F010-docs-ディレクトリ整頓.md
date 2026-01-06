---
id: "F010"
title: "docs ディレクトリ整頓"
type: "framework"
status: "done"
priority: "medium"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/s13219/.claude/plans/encapsulated-stirring-cocke.md"
tags: ["docs", "restructure"]
created_at: "2026-01-02T23:10:00+09:00"
updated_at: "2026-01-02T23:15:00+09:00"
completed_at: "2026-01-03T12:00:00+09:00"
---

# Task F010: docs ディレクトリ整頓

## 説明

docs/ ディレクトリを再構成し、目的の情報が見つかりやすく、重複を排除し、読む順序を明確にする。

## 背景

### 現状
- getting-started/, reference/ などに類似ドキュメントが散在
- 同じ情報が複数ファイルに重複
- 読む順序が不明確

### 改修理由
- 目的の情報が見つかりにくい
- 重複によるメンテナンスコスト増
- 新規ユーザーの学習パスが不明確

## 実装内容

### Phase 1: ディレクトリ作成とアーカイブ ✅ 完了
- [x] `docs/tutorials/` 作成
- [x] `docs/concepts/` 作成
- [x] `docs/_archive/` 作成
- [x] `planning/` → `_archive/planning/` に移動

### Phase 2: ファイル移動・リネーム ✅ 完了
- [x] `getting-started/users-guide.md` → `tutorials/quickstart.md`
- [x] `guides/legacy-code-workflow-for-creator.md` → `guides/legacy-code-creator.md`

### Phase 3: ファイル統合 ✅ 完了
- [x] `concepts/agents.md` 作成（agent-selection-guide.md + agent-details.md 統合）
- [x] `concepts/tasks.md` 作成（task-management-guide.md + task-management-faq.md 統合）
- [x] `concepts/overview.md` 作成（users-guide-reference.md から概念部分抽出）

### Phase 4: 削除・クリーンアップ ✅ 完了
- [x] `reference/README.md` 削除
- [x] 統合済みファイル削除（agent-selection-guide.md, agent-details.md 等）
- [x] `getting-started/` ディレクトリ削除

### Phase 5: index.md 再設計 ✅ 完了
- [x] 目的別ナビゲーションに再構成
- [x] 読み順ガイドを追加

### Phase 6: CLAUDE.md のリンク修正 ✅ 完了
- [x] 116行: `docs/getting-started/users-guide.md` → `docs/tutorials/quickstart.md`
- [x] 116行: `docs/reference/task-management-faq.md` → `docs/concepts/tasks.md`
- [x] 172行: `docs/getting-started/users-guide.md` → `docs/tutorials/quickstart.md`
- [x] 173行: `docs/getting-started/agent-selection-guide.md` → `docs/concepts/agents.md`
- [x] 188行: `docs/getting-started/agent-selection-guide.md` → `docs/concepts/agents.md`
- [x] 17-21行: ディレクトリ構成コメントの更新（getting-started → tutorials, concepts）

### Phase 7: 検証 ✅ 完了
- [x] 全リンクの動作確認
- [x] `/docs-validate` 実行

### Phase 8: 他ファイルのリンク修正 ✅ 完了
- [x] project/README.md (3箇所)
- [x] README.md (4箇所)
- [x] docs/reference/framework-spec.md (多数)
- [x] docs/reference/tools-reference.md (多数)
- [x] docs/framework-development/guide.md (1箇所)
- [x] docs/templates/task-examples/example-framework-task.md (2箇所)
- [x] docs/templates/task-examples/example-task-group.md (1箇所)

## メモ

- Phase 3（ファイル統合）が最も作業量が多い
- 統合時は重複排除しつつ、情報欠損に注意

## 依存関係

- **ブロック**: なし
- **ブロックされる**: なし
- **関連ドキュメント**: docs/index.md, .claude/CLAUDE.md

---

## Detailed Implementation Plan

以下は、プランファイル `~/.claude/plans/encapsulated-stirring-cocke.md` の全内容です。

# docs ディレクトリ整頓計画

## 目的

- 目的の情報が見つかりやすくする
- 同じ情報の重複を排除する
- 読む順序を明確にする

## 新しいディレクトリ構造

```
docs/
├── index.md                      # 再設計（目的別ナビゲーション）
├── CHANGELOG.md                  # 維持
│
├── tutorials/                    # 【新設】クイックスタート
│   └── quickstart.md             # ← getting-started/users-guide.md
│
├── concepts/                     # 【新設】概念説明（深く理解したい人向け）
│   ├── overview.md               # ← users-guide-reference.md から概念部分抽出
│   ├── agents.md                 # ← agent-details.md + agent-selection-guide.md 統合
│   └── tasks.md                  # ← task-management-guide.md + task-management-faq.md 統合
│
├── reference/                    # 維持（逆引き用）
│   ├── framework-spec.md         # 維持
│   ├── spec-writing-guide.md     # 維持
│   ├── tools-reference.md        # 維持
│   ├── design-decisions.md       # 維持
│   └── validation-tools-spec.md  # 維持
│   # 削除: README.md（index.mdに統合）
│
├── guides/                       # 維持（実践ガイド）
│   ├── legacy-code-analysis.md   # 維持
│   └── legacy-code-creator.md    # ← legacy-code-workflow-for-creator.md リネーム
│
├── framework-development/        # 維持
│   ├── guide.md
│   ├── contributing.md
│   └── philosophy.md
│
├── templates/                    # 維持
│
└── _archive/                     # 【新設】古い計画書
    └── planning/                 # ← planning/ 移動
```

## 実行フェーズ

### Phase 1: ディレクトリ作成とアーカイブ

1. `docs/tutorials/` 作成
2. `docs/concepts/` 作成
3. `docs/_archive/` 作成
4. `planning/` → `_archive/planning/` に移動

### Phase 2: ファイル移動・リネーム（単純な移動のみ）

| 移動元 | 移動先 |
|--------|--------|
| `getting-started/users-guide.md` | `tutorials/quickstart.md` |
| `guides/legacy-code-workflow-for-creator.md` | `guides/legacy-code-creator.md` |

### Phase 3: ファイル統合（内容編集を伴う）

#### 3-1. concepts/agents.md の作成
- `getting-started/agent-selection-guide.md` (615行)
- `getting-started/agent-details.md` (820行)
- → 統合して `concepts/agents.md` (~900行)

構成案:
```
# エージェントガイド
## どのエージェントを使うか（選択フローチャート）
## 全エージェント一覧（概要表）
## 各エージェント詳細
```

#### 3-2. concepts/tasks.md の作成
- `getting-started/task-management-guide.md` (985行)
- `reference/task-management-faq.md` (857行)
- → 統合して `concepts/tasks.md` (~800行、重複排除)

構成案:
```
# タスク管理ガイド
## 基本概念
## タスクライフサイクル
## よくある質問（FAQ）
```

#### 3-3. concepts/overview.md の作成
- `getting-started/users-guide-reference.md` (853行) から概念部分を抽出
- → `concepts/overview.md` (~500行)

構成案:
```
# フレームワーク概要
## 設計思想
## 基本概念
## ワークフロー概要
```

### Phase 4: 削除・クリーンアップ

削除対象:
- `reference/README.md`
- `getting-started/agent-selection-guide.md` (統合済み)
- `getting-started/agent-details.md` (統合済み)
- `getting-started/task-management-guide.md` (統合済み)
- `getting-started/users-guide-reference.md` (抽出済み)
- `reference/task-management-faq.md` (統合済み)
- `getting-started/` ディレクトリ（空になる）

### Phase 5: index.md 再設計

新しい index.md の構成:
```markdown
# docs/ ドキュメント索引

## 目的から探す

### 今すぐ始めたい
- [クイックスタート](./tutorials/quickstart.md) - 20分

### 深く理解したい
- [フレームワーク概要](./concepts/overview.md)
- [エージェントガイド](./concepts/agents.md)
- [タスク管理](./concepts/tasks.md)

### 逆引きしたい
- [フレームワーク仕様書](./reference/framework-spec.md)
- [仕様書の書き方](./reference/spec-writing-guide.md)
- [ツールリファレンス](./reference/tools-reference.md)

### 特定シナリオ
- [レガシーコード解析](./guides/legacy-code-analysis.md)

## 読み順ガイド
1. quickstart.md (20分)
2. agents.md の選択フローチャート (10分)
3. 必要に応じて concepts/ や reference/ を参照
```

### Phase 6: CLAUDE.md のリンク修正

| 現在のパス | 新しいパス |
|------------|------------|
| `docs/getting-started/users-guide.md` | `docs/tutorials/quickstart.md` |
| `docs/getting-started/agent-selection-guide.md` | `docs/concepts/agents.md` |
| `docs/reference/task-management-faq.md` | `docs/concepts/tasks.md` |

### Phase 7: 検証

1. 全リンクの動作確認
2. CLAUDE.md からのリンク確認
3. `/docs-validate` 実行

## 修正対象ファイル一覧

### 作成
- `docs/tutorials/quickstart.md`
- `docs/concepts/overview.md`
- `docs/concepts/agents.md`
- `docs/concepts/tasks.md`

### 編集
- `docs/index.md`
- `.claude/CLAUDE.md`

### 削除
- `docs/getting-started/` (ディレクトリごと)
- `docs/reference/README.md`
- `docs/reference/task-management-faq.md`
- `docs/planning/` (ディレクトリごと、_archive に移動後)

### 移動
- `docs/planning/*` → `docs/_archive/planning/`
- `docs/guides/legacy-code-workflow-for-creator.md` → `docs/guides/legacy-code-creator.md`

## 作業見積もり

- Phase 1-2: 単純作業（15分）
- Phase 3: ファイル統合（1-2時間）- 最も時間がかかる
- Phase 4-5: クリーンアップとindex再設計（30分）
- Phase 6-7: リンク修正と検証（15分）

合計: 約2-3時間
