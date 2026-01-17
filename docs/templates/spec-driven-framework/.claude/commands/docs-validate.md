---
description: 仕様書の整合性をチェック
argument-hint: [--all | --file <番号>] [--with-github]
---

# /docs-validate コマンド

仕様書の整合性を検証し、問題点を報告します。

**オプション**: $ARGUMENTS

## 使用者

**🤖 エージェント専用コマンド** - 人間は直接使わない

### 使用エージェント

| エージェント | 使用タイミング | 目的 |
|------------|--------------|------|
| review-agent | レビュー開始時（必須） | ID重複、参照エラーの検出 |
| spec-agent | spec.md作成後（推奨） | 作成内容の自動検証 |
| design-agent | design.md作成後（推奨） | 作成内容の自動検証 |
| behavior-agent | behavior.md作成後（推奨） | 作成内容の自動検証 |
| test-agent | test.md作成後（推奨） | 作成内容の自動検証 |
| critic-agent | 批評前（推奨） | 基本的な整合性確認 |

**自動実行**: エージェントが作成・検証時に自動的に実行

## 使用方法

```
/docs-validate              # 全ファイルをチェック
/docs-validate --file 30101 # 特定ファイルのみ
```

## オプション解析

`$ARGUMENTS` から以下を解析：
- `--file <番号>`: 指定されたファイル番号のみ検証
- `--include-deprecated`: _deprecated/ 内も検証対象に含む
- 引数なし: 全ファイルを検証

## 検証項目

### ID整合性
- [ ] REQ-ID の重複がないか
- [ ] DES-ID の重複がないか
- [ ] BHV-ID の重複がないか
- [ ] TST-ID の重複がないか

### 参照整合性
- [ ] TST が参照する REQ が存在するか
- [ ] spec.md の「テスト: TST-xxx」が test.md に存在するか
- [ ] design.md の共有Component参照リンクが有効か
- [ ] 全ての REQ に対応する TST が存在するか（未テスト要件の検出）

### データ整合性
- [ ] テーブル間参照のIDが参照先に存在するか

### ファイル構造
- [ ] 必須ファイル（overview, spec, design, behavior, test）が揃っているか

### 番号体系
- [ ] ファイル名の番号がフォルダ番号から正しく算出されているか
- [ ] ファイル番号がフォルダ番号と一致するか

### 廃止マーカー
- [ ] DEPRECATED マーカーが付いたファイルの検出・一覧表示

### 実装コメント検証（コードが存在する場合）
- [ ] 実装コードに @spec コメントが存在するか
- [ ] @spec が参照する REQ-ID が仕様書に存在するか
- [ ] @test が参照する TST-ID が仕様書に存在するか
- [ ] @data が参照するデータ定義が 8_data/ に存在するか

### GitHub連携（--with-github オプション）
- [ ] spec.md の Issue 番号が実在するか

## 出力形式

```
=== Validation Report ===

[PASS] REQ-ID 重複チェック
[PASS] DES-ID 重複チェック
[PASS] BHV-ID 重複チェック
[PASS] TST-ID 重複チェック
[FAIL] TST-30105-003 が参照する REQ-30101-005 が存在しません
[WARN] 30102_player_design.md: 共有Component参照が相対パスではありません
[WARN] DEPRECATED: 399_old_feature/ (v0.5 で廃止予定)
[WARN] REQ-30101-003 に対応する TST が存在しません
[FAIL] テーブル参照エラー: enemy_goblin.SkillID (skill_slash) が 80102_skills.md に存在しません

=== Summary ===
PASS: 10, FAIL: 2, WARN: 3
```

## 注意事項

- デフォルトではローカルファイルのみ検証
- `--with-github` を指定すると GitHub API を使用して Issue の存在確認を行う
- `_deprecated/` 内はデフォルトで除外（`--include-deprecated` で含める）

## 終了コード

| コード | 意味 |
|--------|------|
| 0 | 全検証が PASS（WARN のみは 0） |
| 1 | FAIL が 1 件以上存在 |
