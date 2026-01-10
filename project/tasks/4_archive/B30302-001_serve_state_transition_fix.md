# B30302-001: サーブ状態遷移の整合性修正

## 概要

シミュレーターで発見されたサーブ→ラリー状態遷移のバグを修正する。

## 背景

### 現状の問題

`ai_serve_hit_system` および `serve_hit_input_system` が `MatchFlowState::Rally` に直接遷移しており、以下の問題が発生:

1. `serve_to_rally_system` が実行されない
2. `RallyPhase` が `WaitingServe` のまま更新されない
3. サービスボックス判定がスキップされる
4. ポイント判定が機能せず試合が正常終了しない

### 正しいフロー（仕様）

```
Serve状態 → ShotEvent発行のみ
    ↓
serve_to_rally_system が RallyPhase::Serving に遷移
    ↓
ボール着地 → serve_landing_judgment_system
    ↓ [サービスボックス内に着地]
RallyPhase::Rally + MatchFlowState::Rally に遷移
```

## 発見経緯

- **発見タスク**: 37100-002 シミュレーター機能強化
- **発見方法**: ヘッドレスシミュレーター実行時にタイムアウト終了
- **発見日**: 2026-01-10

## 修正内容

### 1. ai_serve_hit_system（ai_serve.rs:219）

```rust
// ❌ 修正前
pub fn ai_serve_hit_system(...) {
    next_state.set(MatchFlowState::Rally);  // 直接遷移
    shot_event_writer.write(ShotEvent {...});
}

// ✅ 修正後
pub fn ai_serve_hit_system(...) {
    // ShotEvent発行のみ、状態遷移は serve_landing_judgment_system で行う
    shot_event_writer.write(ShotEvent {...});
}
```

### 2. serve_hit_input_system（match_flow.rs）

同様に `next_state.set(MatchFlowState::Rally)` を削除。

### 3. serve_landing_judgment_system（確認）

サービスボックス成功時に以下を実行することを確認:
- `RallyPhase::Serving` → `RallyPhase::Rally`
- `MatchFlowState::Serve` → `MatchFlowState::Rally`

## 対象ファイル

| ファイル | 修正内容 |
|---------|---------|
| `src/systems/ai_serve.rs` | `next_state.set(MatchFlowState::Rally)` 削除 |
| `src/systems/match_flow.rs` | `serve_hit_input_system` から同様の削除 |
| `src/systems/serve_landing_judgment.rs` | 状態遷移の確認・必要なら追加 |

## 検証方法

```bash
# 1. ビルド確認
cargo build

# 2. テスト
cargo test

# 3. シミュレーター検証
cargo run --bin headless_sim -- -c test
# → タイムアウトではなく正常終了すること
# → スコアが進行すること
```

## 完了チェックリスト

- [ ] `ai_serve_hit_system` から直接遷移を削除
- [ ] `serve_hit_input_system` から直接遷移を削除
- [ ] `serve_landing_judgment_system` の状態遷移を確認
- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] シミュレーター正常終了確認

## 優先度

high（ゲームが正常終了しない）

## 関連ファイル

- `project/docs/3_ingame/301_scoring/30101_flow_spec.md` - 試合フロー仕様
- `project/docs/3_ingame/309_serve/30902_fault_spec.md` - フォールト仕様
- `project/tasks/3_done/37100-002_simulation_enhancement.md` - 発見タスク

## 参照

- プラン: `~/.claude/plans/twinkly-crunching-lollipop.md`
