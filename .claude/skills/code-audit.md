# code-audit

## 概要

プロジェクト全体の定期的なコード健康診断を実施するスキル。
問題をカテゴリ別に検出し、リファクタタスク（R30XXX-NNN）として自動生成する。

### 参照元ガイドライン
- audit-agent（監査実行時）
- task-manager-agent（タスク化時）

---

## 監査カテゴリ

### 1. 仕様整合性

既存コマンド `/impl-validate` を活用。

**チェック項目:**
- @spec コメントの欠如
- 参照先 REQ の不存在
- 未実装要件

**深刻度:**
| レベル | 説明 |
|--------|------|
| MAJOR | @spec コメント欠如、参照先 REQ 不存在 |
| MINOR | 未実装要件 |

---

### 2. 依存関係

既存コマンド `/deps-check` を活用。

**チェック項目:**
- 循環依存
- 禁止依存
- リンク切れ（Markdown リンク）

**深刻度:**
| レベル | 説明 |
|--------|------|
| CRITICAL | 循環依存 |
| MAJOR | 禁止依存 |
| MINOR | リンク切れ |

---

### 3. コード品質（独自分析）

**チェック項目:**
- 複雑すぎる関数（50行超、ネスト4段超）
- 重複コード
- Long Parameter List（引数5個超）
- Dead Code（未使用の public 関数）

**深刻度:**
| レベル | 説明 |
|--------|------|
| MAJOR | 複雑すぎる関数（50行超、ネスト4段超） |
| MINOR | 重複コード、Long Parameter List |

**検出方法:**
```bash
# 行数が多い関数を検出（Rust）
grep -n "fn " project/src/**/*.rs | while read line; do
  # 関数の行数を計算
done

# ネストが深いコードを検出
grep -n "^        " project/src/**/*.rs  # 4段インデント以上
```

---

### 4. アーキテクチャ（独自分析）

**チェック項目:**
- レイヤー違反（domain → infrastructure 参照など）
- 責務過多（God Object: 300行超のモジュール）
- 結合度過大（1ファイルが10以上のモジュールを参照）

**深刻度:**
| レベル | 説明 |
|--------|------|
| MAJOR | レイヤー違反、責務過多（God Object） |
| MINOR | 結合度過大 |

**検出方法:**
```bash
# モジュール行数の確認
wc -l project/src/**/*.rs | sort -rn | head -20

# use 文の数を確認
grep -c "^use " project/src/**/*.rs | sort -t: -k2 -rn | head -20
```

---

## 監査実行手順

### Step 1: 既存コマンドによる検証

```bash
/impl-validate project/src/**/*.rs
/deps-check
```

### Step 2: コード品質分析

1. 関数行数の計算
2. ネスト深度の計測
3. 重複コードの検出（類似度80%以上）
4. 未使用コードの検出

### Step 3: アーキテクチャ分析

1. モジュール依存関係の可視化
2. レイヤー違反の検出
3. 責務過多モジュールの検出

### Step 4: レポート生成

診断レポートをマークダウン形式で生成。

---

## 出力形式（診断レポート）

```markdown
# コード監査レポート

**実行日時**: 2026-01-07T10:00:00+09:00
**対象範囲**: project/src/**, project/docs/**
**監査者**: audit-agent

---

## サマリー

| カテゴリ | 問題数 | Critical | Major | Minor |
|---------|--------|----------|-------|-------|
| 仕様整合性 | 5 | 0 | 2 | 3 |
| 依存関係 | 2 | 1 | 1 | 0 |
| コード品質 | 8 | 0 | 3 | 5 |
| アーキテクチャ | 3 | 0 | 2 | 1 |
| **合計** | **18** | **1** | **8** | **9** |

---

## 詳細

### 仕様整合性

#### [MAJOR] @spec コメント欠如
- `project/src/game/player.rs:45` - `pub fn jump()`
- `project/src/game/ball.rs:23` - `pub fn hit()`

#### [MINOR] 未実装要件
- REQ-30101-007: ダッシュ機能

---

### 依存関係

#### [CRITICAL] 循環依存
- `player` → `ball` → `court` → `player`

#### [MAJOR] 禁止依存
- `game::domain` → `game::infrastructure`（レイヤー違反）

---

### コード品質

#### [MAJOR] 複雑すぎる関数
- `project/src/game/physics.rs:120` - `calculate_trajectory()` (78行)
- `project/src/game/ai.rs:45` - `decide_action()` (ネスト5段)

#### [MINOR] Long Parameter List
- `project/src/game/shot.rs:30` - `execute_shot(a, b, c, d, e, f)` (6引数)

---

### アーキテクチャ

#### [MAJOR] 責務過多（God Object）
- `project/src/game/game_state.rs` (450行)

#### [MINOR] 結合度過大
- `project/src/game/main.rs` - 15モジュール参照

---

## 推奨タスク一覧

| ID候補 | タイトル | 優先度 | 関連問題数 | 推定工数 |
|--------|---------|--------|-----------|---------|
| R30XXX-001 | @spec コメント追加（player, ball） | medium | 2 | S |
| R30XXX-002 | 循環依存解消（player-ball-court） | high | 1 | M |
| R30XXX-003 | calculate_trajectory 分割 | medium | 1 | S |
| R30XXX-004 | game_state 責務分離 | medium | 1 | L |

---

## 次のアクション

1. **Critical 問題を即時対応**: 循環依存
2. **タスク化確認**: 上記推奨タスクを作成しますか？
```

---

## タスク化フロー

```
audit-agent（診断完了）
  ↓
ユーザー確認「タスクを作成しますか？」
  ↓
task-registration-agent（タスク作成）
  ↓
R30XXX-NNN ファイル生成（project/tasks/1_todo/）
  ↓
refactor-agent（実装時に参照）
```

---

## タスク化テンプレート

### R30XXX-NNN タスクファイル形式

```yaml
---
id: "R30XXX-NNN"
title: "タスクタイトル"
type: "refactor"
status: "todo"
priority: "medium"
related_task: "30XXX"  # 関連する元機能
spec_ids: []
blocked_by: []
blocks: []
audit_source: "2026-01-07"  # 監査レポート日付
severity: "major"  # critical/major/minor
category: "code-quality"  # spec-consistency/dependency/code-quality/architecture
created_at: "2026-01-07T10:00:00+09:00"
updated_at: "2026-01-07T10:00:00+09:00"
---

# Task R30XXX-NNN: タスクタイトル

## Summary

監査で検出された問題の修正。

## 検出された問題

- ファイル: `project/src/game/physics.rs:120`
- 問題: 関数が78行と長すぎる
- 深刻度: MAJOR

## 修正方針

1. 関数を3つに分割
2. 各関数を20行以内に収める
3. 単体テストを追加

## Progress

### Completed

（なし）

## Next Actions

1. 関数の責務を分析
2. 分割ポイントを決定
3. リファクタリング実施
```

---

## 監査実行サイクル

### 推奨頻度

| タイミング | 監査タイプ | 対象範囲 |
|-----------|----------|---------|
| 週次 | 差分監査 | 変更されたファイルのみ |
| スプリント終了時 | フル監査 | プロジェクト全体 |
| リリース前 | フル監査 + Critical 優先 | プロジェクト全体 |

### 差分監査の対象

```bash
# 直近1週間の変更ファイル
git diff --name-only HEAD~7..HEAD -- 'project/src/**/*.rs'
```

---

## 関連ドキュメント

- [impl-comments.md](impl-comments.md) - 実装コメント規約
- [bug-backlog.md](bug-backlog.md) - バグバックログ管理
- [task-file-format.md](task-file-format.md) - タスクファイル形式
