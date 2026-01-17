---
description: 参照リンク切れと禁止依存を検出
argument-hint: [--verbose]
---

# /deps-check コマンド

仕様書間の参照リンク切れと禁止されている依存関係を検出します。

**オプション**: $ARGUMENTS

## 使用者

**🤖 エージェント専用コマンド** - 人間は直接使わない

### 使用エージェント

| エージェント | 使用タイミング | 目的 |
|------------|--------------|------|
| deps-agent | dependencies.md更新後（必須） | 禁止依存・循環依存の検出 |
| review-agent | レビュー開始時（必須） | 依存関係の整合性確認 |

**自動実行**: エージェントが依存関係変更時に自動的に実行

## 使用方法

```
/deps-check            # 基本チェック
/deps-check --verbose  # 詳細表示
```

## オプション解析

`$ARGUMENTS` から以下を解析：
- `--verbose`: 詳細な検出結果を表示
- 引数なし: 基本的な検出結果のみ表示

## 検出項目

### リンク切れ検出
- Markdown リンク `[text](path)` の参照先が存在するか
- 相対パスの解決が正しいか

### 禁止依存の検出

以下の依存関係は禁止されています:

| 依存 | 理由 |
|------|------|
| Player ↔ Enemy | 直接参照禁止、EventSystem経由で疎結合化 |
| Stage → Player / Enemy | Stageはエンティティの存在を知らない |
| 3_ingame ↔ 4_outgame | 相互参照禁止 |
| 8_data → 他層 | データ層は参照される専用 |

## 出力形式

```
=== deps-check ===

## リンク切れ

[FAIL] docs/3_ingame/301_player/30102_player_design.md:45
       リンク切れ: ../209_components/20999_missing.md

[FAIL] docs/3_ingame/302_enemy/30201_enemy_spec.md:23
       リンク切れ: ../../8_data/801_tables/80199_missing.md

## 禁止依存

[FAIL] docs/3_ingame/301_player/30103_player_behavior.md:78
       禁止依存: 302_enemy への直接参照
       推奨: EventSystem 経由で疎結合化してください

[FAIL] docs/8_data/801_tables/80101_enemy_params.md:10
       禁止依存: 3_ingame への参照（データ層は参照される専用）

=== Summary ===
リンク切れ: 2件
禁止依存: 2件
```

## 依存ルール

```
1_project（参照されるのみ）
    ↑
2_architecture
    ↑
3_ingame / 4_outgame

────────────────────────────
8_data（独立層・全層から横断的に参照可・他層を参照しない）
```

## 注意事項

- `_deprecated/` 内はデフォルトで除外
- `--include-deprecated` オプションで含めることも可能
- 問題がある場合は終了コード 1 を返す
