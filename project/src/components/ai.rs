//! AI関連コンポーネント
//! @spec 30301_ai_movement_spec.md

use bevy::prelude::*;

/// AI移動状態
/// @spec 30301_ai_movement_spec.md#req-30301-v05
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AiMovementState {
    /// 待機中（ボールが相手側）
    #[default]
    Idle,
    /// 追跡中（ボールが自分側）
    Tracking,
    /// リカバリー中（ショット後の復帰）
    #[allow(dead_code)]
    Recovering,
}

/// AIコントローラーマーカーコンポーネント
/// @spec 30301_ai_movement_spec.md
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct AiController {
    /// ホームポジション（待機位置）
    /// @spec 30301_ai_movement_spec.md#req-30301-005
    pub home_position: Vec3,
    /// AI移動状態
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    pub movement_state: AiMovementState,
    /// 目標位置（状態に応じて更新）
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    pub target_position: Vec3,
    /// 反応遅延タイマー（秒）- 0より大きい間は追跡を開始しない
    /// @spec 30301_ai_movement_spec.md#req-30301-053
    pub reaction_timer: f32,
    /// 確定済みの目標Z座標（振動防止用）
    /// @spec 30301_ai_movement_spec.md#req-30301-v07-003
    pub locked_target_z: Option<f32>,
    /// 目標をロックした時のボール速度X符号（状態変化検知用）
    /// @spec 30301_ai_movement_spec.md#req-30301-v07-003
    pub lock_ball_velocity_x_sign: Option<bool>,
}
