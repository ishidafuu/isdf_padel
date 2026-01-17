---
description: ファイル内の全IDを一覧表示
argument-hint: <ファイル番号>
---

# /id-list コマンド

指定されたファイル番号に対応するファイル内の全IDを一覧表示します。

**引数**: $ARGUMENTS

## 使用者

**🤖 エージェント専用コマンド** - 人間は直接使わない

### 使用エージェント

| エージェント | 使用タイミング | 目的 |
|------------|--------------|------|
| spec-agent | ID確認時（推奨） | 既存IDの確認 |
| design-agent | ID確認時（推奨） | 既存IDの確認 |
| behavior-agent | ID確認時（推奨） | 既存IDの確認 |
| test-agent | ID確認時（推奨） | 既存IDの確認 |

**自動実行**: エージェントがID確認時に自動的に実行

## 使用方法

```
/id-list 30101      # 30101_player_spec.md 内の全ID
/id-list 30102      # 30102_player_design.md 内の全ID
```

## 実行例

### 仕様書のID一覧

```
ユーザー: /id-list 30101

Claude:
ファイル: docs/3_ingame/301_player/30101_player_spec.md

REQ-30101-001: ジャンプ開始条件
REQ-30101-002: ジャンプ中の移動
REQ-30101-003: 着地判定
REQ-30101-004: 二段ジャンプ

合計: 4件
```

### 設計書のID一覧

```
ユーザー: /id-list 30102

Claude:
ファイル: docs/3_ingame/301_player/30102_player_design.md

DES-30102-001: PlayerEntity
DES-30102-002: MovementComponent
DES-30102-003: JumpState
DES-30102-004: GroundCheckSystem

合計: 4件
```

### IDが存在しない場合

```
ユーザー: /id-list 30105

Claude:
ファイル: docs/3_ingame/301_player/30105_player_test.md

（IDなし）

合計: 0件
```

## 指示

引数として渡されたファイル番号（`$ARGUMENTS`）を対象とします。

1. 指定されたファイル番号に対応するファイルを特定
2. ファイル内の全ID（REQ-, DES-, BHV-, TST-）を抽出
3. 以下の形式で出力:

```
ファイル: docs/3_ingame/301_player/30101_player_spec.md

REQ-30101-001: ジャンプ開始条件
REQ-30101-002: ジャンプ中の移動
REQ-30101-003: 着地判定
REQ-30101-004: 二段ジャンプ

合計: 4件
```

## ファイル番号の特定

ファイル番号（5桁）からパスを特定するルール:

```
[A][BB][CC]
 │  │   └── ファイル種別（01=spec, 02=design, 03=behavior, 04=test, 00=overview）
 │  └────── サブフォルダ番号（00=親直下, 01-99=サブフォルダ）
 └───────── 親フォルダ番号（1-9）
```

※ spec.md では `[AAA][BB]` と表記（AAA = A × 100 + BB で「フォルダ番号」）

**変換例**:

| ファイル番号 | 親(A) | サブ(BB) | 種別(CC) | パス |
|-------------|-------|----------|----------|------|
| 30101 | 3 | 01 | 01 | `3_ingame/301_player/30101_*_spec.md` |
| 30102 | 3 | 01 | 02 | `3_ingame/301_player/30102_*_design.md` |
| 30000 | 3 | 00 | 00 | `3_ingame/30000_overview.md` |
| 20901 | 2 | 09 | 01 | `2_architecture/209_components/20901_*.md` |

## 注意事項

- `_deprecated/` 内はデフォルトで除外
- `--include-deprecated` オプションで含めることも可能
- ファイルが見つからない場合はエラーを表示

## 終了コード

| コード | 意味 |
|--------|------|
| 0 | 正常終了（ID が 0 件でも正常） |
| 1 | ファイルが見つからなかった |
