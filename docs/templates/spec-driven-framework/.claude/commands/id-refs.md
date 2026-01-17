---
description: IDの参照箇所を検索
argument-hint: <ID>
---

# /id-refs コマンド

指定されたIDが参照されている箇所を全て検索します。

**引数**: $ARGUMENTS

## 使用者

**🔀 人間・エージェント共用コマンド**

### 人間の使用

IDの影響範囲を調べる際に使用（任意）

### エージェントの使用

| エージェント | 使用タイミング | 目的 |
|------------|--------------|------|
| review-agent | レビュー時（推奨） | 影響範囲の確認 |
| deps-agent | 依存追加時（推奨） | 依存元の影響範囲確認 |
| data-agent | データ削除前（推奨） | データ参照元の確認 |

## 使用方法

```
/id-refs REQ-30101-001
/id-refs DES-30102-001
/id-refs HealthComponent
```

## 実行例

### 要件IDの参照検索

```
ユーザー: /id-refs REQ-30101-001

Claude:
REQ-30101-001 の参照箇所:

1. docs/3_ingame/301_player/30103_player_behavior.md:23
   - 参照元: BHV-30103-001
   - コンテキスト: "REQ-30101-001 の条件を満たした場合..."

2. docs/3_ingame/301_player/30105_player_test.md:45
   - 参照元: TST-30105-001
   - コンテキスト: "TST-30105-001 は REQ-30101-001 を検証する"

3. src/Player/PlayerJumpSystem.cs:12
   - コメント: @spec REQ-30101-001
   - コンテキスト: 実装コード内

合計: 3件
```

### Component名の参照検索

```
ユーザー: /id-refs HealthComponent

Claude:
HealthComponent の参照箇所:

1. docs/3_ingame/302_enemy/30202_enemy_design.md:34
   - コンテキスト: "EnemyEntity は HealthComponent を持つ"

2. docs/3_ingame/301_player/30102_player_design.md:56
   - コンテキスト: "PlayerEntity は HealthComponent を持つ"

3. docs/2_architecture/209_components/20901_shared_components.md:12
   - 定義箇所（除外）

合計: 2件（定義箇所除外）
```

### 参照がない場合

```
ユーザー: /id-refs REQ-30101-099

Claude:
REQ-30101-099 の参照箇所:

（参照なし）

合計: 0件
```

## 指示

引数として渡された ID または名前（`$ARGUMENTS`）を検索対象とします。

1. `docs/` ディレクトリ内を検索
2. 指定されたIDまたは名前が **参照** されている箇所を特定
3. 定義箇所は除外し、参照のみを表示
4. 以下の形式で出力:

```
REQ-30101-001 の参照箇所:

1. docs/3_ingame/301_player/30103_player_behavior.md:23
   - 参照元: BHV-30103-001

2. docs/3_ingame/301_player/30104_player_test.md:45
   - 参照元: TST-30105-001

合計: 2件
```

## 検索対象

- 仕様書内の ID 参照
- `@spec REQ-xxx` コメント（実装コード内）
- `@test TST-xxx` コメント（実装コード内）
- Component名やファイル名の参照

## 注意事項

- `_deprecated/` 内はデフォルトで除外
- `--include-deprecated` オプションで含めることも可能
- 実装コードも検索対象に含める場合は `--include-code` オプション

## 終了コード

| コード | 意味 |
|--------|------|
| 0 | 正常終了（参照が 0 件でも正常） |
| 1 | 検索中にエラーが発生した |
