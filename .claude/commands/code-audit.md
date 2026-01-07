---
description: プロジェクト全体のコード健康診断を実行
argument-hint: [--full|--diff|--scope <path>]
---

# /code-audit コマンド

プロジェクト全体の定期的なコード健康診断を実施し、問題をカテゴリ別にレポートします。

**引数**: $ARGUMENTS

## 使用者

**🤖 エージェント専用コマンド** - 人間は直接使わない

### 使用エージェント

| エージェント | 使用タイミング | 目的 |
|------------|--------------|------|
| audit-agent | 定期監査時 | プロジェクト全体の健康診断 |

**自動実行**: ユーザーが「コードを監査して」「健康診断して」と指示した際に実行

---

## オプション

| オプション | 説明 | デフォルト |
|-----------|------|----------|
| `--full` | プロジェクト全体をフル監査 | ✅ デフォルト |
| `--diff` | 直近の変更ファイルのみ差分監査 | - |
| `--scope <path>` | 指定パスのみを監査 | - |
| `--severity <level>` | 指定深刻度以上のみ表示（critical/major/minor） | minor |
| `--category <cat>` | 指定カテゴリのみ監査（spec/deps/quality/arch） | all |
| `--no-task` | タスク化提案をスキップ | - |

---

## 使用方法

```bash
# フル監査（デフォルト）
/code-audit

# 差分監査（直近の変更のみ）
/code-audit --diff

# 特定ディレクトリのみ
/code-audit --scope project/src/game/

# 深刻度フィルタ
/code-audit --severity major

# カテゴリ指定
/code-audit --category quality
```

---

## 監査カテゴリ

| カテゴリ | 検証内容 | 既存コマンド活用 |
|---------|---------|----------------|
| 仕様整合性（spec） | @spec コメント、REQ 参照 | `/impl-validate` |
| 依存関係（deps） | 循環依存、禁止依存、リンク切れ | `/deps-check` |
| コード品質（quality） | 複雑度、重複、Long Parameter List | 独自分析 |
| アーキテクチャ（arch） | レイヤー違反、責務過多、結合度 | 独自分析 |

---

## 指示

引数として渡されたオプション（`$ARGUMENTS`）に従って監査を実行します。

### Step 1: 対象範囲の決定

```bash
# --full（デフォルト）
対象: project/src/**, project/docs/**

# --diff
git diff --name-only HEAD~7..HEAD -- 'project/src/**/*.rs' 'project/docs/**/*.md'

# --scope <path>
対象: 指定パス配下
```

### Step 2: 既存コマンドによる検証

```bash
# 仕様整合性
/impl-validate <対象ファイル>

# 依存関係
/deps-check
```

### Step 3: 独自分析

#### コード品質分析

```bash
# 関数行数（50行超を検出）
# ネスト深度（4段超を検出）
# Long Parameter List（5引数超を検出）
# 重複コード検出
```

#### アーキテクチャ分析

```bash
# モジュール行数（300行超を検出）
# use 文数（10超を検出）
# レイヤー違反検出
```

### Step 4: レポート生成

診断レポートをマークダウン形式で生成。

### Step 5: タスク化提案

`--no-task` が指定されていない場合、推奨タスク一覧を提示し、ユーザーに確認。

---

## 出力形式

```
=== Code Audit Report ===

Date: 2026-01-07T10:00:00+09:00
Scope: project/src/**, project/docs/**
Mode: full

--- Summary ---

| Category | Total | Critical | Major | Minor |
|----------|-------|----------|-------|-------|
| Spec Consistency | 5 | 0 | 2 | 3 |
| Dependencies | 2 | 1 | 1 | 0 |
| Code Quality | 8 | 0 | 3 | 5 |
| Architecture | 3 | 0 | 2 | 1 |
| **Total** | **18** | **1** | **8** | **9** |

--- Issues (sorted by severity) ---

[CRITICAL] Circular dependency detected
  player → ball → court → player
  → Recommend: R30XXX-001

[MAJOR] @spec comment missing
  project/src/game/player.rs:45 - pub fn jump()
  → Recommend: R30XXX-002

[MAJOR] Function too long (78 lines)
  project/src/game/physics.rs:120 - calculate_trajectory()
  → Recommend: R30XXX-003

...

--- Recommended Tasks ---

| ID | Title | Priority | Issues | Effort |
|----|-------|----------|--------|--------|
| R30XXX-001 | Resolve circular dependency | high | 1 | M |
| R30XXX-002 | Add @spec comments | medium | 2 | S |
| R30XXX-003 | Split calculate_trajectory | medium | 1 | S |

=== End of Report ===

Create tasks? (y/n)
```

---

## 深刻度の定義

| レベル | 説明 | 対応期限 |
|--------|------|---------|
| CRITICAL | ビルド失敗、実行時エラーの可能性 | 即時 |
| MAJOR | 保守性・品質に重大な影響 | 次スプリント |
| MINOR | 改善推奨だが緊急性なし | バックログ |

---

## 終了コード

| コード | 意味 |
|--------|------|
| 0 | 問題なし |
| 1 | MINOR のみ検出 |
| 2 | MAJOR 以上検出 |
| 3 | CRITICAL 検出 |

---

## 使用例

### 定期フル監査

```bash
/code-audit --full
```

### CI での差分監査

```bash
/code-audit --diff --severity major --no-task
```

### 特定モジュールの品質チェック

```bash
/code-audit --scope project/src/game/player/ --category quality
```

---

## 関連コマンド

- `/impl-validate` - 仕様整合性の個別検証
- `/deps-check` - 依存関係の個別検証
- `/docs-validate` - ドキュメント整合性の検証

---

## 関連スキル

- `skills/code-audit.md` - 監査手順・チェック項目の詳細

---

## 注意事項

### 監査頻度の推奨

| タイミング | 監査タイプ |
|-----------|----------|
| 週次 | `--diff` |
| スプリント終了時 | `--full` |
| リリース前 | `--full --severity critical` |

### パフォーマンス

- フル監査は大規模プロジェクトで数分かかる場合がある
- 差分監査は通常数秒で完了
- `--scope` で範囲を絞ることで高速化可能
