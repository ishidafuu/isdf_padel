# B30-001: サーブ着地点計算のZ座標バグ修正

## 概要

サーブ着地点計算でZ座標にマージンを加算する際、サービスボックス外に着地点が設定されてしまうバグを修正する。

## 問題詳細

- **タイプ**: バグ修正
- **優先度**: Critical
- **原因特定済み**: Yes
- **リプレイ**: `replay_20260119_072528.replay` (Rally 1, 2, 6)
- **QAレポート**: `project/qa_reports/2026-01-19_072511/`

## 症状

- DoubleFault頻発（37.5%）
- サーブがサービスボックス外に着地

## 根本原因

`src/systems/trajectory_calculator/serve_trajectory.rs:40-42` でZ座標計算時にマージンを加算し、サービスボックス外に着地点を設定している。

```rust
// 現在のコード（バグ）
let target_z = lerp(
    service_box.z_min + margin,  // -5.0 + 0.5 = -4.5
    service_box.z_max + margin,  // 0.0 + 0.5 = 0.5 ← サービスボックス外！
    width_t,
);
```

**問題**: `z_max + margin` でサービスボックスの外側に着地点が設定される。

## 修正内容

マージンは内側に適用すべき。`z_max` にはマージンを減算する。

```rust
// 修正後
let target_z = lerp(
    service_box.z_min + margin,  // 内側に寄せる（-5.0 + 0.5 = -4.5）
    service_box.z_max - margin,  // 内側に寄せる（0.0 - 0.5 = -0.5）
    width_t,
);
```

## 修正対象ファイル

- `project/src/systems/trajectory_calculator/serve_trajectory.rs:40-42`

## 検証方法

1. `/qa-cycle` でQAサイクル再実行
2. リプレイビューアーでサーブ着地点を確認
3. DoubleFault率が有意に低下していることを確認

## 関連

- 関連イベント: DoubleFault
- 関連コンポーネント: ServeTrajectory, ServiceBox
