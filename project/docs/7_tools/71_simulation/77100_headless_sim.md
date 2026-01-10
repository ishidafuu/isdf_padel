# 77100: Headless Simulation

## 概要

AI対AIの自動対戦を描画なしで高速実行し、ゲームロジックの異常を検出する開発支援ツール。

## 目的

- テストでは検出困難な長時間動作での問題を発見
- 物理異常、状態遷移異常、無限ループ等を自動検出
- 試合結果の統計を取得

## 使用方法

### 基本実行

```bash
# project/ ディレクトリで実行
cargo run --bin headless_sim -- -n 10
```

### CLI オプション

| オプション | 短縮 | 説明 | デフォルト |
|-----------|------|------|-----------|
| `--matches` | `-n` | 試合数 | 10 |
| `--timeout` | `-t` | 1試合の最大秒数 | 300 |
| `--output` | `-o` | JSON出力パス | なし |
| `--seed` | `-s` | 乱数シード（再現性用） | なし |
| `--verbose` | `-v` | 詳細ログ | false |

### 実行例

```bash
# 3試合実行
cargo run --bin headless_sim -- -n 3

# 結果をJSONに出力
cargo run --bin headless_sim -- -n 10 -o results.json

# シード指定（再現性確保）
cargo run --bin headless_sim -- -n 5 -s 12345

# 詳細ログ付き
cargo run --bin headless_sim -- -n 3 -v
```

## 設定ファイル

異常検出の閾値は `assets/config/simulation_config.ron` で設定可能。

```ron
(
    anomaly_thresholds: (
        bounds_margin: 50.0,        // コート外判定マージン（m）
        height_limit: 100.0,        // 高さ上限（m）
        height_floor: -10.0,        // 高さ下限（m）
        state_stuck_secs: 60.0,     // 状態遷移スタック判定（秒）
        infinite_rally_secs: 300.0, // 無限ラリー判定（秒）
        max_velocity: 1000.0,       // 物理異常速度閾値
    ),
)
```

ファイルが存在しない場合はデフォルト値を使用。

## 異常検出

### 検出対象

| 異常タイプ | 説明 | 設定キー |
|-----------|------|----------|
| NaN座標 | Player/Ballの座標がNaN | - |
| NaN速度 | 速度ベクトルがNaN | - |
| コート外消失 | エンティティがコート外に消失 | `bounds_margin` |
| 状態遷移スタック | 同一状態が継続 | `state_stuck_secs` |
| 無限ラリー | ラリーが継続 | `infinite_rally_secs` |
| 物理異常 | 速度が異常に大きい | `max_velocity` |

### 出力形式

#### コンソール出力

```
========================================
       SIMULATION REPORT
========================================
Timestamp:         2025-01-10 12:34:56
Total Matches:     10
Completed:         10
Player 1 Wins:     4
Player 2 Wins:     6
Total Anomalies:   0
Avg Duration:      45.32s
Avg Rally Count:   12.5
========================================
```

#### JSON出力

```json
{
  "timestamp": "2025-01-10 12:34:56",
  "total_matches": 10,
  "completed_matches": 10,
  "player1_wins": 4,
  "player2_wins": 6,
  "total_anomalies": 0,
  "avg_duration_secs": 45.32,
  "avg_rally_count": 12.5,
  "matches": [...]
}
```

## アーキテクチャ

### ファイル構成

```
project/
├── assets/config/
│   └── simulation_config.ron  # シミュレーター設定（異常検出閾値）
└── src/
    ├── lib.rs                     # 共有ロジック公開
    ├── bin/
    │   └── headless_sim.rs        # CLIエントリポイント
    └── simulation/
        ├── mod.rs                 # モジュール定義
        ├── config.rs              # 設定ファイル読み込み
        ├── headless_plugins.rs    # ヘッドレス用プラグインセット
        ├── anomaly_detector.rs    # 異常検出システム
        ├── simulation_runner.rs   # シミュレーション制御
        └── result_reporter.rs     # 結果出力
```

### 設計方針

- **別バイナリ方式**: 既存ゲームコードへの影響最小化
- **MinimalPlugins**: 描画系を除外した軽量構成
- **両プレイヤーAI**: Player 1, 2 共に AiController を付与

## 制限事項

- 現在はスタブ実装（実際のBevy App実行は未実装）
- シード指定による完全な再現性は未保証

## 関連ファイル

- `project/src/main.rs` - 通常ゲームのプラグイン構成
- `project/src/systems/ai_movement.rs` - AI移動ロジック
- `project/src/systems/match_flow.rs` - 試合状態遷移
