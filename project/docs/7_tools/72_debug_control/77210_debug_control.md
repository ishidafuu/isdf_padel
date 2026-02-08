# 77210: Debug Control

## 概要

ゲーム画面とは別プロセスで、デバッグ状態とゲーム調整パラメータを切り替えるためのCLIツール。

- 実行中上書き: `assets/config/debug_runtime.ron`
- 起動時環境変数プロファイル: `assets/config/debug_env.ron`
- GUI項目定義: `assets/config/debug_fields.ron`

## 目的

- `game_config.ron` を直接編集しなくても、デバッグ設定を切り替えられるようにする
- 起動時環境変数と実行中調整の運用を分離する
- ゲーム本体とは別窓（別アプリ）で運用できるようにする

## 要求

- **REQ-77210-001**  
  `debug_control` は実行中上書き設定ファイル（`debug_runtime.ron`）を読み書きできること。

- **REQ-77210-002**  
  ゲーム起動時に環境変数からデバッグ上書きを読み取り、`GameConfig` に適用できること。

- **REQ-77210-003**  
  `debug_control` は起動時環境変数プロファイル（`debug_env.ron`）を管理できること。

- **REQ-77210-004**  
  実効設定は `base game_config` + `startup env overrides` + `runtime overrides` の順に合成されること。

- **REQ-77210-005**  
  GUIの調整項目は `debug_fields.ron` から読み込み、コードのハードコーディングなしで表示順・表示名・範囲を変更できること。

## CLI

```bash
# 状態確認
cargo run --bin debug_control -- status

# 実行中上書きの更新
cargo run --bin debug_control -- runtime --practice-infinite-mode false --player-move-speed 4.0

# 起動時環境変数プロファイル更新
cargo run --bin debug_control -- env --set RUST_LOG=info --set PADEL_PRACTICE_INFINITE_MODE=0

# 設定済み環境変数でゲーム起動
cargo run --bin debug_control -- launch

# 別窓エディタで設定ファイルを開く
cargo run --bin debug_control -- open-editor --target runtime
```

## GUI

```bash
# 別ウィンドウのデバッグ制御GUIを起動
cargo run --bin debug_control_gui
```

GUIでは以下を操作できる。

- `debug_runtime.ron` のトグル/数値調整
- `debug_env.ron` の環境変数編集
- 実効値プレビュー（base + startup env + runtime）
- 環境変数プロファイル付きでゲーム起動
- `Cmd+S`（Windows/Linuxは `Ctrl+S`）で runtime/env をまとめて保存
- `debug_fields.ron` を編集して表示項目を追加/削除/並び替え

## ゲーム側の反映

- `game_config.ron` のホットリロードは従来どおり有効
- `debug_runtime.ron` 更新時は次フレーム以降に再合成して反映
- `debug_runtime.ron` が削除された場合は runtime 上書きを無効化
