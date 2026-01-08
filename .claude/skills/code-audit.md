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

### 5. メモリアロケーション（unity-ai-reviewer: gc_allocation 相当）

Rust はガベージコレクションを持たないが、毎フレームのヒープアロケーションは性能劣化の原因となる。

**チェック項目:**
- `clone()` のホットパス使用
- `Vec::new()`, `vec![]` の毎フレーム生成
- `String::new()`, `to_string()`, `format!()` の毎フレーム使用
- 文字列結合（`+` 演算子）
- `collect::<Vec<_>>()` のホットパス使用
- `Box::new()` の毎フレーム使用

**深刻度:**
| レベル | 説明 |
|--------|------|
| MAJOR | 毎フレーム実行されるシステム内でのアロケーション |
| MINOR | 初期化時・イベント発火時のみのアロケーション |

**検出方法:**
```bash
# clone() の使用箇所
grep -rn "\.clone()" project/src/**/*.rs

# Vec::new() / vec![] の使用箇所
grep -rn "Vec::new()\|vec!\[" project/src/**/*.rs

# format! / to_string の使用箇所
grep -rn "format!\|\.to_string()" project/src/**/*.rs

# 文字列結合
grep -rn '+ "' project/src/**/*.rs
```

**許容ケース（除外対象）:**
- `fn setup(` や `fn build(` 内（初期化時のみ実行）
- `Added<>`, `Changed<>` フィルタ付きクエリ内
- イベントハンドラ内（頻度が低い）

---

### 6. 実行時エラーリスク（unity-ai-reviewer: runtime_error 相当）

予期せぬクラッシュを防ぐため、エラーハンドリングの欠如を検出。

**チェック項目:**
- `unwrap()` の未処理使用
- `expect()` の未処理使用
- `panic!()` の直接使用
- 配列への直接インデックス（`arr[i]`、境界チェックなし）
- 整数オーバーフローリスク（`-` 演算子の符号なし整数）

**深刻度:**
| レベル | 説明 |
|--------|------|
| CRITICAL | ユーザー入力や外部データに対する `unwrap()` |
| MAJOR | ゲームロジック内の `unwrap()`、直接インデックス |
| MINOR | 初期化時のみの `unwrap()`（設定ファイル読み込み等） |

**検出方法:**
```bash
# unwrap() / expect() の使用箇所
grep -rn "\.unwrap()\|\.expect(" project/src/**/*.rs

# panic! の使用箇所
grep -rn "panic!" project/src/**/*.rs

# 直接インデックス（簡易検出）
grep -rn "\[.*\]" project/src/**/*.rs | grep -v "impl\|where\|type"

# 整数オーバーフローリスク
grep -rn "\.saturating_sub\|\.checked_sub\|\.wrapping_sub" project/src/**/*.rs
```

**推奨対処:**
- `unwrap()` → `unwrap_or_default()`, `ok()`, `?` 演算子
- 直接インデックス → `.get()`, `.get_mut()`
- 整数演算 → `saturating_sub`, `checked_sub`

---

### 7. 車輪の再発明（unity-ai-reviewer: wheel_reinvention 相当）

標準ライブラリや Bevy のエコシステムで既に提供されている機能の再実装を検出。

**チェック項目:**
- Bevy 標準機能の再実装
  - 独自 Timer 実装（`Res<Time>` で代替可能）
  - 独自 EventWriter 実装
  - 独自 State 管理（`States` で代替可能）
- 重複ユーティリティ関数
  - 同一機能の複数実装
  - 似た名前の関数群
- 既存クレートで代替可能な処理
  - 数学ユーティリティ（`glam` で代替）
  - 乱数処理（`rand` で代替）

**深刻度:**
| レベル | 説明 |
|--------|------|
| MINOR | すべて（機能的には問題ないが、メンテナンス負荷増） |

**検出方法:**
```bash
# 独自 Timer 的なパターン
grep -rn "elapsed\|duration\|f32.*time" project/src/**/*.rs | grep -v "bevy"

# 重複関数名の検出
grep -rn "^pub fn " project/src/**/*.rs | cut -d: -f3 | sort | uniq -d
```

---

### 8. パフォーマンス（unity-ai-reviewer: efficiency 相当）

ゲームループに影響するパフォーマンス問題を検出。

**チェック項目:**
- O(n²) 以上の計算量
  - ネストした for ループ
  - 毎フレームのソート
- 毎フレームの重い処理
  - 文字列パース
  - ファイル I/O
- 非効率なクエリパターン
  - `Query<&Player>` のネスト（O(n²)）
  - `Changed`/`Added` フィルタの未活用
  - 不必要な `With<>`/`Without<>` の欠如

**深刻度:**
| レベル | 説明 |
|--------|------|
| MAJOR | O(n²) 以上のホットパス処理 |
| MINOR | 非効率だが O(n) 以内の処理 |

**検出方法:**
```bash
# ネストした for ループ
grep -rn "for .* in" project/src/**/*.rs -A 5 | grep "for .* in"

# 毎フレームソート
grep -rn "\.sort\|\.sort_by" project/src/**/*.rs

# Query のネスト（簡易検出）
grep -rn "Query<" project/src/**/*.rs -A 10 | grep -E "for.*Query|Query.*for"
```

**推奨対処:**
- ネストループ → `HashMap`/`HashSet` でインデックス化
- 毎フレームソート → 変更時のみソート、`Changed<>` フィルタ活用
- Query ネスト → `join()` または事前計算

---

### 9. セキュリティ（unity-ai-reviewer: security 相当）

ゲームでは優先度は低いが、将来のネットワーク対応等に備えて検出。

**チェック項目:**
- `unsafe` ブロックの使用
- 入力バリデーションの欠如
  - ユーザー名長さチェック
  - 数値範囲チェック
- ファイルパス操作のサニタイズ不足
  - パストラバーサル（`../` を含むパス）
  - 絶対パスの直接使用

**深刻度:**
| レベル | 説明 |
|--------|------|
| CRITICAL | `unsafe` の不適切な使用（メモリ安全性違反） |
| MAJOR | 外部入力に対するバリデーション欠如 |
| MINOR | ローカルゲームでの入力チェック欠如 |

**検出方法:**
```bash
# unsafe の使用箇所
grep -rn "unsafe" project/src/**/*.rs

# ファイルパス操作
grep -rn "PathBuf\|Path::new\|std::fs" project/src/**/*.rs

# 外部入力処理
grep -rn "read_line\|stdin\|args()" project/src/**/*.rs
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
| メモリアロケーション | 4 | 0 | 2 | 2 |
| 実行時エラーリスク | 6 | 2 | 3 | 1 |
| 車輪の再発明 | 2 | 0 | 0 | 2 |
| パフォーマンス | 3 | 0 | 1 | 2 |
| セキュリティ | 1 | 1 | 0 | 0 |
| **合計** | **34** | **4** | **14** | **16** |

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
category: "code-quality"  # spec-consistency/dependency/code-quality/architecture/memory/runtime-error/wheel-reinvention/performance/security
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
