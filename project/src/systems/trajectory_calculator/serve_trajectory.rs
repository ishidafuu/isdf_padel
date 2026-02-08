//! サーブ軌道計算
//! @spec 30605_trajectory_calculation_spec.md

use bevy::prelude::*;

use crate::core::CourtSide;
use crate::resource::config::{GameConfig, ServeSide};
use crate::systems::match_control::get_service_box;

use super::launch_angle::calculate_launch_angle;
use super::physics_utils::{
    calculate_direction_vector, calculate_effective_gravity, lerp, CourtSideExt,
};
use super::types::{ServeTrajectoryContext, TrajectoryResult};

/// サーブ用着地地点を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-050
/// @spec 30605_trajectory_calculation_spec.md#req-30605-051
/// @spec 30605_trajectory_calculation_spec.md#req-30605-052
pub fn calculate_serve_landing_position(
    input: Vec2,
    server: CourtSide,
    serve_side: ServeSide,
    config: &GameConfig,
) -> Vec3 {
    let service_box = get_service_box(server, serve_side, config);
    let margin = config.trajectory.landing_margin;

    // REQ-30605-051: 前後入力による深さ調整
    // input.y: -1.0=ネット際, 0.0=中央, +1.0=サービスライン際
    let depth_t = (input.y + 1.0) / 2.0; // -1..1 → 0..1
    let target_x = lerp(
        service_box.x_min + margin * server.sign(),
        service_box.x_max - margin * server.sign(),
        depth_t,
    );

    // REQ-30605-052: 左右入力によるコース調整
    // input.x: -1.0=左端, 0.0=中央, +1.0=右端
    let width_t = (input.x + 1.0) / 2.0; // -1..1 → 0..1
    let target_z = lerp(
        service_box.z_min + margin,
        service_box.z_max - margin,
        width_t,
    );

    Vec3::new(target_x, 0.0, target_z)
}

/// サーブ用弾道を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-050
/// @spec 30605_trajectory_calculation_spec.md#req-30605-053
/// @spec 30605_trajectory_calculation_spec.md#req-30605-054
pub fn calculate_serve_trajectory(
    ctx: &ServeTrajectoryContext,
    config: &GameConfig,
) -> TrajectoryResult {
    let trajectory_config = &config.trajectory;
    let court_config = &config.court;

    // 1. サービスボックス内の着地地点を決定
    let landing_position =
        calculate_serve_landing_position(ctx.input, ctx.server, ctx.serve_side, config);

    // 2. 有効重力を計算（サーブはフラット: spin = 0）
    let effective_gravity = calculate_effective_gravity(0.0, ctx.hit_position.y, config);

    // 3. 発射角度と調整後初速を計算（サーブは着地点調整なし）
    let (launch_angle, adjusted_speed, adjusted_landing) = calculate_launch_angle(
        ctx.hit_position,
        landing_position,
        ctx.base_speed,
        effective_gravity,
        trajectory_config,
        court_config.net_x,
        court_config.net_height,
    );

    // 4. 方向ベクトルを計算
    let direction = calculate_direction_vector(ctx.hit_position, adjusted_landing, launch_angle);

    TrajectoryResult {
        launch_angle,
        final_speed: adjusted_speed,
        direction,
        landing_position: adjusted_landing,
    }
}
