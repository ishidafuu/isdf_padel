# 検証ツール仕様書

仕様書とコードの整合性を検証するツール群の仕様。

---

## ツール1: validate-ids.py

### 目的

仕様書内のID重複・命名規則違反を検出する。

### 機能（簡略化版）

#### 必須機能
1. **ID抽出** - 正規表現で全ID抽出
2. **重複検出** - 同じIDが複数箇所に存在
3. **命名規則検証** - `[TYPE]-[AAAXX]-[NNN]` 形式チェック
4. **フォルダコード検証** - IDとファイルパスの一致確認

#### 削除機能（不要）
- ~~ID予約範囲チェック~~ - 予約システムがないため不要
- ~~予約ファイル読み込み~~ - `.id-reservations.yml` 不使用

### 入力

```bash
python3 scripts/validate-ids.py [--path docs/] [--format text|json]
```

### 出力例

```
🔍 ID Validation Report

✅ Total IDs: 247
❌ Errors: 2
⚠️  Warnings: 3

━━━ ERRORS ━━━

[E001] Duplicate ID
  ID: REQ-30101-001
  Locations:
    - docs/3_ingame/301_player/30101_spec.md:15
    - docs/3_ingame/302_enemy/30201_spec.md:23

[E002] Invalid format
  ID: REQ-301-001 (should be REQ-30101-001)
  File: docs/3_ingame/301_player/30102_design.md:45

━━━ WARNINGS ━━━

[W001] Gap in sequence
  Range: REQ-30101-001～050
  Missing: 003, 007
  Note: Gaps are allowed but may indicate deleted requirements

Exit code: 1
```

### 実装サイズ

- **コア機能**: 約150行
- **レポート生成**: 約100行
- **合計**: 約250行

---

## ツール2: validate-links.py

### 目的

仕様書間の参照リンク（`REQ-30101-001`）が存在するか検証。

### 機能（変更なし）

1. **ID定義インデックス構築** - 全仕様書からID定義を抽出
2. **ID参照検索** - 全仕様書・コードからID参照を抽出
3. **リンク切れ検出** - 参照先が存在しないIDを検出
4. **孤立ID検出** - 定義されているが参照されていないID

### 入力

```bash
python3 scripts/validate-links.py [--path docs/] [--include-code] [--format text|json]
```

### 出力例

```
🔗 Link Validation Report

📊 Statistics:
  Total links: 523
  Broken links: 7
  Orphaned IDs: 12

━━━ BROKEN LINKS ━━━

[1] REQ-30101-002
  Referenced in:
    - docs/3_ingame/301_player/30103_behavior.md:34
    - src/Player/PlayerJump.cs:12 (@spec comment)

  Status: Not found in any spec
  Suggestion: Did you mean REQ-30101-001?

━━━ ORPHANED IDs ━━━

REQ-30101-005 (docs/.../30101_spec.md:67) - Never referenced
```

### 実装サイズ

- **インデックス構築**: 約100行
- **リンク検証**: 約100行
- **レポート生成**: 約100行
- **合計**: 約300行

---

## ツール3: validate-impl-comments.py

### 目的

実装コード内の `@spec`/`@test`/`@data` コメントが正しいIDを参照しているか検証。

### 機能（最重要）

1. **コードパース** - C#/Python/TypeScript等から関数抽出
2. **@specコメント検出** - 各関数の直前コメントを解析
3. **ID存在確認** - コメント内のIDが仕様書に存在するか
4. **カバレッジ計算** - 実装済み仕様の割合

### 優先度が高い理由

**実装フェーズでの並列実行**を想定しているため、以下が重要：
- 複数セッションで実装時、各実装者が正しいIDを参照しているか
- 仕様と実装の対応が保証されているか
- 実装漏れがないか（カバレッジ）

### 入力

```bash
python3 scripts/validate-impl-comments.py [--code-path src/] [--docs-path docs/]
```

### 出力例

```
📝 Implementation Comment Validation

📊 Statistics:
  Total functions: 143
  With @spec: 98 (68%)
  Missing @spec: 45 (32%)

━━━ ERRORS ━━━

[E001] Invalid @spec reference
  File: src/Player/PlayerJump.cs:45
  Function: DoubleJump()
  Comment: @spec REQ-99999-999

  Error: ID not found in specs

[E002] Missing @spec comment
  File: src/Enemy/EnemyAI.cs:12
  Function: public void Patrol()

  Error: Public function lacks @spec comment

━━━ COVERAGE ━━━

Implemented specs: 98/120 (82%)
Not implemented:
  - REQ-30101-007
  - REQ-30201-003
  ...
```

### 実装サイズ

- **言語別パーサー**: 約200行（C#のみ、他言語は後で追加）
- **検証ロジック**: 約100行
- **レポート生成**: 約100行
- **合計**: 約400行

---

## 統合実行スクリプト

### validate-all.sh

```bash
#!/bin/bash
# scripts/validate-all.sh

echo "🔍 Spec-Driven Framework Validation"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

EXIT_CODE=0

# 1. ID検証
echo "[1/3] Validating IDs..."
python3 scripts/validate-ids.py --path docs/
[ $? -ne 0 ] && EXIT_CODE=1

# 2. リンク検証
echo "[2/3] Validating Links..."
python3 scripts/validate-links.py --path docs/ --include-code
[ $? -ne 0 ] && EXIT_CODE=1

# 3. 実装コメント検証
echo "[3/3] Validating Implementation Comments..."
if [ -d "src/" ]; then
  python3 scripts/validate-impl-comments.py --code-path src/ --docs-path docs/
  [ $? -ne 0 ] && EXIT_CODE=1
else
  echo "⚠️  Skipped (no src/ directory)"
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
[ $EXIT_CODE -eq 0 ] && echo "✅ All validations passed" || echo "❌ Some validations failed"

exit $EXIT_CODE
```

---

## Git Hooks統合

### .git-hooks/pre-commit

```bash
#!/bin/bash

echo "🔍 Running validation checks..."

# 変更されたマークダウンファイルがあればID検証
if git diff --cached --name-only | grep -q '\.md$'; then
  python3 scripts/validate-ids.py --path docs/
  [ $? -ne 0 ] && exit 1

  python3 scripts/validate-links.py --path docs/
  [ $? -ne 0 ] && exit 1
fi

# 変更されたコードファイルがあれば@spec検証
if git diff --cached --name-only | grep -qE '\.(cs|py|ts|js|gd)$'; then
  if [ -d "src/" ]; then
    python3 scripts/validate-impl-comments.py --code-path src/ --docs-path docs/
    [ $? -ne 0 ] && exit 1
  fi
fi

echo "✅ Validation passed"
```

---

## commands/docs-validate.md の更新

```markdown
# /docs-validate コマンド

## 概要

仕様書とコードの整合性を検証する。

## 実行内容

```bash
bash scripts/validate-all.sh
```

以下を順次実行：
1. ID重複・命名規則チェック
2. リンク切れチェック
3. 実装コメントチェック（src/ が存在する場合）

## 使用タイミング

- コミット前（自動: Git hook）
- PR作成前（手動）
- 定期的な整合性確認（週次等）

## エラーの対応

### ID重複エラー

```
[E001] Duplicate ID: REQ-30101-001
```

→ 一方のIDを変更（連番を付与）

### リンク切れエラー

```
[1] REQ-30101-002 not found
```

→ 仕様書でIDを検索し、正しいIDに修正

### @spec コメント欠落

```
[E002] Missing @spec: src/Player/Jump.cs
```

→ 対応する仕様IDを調べて `@spec` コメント追加
```
