---
description: ゲームを起動
argument-hint: [--release]
---

# /run-game コマンド

Padel Game を起動します。

**引数**: $ARGUMENTS

## 使用者

**👤 人間向けコマンド** - ゲームの動作確認に使用

## 使用方法

```
/run-game              # 開発ビルドで起動
/run-game --release    # リリースビルドで起動
```

## 指示

1. `project/` ディレクトリで `cargo run` を実行
2. 引数に `--release` が含まれる場合は `cargo run --release` を実行
3. ビルドエラーがあれば報告

### 実行コマンド

```bash
# 開発ビルド（デフォルト）
source ~/.cargo/env && cd /Users/s13219/repo/spec-driven-framework/project && cargo run

# リリースビルド（--release指定時）
source ~/.cargo/env && cd /Users/s13219/repo/spec-driven-framework/project && cargo run --release
```

## 注意事項

- 初回ビルドは依存クレートのコンパイルで数分かかる
- ウィンドウが表示されればゲームは正常に起動している
- `Ctrl+C` または `Cmd+Q` で終了
