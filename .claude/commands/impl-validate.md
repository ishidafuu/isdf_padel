---
description: 実装が仕様書に対応しているか検証
argument-hint: <file-path>
---

# /impl-validate コマンド

実装ファイルの `@spec` コメントを検証し、仕様書にない実装を検出します。

**引数**: $ARGUMENTS（実装ファイルのパス）

## 使用者

**🤖 エージェント専用コマンド** - 人間は直接使わない

### 使用エージェント

| エージェント | 使用タイミング | 目的 |
|------------|--------------|------|
| review-agent | 実装後（推奨） | @spec コメントの自動検証 |

**自動実行**: エージェントが実装レビュー時に自動的に実行

## 使用方法

```
/impl-validate src/Player/PlayerJumpSystem.cs
/impl-validate src/Enemy/EnemyAI.gd
/impl-validate src/**/*.cs
```

## 目的

**AIの暴走を防ぐための検証コマンド**

- 仕様書にない機能が実装されていないか検出
- `@spec` コメントが正しく仕様書を参照しているか確認
- 実装と仕様書の乖離を早期発見

## 実装コメント規約

`@spec` / `@data` コメントの詳細は **skills/impl-comments.md** を参照してください。

## 検証項目

### 1. @spec コメントの存在確認

すべての public class/function に `@spec` コメントが付いているか。

```csharp
// ✅ 良い例
// @spec REQ-30101-001
// @spec REQ-30101-002
public class PlayerJumpSystem : ISystem
{
    // ...
}

// ❌ 悪い例（@spec コメントなし）
public class PlayerJumpSystem : ISystem
{
    // ...
}
```

### 2. 参照先の存在確認

`@spec` コメントで参照している REQ-ID が実際に存在するか。

```
// @spec REQ-30101-001
     ↓
docs/3_ingame/301_player/30101_player_spec.md を検索
     ↓
### REQ-30101-001: ... が存在するか確認
```

### 3. 逆方向チェック（未実装要件の検出）

仕様書にある REQ-ID に対応する実装があるか。

```
30101_player_spec.md に REQ-30101-003 がある
     ↓
src/ 内に @spec REQ-30101-003 があるか検索
     ↓
なければ「未実装」として警告
```

## 指示

引数として渡されたファイルパス（`$ARGUMENTS`）を検証対象とします。

### Step 1: @spec コメントを抽出

1. 対象ファイルから `@spec REQ-xxxxx-xxx` 形式のコメントをすべて抽出
2. 各 REQ-ID をリスト化

### Step 2: 参照先の存在確認

各 REQ-ID について：

1. ID から 5桁番号（xxxxx）を抽出
2. `docs/` 内で `xxxxx_*_spec.md` を検索
3. そのファイル内に REQ-ID が存在するか確認

### Step 3: 結果を出力

```
=== Implementation Validation ===

Target: src/Player/PlayerJumpSystem.cs

[PASS] @spec REQ-30101-001
  ✅ Found in docs/3_ingame/301_player/30101_player_spec.md:45

[PASS] @spec REQ-30101-002
  ✅ Found in docs/3_ingame/301_player/30101_player_spec.md:67

[FAIL] @spec REQ-30101-099
  ❌ NOT FOUND in any spec file
  ⚠️  File modified at: 2025-12-19 10:30
  ⚠️  Spec last modified at: 2025-12-18 15:20
  → This may be an unauthorized implementation

=== Summary ===
PASS: 2
FAIL: 1

⚠️  WARNING: Unauthorized implementations detected
```

### Step 4: 未実装要件の検出（オプション）

`--check-coverage` フラグが指定された場合：

1. 対象ファイルが対応する spec.md を特定
2. spec.md 内のすべての REQ-ID を抽出
3. 各 REQ-ID に対応する `@spec` コメントが実装ファイル群に存在するか確認
4. 存在しない場合は「未実装」として警告

```
=== Coverage Check ===

Spec file: docs/3_ingame/301_player/30101_player_spec.md

[IMPLEMENTED] REQ-30101-001 → PlayerJumpSystem.cs
[IMPLEMENTED] REQ-30101-002 → PlayerJumpSystem.cs
[NOT IMPLEMENTED] REQ-30101-003
  ⚠️  No implementation found with @spec REQ-30101-003

=== Coverage Summary ===
Implemented: 2/3 (66%)
Not implemented: 1
```

## オプション

| オプション | 説明 |
|-----------|------|
| `--check-coverage` | 未実装要件も検出 |
| `--strict` | @spec コメントがない public class/function をエラー扱い |
| `--exclude-deprecated` | _deprecated/ 内を除外（デフォルト） |
| `--format=json` | JSON形式で出力 |

