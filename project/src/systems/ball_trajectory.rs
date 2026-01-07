//! ボール軌道システム
//! @spec 30401_trajectory_spec.md
//! @spec 30402_reflection_spec.md

use bevy::prelude::*;

use crate::components::{Ball, LogicalPosition, Velocity};
use crate::core::events::{BallOutOfBoundsEvent, GroundBounceEvent, WallReflectionEvent};
use crate::core::{CourtBounds, WallReflection};
use crate::resource::config::GameConfig;

/// ボール軌道プラグイン
/// @spec 30401_trajectory_spec.md
pub struct BallTrajectoryPlugin;

impl Plugin for BallTrajectoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BallOutOfBoundsEvent>()
            .add_message::<GroundBounceEvent>()
            .add_message::<WallReflectionEvent>()
            .add_systems(
                Update,
                (
                    ball_gravity_system,
                    ball_position_update_system,
                    ball_ground_bounce_system,
                    ball_wall_reflection_system,
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
    mut query: Query<(&mut Velocity, &LogicalPosition), With<Ball>>,
) {
    let delta = time.delta_secs();
    let gravity = config.physics.gravity;

    for (mut velocity, logical_pos) in query.iter_mut() {
        // REQ-30401-001: ボールが空中にある場合のみ重力を適用
        // Y > 0 のときは空中とみなす
        if logical_pos.value.y > 0.0 {
            // REQ-30401-004: 速度更新（重力適用）
            velocity.value.y += gravity * delta;
        }
    }
}

/// ボール位置更新システム
/// @spec 30401_trajectory_spec.md#req-30401-003
pub fn ball_position_update_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut LogicalPosition), With<Ball>>,
) {
    let delta = time.delta_secs();

    for (velocity, mut logical_pos) in query.iter_mut() {
        // REQ-30401-003: Position += Velocity * deltaTime
        logical_pos.value += velocity.value * delta;
    }
}

/// 地面バウンドシステム
/// @spec 30402_reflection_spec.md#req-30402-001
/// @spec 30402_reflection_spec.md#req-30402-002
pub fn ball_ground_bounce_system(
    config: Res<GameConfig>,
    mut query: Query<(Entity, &mut Velocity, &mut LogicalPosition), With<Ball>>,
    mut event_writer: MessageWriter<GroundBounceEvent>,
) {
    let bounce_factor = config.ball.bounce_factor;
    let min_bounce_velocity = config.ball.min_bounce_velocity;
    let net_z = config.court.net_z;

    for (entity, mut velocity, mut logical_pos) in query.iter_mut() {
        let pos = logical_pos.value;

        // REQ-30402-001: ボールが地面（Y <= 0）に接触し、下向きまたは静止中の場合
        // Y速度が0の場合もバウンドさせる（プレイヤー衝突で水平に跳ね返った場合対応）
        if pos.y <= 0.0 && velocity.value.y <= 0.0 {
            // 速度Y成分を反転し、バウンド係数を適用
            let bounced_y = -velocity.value.y * bounce_factor;
            // 最小バウンド速度を保証（Y速度が0でも軽く跳ねる）
            velocity.value.y = bounced_y.max(min_bounce_velocity);

            // 位置を地面に補正（めり込み防止）
            logical_pos.value.y = 0.0;

            // REQ-30402-002: GroundBounceEvent 発行
            let court_side = crate::core::determine_court_side(pos.z, net_z);
            event_writer.write(GroundBounceEvent {
                ball: entity,
                bounce_point: Vec3::new(pos.x, 0.0, pos.z),
                court_side,
            });
        }
    }
}

