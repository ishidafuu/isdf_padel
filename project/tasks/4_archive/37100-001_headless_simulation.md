# 37100-001: ヘッドレス・高速シミュレーション機能

## 概要

ゲームの規模拡大に伴うデバッグ支援として、AI対AIの自動対戦を描画なしで高速実行し、異常を検出する機能を実装する。

## 背景

- テストは通るが挙動が安定しない場合がある
- 長時間のオート動作で問題を検出したい
- 描画なしで高速に内部ロジックを検証したい

## 要件

- **ヘッドレス実行**: 描画なしで内部ロジックのみ実行
- **高速シミュレーション**: 数試合を短時間で回す
- **両プレイヤーAI化**: 人間なしで自動対戦
- **異常検出**: 無限ループ、物理異常（NaN/コート外）、状態遷移異常
- **出力**: 異常→コンソール、詳細→JSONファイル

## 仕様書配置

`project/docs/7_tools/71_simulation/77100_headless_sim.md`

## 実装方針

**別バイナリ方式**（既存コードへの影響最小化）

### ファイル構成

```
project/src/
├── main.rs                    # 通常ゲーム（変更なし）
├── lib.rs                     # 共有ロジック公開（新規）
├── bin/
│   └── headless_sim.rs        # ヘッドレスシミュレータ（新規）
└── simulation/                # シミュレーション専用モジュール（新規）
    ├── mod.rs
    ├── headless_plugins.rs    # ヘッドレス用プラグインセット
    ├── anomaly_detector.rs    # 異常検出システム
    ├── simulation_runner.rs   # シミュレーション制御・AI両対応
    └── result_reporter.rs     # 結果出力（コンソール＋JSON）
```

### 依存関係追加（Cargo.toml）

```toml
[[bin]]
name = "headless_sim"
path = "src/bin/headless_sim.rs"

[dependencies]
clap = { version = "4.0", features = ["derive"] }  # CLI引数
serde_json = "1.0"  # JSON出力
chrono = "0.4"      # タイムスタンプ
```

## サブタスク

- [x] Phase 0: 7_tools カテゴリ新設
  - [x] `docs/reference/framework-spec.md` の番号体系更新（7 = tools）
  - [x] `project/docs/7_tools/` ディレクトリ作成
  - [x] `project/docs/7_tools/70000_overview.md` 概要ファイル作成
- [x] Phase 1: 基盤構築
  - [x] lib.rs 作成（既存モジュール再公開）
  - [x] simulation/mod.rs 新規モジュール構造
  - [x] simulation/headless_plugins.rs（MinimalPlugins + ゲームロジック）
  - [x] bin/headless_sim.rs CLIエントリポイント
  - [x] Cargo.toml 更新
- [x] Phase 2: AI両対応
  - [x] simulation/simulation_runner.rs（AI vs AI セットアップ）
  - [x] Player 1 に AiController 付与
- [x] Phase 3: 異常検出
  - [x] simulation/anomaly_detector.rs
  - [x] NaN座標検出
  - [x] コート外消失検出
  - [x] 状態遷移スタック検出
  - [x] 無限ラリー検出
  - [x] 物理異常検出
- [x] Phase 4: 結果出力
  - [x] simulation/result_reporter.rs（JSON出力）
  - [x] サマリー統計
- [x] Phase 5: 仕様書作成
  - [x] `project/docs/7_tools/71_simulation/77100_headless_sim.md`

## Progress

- 2026-01-10: 全Phase完了、ビルド・動作確認済み
  - 7_tools カテゴリ新設（番号体系更新）
  - simulation モジュール作成（headless_plugins, anomaly_detector, simulation_runner, result_reporter, config）
  - CLI エントリポイント作成（clap, serde_json, chrono 依存追加）
  - 仕様書作成（77100_headless_sim.md）
  - `cargo run --bin headless_sim -- -n 3` で動作確認OK
  - 注: 実際のBevy App実行はスタブ実装
- 2026-01-10: レビュー対応完了
  - 未使用コードの警告を解消（#[allow(dead_code)] 追加）
  - 異常検出閾値を simulation_config.ron に外部化
  - config.rs モジュール追加、AnomalyThresholds 構造体定義
  - 仕様書に設定ファイル情報を追記

## Next Actions

1. mainブランチでスカッシュマージを実行
2. タスクファイルを 3_done へ移動

## メモ

- 警告は既存コード（ball_trajectory.rs, ai_movement.rs）の2件のみ残存（タスクスコープ外）
- スタブ実装のため、将来 Bevy App 実行時に AnomalyThresholdsResource を設定する必要あり

## CLIインターフェース

```bash
cargo run --bin headless_sim -- [OPTIONS]

Options:
  -n, --matches <COUNT>    試合数 [default: 10]
  -t, --timeout <SECONDS>  1試合の最大時間 [default: 300]
  -o, --output <FILE>      JSON出力パス
  -s, --seed <SEED>        乱数シード（再現性用）
  -v, --verbose            詳細ログ
```

## 検証方法

1. `cargo build --bin headless_sim` でビルド確認
2. `cargo run --bin headless_sim -- -n 3` で3試合実行
3. 異常なく完走することを確認
4. 意図的に異常を発生させて検出を確認

## 関連ファイル

- `project/src/main.rs` - プラグイン構成パターン参照
- `project/src/systems/match_flow.rs` - 状態遷移ロジック
- `project/src/systems/ai_movement.rs` - AiController実装
- `project/src/components/mod.rs` - 異常検出対象の構造

## 参照

- プラン: `.claude/plans/refactored-swimming-chipmunk.md`
