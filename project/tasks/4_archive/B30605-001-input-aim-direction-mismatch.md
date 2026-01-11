# B30605-001: 入力方向とボール狙い位置の不一致

## 概要

十字キー入力とボールの狙い位置が直感と一致していない。

## 問題

| 入力 | 期待動作 | 現在の動作 |
|------|----------|------------|
| W/S | コースの上下（画面上/下） | 無視される |
| A/D | 深さ（ネット側/ベースライン側） | コース左右として扱われる |

## 原因

1. `shot.rs:119` で `input.y`（W/S）を強制的にゼロにしている
2. `landing_position.rs` で `input.x`/`input.y` の役割がユーザー期待と逆

## 修正対象ファイル

- `project/src/systems/input/shot.rs`
- `project/src/systems/trajectory_calculator/landing_position.rs`

## 修正内容

### shot.rs

```rust
// 修正前（119行目）
let direction = Vec2::new(raw_direction.x, 0.0);

// 修正後
let direction = raw_direction;
```

### landing_position.rs

input.x と input.y の役割を入れ替え：
- `input.x` (A/D) → 深さ調整
- `input.y` (W/S) → コース調整

## 検証方法

`/run-game` で以下を確認：
- W: 画面上側に狙う
- S: 画面下側に狙う
- A: ネット側（浅い）に狙う
- D: ベースライン側（深い）に狙う

## 関連仕様

- 30605_trajectory_calculation_spec.md
- 30602_shot_direction_spec.md
