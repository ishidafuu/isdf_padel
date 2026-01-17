//! ゲームイベント種別定義
//! @spec 77100_headless_sim.md

use bevy::prelude::*;

use crate::core::CourtSide;

/// ゲームイベント種別
#[derive(Debug, Clone)]
pub enum GameEvent {
    /// ショット実行
    BallHit {
        player: u8,
        shot_type: String,
    },
    /// 地面バウンス
    Bounce {
        position: Vec3,
        court_side: CourtSide,
    },
    /// 壁反射
    WallReflect {
        position: Vec3,
        wall_type: String,
    },
    /// ポイント獲得
    Point {
        winner: u8,
        reason: String,
    },
    /// フォールト
    Fault {
        fault_type: String,
    },
    /// 状態遷移
    StateChange {
        from: String,
        to: String,
    },
    /// ショット属性計算詳細
    ShotAttributesCalculated {
        player_id: u8,
        input_mode: String,
        hit_height: f32,
        bounce_elapsed: Option<f32>,
        approach_dot: f32,
        ball_distance: f32,
        /// 中間係数 (power, stability, angle)
        height_factors: (f32, f32, f32),
        /// 中間係数 (power, stability, angle)
        timing_factors: (f32, f32, f32),
        /// 中間係数 (power, angle)
        approach_factors: (f32, f32),
        /// 中間係数 (power, stability, accuracy)
        distance_factors: (f32, f32, f32),
        /// 最終結果
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

impl GameEvent {
    /// イベント種別名を取得
    pub(crate) fn type_name(&self) -> &'static str {
        match self {
            GameEvent::BallHit { .. } => "BallHit",
            GameEvent::Bounce { .. } => "Bounce",
            GameEvent::WallReflect { .. } => "WallReflect",
            GameEvent::Point { .. } => "Point",
            GameEvent::Fault { .. } => "Fault",
            GameEvent::StateChange { .. } => "StateChange",
            GameEvent::ShotAttributesCalculated { .. } => "ShotAttributesCalculated",
            GameEvent::AiMovementDecision { .. } => "AiMovementDecision",
            GameEvent::PhysicsAnomaly { .. } => "PhysicsAnomaly",
        }
    }
}