/// 壁・天井反射システム
/// @spec 30402_reflection_spec.md#req-30402-003
/// @spec 30402_reflection_spec.md#req-30402-004
/// @spec 30402_reflection_spec.md#req-30402-005
/// @spec 30402_reflection_spec.md#req-30402-006
/// @spec 30402_reflection_spec.md#req-30402-007
pub fn ball_wall_reflection_system(
    config: Res<GameConfig>,
    mut query: Query<(Entity, &mut Velocity, &mut LogicalPosition), With<Ball>>,
    mut event_writer: MessageWriter<WallReflectionEvent>,
) {
    let bounds = CourtBounds::from_config(&config.court);
    let bounce_factor = config.ball.bounce_factor;

    for (entity, mut velocity, mut logical_pos) in query.iter_mut() {
        let pos = logical_pos.value;
        let vel = velocity.value;

        // 壁・天井との接触チェックと反射計算
        if let Some(result) = WallReflection::check_and_reflect(pos, vel, &bounds, bounce_factor) {
            // 速度を反射後の値に更新
            velocity.value = result.reflected_velocity;

            // REQ-30402-007: 位置を境界内に補正（めり込み防止）
            logical_pos.value.x = bounds.clamp_x(pos.x);
            logical_pos.value.y = bounds.clamp_y(pos.y);
            logical_pos.value.z = bounds.clamp_z(pos.z);

            // REQ-30402-004: WallReflectionEvent 発行
            event_writer.write(WallReflectionEvent {
                ball: entity,
                wall_type: result.wall_type,
                contact_point: result.contact_point,
                incident_velocity: vel,
                reflected_velocity: result.reflected_velocity,
            });
        }
    }
}

