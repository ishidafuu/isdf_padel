---
description: EARS記法の正確性を検証
argument-hint: <spec-file-path>
---

# /ears-validate コマンド

仕様書のEARS記法を検証し、曖昧な表現を検出します。

**引数**: $ARGUMENTS（仕様書ファイルのパス）

## 使用者

**🤖 エージェント専用コマンド** - 人間は直接使わない

### 使用エージェント

| エージェント | 使用タイミング | 目的 |
|------------|--------------|------|
| critic-agent | spec.md批評時（必須） | 曖昧な表現の自動検出 |

**自動実行**: critic-agent が spec.md を批評する際に自動的に実行

## 使用方法

```
/ears-validate docs/3_ingame/301_player/30101_player_spec.md
/ears-validate docs/**/*_spec.md
```

## 目的

**曖昧な仕様書を書かせないためのガードレール**

- EARS記法のパターンに従っているか検証
- 曖昧な表現（「すぐに」「適切に」「うまく」）を検出
- 測定可能な基準が含まれているか確認

## EARS記法について

EARS記法の詳細は **skills/ears.md** を参照してください。

5つのパターン:
- **Ubiquitous**: THE SYSTEM SHALL [動作]
- **Event-driven**: WHEN [イベント], THE SYSTEM SHALL [動作]
- **State-driven**: WHILE [状態], THE SYSTEM SHALL [動作]
- **Optional**: WHERE [条件], THE SYSTEM SHALL [動作]
- **Unwanted**: IF [望ましくない状況], THEN THE SYSTEM SHALL [対応]

## 検証項目

### 1. EARS パターンへの準拠

各要件（REQ-xxxxx-xxx）がいずれかのEARSパターンに従っているか。

```
### REQ-30101-001: ジャンプ開始
WHEN player presses jump button
AND player is grounded
THE SYSTEM SHALL apply upward velocity 12m/s
```

**チェックポイント**:
- `WHEN`, `WHILE`, `IF`, `WHERE`, `THE SYSTEM SHALL` のいずれかを含む
- パターンの構造が正しい

### 2. 曖昧な表現の検出

以下の曖昧な表現を含んでいないか。

| 曖昧な表現 | 問題 | 代替案 |
|----------|------|--------|
| 「すぐに」 | 時間が不明確 | 「同フレーム内に」「0.1秒以内に」 |
| 「適切に」 | 基準が不明確 | 具体的な数値・条件を記述 |
| 「うまく」 | 評価基準なし | 測定可能な基準を記述 |
| 「なるべく」 | 曖昧な程度 | 「90%以上」「10回中9回」 |
| 「できるだけ」 | 努力目標 | 必須要件として明確化 |
| 「基本的に」 | 例外が不明 | 例外ケースを明示 |

### 3. 測定可能な基準の確認

数値・時間・条件が具体的に記述されているか。

```
✅ 良い例（測定可能）
THE SYSTEM SHALL apply upward velocity 12m/s
THE SYSTEM SHALL complete within 0.1s
THE SYSTEM SHALL succeed 95% of the time

❌ 悪い例（測定不可能）
THE SYSTEM SHALL apply appropriate velocity
THE SYSTEM SHALL complete quickly
THE SYSTEM SHALL succeed most of the time
```

### 4. AND/OR の明確性

複数条件の関係が明確か。

```
✅ 良い例
WHEN player presses jump button
AND player is grounded
AND player has stamina > 0

❌ 悪い例（関係が不明確）
WHEN player presses jump button, is grounded, has stamina
```

## 指示

引数として渡されたファイルパス（`$ARGUMENTS`）を検証対象とします。

### Step 1: 要件を抽出

1. `### REQ-xxxxx-xxx:` で始まる見出しをすべて抽出
2. 各要件の本文を取得

### Step 2: EARSパターンを検証

各要件について：

1. EARS キーワードを検出（WHEN, WHILE, IF, WHERE, THE SYSTEM SHALL）
2. パターンに合致するか確認
3. 合致しない場合は警告

