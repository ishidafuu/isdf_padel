# 77200: Telemetry Extension Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2026-01-16

## 概要

LLMベースのQAシステムに向けて、既存EventTracerを拡張し、ショット計算・AI決定・物理異常の詳細をトレース可能にする。

## 目的

- ショット属性計算の中間係数を記録し、ショット品質分析を可能に
- AI移動決定の理由を記録し、AI挙動の自然さ分析を可能に
- 物理異常をリアルタイムで検出・マーキング

## 関連仕様

- 77100: ヘッドレスシミュレーション
- 77103: リプレイシステム
- 30604: ショット属性計算

## データ構造

### 新規GameEventバリアント

```rust
pub enum GameEvent {
    // --- 既存イベント ---
    BallHit { player: u8, shot_type: String },
    Bounce { position: Vec3, court_side: CourtSide },
    WallReflect { position: Vec3, wall_type: String },
    Point { winner: u8, reason: String },
    Fault { fault_type: String },
    StateChange { from: String, to: String },

    // --- Phase 1 新規イベント ---

    /// ショット属性計算詳細
    ShotAttributesCalculated {
        player_id: u8,
        input_mode: String,           // "Push" / "Hold"
        hit_height: f32,
        bounce_elapsed: Option<f32>,
        approach_dot: f32,
        ball_distance: f32,
        // 中間係数
        height_factors: (f32, f32, f32),   // (power, stability, angle)
        timing_factors: (f32, f32, f32),
        approach_factors: (f32, f32),       // (power, angle)
        distance_factors: (f32, f32, f32), // (power, stability, accuracy)
        // 最終結果
        final_power: f32,
        final_stability: f32,
        final_angle: f32,
        final_spin: f32,
        final_accuracy: f32,
    },

    /// AI移動決定詳細
    AiMovementDecision {
        player_id: u8,
        movement_state: String,
        ball_coming_to_me: bool,
        reaction_timer: f32,
        landing_time: Option<f32>,
        landing_position: Option<Vec3>,
        trajectory_line_z: f32,
        arrival_distance: f32,
        target_position: Vec3,
    },

    /// 物理異常マーカー
    PhysicsAnomaly {
        anomaly_type: String,
        position: Vec3,
        velocity: Vec3,
        expected_value: f32,
        actual_value: f32,
        severity: String,
    },
}
```

### TraceConfig拡張

```rust
pub struct TraceConfig {
    // 既存フィールド
    pub enabled: bool,
    pub position: bool,
    pub velocity: bool,
    pub events: bool,
    pub interval_frames: u32,

    // 新規フィールド
    pub shot_attributes: bool,    // ショット属性記録
    pub ai_decisions: bool,       // AI決定記録
    pub physics_anomalies: bool,  // 物理異常記録
}
```

## Core Requirements (MVP)

### ショット属性トレース

#### REQ-77200-001: ショット属性計算記録
**WHEN** `calculate_shot_attributes` が呼び出される
**THE SYSTEM SHALL** 計算の中間係数と最終結果を記録する
- 入力コンテキスト（input_mode, hit_height, bounce_elapsed等）
- 各係数（height_factors, timing_factors, approach_factors, distance_factors）
- 最終属性（power, stability, angle, spin, accuracy）
**テスト**: ショット実行後にShotAttributesCalculatedイベントがトレースに含まれることを確認

#### REQ-77200-002: トレース設定による制御
**WHEN** `trace.shot_attributes` が false の場合
**THE SYSTEM SHALL** ショット属性のトレースをスキップする
**テスト**: 設定オフ時にShotAttributesCalculatedイベントが記録されないことを確認

### AI移動決定トレース

#### REQ-77200-003: AI移動決定記録
**WHEN** AIプレイヤーの移動目標が更新される
**THE SYSTEM SHALL** 決定理由を記録する
- 移動状態（Tracking / Idle / Recovering）
- ボール軌道情報（landing_time, landing_position, trajectory_line_z）
- 目標位置（target_position）
- 到達距離（arrival_distance）
**テスト**: AI移動更新時にAiMovementDecisionイベントがトレースに含まれることを確認

#### REQ-77200-004: 反応遅延の記録
**WHEN** AIプレイヤーが反応遅延中である
**THE SYSTEM SHALL** 反応タイマーの値を記録する
**テスト**: 反応遅延中のreaction_timerが正の値で記録されることを確認

#### REQ-77200-007: AI決定トレース設定による制御
**WHEN** `trace.ai_decisions` が false の場合
**THE SYSTEM SHALL** AI移動決定のトレースをスキップする
**テスト**: 設定オフ時にAiMovementDecisionイベントが記録されないことを確認

### 物理異常検出

#### REQ-77200-005: 速度スパイク検出
**WHEN** ボール速度が前フレームから閾値以上変化する
**THE SYSTEM SHALL** PhysicsAnomalyイベントを記録する
- anomaly_type: "VelocitySpike"
- expected_value: 前フレーム速度
- actual_value: 現フレーム速度
**テスト**: 急激な速度変化時にVelocitySpikeイベントが記録されることを確認

#### REQ-77200-006: 異常バウンス検出
**WHEN** バウンス後の速度ベクトルが物理的に不自然である
**THE SYSTEM SHALL** PhysicsAnomalyイベントを記録する
- anomaly_type: "UnexpectedBounce"
- 入射角と反射角の差が閾値以上
**テスト**: 不自然なバウンス時にUnexpectedBounceイベントが記録されることを確認

## Extended Requirements

### REQ-77200-101: 軌道予測記録
**WHEN** ショットが実行される
**THE SYSTEM SHALL** 軌道予測結果を記録する
- 打ち出し速度ベクトル
- 予測着弾位置
- ネットクリアランス
- 飛行時間

### REQ-77200-102: プレイヤー間距離記録
**WHILE** ラリー中
**THE SYSTEM SHALL** 定期的にプレイヤー間距離を記録する
- フレーム間隔設定に従う

## 設定ファイル

### trace_config.ron 拡張

```ron
(
    enabled: true,
    position: true,
    velocity: true,
    events: true,
    interval_frames: 1,
    // 新規
    shot_attributes: true,
    ai_decisions: true,
    physics_anomalies: true,
)
```

## 出力形式

### JSON形式（抜粋）

```json
{
  "frame": 1234,
  "timestamp": 45.67,
  "events": [
    {
      "type": "ShotAttributesCalculated",
      "player_id": 1,
      "input_mode": "Push",
      "hit_height": 2.45,
      "final_power": 18.5,
      "final_spin": 0.6,
      // ...
    }
  ]
}
```

## 実装対象ファイル

| ファイル | 変更内容 |
|---------|---------|
| `simulation/event_tracer.rs` | GameEvent enum 拡張、to_json/to_csv 拡張 |
| `systems/shot/attributes.rs` | トレース呼び出し追加 |
| `systems/ai/movement.rs` | トレース呼び出し追加 |
| `simulation/config.rs` | TraceConfig 拡張 |
| `simulation/anomaly_detector.rs` | PhysicsAnomaly記録追加 |
