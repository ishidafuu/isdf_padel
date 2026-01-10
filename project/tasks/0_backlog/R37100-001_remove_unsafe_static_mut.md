# R37100-001: simulation_runner.rs の unsafe コード除去

## 概要

`simulation_runner.rs` のデバッグシステムで使用されている `static mut` + `unsafe` ブロックを Bevy の `Local<T>` に置き換える。

## 背景

### 現状の問題コード

```rust
// simulation_runner.rs:316-320
fn debug_simulation_state(...) {
    static mut LAST_LOG_TIME: f32 = 0.0;
    let elapsed = unsafe { LAST_LOG_TIME };

    if time.elapsed_secs() - elapsed > 5.0 {
        unsafe {
            LAST_LOG_TIME = time.elapsed_secs();
        }
        // ログ出力
    }
}

// simulation_runner.rs:361-378
fn debug_state_transitions(...) {
    static mut LAST_FLOW_STATE: Option<MatchFlowState> = None;
    static mut LAST_PHASE: Option<RallyPhase> = None;
    unsafe {
        if LAST_FLOW_STATE != Some(current_flow) { ... }
        if LAST_PHASE != Some(state.phase) { ... }
    }
}
```

### 問題点

- `static mut` は Rust の安全性原則に反する
- データ競合の可能性（シングルスレッドでは実際には安全だが）
- Bevy の推奨パターンに従っていない

## 修正方針

Bevy の `Local<T>` リソースを使用:

```rust
fn debug_simulation_state(
    sim_config: Res<SimulationConfig>,
    time: Res<Time>,
    mut last_log_time: Local<f32>,  // ← Local<T> 使用
    // ...
) {
    if time.elapsed_secs() - *last_log_time > 5.0 {
        *last_log_time = time.elapsed_secs();
        // ログ出力
    }
}

fn debug_state_transitions(
    sim_config: Res<SimulationConfig>,
    match_flow_state: Res<State<MatchFlowState>>,
    rally_state: Option<Res<RallyState>>,
    mut last_flow_state: Local<Option<MatchFlowState>>,  // ← Local<T>
    mut last_phase: Local<Option<RallyPhase>>,           // ← Local<T>
) {
    let current_flow = *match_flow_state.get();
    if *last_flow_state != Some(current_flow) {
        eprintln!("[DEBUG] MatchFlowState: {:?}", current_flow);
        *last_flow_state = Some(current_flow);
    }
    // ...
}
```

## 対象ファイル

| ファイル | 修正内容 |
|---------|---------|
| `src/simulation/simulation_runner.rs` | `static mut` → `Local<T>` |

## 修正箇所

1. **debug_simulation_state（行316-348）**
   - `static mut LAST_LOG_TIME` → `Local<f32>`

2. **debug_state_transitions（行350-380）**
   - `static mut LAST_FLOW_STATE` → `Local<Option<MatchFlowState>>`
   - `static mut LAST_PHASE` → `Local<Option<RallyPhase>>`

## 検証方法

```bash
# 1. ビルド確認
cargo build --bin headless_sim

# 2. verbose モードで実行
cargo run --bin headless_sim -- -c debug
# → デバッグログが正常に出力されること
```

## 完了チェックリスト

- [ ] `debug_simulation_state` の修正
- [ ] `debug_state_transitions` の修正
- [ ] `unsafe` ブロックの完全削除
- [ ] ビルド成功
- [ ] verbose モードでの動作確認

## 優先度

low（動作に影響なし、デバッグ用途のみ）

## 発見経緯

- **発見タスク**: 37100-002 レビュー時
- **発見日**: 2026-01-10

## 関連ファイル

- `project/docs/7_tools/71_simulation/77100_headless_sim.md` - シミュレーター仕様
- `project/tasks/3_done/37100-002_simulation_enhancement.md` - 発見タスク