### Step 3: 曖昧な表現を検出

各要件について、以下の表現を検索：

- 「すぐに」「速やかに」「迅速に」
- 「適切に」「妥当な」「十分な」
- 「うまく」「よく」「正しく」
- 「なるべく」「できるだけ」「可能な限り」
- 「基本的に」「通常」「一般的に」

### Step 4: 測定可能性を確認

以下が含まれているか：
- 数値（12m/s, 0.1s, 95%）
- 時間単位（秒、フレーム）
- 比較演算子（>, <, >=, <=, ==）
- 具体的な条件（is grounded, has stamina）

### Step 5: 結果を出力

```
=== EARS Validation ===

File: docs/3_ingame/301_player/30101_player_spec.md

[PASS] REQ-30101-001: ジャンプ開始
  ✅ Pattern: Event-driven (WHEN ... THE SYSTEM SHALL)
  ✅ Measurable: "12m/s" found
  ✅ No ambiguous terms

[WARN] REQ-30101-005: 着地処理
  ⚠️  Ambiguous term: "すぐに"
  Suggestion: Replace with "同フレーム内に" or "0.1秒以内に"
  Location: Line 67

[FAIL] REQ-30101-007: 壁判定
  ❌ Does not match any EARS pattern
  Current: "プレイヤーは壁にぶつからない"
  Suggested: "WHEN player collides with wall, THE SYSTEM SHALL stop movement"
  Location: Line 89

=== Summary ===
PASS: 10
WARN: 3
FAIL: 2

Total requirements: 15
EARS compliance: 80%
```

## オプション

| オプション | 説明 |
|-----------|------|
| `--strict` | WARN も FAIL 扱いにする |
| `--suggest` | 修正案を自動生成 |
| `--format=json` | JSON形式で出力 |
| `--lang=ja` | 日本語の曖昧表現も検出 |

## 使用例

### 基本的な検証

```
/ears-validate docs/3_ingame/301_player/30101_player_spec.md
```

### 厳格モード

```
/ears-validate docs/**/*_spec.md --strict
```

### 修正案付き

```
/ears-validate docs/3_ingame/301_player/30101_player_spec.md --suggest
```

出力例:
```
[FAIL] REQ-30101-007: 壁判定
  ❌ Does not match any EARS pattern
  Current: "プレイヤーは壁にぶつからない"

  Suggested rewrite:

  ### REQ-30101-007: 壁判定
  WHEN player collides with wall
  THE SYSTEM SHALL stop movement
  AND THE SYSTEM SHALL set velocity to zero
```

## CI/CD 統合

### spec-agent での自動実行

spec-agent が仕様書を作成したら、自動的に実行：

```
spec-agent: 仕様書を作成しました
  ↓
/ears-validate (自動実行)
  ↓
FAIL があれば修正を促す
```

### GitHub Actions での使用

```yaml
name: EARS Validation

on:
  pull_request:
    paths:
      - 'docs/**/*_spec.md'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Validate EARS notation
        run: |
          for file in $(git diff --name-only origin/main | grep '_spec.md$'); do
            claude-code /ears-validate "$file" --strict
          done
```

## エラーメッセージ

### E001: EARS パターン不一致

```
ERROR E001: Does not match any EARS pattern
File: docs/3_ingame/301_player/30101_player_spec.md
Line: 45
Requirement: REQ-30101-003

Current text: "ジャンプ中に攻撃できる"

This requirement does not follow any EARS pattern.
Use one of: WHEN/WHILE/IF/WHERE/THE SYSTEM SHALL
```

### E002: 曖昧な表現

```
ERROR E002: Ambiguous term detected
File: docs/3_ingame/301_player/30101_player_spec.md
Line: 67
Requirement: REQ-30101-005
Term: "すぐに"

Replace with measurable criteria:
- "同フレーム内に"
- "0.1秒以内に"
- "次のUpdate()で"
```

### E003: 測定不可能

