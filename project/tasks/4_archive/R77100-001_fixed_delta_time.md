# R77100-001: ヘッドレスシミュレーション高速化（固定タイムステップ導入）

## 概要
物理計算の結果を変えずに、ヘッドレスシミュレーションを高速で実行できるようにする。

## 背景
- 現在、ヘッドレスシミュレーションは実時間で動作（1試合約90秒）
- `time.delta_secs()` が実時間を返すため、ウェイトを外しても高速化できない
- 固定タイムステップを導入することで、物理計算を変えずに高速化可能

## 実装内容

### 1. FixedDeltaTime リソース作成
- `project/src/resource/fixed_delta.rs` (新規)
- 固定の delta_secs (1/60秒) を提供

### 2. 物理系システムの修正
以下のシステムで `time.delta_secs()` → `fixed_dt.delta_secs` に変更:
- `systems/jump.rs` (2箇所)
- `systems/movement.rs` (1箇所)
- `systems/ball_trajectory.rs` (4箇所)
- `systems/knockback.rs` (2箇所)
- `systems/ai_movement.rs` (1箇所)
- `systems/serve.rs` (3箇所)
- `systems/shot_attributes.rs` (1箇所)
- `systems/shot_input.rs` (1箇所)
- `character/systems.rs` (1箇所)

### 3. シミュレーション関連の修正
- `simulation/simulation_runner.rs` (1箇所 + Duration::ZERO設定)
- `simulation/anomaly_detector.rs` (3箇所)

### 4. main.rs にリソース挿入

## 検証方法
1. 同じシード (seed: Some(12345)) で等倍実行と高速実行を比較
2. 勝者・ゲーム内時間が一致することを確認
3. 高速実行で実時間が約1/10になることを確認

## 期待される効果
- 等倍: 約90秒で1試合
- 高速: 約9秒で1試合
- 物理計算結果: 完全一致

## 関連
- @spec 77100_headless_sim.md