## 使用例

### 基本的な検証

```
/impl-validate src/Player/PlayerJumpSystem.cs
```

### カバレッジチェック付き

```
/impl-validate src/Player/PlayerJumpSystem.cs --check-coverage
```

### 厳格モード

```
/impl-validate src/**/*.cs --strict
```

### JSON出力（CI用）

```
/impl-validate src/**/*.cs --format=json > validation-report.json
```

## CI/CD 統合

### GitHub Actions での使用例

```yaml
name: Spec Validation

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Validate Implementation
        run: |
          claude-code /impl-validate src/**/*.cs --strict
```

### Pre-commit hook での使用

`.git/hooks/pre-commit` に追加：

```bash
#!/bin/bash

# 変更された実装ファイルを検証
for file in $(git diff --cached --name-only | grep -E '\.(cs|gd)$'); do
  claude-code /impl-validate "$file" --strict
  if [ $? -ne 0 ]; then
    echo "❌ Implementation validation failed for $file"
    exit 1
  fi
done
```

## エラーメッセージ

### E001: @spec コメントなし

```
ERROR E001: Missing @spec comment
File: src/Player/PlayerAttackSystem.cs
Line: 12
Class: PlayerAttackSystem

All public classes must have @spec comments linking to requirements.
```

### E002: 参照先が存在しない

```
ERROR E002: Referenced requirement not found
File: src/Player/PlayerJumpSystem.cs
Line: 5
Comment: @spec REQ-30101-099
Expected: docs/3_ingame/301_player/30101_player_spec.md

The referenced requirement does not exist in the spec file.
This may indicate:
- Unauthorized implementation (not in spec)
- Typo in REQ-ID
- Spec file was moved/deleted
```

### E003: 未実装要件

```
ERROR E003: Unimplemented requirement
Spec: docs/3_ingame/301_player/30101_player_spec.md
Requirement: REQ-30101-003

No implementation found with @spec REQ-30101-003
```

## 終了コード

| コード | 意味 |
|--------|------|
| 0 | すべての検証に合格 |
| 1 | 検証エラーあり |
| 2 | 引数エラー |

## 注意事項

### 検証対象

- C# ファイル（`.cs`）
- GDScript ファイル（`.gd`）
- Python ファイル（`.py`）
- TypeScript ファイル（`.ts`）

### 除外対象

- テストファイル（`*Test.cs`, `*_test.gd`）
- 自動生成ファイル（`.g.cs`）
- `_deprecated/` 内のファイル

### 制限事項

- private メソッドは検証対象外（publicのみ）
- インターフェース定義は検証対象外
- 抽象クラスは検証対象外（具象クラスのみ）

## 関連コマンド

- `/id` - ID の定義箇所を表示
- `/id-refs` - ID の参照箇所を検索
- `/docs-validate` - 仕様書全体の整合性チェック
- `/ears-validate` - EARS記法の検証

## トラブルシューティング

### Q: すべて FAIL になる

**A**: ファイルパスが間違っている可能性があります。

```bash
# ✅ 正しい
/impl-validate src/Player/PlayerJumpSystem.cs

# ❌ 間違い（相対パスが不正）
/impl-validate PlayerJumpSystem.cs
```

### Q: 誤検知される

**A**: REQ-ID の形式を確認してください。

```csharp
// ✅ 正しい形式
// @spec REQ-30101-001

// ❌ 間違い（ハイフンが違う）
// @spec REQ_30101_001

// ❌ 間違い（スペース）
// @spec REQ - 30101 - 001
```

### Q: 未実装として検出されるが実装済み

**A**: 複数ファイルに分散している可能性があります。

```bash
# すべての実装ファイルを検索
grep -r "@spec REQ-30101-003" src/
```

## 設計意図

このコマンドは、**AIの暴走を防ぐための最後の砦**です。

### なぜ必要か

1. **AIは仕様書を読み飛ばす**
   - 実装中に「これもあったほうが良い」と勝手に追加する

2. **人間のレビューは見逃す**
   - コードレビューで @spec コメントまで確認するのは困難

3. **仕様とコードの乖離は蓄積する**
   - 放置すると後から修正が困難

### 使用タイミング

- **実装完了時**: impl-agent が実装を完了したら即実行
- **PR作成前**: GitHub Actions で自動実行
- **コミット前**: pre-commit hook で自動実行

---

**重要**: このコマンドが FAIL を返した場合、**絶対にコミットしないでください。**
