# F009: docs/index.md 整頓

## 概要

`docs/index.md` と実際のディレクトリ構造の不整合を解消する。

## 背景

index.md に記載されているリンクやセクションの一部が、実際のファイル構造と一致していない。
リンク切れの解消と未記載ファイルの追加により、ドキュメントの信頼性を向上させる。

## タスク種別

- [x] フレームワーク開発（FXXX）

## 対象ファイル

- `docs/index.md`

## 作業内容

### 1. 削除するセクション

| セクション | 理由 |
|-----------|------|
| 6. アーキテクチャドキュメント（2_architecture/） | プロジェクト用 |
| 7. 参照資料（9_reference/） | プロジェクト用 |

### 2. 削除するリンク（reference/ 内）

| ファイル | 理由 |
|---------|------|
| `integrity-check-report.md` | 存在しない |
| `git-github-operations.md` | 存在しない |
| `agent-prohibited-actions-template.md` | 存在しない |

### 3. 修正するリンク

| 現在 | 修正後 |
|-----|-------|
| `../skills/ears.md` | `../.claude/skills/ears.md` |

### 4. 追加するファイル

**getting-started/ セクション:**
- `agent-details.md` - エージェント詳細
- `task-management-guide.md` - タスク管理ガイド

**reference/ セクション:**
- `README.md` - リファレンス概要
- `task-management-faq.md` - タスク管理FAQ

**templates/ セクション:**
- `task-examples/` ディレクトリ（4ファイル）

### 5. ディレクトリ構成図の更新

末尾の構成図から 2_architecture/, 9_reference/ を削除し、実態に合わせる

## 完了条件

- [x] 全リンク切れが解消されている
- [x] 実在する全ファイルが index.md に記載されている
- [x] ディレクトリ構成図が実態と一致している

## 関連プランファイル

- `~/.claude/plans/twinkling-shimmying-lampson.md`
