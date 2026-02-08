//! ボール軌道システムテスト
//! @spec 30401_trajectory_spec.md
//! @spec 30402_reflection_spec.md

use bevy::prelude::*;

use crate::systems::court_factory::create_court_bounds;

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
    use crate::core::WallReflection;
    use crate::resource::CourtConfig;

    let config = CourtConfig {
        width: 10.0,
        depth: 6.0,
        ceiling_height: 5.0,
        max_jump_height: 5.0,
        net_height: 1.0,
        net_x: 0.0,
        service_box_depth: 1.5,
        outer_wall_z: 8.0,
        outer_wall_x: 10.0,
    };
    let bounds = create_court_bounds(&config);
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
    use crate::core::WallReflection;
    use crate::resource::CourtConfig;

    let config = CourtConfig {
        width: 10.0,
        depth: 6.0,
        ceiling_height: 5.0,
        max_jump_height: 5.0,
        net_height: 1.0,
        net_x: 0.0,
        service_box_depth: 1.5,
        outer_wall_z: 8.0,
        outer_wall_x: 10.0,
    };
    let bounds = create_court_bounds(&config);
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
    use crate::core::WallReflection;
    use crate::resource::CourtConfig;

    let config = CourtConfig {
        width: 10.0,
        depth: 6.0,
        ceiling_height: 5.0,
        max_jump_height: 5.0,
        net_height: 1.0,
        net_x: 0.0,
        service_box_depth: 1.5,
        outer_wall_z: 8.0,
        outer_wall_x: 10.0,
    };
    let bounds = create_court_bounds(&config);
    let bounce_factor = 0.8_f32;

    // 1P側奥壁（back_left = -3.0）に向かう速度
    // 新座標系: X=打ち合い方向, back_leftは-X側
    let pos = Vec3::new(-3.0, 2.5, 0.0);
    let vel = Vec3::new(-10.0, 3.0, 5.0);

    let result = WallReflection::check_and_reflect(pos, vel, &bounds, bounce_factor);
    assert!(result.is_some());

    let reflected = result.unwrap().reflected_velocity;
    // X成分のみ反転・減衰、他成分は維持
    assert!((reflected.x - 8.0).abs() < 0.001); // -(-10.0) * 0.8 = 8.0
    assert!((reflected.y - 3.0).abs() < 0.001); // 維持
    assert!((reflected.z - 5.0).abs() < 0.001); // 維持
}

/// TST-30404-012: めり込み防止テスト
/// @spec 30402_reflection_spec.md#req-30402-007
#[test]
fn test_position_clamp() {
    #[allow(unused_imports)]
    use crate::core::CourtBounds;
    use crate::resource::CourtConfig;

    let config = CourtConfig {
        width: 10.0,
        depth: 6.0,
        ceiling_height: 5.0,
        max_jump_height: 5.0,
        net_height: 1.0,
        net_x: 0.0,
        service_box_depth: 1.5,
        outer_wall_z: 8.0,
        outer_wall_x: 10.0,
    };
    let bounds = create_court_bounds(&config);

    // 壁をはみ出した位置をクランプ
    // 新座標系: X=打ち合い方向（depth）[-3,3], Z=コート幅（width）[-5,5]
    let out_x = -6.0_f32; // X方向: back_left(-3)を超える
    let out_y = 6.0_f32; // Y方向: ceiling(5)を超える
    let out_z = 7.0_f32; // Z方向: right(5)を超える

    let clamped_x = bounds.clamp_x(out_x);
    let clamped_y = bounds.clamp_y(out_y);
    let clamped_z = bounds.clamp_z(out_z);

    assert!((clamped_x - (-3.0)).abs() < 0.001); // クランプ: -3.0（back_left）
    assert!((clamped_y - 5.0).abs() < 0.001); // クランプ: 5.0（ceiling）
    assert!((clamped_z - 5.0).abs() < 0.001); // クランプ: 5.0（right）
}
