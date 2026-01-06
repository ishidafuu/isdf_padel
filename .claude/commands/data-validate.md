---
description: データと実装の整合性を検証
argument-hint: <data-file-path>
---

# /data-validate コマンド

8_data/ のMarkdownテーブルと実装コードの整合性を検証します。

**引数**: $ARGUMENTS（データファイルのパス）

## 使用者

**🤖 エージェント専用コマンド** - 人間は直接使わない

### 使用エージェント

| エージェント | 使用タイミング | 目的 |
|------------|--------------|------|
| data-agent | テーブル更新後（必須） | データ整合性の自動検証 |
| review-agent | データ変更後（推奨） | データとコードの整合性確認 |

**自動実行**: エージェントがデータテーブル変更時に自動的に実行

## 使用方法

```
/data-validate docs/8_data/801_tables/80101_enemy_params.md
/data-validate docs/8_data/**/*.md
```

## 目的

**手動データ転写の安全性を確保**

- Markdownテーブルと実装コードの値が一致しているか検証
- @data コメントの参照先が存在するか確認
- データの欠落を検出

## 前提：手動データ転写

このフレームワークでは、データは **手動で転写** します：

```markdown
<!-- docs/8_data/801_tables/80101_enemy_params.md -->
| ID | Name | HP | Attack | Speed |
|----|------|----|----|-------|
| enemy_slime | Slime | 10 | 2 | 1.0 |
| enemy_goblin | Goblin | 25 | 5 | 1.5 |
```

↓ 手動転写

```csharp
// @data 80101_enemy_params.md#enemy_slime
public static readonly EnemyParam Slime = new(
    hp: 10,
    attack: 2,
    speed: 1.0f
);

// @data 80101_enemy_params.md#enemy_goblin
public static readonly EnemyParam Goblin = new(
    hp: 25,
    attack: 5,
    speed: 1.5f
);
```

## 検証項目

### 1. @data コメントの存在確認

データを使う実装に `@data` コメントが付いているか。

```csharp
// ✅ 良い例
// @data 80101_enemy_params.md#enemy_slime
public static readonly EnemyParam Slime = ...

// ❌ 悪い例（@data コメントなし）
public static readonly EnemyParam Slime = ...
```

### 2. 参照先の存在確認

`@data` コメントで参照しているファイル・行が存在するか。

```
// @data 80101_enemy_params.md#enemy_slime
     ↓
docs/8_data/801_tables/80101_enemy_params.md を検索
     ↓
| enemy_slime | ... | が存在するか確認
```

### 3. 値の整合性チェック

Markdownテーブルと実装コードの値が一致しているか。

```markdown
<!-- Markdown -->
| enemy_slime | Slime | 10 | 2 | 1.0 |
```

```csharp
// コード
hp: 10,     // ✅ 一致
attack: 2,  // ✅ 一致
speed: 1.0f // ✅ 一致
```

### 4. 逆方向チェック

Markdownテーブルのすべての行に対応する実装があるか。

```
80101_enemy_params.md に enemy_bat がある
     ↓
src/ 内に @data 80101_enemy_params.md#enemy_bat があるか検索
     ↓
なければ「未実装」として警告
```

## 指示

引数として渡されたファイルパス（`$ARGUMENTS`）を検証対象とします。

### Step 1: Markdownテーブルを抽出

データファイルから Markdown テーブルを抽出：

```markdown
| ID | Name | HP | Attack | Speed |
|----|------|----|----|-------|
| enemy_slime | Slime | 10 | 2 | 1.0 |
| enemy_goblin | Goblin | 25 | 5 | 1.5 |
```

↓ パース

```
Row 1:
  ID: enemy_slime
  Name: Slime
  HP: 10
  Attack: 2
  Speed: 1.0

Row 2:
  ID: enemy_goblin
  Name: Goblin
  HP: 25
  Attack: 5
  Speed: 1.5
```

### Step 2: 実装ファイルを検索

@data コメントで参照している実装を検索：

```bash
grep -r "@data 80101_enemy_params.md#enemy_slime" src/
```

### Step 3: 値を比較

実装コードから値を抽出して比較：

