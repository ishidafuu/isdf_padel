//! メイン弾道計算
//! @spec 30605_trajectory_calculation_spec.md

use crate::resource::config::GameConfig;

use super::landing_position::{apply_landing_deviation, calculate_landing_position};
use super::launch_angle::calculate_launch_angle;
use super::physics_utils::{calculate_direction_vector, calculate_effective_gravity};
use super::types::{TrajectoryContext, TrajectoryResult};

/// 弾道を計算（メイン関数）
/// @spec 30605_trajectory_calculation_spec.md
pub fn calculate_trajectory(ctx: &TrajectoryContext, config: &GameConfig) -> TrajectoryResult {
    let court_config = &config.court;
    let trajectory_config = &config.trajectory;

    // 1. 着地地点を決定
    let raw_landing = calculate_landing_position(ctx, court_config, trajectory_config);

    // 2. 精度によるズレを適用
    let landing_with_deviation = apply_landing_deviation(raw_landing, ctx.accuracy, trajectory_config);

    // 3. 有効重力を計算
    let effective_gravity = calculate_effective_gravity(ctx.spin, ctx.ball_position.y, config);

    // 4. 発射角度と調整後初速を計算（着地点も調整される可能性あり）
    let (launch_angle, adjusted_speed, landing_position) = calculate_launch_angle(
        ctx.ball_position,
        landing_with_deviation,
        ctx.base_speed,
        effective_gravity,
        trajectory_config,
        court_config.net_x,
        court_config.net_height,
    );

    // 6. 最終初速（角度計算と一貫性を保つため、speed_factor は適用しない）
    // 注: speed_factor を適用すると、角度計算時の速度と実際の速度が乖離し、
    //     着地点予測と実際の着地位置にズレが生じる
    let final_speed = adjusted_speed;

    // 8. 方向ベクトルを計算
    let direction = calculate_direction_vector(ctx.ball_position, landing_position, launch_angle);

    TrajectoryResult {
        launch_angle,
        final_speed,
        direction,
        landing_position,
    }
}
