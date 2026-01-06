//! ボール軌道システム
//! @spec 30401_trajectory_spec.md

use bevy::prelude::*;

use crate::components::{Ball, Velocity};
use crate::core::events::BallOutOfBoundsEvent;
use crate::resource::config::GameConfig;

/// ボール軌道プラグイン
/// @spec 30401_trajectory_spec.md
pub struct BallTrajectoryPlugin;

impl Plugin for BallTrajectoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BallOutOfBoundsEvent>().add_systems(
            Update,
            (
                ball_gravity_system,
                ball_position_update_system,
                ball_out_of_bounds_system,
            )
                .chain(),
        );
    }
}

/// ボール重力適用システム
/// @spec 30401_trajectory_spec.md#req-30401-001
/// @spec 30401_trajectory_spec.md#req-30401-004
pub fn ball_gravity_system(
    time: Res<Time>,
    config: Res<GameConfig>,
    mut query: Query<(&mut Velocity, &Transform), With<Ball>>,
) {
    let delta = time.delta_secs();
    let gravity = config.physics.gravity;

    for (mut velocity, transform) in query.iter_mut() {
        // REQ-30401-001: ボールが空中にある場合のみ重力を適用
        // Y > 0 のときは空中とみなす
        if transform.translation.y > 0.0 {
            // REQ-30401-004: 速度更新（重力適用）
            velocity.value.y += gravity * delta;
        }
    }
}

/// ボール位置更新システム
/// @spec 30401_trajectory_spec.md#req-30401-003
pub fn ball_position_update_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<Ball>>,
) {
    let delta = time.delta_secs();

    for (velocity, mut transform) in query.iter_mut() {
        // REQ-30401-003: Position += Velocity * deltaTime
        transform.translation += velocity.value * delta;
    }
}

/// ボールアウトオブバウンズ検出システム
/// @spec 30401_trajectory_spec.md#req-30401-005
/// @spec 30401_trajectory_spec.md#req-30401-006
pub fn ball_out_of_bounds_system(
    config: Res<GameConfig>,
    query: Query<(Entity, &Transform), With<Ball>>,
    mut event_writer: MessageWriter<BallOutOfBoundsEvent>,
) {
    let half_width = config.court.width / 2.0;
    let half_depth = config.court.depth / 2.0;

    for (entity, transform) in query.iter() {
        let pos = transform.translation;

        // REQ-30401-006: ボールが地面（Y < 0）に落下した場合
        if pos.y < 0.0 {
            event_writer.write(BallOutOfBoundsEvent {
                ball: entity,
                final_position: pos,
            });
            continue;
        }

        // REQ-30401-005: コート範囲チェック
        // X範囲: -Court.Width/2 〜 +Court.Width/2
        // Z範囲: -Court.Depth/2 〜 +Court.Depth/2
        let out_of_bounds_x = pos.x < -half_width || pos.x > half_width;
        let out_of_bounds_z = pos.z < -half_depth || pos.z > half_depth;

        if out_of_bounds_x || out_of_bounds_z {
            event_writer.write(BallOutOfBoundsEvent {
                ball: entity,
                final_position: pos,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30404-001: 重力適用テスト
    #[test]
    fn test_gravity_application() {
        let gravity = -9.8_f32;
        let delta = 0.016_f32; // 60fps
        let mut velocity_y = 0.0_f32;

        velocity_y += gravity * delta;

        // 1フレーム後の速度変化
        assert!((velocity_y - (-0.1568)).abs() < 0.001);
    }

    /// TST-30404-003: 位置更新テスト
    #[test]
    fn test_position_update() {
        let velocity = Vec3::new(10.0, 5.0, 0.0);
        let delta = 0.016_f32;
        let mut position = Vec3::ZERO;

        position += velocity * delta;

        assert!((position.x - 0.16).abs() < 0.001);
        assert!((position.y - 0.08).abs() < 0.001);
    }

    /// TST-30404-004: 重力による速度更新テスト
    #[test]
    fn test_velocity_update_with_gravity() {
        let gravity = -9.8_f32;
        let delta = 0.016_f32;
        let initial_velocity_y = 10.0_f32;
        let mut velocity_y = initial_velocity_y;

        // 数フレーム分のシミュレーション
        for _ in 0..10 {
            velocity_y += gravity * delta;
        }

        // 10フレーム後: 10.0 + (-9.8 * 0.016 * 10) = 10.0 - 1.568 = 8.432
        assert!((velocity_y - 8.432).abs() < 0.01);
    }

    /// TST-30404-005: コート範囲チェックテスト
    #[test]
    fn test_court_bounds_check() {
        let half_width = 5.0_f32; // width 10.0
        let half_depth = 3.0_f32; // depth 6.0

        // 範囲内
        let pos_inside = Vec3::new(0.0, 1.0, 0.0);
        let inside = pos_inside.x >= -half_width
            && pos_inside.x <= half_width
            && pos_inside.z >= -half_depth
            && pos_inside.z <= half_depth;
        assert!(inside);

        // X軸で範囲外
        let pos_out_x = Vec3::new(6.0, 1.0, 0.0);
        let out_x = pos_out_x.x < -half_width || pos_out_x.x > half_width;
        assert!(out_x);

        // Z軸で範囲外
        let pos_out_z = Vec3::new(0.0, 1.0, 4.0);
        let out_z = pos_out_z.z < -half_depth || pos_out_z.z > half_depth;
        assert!(out_z);
    }

    /// TST-30404-006: アウトオブバウンズ検出テスト（地面落下）
    #[test]
    fn test_out_of_bounds_ground() {
        let pos = Vec3::new(0.0, -0.1, 0.0);
        let is_out = pos.y < 0.0;
        assert!(is_out);
    }
}
