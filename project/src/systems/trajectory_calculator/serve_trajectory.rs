//! サーブ軌道計算
//! @spec 30605_trajectory_calculation_spec.md

use bevy::prelude::*;

use crate::core::CourtSide;
use crate::resource::config::{GameConfig, ServeSide};
use crate::systems::match_control::get_service_box;

use super::launch_angle::calculate_launch_angle;
use super::physics_utils::{
    calculate_direction_vector, calculate_effective_gravity, calculate_speed_for_target, lerp,
    CourtSideExt,
};
use super::types::{ServeTrajectoryContext, TrajectoryResult};

#[inline]
fn calculate_toss_factor(toss_velocity_y: f32, config: &GameConfig) -> f32 {
    let min_v = config
        .serve
        .toss_velocity_min_y
        .min(config.serve.toss_velocity_max_y);
    let max_v = config
        .serve
        .toss_velocity_min_y
        .max(config.serve.toss_velocity_max_y);
    let range = (max_v - min_v).max(0.001);
    ((toss_velocity_y - min_v) / range).clamp(0.0, 1.0)
}

#[inline]
fn calculate_contact_factor(hit_height_y: f32, config: &GameConfig) -> f32 {
    let min_h = config.serve.hit_height_min.min(config.serve.hit_height_max);
    let max_h = config.serve.hit_height_min.max(config.serve.hit_height_max);
    let range = (max_h - min_h).max(0.001);
    ((hit_height_y - min_h) / range).clamp(0.0, 1.0)
}

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

    // サーブの長短は打点（ヒット高さ）で決まるため、十字キーでは固定中間値を使用
    let depth_t = 0.5;
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
    let mut landing_position =
        calculate_serve_landing_position(ctx.input, ctx.server, ctx.serve_side, config);
    let toss_factor = calculate_toss_factor(ctx.toss_velocity_y, config);
    let contact_factor = calculate_contact_factor(ctx.hit_position.y, config);

    // 高い打点ほど奥、低い打点ほど手前に寄せる（サービスボックス内にクランプ）
    let service_box = get_service_box(ctx.server, ctx.serve_side, config);
    let margin = config.trajectory.landing_margin;
    let x_low = service_box.x_min.min(service_box.x_max) + margin;
    let x_high = service_box.x_min.max(service_box.x_max) - margin;
    let depth_shift = lerp(
        -config.serve.toss_depth_shift,
        config.serve.toss_depth_shift,
        contact_factor,
    );
    landing_position.x =
        (landing_position.x + depth_shift * ctx.server.sign()).clamp(x_low, x_high);

    // 2. 有効重力を計算（サーブはフラット: spin = 0）
    let effective_gravity = calculate_effective_gravity(0.0, ctx.hit_position.y, config);

    // 3. 発射角度と調整後初速を計算（サーブは着地点調整なし）
    let (mut launch_angle, mut adjusted_speed, adjusted_landing) = calculate_launch_angle(
        ctx.hit_position,
        landing_position,
        ctx.base_speed,
        effective_gravity,
        trajectory_config,
        court_config.net_x,
        court_config.net_height,
    );

    // 高トス時は発射角を上げて高弾道化する
    let launch_angle_bonus = toss_factor * config.serve.toss_launch_angle_bonus_deg;
    if launch_angle_bonus > 0.01 {
        let boosted_angle =
            (launch_angle + launch_angle_bonus).min(trajectory_config.max_launch_angle);
        if (boosted_angle - launch_angle).abs() > 0.01 {
            let dx = adjusted_landing.x - ctx.hit_position.x;
            let dz = adjusted_landing.z - ctx.hit_position.z;
            let distance = (dx * dx + dz * dz).sqrt();
            let height_diff = adjusted_landing.y - ctx.hit_position.y;
            let boosted_speed =
                calculate_speed_for_target(boosted_angle, distance, effective_gravity, height_diff);
            if boosted_speed > 0.0 {
                launch_angle = boosted_angle;
                adjusted_speed = boosted_speed;
            }
        }
    }

    // 4. 方向ベクトルを計算
    let direction = calculate_direction_vector(ctx.hit_position, adjusted_landing, launch_angle);

    TrajectoryResult {
        launch_angle,
        final_speed: adjusted_speed,
        direction,
        landing_position: adjusted_landing,
    }
}
