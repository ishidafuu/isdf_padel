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
# デフォルト設定で実行（simulation_config.ron を使用）
cargo run --bin headless_sim

# 設定名を指定
cargo run --bin headless_sim -- -c debug
# → assets/config/simulation_debug.ron を読み込み
```

### CLI オプション

| オプション | 短縮 | 説明 | デフォルト |
|-----------|------|------|-----------|
| `--config` | `-c` | 設定名（例: "debug" → simulation_debug.ron） | なし |

### 設定ファイル解決ルール

- `-c <name>` → `assets/config/simulation_<name>.ron`
- 指定なし → `assets/config/simulation_config.ron`

### 実行例

```bash
# デフォルト設定で実行
cargo run --bin headless_sim

# デバッグ用設定（トレース有効、verbose）
cargo run --bin headless_sim -- -c debug

# ストレステスト用設定（100試合）
cargo run --bin headless_sim -- -c stress

# テスト用設定（短時間タイムアウト）
cargo run --bin headless_sim -- -c test
```

## 設定ファイル

### 構造

```ron
(
    // === 実行設定 ===
    execution: (
        match_count: 10,        // 実行試合数
        timeout_secs: 300,      // 1試合タイムアウト（秒）
        seed: None,             // 乱数シード（None=ランダム）
        verbose: false,         // 詳細ログ
    ),

    // === 出力設定 ===
    output: (
        result_file: None,      // JSON結果出力先（None=出力なし）
        trace_file: None,       // トレース出力先
    ),

    // === トレース設定 ===
    trace: (
        enabled: false,         // トレース有効化
        position: true,         // 座標記録
        velocity: true,         // 速度記録
        events: false,          // イベント記録
        interval_frames: 10,    // 記録間隔（フレーム）
    ),

    // === 異常検出の閾値設定 ===
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

### 用途別プリセット

| ファイル | 用途 | 特徴 |
|---------|------|------|
| `simulation_config.ron` | デフォルト | 標準的な設定 |
| `simulation_debug.ron` | デバッグ | verbose有効、トレース出力 |
| `simulation_stress.ron` | ストレステスト | 100試合、長時間 |
| `simulation_test.ron` | テスト | 短時間タイムアウト |

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
| タイムアウト | 試合時間超過 | `timeout_secs` |

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
│   ├── simulation_config.ron   # デフォルト設定
│   ├── simulation_debug.ron    # デバッグ用
│   ├── simulation_stress.ron   # ストレステスト用
│   └── simulation_test.ron     # テスト用
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
- **設定ファイル駆動**: 用途別にプリセット設定を用意

## 制限事項

- シード指定による完全な再現性は未保証（Phase 1.4で対応予定）
- マッチ終了条件のバランス次第で `timeout_secs` 到達が起こり得る（KPIで監視）

## QA運用

標準実行は `project/scripts/qa_cycle.sh` を使用する。

```bash
cd project
./scripts/qa_cycle.sh -c debug
```

詳細は `project/docs/7_tools/71_simulation/77300_qa_cycle_runbook.md` を参照。

## 関連ファイル

- `project/src/main.rs` - 通常ゲームのプラグイン構成
- `project/src/systems/ai_movement.rs` - AI移動ロジック
- `project/src/systems/match_flow.rs` - 試合状態遷移
