# GitHub ワークフロー

仕様書駆動開発における GitHub Issue/PR の運用ガイド。

## 利用エージェント

- 🐙 github-agent - Issue/PR の作成・管理
- 💻 impl-agent - ブランチ作成・コミット
- ✅ review-agent - PR レビュー

---

## 基本方針

- **GitHub Issue を タスク管理の Single Source of Truth とする**
- 仕様書（spec/design/behavior）から Issue を参照する
- tasks.md は作成しない

---

## Issue 操作コマンド

```bash
# Issue 一覧確認
gh issue list --state open

# Issue 詳細確認
gh issue view <番号>

# 作業開始時
gh issue develop <番号> --checkout
# または
git checkout -b feature/#<番号>-<機能名>

# PR 作成（Issue 連携）
gh pr create --title "[機能番号] 内容" --body "Fixes #<番号>"
```

---

## ブランチ命名規則

```
feature/#<Issue番号>-<機能名>
例: feature/#12-jump
```

---

## コミットメッセージ

```
[ファイル番号] 種別: 内容

Fixes #<Issue番号>  ← PR にマージ時 Issue 自動クローズ
```

### 種別（仕様書）

| 種別 | 用途 |
|------|------|
| spec | 要件変更 |
| design | データ構造変更 |
| behavior | ロジック変更 |
| test | テスト追加・変更 |

### 種別（実装）

| 種別 | 用途 |
|------|------|
| feat | 新機能実装 |
| fix | バグ修正 |
| refactor | リファクタリング |
| balance | 8_data/ の値調整（例: 敵HP 50→55）。仕様書更新不要 |

### Issue 省略可の特例

以下のケースでは Issue 作成を省略し、コミットメッセージのみで処理してよい:

- 明白なバグ修正（Hotfix）
- ドキュメントの誤字修正
- 議論の余地がない軽微な変更

```bash
# Issue なしの軽微な修正
git commit -m "fix: ダメージ計算の0除算を修正"
git commit -m "docs: README の誤字修正"
```

---

## ラベル

### 基本ラベル（タスク種別）

| ラベル | 用途 |
|---|---|
| `spec` | 仕様策定 |
| `impl` | 実装 |
| `bug` | バグ修正 |
| `blocked` | 保留中 |

### 機能別ラベル（拡張可能）

以下は初期例。プロジェクトの進行に応じて **必要に応じて追加してください**。

| ラベル | 用途 |
|---|---|
| `feat/player` | Player機能 |
| `feat/enemy` | Enemy機能 |
| `feat/stage` | Stage機能 |
| `feat/title` | Title機能 |
| `feat/save` | Save/Load機能 |

#### 新規ラベル追加方法

```bash
# ラベル作成
gh label create "feat/ui" --description "UI/UX機能" --color "0052CC"

# 既存Issueに適用
gh issue edit <番号> --add-label "feat/ui"
```

**命名規則**: `feat/<機能名>` の形式を推奨

---

## 禁止事項

- 仕様書を更新せずに実装を変更する
- Issue を作らずに実装を始める（軽微な修正を除く）
- PR に Fixes #xx を書き忘れる

---

## Issue テンプレート

### 機能実装 Issue

```markdown
## 概要
[実装内容の簡潔な説明]

## 関連仕様
- REQ-[番号]-xxx: [要件名]

## 完了条件
- [ ] 仕様書の要件を満たす実装
- [ ] テストコード作成（TST-xxx対応）
- [ ] 動作確認完了

## 参考
- [spec.md](リンク)
- [design.md](リンク)

## 開発開始
gh issue develop <この Issue の番号> --checkout
```

### 仕様策定 Issue

```markdown
## 概要
[仕様策定の内容]

## 対象ファイル
- [ ] xxx_spec.md
- [ ] xxx_design.md
- [ ] xxx_behavior.md

## 完了条件
- [ ] 仕様書作成/更新完了
- [ ] レビュー完了
```

---

## マイルストーン

```
v0.1 - Player基本動作
v0.2 - Enemy基本動作
v0.3 - Stage基本実装
...
```

---

## Projects ボード（カンバン）

```
┌─────────────┬─────────────┬─────────────┬─────────────┐
│  Backlog    │   Todo      │   Doing     │   Done      │
├─────────────┼─────────────┼─────────────┼─────────────┤
│ 未整理      │ 着手可能    │ 作業中      │ 完了        │
│ 検討中      │ 仕様確定済  │             │             │
└─────────────┴─────────────┴─────────────┴─────────────┘
```
