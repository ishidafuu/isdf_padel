---
description: Git履歴から変更サマリを生成
argument-hint: [<ファイル番号>] [--since <日付>]
---

# /docs-summary コマンド

Git履歴から仕様書の変更サマリを生成します。

**オプション**: $ARGUMENTS

## 使用者

**🧑 人間専用コマンド** - 仕様書の変更履歴を把握する際に使用

エージェントはこのコマンドを使用しません。

## 使用方法

```
/docs-summary                    # 全仕様書の最近の変更
/docs-summary 30101              # 特定ファイルの変更履歴
/docs-summary --since 2024-01-01 # 指定日以降の変更
```

## オプション解析

`$ARGUMENTS` から以下を解析：
- `<ファイル番号>`: 指定されたファイル番号の変更履歴のみ表示
- `--since <日付>`: 指定日以降の変更のみ表示
- 引数なし: 全仕様書の直近の変更を表示

## 指示

1. Git履歴から指定された範囲の変更を取得
2. 変更内容をサマリ形式で表示
3. 以下の形式で出力:

```
=== docs-summary ===

## 最近の変更 (直近10件)

### 2024-12-15
- 30101_player_spec.md
  - REQ-30101-005 を追加（二段ジャンプ）
  - REQ-30101-002 を修正（ジャンプ速度調整）

### 2024-12-14
- 30102_player_design.md
  - DES-30102-003 を追加（JumpStateComponent）

### 2024-12-13
- 30201_enemy_spec.md
  - 新規作成

=== Summary ===
変更ファイル数: 3
追加されたID: 2
修正されたID: 1
```

## Git コマンド

```bash
# 基本
git log --oneline --name-only -- docs/

# 特定ファイル
git log --oneline -p -- docs/3_ingame/301_player/30101_player_spec.md

# 日付指定
git log --since="2024-01-01" --oneline -- docs/
```

## 注意事項

- Git リポジトリが必要
- `_deprecated/` への移動も変更として表示

## 終了コード

| コード | 意味 |
|--------|------|
| 0 | 正常終了 |
| 1 | Git リポジトリでない、またはエラー発生 |