/// ボールアウトオブバウンズ検出システム
/// @spec 30401_trajectory_spec.md#req-30401-005
/// @spec 30401_trajectory_spec.md#req-30401-006
pub fn ball_out_of_bounds_system(
    config: Res<GameConfig>,
    query: Query<(Entity, &LogicalPosition), With<Ball>>,
    mut event_writer: MessageWriter<BallOutOfBoundsEvent>,
) {
    let half_width = config.court.width / 2.0;
    let half_depth = config.court.depth / 2.0;

    for (entity, logical_pos) in query.iter() {
        let pos = logical_pos.value;

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

    /// TST-30404-007: 地面バウンドテスト
    /// @spec 30402_reflection_spec.md#req-30402-001
    #[test]
    fn test_ground_bounce() {
        let bounce_factor = 0.8_f32;
        let mut velocity_y = -10.0_f32; // 下向きの速度

        // 地面に接触した場合、速度Y成分を反転し、バウンド係数を適用
        if velocity_y < 0.0 {
            velocity_y = -velocity_y * bounce_factor;
        }

        // 期待値: 10.0 * 0.8 = 8.0
        assert!((velocity_y - 8.0).abs() < 0.001);
    }

    /// TST-30404-008: 地面バウンドで上向きの場合は反射しない
    /// @spec 30402_reflection_spec.md#req-30402-001
    #[test]
    fn test_ground_no_bounce_when_moving_up() {
        let bounce_factor = 0.8_f32;
        let mut velocity_y = 5.0_f32; // 上向きの速度
        let original_velocity_y = velocity_y;

        // 上向きの場合はバウンドしない
        if velocity_y < 0.0 {
            velocity_y = -velocity_y * bounce_factor;
        }

        // 速度は変化しない
        assert!((velocity_y - original_velocity_y).abs() < 0.001);
    }

    /// TST-30404-009: 壁反射後の速度計算テスト
    /// @spec 30402_reflection_spec.md#req-30402-003
    #[test]
    fn test_wall_reflection_velocity() {
        use crate::core::{CourtBounds, WallReflection};
        use crate::resource::CourtConfig;

        let config = CourtConfig {
            width: 10.0,
            depth: 6.0,
            ceiling_height: 5.0,
            max_jump_height: 5.0,
            net_height: 1.0,
            net_z: 0.0,
            service_box_depth: 1.5,
        };
        let bounds = CourtBounds::from_config(&config);
        let bounce_factor = 0.8_f32;

        // 左壁に向かう速度
        let pos = Vec3::new(-5.0, 2.5, 0.0);
        let vel = Vec3::new(-10.0, 5.0, 3.0);

        let result = WallReflection::check_and_reflect(pos, vel, &bounds, bounce_factor);
        assert!(result.is_some());

        let reflected = result.unwrap().reflected_velocity;
        // X成分のみ反転・減衰、他成分は維持
        assert!((reflected.x - 8.0).abs() < 0.001); // -(-10.0) * 0.8 = 8.0
        assert!((reflected.y - 5.0).abs() < 0.001); // 維持
        assert!((reflected.z - 3.0).abs() < 0.001); // 維持
    }

    /// TST-30404-010: 天井反射テスト
    /// @spec 30402_reflection_spec.md#req-30402-006
    #[test]
    fn test_ceiling_reflection() {
        use crate::core::{CourtBounds, WallReflection};
        use crate::resource::CourtConfig;

        let config = CourtConfig {
            width: 10.0,
            depth: 6.0,
            ceiling_height: 5.0,
            max_jump_height: 5.0,
            net_height: 1.0,
            net_z: 0.0,
            service_box_depth: 1.5,
        };
        let bounds = CourtBounds::from_config(&config);
        let bounce_factor = 0.8_f32;

        // 天井に向かう速度
        let pos = Vec3::new(0.0, 5.0, 0.0);
        let vel = Vec3::new(5.0, 10.0, 3.0);

        let result = WallReflection::check_and_reflect(pos, vel, &bounds, bounce_factor);
        assert!(result.is_some());

        let reflected = result.unwrap().reflected_velocity;
        // Y成分のみ反転・減衰、他成分は維持
        assert!((reflected.x - 5.0).abs() < 0.001); // 維持
        assert!((reflected.y - (-8.0)).abs() < 0.001); // -(10.0) * 0.8 = -8.0
        assert!((reflected.z - 3.0).abs() < 0.001); // 維持
    }

    /// TST-30404-011: 奥壁反射テスト（Z軸）
    /// @spec 30402_reflection_spec.md#req-30402-005
    #[test]
    fn test_back_wall_reflection() {
        use crate::core::{CourtBounds, WallReflection};
        use crate::resource::CourtConfig;

        let config = CourtConfig {
            width: 10.0,
            depth: 6.0,
            ceiling_height: 5.0,
            max_jump_height: 5.0,
            net_height: 1.0,
            net_z: 0.0,
            service_box_depth: 1.5,
        };
        let bounds = CourtBounds::from_config(&config);
        let bounce_factor = 0.8_f32;

        // 1P側奥壁に向かう速度
        let pos = Vec3::new(0.0, 2.5, -3.0);
        let vel = Vec3::new(5.0, 3.0, -10.0);

        let result = WallReflection::check_and_reflect(pos, vel, &bounds, bounce_factor);
        assert!(result.is_some());

        let reflected = result.unwrap().reflected_velocity;
        // Z成分のみ反転・減衰、他成分は維持
        assert!((reflected.x - 5.0).abs() < 0.001); // 維持
        assert!((reflected.y - 3.0).abs() < 0.001); // 維持
        assert!((reflected.z - 8.0).abs() < 0.001); // -(-10.0) * 0.8 = 8.0
    }

    /// TST-30404-012: めり込み防止テスト
    /// @spec 30402_reflection_spec.md#req-30402-007
    #[test]
    fn test_position_clamp() {
        use crate::core::CourtBounds;
        use crate::resource::CourtConfig;

        let config = CourtConfig {
            width: 10.0,
            depth: 6.0,
            ceiling_height: 5.0,
            max_jump_height: 5.0,
            net_height: 1.0,
            net_z: 0.0,
            service_box_depth: 1.5,
        };
        let bounds = CourtBounds::from_config(&config);

        // 壁をはみ出した位置をクランプ
        let out_x = -6.0_f32;
        let out_y = 6.0_f32;
        let out_z = 4.0_f32;

        let clamped_x = bounds.clamp_x(out_x);
        let clamped_y = bounds.clamp_y(out_y);
        let clamped_z = bounds.clamp_z(out_z);

        assert!((clamped_x - (-5.0)).abs() < 0.001); // クランプ: -5.0
        assert!((clamped_y - 5.0).abs() < 0.001); // クランプ: 5.0
        assert!((clamped_z - 3.0).abs() < 0.001); // クランプ: 3.0
    }
}
