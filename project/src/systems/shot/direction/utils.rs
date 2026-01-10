//! ショット方向計算ユーティリティ
//! @spec 30602_shot_direction_spec.md

use bevy::prelude::*;

use crate::components::{Ball, InputState, LogicalPosition, Player, Velocity};
use crate::core::CourtSide;
use crate::resource::config::GameConfig;
use crate::resource::debug::LastShotDebugInfo;
use crate::systems::trajectory_calculator::TrajectoryResult;

/// プレイヤー情報を取得
pub(super) fn get_player_info(
    player_query: &Query<(&Player, &LogicalPosition, &Velocity, &InputState), Without<Ball>>,
    player_id: u8,
) -> Option<(Vec3, Vec3, f32)> {
    player_query
        .iter()
        .find(|(p, _, _, _)| p.id == player_id)
        .map(|(_, pos, vel, input_state)| (pos.value, vel.value, input_state.hold_time))
}

/// デバッグ情報を更新
/// @spec 30602_shot_direction_spec.md
#[allow(clippy::too_many_arguments)]
pub(super) fn update_shot_debug_info(
    debug_info: &mut LastShotDebugInfo,
    player_id: u8,
    ball_pos: Vec3,
    input: Vec2,
    court_side: CourtSide,
    effective_power: f32,
    spin: f32,
    accuracy: f32,
    trajectory_result: &TrajectoryResult,
    shot_velocity: Vec3,
    config: &GameConfig,
) {
    // discriminant と g_eff を再計算
    let g_eff = crate::systems::trajectory_calculator::calculate_effective_gravity(
        spin,
        ball_pos.y,
        config,
    );
    let dx = trajectory_result.landing_position.x - ball_pos.x;
    let dz = trajectory_result.landing_position.z - ball_pos.z;
    let horizontal_distance = (dx * dx + dz * dz).sqrt();
    let h = trajectory_result.landing_position.y - ball_pos.y;
    let v = trajectory_result.final_speed;
    let v2 = v * v;
    let v4 = v2 * v2;
    let discriminant = v4 - g_eff * (g_eff * horizontal_distance * horizontal_distance + 2.0 * h * v2);

    debug_info.is_valid = true;
    debug_info.player_id = player_id;
    debug_info.ball_pos = ball_pos;
    debug_info.input = input;
    debug_info.court_side = Some(court_side);
    debug_info.power = effective_power;
    debug_info.spin = spin;
    debug_info.accuracy = accuracy;
    debug_info.landing = trajectory_result.landing_position;
    debug_info.launch_angle = trajectory_result.launch_angle;
    debug_info.final_speed = trajectory_result.final_speed;
    debug_info.velocity = shot_velocity;
    debug_info.discriminant = discriminant;
    debug_info.g_eff = g_eff;
}

/// 安定性による威力減衰係数を計算
/// @spec 30604_shot_attributes_spec.md#req-30604-069
/// ランダム性なし: 同じ入力 → 同じ出力
pub(super) fn calculate_stability_power_factor(
    stability: f32,
    config: &crate::resource::config::ShotAttributesConfig,
) -> f32 {
    if stability >= config.stability_threshold {
        return 1.0;
    }

    // 安定性が低いほど威力減衰
    let power_reduction =
        (config.stability_threshold - stability) / config.stability_threshold;
    1.0 - power_reduction * 0.5
}