```
ERROR E003: Not measurable
File: docs/3_ingame/301_player/30101_player_spec.md
Line: 89
Requirement: REQ-30101-007

Current: "THE SYSTEM SHALL respond quickly"

Add measurable criteria:
- Specific time (e.g., "within 0.1s")
- Specific count (e.g., "within 3 frames")
- Specific percentage (e.g., "95% of the time")
```

## 終了コード

| コード | 意味 |
|--------|------|
| 0 | すべての検証に合格 |
| 1 | WARN または FAIL あり |
| 2 | 引数エラー |

## critic-agent との連携

このコマンドは critic-agent に統合されています。

```
critic-agent: 仕様書を批評します
  ↓
内部で /ears-validate を実行
  ↓
EARS記法の問題も含めて報告
```

**使い分け**:
- **critic-agent**: 仕様の内容を批評（矛盾、漏れ、曖昧さ）
- **/ears-validate**: EARS記法の形式を検証（パターン、表現、測定可能性）

## 日本語対応

`--lang=ja` オプションで日本語の曖昧表現も検出：

```
曖昧な表現（日本語）:
- 「すぐに」「速やかに」「迅速に」「直ちに」
- 「適切に」「妥当な」「十分な」「必要な」
- 「うまく」「よく」「正しく」「きちんと」
- 「なるべく」「できるだけ」「可能な限り」「極力」
- 「基本的に」「通常」「一般的に」「原則として」
```

## 注意事項

### 検証対象

- `*_spec.md` ファイルのみ
- `### REQ-xxxxx-xxx:` で始まる要件
- 本文（前文やメタデータは除外）

### 除外対象

- `_deprecated/` 内のファイル
- Game Feel Requirements セクション（感覚的なので曖昧OK）
- Extensibility Requirements セクション
- コメント（`<!-- ... -->`）

### Game Feel との違い

**Functional Requirements（EARS記法）**:
- 厳密な記述が必要
- 測定可能でなければならない
- このコマンドで検証される

**Game Feel Requirements**:
- 感覚的な記述OK
- 曖昧な表現も許容
- このコマンドでは検証されない

## トラブルシューティング

### Q: Game Feel セクションもエラーになる

**A**: Game Feel は検証対象外です。セクション名を確認してください。

```markdown
✅ 検証されない（正しい）
## 2. Game Feel Requirements

❌ 検証される（間違い）
## 2. Requirements
```

### Q: 日本語の要件がすべて FAIL になる

**A**: EARS キーワードは英語です。

```markdown
❌ 間違い
プレイヤーがジャンプボタンを押したとき
システムは上向きの速度を適用する

✅ 正しい
WHEN player presses jump button
THE SYSTEM SHALL apply upward velocity 12m/s
```

### Q: 数値を含んでいるのに測定不可能と言われる

**A**: 単位や文脈を確認してください。

```markdown
❌ 測定不可能
THE SYSTEM SHALL use value 10

✅ 測定可能
THE SYSTEM SHALL apply velocity 10m/s
THE SYSTEM SHALL complete in 10 seconds
THE SYSTEM SHALL spawn 10 enemies
```

## 設計意図

このコマンドは、**AIに曖昧な仕様書を書かせないためのガードレール**です。

### なぜ必要か

1. **AIは曖昧な表現を好む**
   - 「適切に」「うまく」「すぐに」を多用する
   - 自然言語としては流暢だが、実装には不十分

2. **曖昧さは実装時の迷走を招く**
   - 「すぐに」→ 0.1秒？ 1フレーム？ 即座？
   - 「適切に」→ 何が適切？

3. **EARS記法は実装可能性を保証する**
   - パターンに従えば、実装に迷わない
   - 測定可能な基準があれば、テストも書ける

### 使用タイミング

- **spec-agent が仕様書を作成した直後**: 即実行
- **critic-agent の一部として**: 自動実行
- **PR作成前**: GitHub Actions で自動実行

---

**重要**: このコマンドが FAIL を返した場合、**spec-agent に修正させてください。**