```csharp
// @data 80101_enemy_params.md#enemy_slime
public static readonly EnemyParam Slime = new(
    hp: 10,      // ← Markdown: 10 → ✅ 一致
    attack: 2,   // ← Markdown: 2 → ✅ 一致
    speed: 1.0f  // ← Markdown: 1.0 → ✅ 一致
);
```

### Step 4: 結果を出力

```
=== Data Validation ===

File: docs/8_data/801_tables/80101_enemy_params.md

[PASS] enemy_slime
  ✅ Implementation found: src/Data/EnemyParams.cs:12
  ✅ HP: 10 → 10 (match)
  ✅ Attack: 2 → 2 (match)
  ✅ Speed: 1.0 → 1.0f (match)

[FAIL] enemy_goblin
  ✅ Implementation found: src/Data/EnemyParams.cs:20
  ❌ HP: 25 → 20 (mismatch)
  ✅ Attack: 5 → 5 (match)
  ✅ Speed: 1.5 → 1.5f (match)

  Markdown value: 25
  Code value: 20
  Last modified: Markdown (2025-12-19), Code (2025-12-20)
  → Code is newer, was this intentional?

[WARN] enemy_bat
  ⚠️  No implementation found with @data 80101_enemy_params.md#enemy_bat
  → This data is not used in code

=== Summary ===
PASS: 1
FAIL: 1
WARN: 1

Total rows: 3
Implemented: 2/3 (66%)
```

## オプション

| オプション | 説明 |
|-----------|------|
| `--strict` | WARN も FAIL 扱いにする |
| `--auto-fix` | Markdownの値でコードを自動修正 |
| `--format=json` | JSON形式で出力 |
| `--check-coverage` | 未実装データも検出 |

## 使用例

### 基本的な検証

```
/data-validate docs/8_data/801_tables/80101_enemy_params.md
```

### 自動修正

```
/data-validate docs/8_data/801_tables/80101_enemy_params.md --auto-fix

Fixing mismatches...

enemy_goblin.hp: 20 → 25
  File: src/Data/EnemyParams.cs:21
  Change: hp: 20 → hp: 25

Apply changes? [y/n]: y

✅ Fixed 1 mismatch
```

### カバレッジチェック

```
/data-validate docs/8_data/**/*.md --check-coverage

=== Coverage Report ===

80101_enemy_params.md:
  Implemented: 2/3 (66%)
  Missing: enemy_bat

80102_item_params.md:
  Implemented: 5/5 (100%)

Total: 7/8 (87%)
```

## エラーメッセージ

### E001: 値の不一致

```
ERROR E001: Value mismatch
Data file: docs/8_data/801_tables/80101_enemy_params.md
Row: enemy_goblin
Field: HP
Markdown: 25
Code: 20
Location: src/Data/EnemyParams.cs:21

The values do not match.
This indicates:
- Manual transcription error
- Intentional code modification (balance adjustment)
- Markdown not updated after code change

Action: Update either Markdown or code to match.
```

### E002: @data コメントなし

```
ERROR E002: Missing @data comment
File: src/Data/EnemyParams.cs
Line: 12
Variable: Slime

Data constants must have @data comments.
Add: // @data 80101_enemy_params.md#enemy_slime
```

### E003: 参照先が存在しない

```
ERROR E003: Referenced data not found
File: src/Data/EnemyParams.cs
Line: 12
Comment: @data 80101_enemy_params.md#enemy_dragon

The referenced data row does not exist in:
  docs/8_data/801_tables/80101_enemy_params.md

Check:
- Row ID is correct
- Data file path is correct
- Data file exists
```

## CI/CD 統合

### GitHub Actions での使用

```yaml
name: Data Validation

on:
  pull_request:
    paths:
      - 'docs/8_data/**/*.md'
      - 'src/Data/**'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Validate data integrity
        run: |
          claude-code /data-validate docs/8_data/**/*.md --strict
```

### Pre-commit hook での使用

```bash
#!/bin/bash

# データファイルまたは実装が変更された場合
if git diff --cached --name-only | grep -E '8_data/.*\.md|src/Data/'; then
  echo "Validating data integrity..."
  claude-code /data-validate docs/8_data/**/*.md
  if [ $? -ne 0 ]; then
    echo "❌ Data validation failed"
    exit 1
  fi
fi
```

