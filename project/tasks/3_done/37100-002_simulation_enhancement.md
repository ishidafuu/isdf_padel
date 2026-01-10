# 37100-002: シミュレーター機能強化

## 概要

37100-001で作成したヘッドレスシミュレーターを実用レベルに完成させる。
Claude Codeの動作確認に使用できるよう、Bevy App実際実行とデバッグ支援機能を実装する。

## 背景

- 37100-001はスタブ実装（ダミー結果を返している）
- 実際のゲームロジック実行が未実装
- Claude Codeがコード変更後の検証に使用したい

## 主なユースケース

1. **実装後の動作確認**: コード変更後に異常がないか自動チェック
2. **バグ再現・調査**: シード固定で再現、状態確認
3. **物理演算の検証**: ボール軌道や衝突判定の確認

## 要件

### Phase 1: 基本機能完成（必須）

| 機能 | 説明 |
|------|------|
| **1.1 設定ファイル拡張** | execution/output/trace セクション追加 |
| **1.2 CLI簡略化** | `-c <name>` のみ（設定名で指定） |
| **1.3 Bevy App実際実行** | スタブを本実装に置き換え |
| **1.4 シード固定** | 乱数シードの固定による再現性 |
| **1.5 タイムアウト** | 試合時間超過で強制終了 |

### Phase 2: デバッグ支援機能（将来対応）

| 機能 | 説明 |
|------|------|
| **2.1 座標・速度トレース** | プレイヤー/ボールの時系列データ出力 |
| **2.2 状態ダンプ** | 任意時点のエンティティ状態をJSON出力 |
| **2.3 イベントログ** | ショット、バウンド等のイベント時系列 |

## 設定ファイル設計

### 構造（拡張後）

```ron
// assets/config/simulation_config.ron
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
        enabled: false,
        position: true,         // 座標記録
        velocity: true,         // 速度記録
        events: false,          // イベント記録
        interval_frames: 10,    // 記録間隔（フレーム）
    ),

    // === 異常検出の閾値設定（既存） ===
    anomaly_thresholds: (
        bounds_margin: 50.0,
        height_limit: 100.0,
        height_floor: -10.0,
        state_stuck_secs: 60.0,
        infinite_rally_secs: 300.0,
        max_velocity: 1000.0,
    ),
)
```

### CLI設計

```bash
# デフォルト設定で実行（simulation_config.ron を使用）
cargo run --bin headless_sim

# 設定名を指定
cargo run --bin headless_sim -- -c debug
# → assets/config/simulation_debug.ron を読み込み
```

**解決ルール**: `-c <name>` → `assets/config/simulation_<name>.ron`

### 用途別プリセット

```
assets/config/
├── simulation_config.ron        # デフォルト設定
├── simulation_debug.ron         # デバッグ用（トレース有効、1試合）
├── simulation_stress.ron        # ストレステスト用（100試合）
└── simulation_test.ron          # テスト用（短時間タイムアウト）
```

## サブタスク

- [x] Phase 1.1: 設定ファイル構造の拡張
  - [x] SimulationFileConfig に execution/output/trace セクション追加
  - [x] config.rs 更新
- [x] Phase 1.2: CLI引数を設定名のみに変更
  - [x] headless_sim.rs から -n/-t/-s/-o/-v オプション削除
  - [x] -c <name> オプションのみに
  - [x] 解決ルール実装
- [x] Phase 1.3: Bevy App実際実行の実装
  - [x] simulation_runner.rs の run_single_match() 本実装
  - [x] HeadlessPlugins を使用したApp構築
  - [x] 試合終了検出
- [ ] Phase 1.4: シード固定の実装（将来対応）
  - [ ] 乱数生成の制御
  - [ ] 設定ファイルから seed 読み込み
- [x] Phase 1.5: タイムアウトの実装
  - [x] 試合時間超過で強制終了
  - [x] タイムアウト結果の記録
- [x] プリセット設定ファイルの作成
  - [x] simulation_debug.ron
  - [x] simulation_stress.ron
  - [x] simulation_test.ron
- [ ] Phase 2.1-2.3: デバッグ支援機能（将来対応）

## 対象ファイル

### 修正対象
- `project/src/bin/headless_sim.rs` - CLI引数変更
- `project/src/simulation/config.rs` - 設定構造拡張
- `project/src/simulation/simulation_runner.rs` - Bevy App実行
- `project/src/simulation/headless_plugins.rs` - 不要コード削除
- `project/assets/config/simulation_config.ron` - 設定ファイル

### 新規作成
- `project/assets/config/simulation_debug.ron` - デバッグ用プリセット
- `project/assets/config/simulation_stress.ron` - ストレステスト用プリセット
- `project/assets/config/simulation_test.ron` - テスト用プリセット

## 検証方法

### Phase 1 検証
```bash
# 1. ビルド確認
cargo build --bin headless_sim

# 2. 基本実行（実際にゲームが動くこと）
cargo run --bin headless_sim -- -c test

# 3. タイムアウト確認
# → simulation_test.ron に timeout_secs: 10 を設定済み
cargo run --bin headless_sim -- -c test
# → 10秒でタイムアウト終了することを確認
```

## Progress

- 2026-01-10: タスク作成
- 2026-01-10: Phase 1.1, 1.2, 1.3, 1.5 実装完了
- 2026-01-10: プリセット設定ファイル作成
- 2026-01-10: 仕様書更新

## 発見した問題

### ゲームロジックのバグ

シミュレーターで試合を実行すると、以下の問題が発見された:

1. **RallyPhaseがWaitingServeのまま更新されない**
   - `ai_serve_hit_system` が `MatchFlowState::Rally` に直接遷移させている
   - `serve_to_rally_system` が実行されず、`RallyPhase` が `Serving` に更新されない
   - 結果として、ポイント判定システムが正しく機能しない

2. **試合が正常終了しない**
   - タイムアウトで終了するが、スコアが進まない
   - ボールは動いているが、ツーバウンド判定等が機能していない

**推奨修正**: `ai_serve_hit_system` の状態遷移ロジックを見直し、`serve_to_rally_system` と整合させる

## Next Actions

1. ゲームロジックのバグ修正（別タスク）
2. Phase 1.4 シード固定の実装（必要に応じて）
3. Phase 2 デバッグ支援機能（必要に応じて）

## 関連ファイル

- `project/tasks/3_done/37100-001_headless_simulation.md` - 前タスク
- `project/docs/7_tools/71_simulation/77100_headless_sim.md` - 仕様書

## 参照

- プラン: `.claude/plans/wild-swimming-russell.md`
