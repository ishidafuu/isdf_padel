# B30301-002: AI移動システム改善（インターセプト方式）

## 概要

AIがサービスボックス付近でガタガタ震え、ボールを適切に追わない問題を修正。
「着地点予測」方式から「インターセプト」方式に変更。

## 問題

1. **振動**: 予測誤差が毎フレーム再計算され、目標位置が毎フレーム変動
2. **不適切な追跡**: 着地点を追う（外野手方式）ため、テニスの動きと異なる
3. **ラリー不成立**: 上記により適切なショットが打てない

## 修正方針

### インターセプト方式

- AIはX座標を固定し、ボールが通過するZ座標を予測して移動
- 短いボール（自分のX座標に届かない）の場合はボール現在位置を追跡
- 目標ロック機構で振動を防止（誤差は状態変化時のみ適用）

## 修正内容

### 1. AiControllerにロック用フィールド追加

**ファイル**: `src/components/ai.rs`

```rust
pub locked_target_z: Option<f32>,
pub lock_ball_velocity_x: Option<f32>,
```

### 2. インターセプト計算関数追加

**ファイル**: `src/systems/ai/movement.rs`

- `calculate_intercept_position()` - インターセプト位置計算
- `is_short_ball()` - 短いボール判定
- `apply_z_error()` - Z座標のみに誤差適用

### 3. ai_movement_system修正

**ファイル**: `src/systems/ai/movement.rs`

- `calculate_landing_position()` → `calculate_intercept_position()` に置換
- 状態変化検知・目標ロック機構実装

### 4. 仕様書更新

**ファイル**: `docs/3_ingame/303_ai/30301_ai_movement_spec.md`

新規要件追加：
- REQ-30301-v07-001: インターセプト方式移動
- REQ-30301-v07-002: 短いボール判定
- REQ-30301-v07-003: 予測誤差の適用タイミング

## 検証

1. `cargo build` - コンパイル確認
2. `/run-game` - ゲーム起動
3. 確認項目:
   - AIの振動が解消
   - AIがZ方向のみ移動、X座標維持
   - 短いボールには前に出て追跡
   - ラリーが継続

## 関連

- 詳細プラン: `~/.claude/plans/elegant-wiggling-beacon.md`
