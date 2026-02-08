# Codex運用ガイド（isdf_padel）

このリポジトリでCodexを使う際の最小ガイドです。

## 基本方針

- 対話は日本語で行う
- コミットメッセージは日本語で書く
- 仕様書駆動（`project/docs/`）を優先し、実装だけ先行しない

## 作業ディレクトリ

- ゲーム本体: `project/`
- 仕様書: `project/docs/`
- タスク管理: `project/tasks/`

## 開発コマンド（`project/`で実行）

- テスト: `cargo test`
- ゲーム起動: `cargo run`
- ヘッドレスシミュレーション（debug）: `cargo run --bin headless_sim -- -c debug`
- ヘッドレスシミュレーション（stress）: `cargo run --bin headless_sim -- -c stress`
- ナラティブ生成: `cargo run --bin trace_narrator -- debug_trace.jsonl`

## 注意点（ハマりやすい点）

- `headless_sim` の `--config` は「設定名」を受け取る
  - 例: `-c debug` -> `assets/config/simulation_debug.ron`
  - ファイルパスを直接渡す実装にはなっていない
- QA時は `trace.shot_attributes` の有効化有無を確認する
  - 無効だと `trace_narrator` のショット統計が0になる

## 実装運用

- 既知バグは `project/tasks/0_backlog/` を起点に `1_todo` へ昇格して着手する
- 変更時は以下をセットで確認する
  - 仕様書との整合（対象 `REQ`）
  - テスト追加/更新
  - `cargo test`
  - 必要に応じて `headless_sim` で回帰確認

## Git運用

- ブランチは `codex/` プレフィックスを使う
- 破壊的なGit操作（`reset --hard` など）は行わない
- 基本運用として、対応が完了したらその都度コミットする
- 並列セッションでの作業を考慮し、自分の対応範囲外のファイルを勝手に削除・改変しない