## データ構造のルール

### Markdownテーブルの形式

```markdown
| ID | Field1 | Field2 | ... |
|----|--------|--------|-----|
| row_id | value1 | value2 | ... |
```

**ルール**:
- 1列目は必ず ID（一意な識別子）
- 列名は実装のフィールド名と対応
- 値は実装可能な型（数値、文字列、真偽値）

### @data コメントの形式

```
// @data <file>#<row-id>
```

**例**:
```csharp
// @data 80101_enemy_params.md#enemy_slime
```

**ルール**:
- ファイル名は 5桁番号から始まる
- `#` の後は Markdown テーブルの ID 列の値
- 実装の直前に配置

## データ更新のワークフロー

### パターンA: データ主導（Markdown → Code）

```
1. Markdownテーブルを更新
2. /data-validate でチェック
3. FAIL が出る
4. コードを手動で更新
5. /data-validate でチェック
6. PASS になったらコミット
```

### パターンB: バランス調整（Code → Markdown）

```
1. ゲームプレイで調整が必要と判明
2. コードを直接修正（素早く調整）
3. /data-validate でチェック
4. FAIL が出る
5. Markdownを更新（記録を同期）
6. /data-validate でチェック
7. PASS になったらコミット
```

### パターンC: 自動修正

```
1. Markdownテーブルを更新
2. /data-validate --auto-fix
3. コードが自動的に更新される
4. レビューして問題なければコミット
```

## ベストプラクティス

### 1. データ更新前に検証

```bash
# 現在の状態を確認
/data-validate docs/8_data/**/*.md

# すべて PASS であることを確認してから更新
```

### 2. 更新後すぐに検証

```bash
# Markdown更新
vim docs/8_data/801_tables/80101_enemy_params.md

# 即座に検証
/data-validate docs/8_data/801_tables/80101_enemy_params.md
```

### 3. PR作成前に全チェック

```bash
/data-validate docs/8_data/**/*.md --check-coverage
```

## トラブルシューティング

### Q: 値は一致しているのに FAIL になる

**A**: 型の違いを確認してください。

```
Markdown: 1.0
Code: 1.0f  # float型

→ 型まで含めて一致している必要があります
```

### Q: すべて FAIL になる

**A**: @data コメントの形式を確認してください。

```csharp
// ❌ 間違い
// @data enemy_slime

// ✅ 正しい
// @data 80101_enemy_params.md#enemy_slime
```

### Q: 自動修正が失敗する

**A**: コードの構造が複雑すぎる可能性があります。

手動で修正してください。

## 将来の拡張

### Phase 2: 自動生成

現在は手動転写ですが、将来的に自動生成も検討：

```bash
/data-generate docs/8_data/801_tables/80101_enemy_params.md --lang=csharp

Generated: src/Data/EnemyParams.cs (auto-generated, do not edit)
```

### Phase 3: 双方向同期

コード変更を Markdown に自動反映：

```bash
/data-sync src/Data/EnemyParams.cs

Synced to: docs/8_data/801_tables/80101_enemy_params.md
```

## 関連コマンド

- `/impl-validate` - 実装の検証
- `/ears-validate` - EARS記法の検証
- `/docs-validate` - 仕様書全体の整合性チェック

## 設計意図

このコマンドは、**手動データ転写の安全性を確保**するためのものです。

### なぜ手動転写なのか

1. **柔軟性**
   - ゲームバランス調整時にコードを直接変更できる
   - Markdownは「記録」として機能

2. **シンプル**
   - 自動生成システムは複雑
   - 手動転写 + 検証の方がシンプル

3. **段階的移行**
   - 将来的に自動生成に移行可能
   - まずは手動で始める

### 使用タイミング

- **データ更新後**: 必ず検証
- **PR作成前**: 全データを検証
- **CI/CD**: 自動実行

---

**重要**: データとコードの不一致は、ゲームバランスの崩壊に直結します。
このコマンドで定期的に検証してください。
