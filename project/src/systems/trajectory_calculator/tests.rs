//! 弾道計算テスト
//! @spec 30605_trajectory_calculation_spec.md

use bevy::prelude::*;

use crate::core::CourtSide;
use crate::resource::config::*;
use crate::systems::match_control::get_service_box;

use super::landing_position::{apply_landing_deviation, calculate_landing_position};
use super::launch_angle::calculate_launch_angle;
use super::physics_utils::{calculate_effective_gravity, calculate_speed_factors};
use super::serve_trajectory::calculate_serve_landing_position;
use super::types::TrajectoryContext;

fn make_test_config() -> GameConfig {
    // テスト用の最小限の設定
    GameConfig {
        physics: PhysicsConfig {
            gravity: -9.8,
            max_fall_speed: -20.0,
        },
        court: CourtConfig {
            width: 12.0,
            depth: 16.0,
            ceiling_height: 100.0,
            max_jump_height: 5.0,
            net_height: 1.0,
            net_x: 0.0,
            service_box_depth: 5.0,
            outer_wall_z: 10.0,
            outer_wall_x: 12.0,
        },
        player: PlayerConfig {
            move_speed: 5.0,
            move_speed_z: 4.0,
            max_speed: 10.0,
            jump_force: 8.0,
            friction: 0.9,
            air_control_factor: 0.5,
            x_min: -3.0,
            x_max: 3.0,
        },
        ball: BallConfig {
            normal_shot_speed: 10.0,
            power_shot_speed: 15.0,
            bounce_factor: 0.8,
            radius: 0.2,
            min_bounce_velocity: 1.0,
            wall_bounce_factor: 0.8,
        },
        collision: CollisionConfig {
            character_radius: 0.5,
            z_tolerance: 0.3,
        },
        knockback: KnockbackConfig {
            enabled: true,
            duration: 0.5,
            speed_multiplier: 0.5,
            invincibility_time: 1.0,
        },
        shot: ShotConfig {
            max_distance: 1.5,
            max_height_diff: 2.0,
            cooldown_time: 0.5,
            normal_shot_angle: 45.0,
            jump_shot_angle: 30.0,
            jump_threshold: 0.5,
        },
        scoring: ScoringConfig {
            point_values: vec![0, 15, 30, 40],
            games_to_win_set: 6,
            sets_to_win_match: 1,
            point_end_delay: 1.5,
        },
        input: InputConfig {
            jump_buffer_time: 0.1,
            shot_buffer_time: 0.05,
            normalization_threshold: 1.0,
            input_sensitivity: 1.0,
        },
        input_keys: InputKeysConfig::default(),
        gamepad_buttons: GamepadButtonsConfig::default(),
        shadow: ShadowConfig::default(),
        shot_attributes: ShotAttributesConfig::default(),
        ai: AiConfig::default(),
        visual_feedback: VisualFeedbackConfig::default(),
        player_visual: PlayerVisualConfig::default(),
        serve: ServeConfig::default(),
        spin_physics: SpinPhysicsConfig::default(),
        trajectory: TrajectoryConfig::default(),
        character: CharacterConfig::default(),
    }
}

/// TST-30605-001: ニュートラル着地テスト
/// @spec 30605_trajectory_calculation_spec.md#req-30605-010
#[test]
fn test_neutral_landing_position() {
    let config = make_test_config();
    let ctx = TrajectoryContext {
        input: Vec2::ZERO,
        court_side: CourtSide::Left,
        ball_position: Vec3::new(-5.0, 1.0, 0.0),
        spin: 0.0,
        base_speed: 15.0,
        accuracy: 1.0,
    };

    let landing = calculate_landing_position(&ctx, &config.court, &config.trajectory);

    // ニュートラル時: X = net_x + default_landing_depth = 0 + 4.0 = 4.0
    assert!(
        (landing.x - 4.0).abs() < 0.1,
        "Expected X near 4.0, got {}",
        landing.x
    );
    // 中央
    assert!(
        landing.z.abs() < 0.1,
        "Expected Z near 0, got {}",
        landing.z
    );
}

/// TST-30605-002: 前入力（ネット際）着地テスト
/// @spec 30605_trajectory_calculation_spec.md#req-30605-011
#[test]
fn test_forward_landing_position() {
    let config = make_test_config();
    let ctx = TrajectoryContext {
        input: Vec2::new(-1.0, 0.0), // ネット際（X軸で深さ調整）
        court_side: CourtSide::Left,
        ball_position: Vec3::new(-5.0, 1.0, 0.0),
        spin: 0.0,
        base_speed: 15.0,
        accuracy: 1.0,
    };

    let landing = calculate_landing_position(&ctx, &config.court, &config.trajectory);

    // ネット際: X = net_x + margin = 0 + 0.5 = 0.5
    assert!(landing.x < 2.0, "Expected X near net, got {}", landing.x);
}

/// TST-30605-003: 後入力（ベースライン際）着地テスト
/// @spec 30605_trajectory_calculation_spec.md#req-30605-011
#[test]
fn test_backward_landing_position() {
    let config = make_test_config();
    let ctx = TrajectoryContext {
        input: Vec2::new(1.0, 0.0), // ベースライン際（X軸で深さ調整）
        court_side: CourtSide::Left,
        ball_position: Vec3::new(-5.0, 1.0, 0.0),
        spin: 0.0,
        base_speed: 15.0,
        accuracy: 1.0,
    };

    let landing = calculate_landing_position(&ctx, &config.court, &config.trajectory);

    // ベースライン際: X = baseline - margin = 8.0 - 0.5 = 7.5
    assert!(
        landing.x > 6.0,
        "Expected X near baseline, got {}",
        landing.x
    );
}

/// TST-30605-004: 左右入力による着地テスト
/// @spec 30605_trajectory_calculation_spec.md#req-30605-012
#[test]
fn test_side_landing_position() {
    let config = make_test_config();

    // 右入力（Y軸で幅調整）
    let ctx_right = TrajectoryContext {
        input: Vec2::new(0.0, 1.0),
        court_side: CourtSide::Left,
        ball_position: Vec3::new(-5.0, 1.0, 0.0),
        spin: 0.0,
        base_speed: 15.0,
        accuracy: 1.0,
    };
    let landing_right = calculate_landing_position(&ctx_right, &config.court, &config.trajectory);

    // 右サイド: Z = (width/2 - margin) = 6.0 - 0.5 = 5.5
    assert!(
        landing_right.z > 4.0,
        "Expected Z positive for right input, got {}",
        landing_right.z
    );

    // 左入力（Y軸で幅調整）
    let ctx_left = TrajectoryContext {
        input: Vec2::new(0.0, -1.0),
        court_side: CourtSide::Left,
        ball_position: Vec3::new(-5.0, 1.0, 0.0),
        spin: 0.0,
        base_speed: 15.0,
        accuracy: 1.0,
    };
    let landing_left = calculate_landing_position(&ctx_left, &config.court, &config.trajectory);

    assert!(
        landing_left.z < -4.0,
        "Expected Z negative for left input, got {}",
        landing_left.z
    );
}

/// TST-30605-050: サーブ着地Z座標がサービスボックス内に収まること
/// @spec 30605_trajectory_calculation_spec.md#req-30605-052
#[test]
fn test_serve_landing_position_stays_inside_service_box_z() {
    let config = make_test_config();
    let service_box = get_service_box(CourtSide::Left, ServeSide::Deuce, &config);

    // 左右端入力のどちらでもサービスボックス内に収まること
    let left_course = calculate_serve_landing_position(
        Vec2::new(-1.0, 0.0),
        CourtSide::Left,
        ServeSide::Deuce,
        &config,
    );
    let right_course = calculate_serve_landing_position(
        Vec2::new(1.0, 0.0),
        CourtSide::Left,
        ServeSide::Deuce,
        &config,
    );

    assert!(
        left_course.z >= service_box.z_min && left_course.z <= service_box.z_max,
        "Left course z={} is outside [{}, {}]",
        left_course.z,
        service_box.z_min,
        service_box.z_max
    );
    assert!(
        right_course.z >= service_box.z_min && right_course.z <= service_box.z_max,
        "Right course z={} is outside [{}, {}]",
        right_course.z,
        service_box.z_min,
        service_box.z_max
    );
}

/// TST-30605-005: Right側の着地計算テスト
/// @spec 30605_trajectory_calculation_spec.md#req-30605-013
#[test]
fn test_right_side_landing_position() {
    let config = make_test_config();

    // Right側のニュートラル入力
    let ctx = TrajectoryContext {
        input: Vec2::ZERO,
        court_side: CourtSide::Right,
        ball_position: Vec3::new(5.0, 1.0, 0.0),
        spin: 0.0,
        base_speed: 15.0,
        accuracy: 1.0,
    };

    let landing = calculate_landing_position(&ctx, &config.court, &config.trajectory);

    // Right側のニュートラル時: X = net_x - default_landing_depth = 0 - 4.0 = -4.0
    assert!(
        (landing.x - (-4.0)).abs() < 0.1,
        "Expected X near -4.0 for Right side neutral, got {}",
        landing.x
    );

    // 中央: Z = 0
    assert!(
        landing.z.abs() < 0.5,
        "Expected Z near 0 for neutral input, got {}",
        landing.z
    );
}

/// TST-30605-010: 発射角度逆算テスト
/// @spec 30605_trajectory_calculation_spec.md#req-30605-021
#[test]
fn test_launch_angle_calculation() {
    let config = make_test_config();
    let trajectory_config = &config.trajectory;
    let court_config = &config.court;

    let start = Vec3::new(-5.0, 1.0, 0.0);
    let target = Vec3::new(5.0, 0.0, 0.0);
    let base_speed = 15.0;
    let effective_gravity = 9.8;

    let (angle, _speed, _landing) = calculate_launch_angle(
        start,
        target,
        base_speed,
        effective_gravity,
        trajectory_config,
        court_config.net_x,
        court_config.net_height,
    );

    // 角度が有効範囲内
    assert!(
        angle >= trajectory_config.min_launch_angle,
        "Angle {} below min",
        angle
    );
    assert!(
        angle <= trajectory_config.max_launch_angle,
        "Angle {} above max",
        angle
    );
}

/// TST-30605-011: 有効重力テスト（トップスピン）
/// @spec 30605_trajectory_calculation_spec.md#req-30605-020
#[test]
fn test_effective_gravity_topspin() {
    let config = make_test_config();
    let _base_gravity = config.physics.gravity.abs();

    let g_topspin = calculate_effective_gravity(0.5, 1.0, &config);
    let g_neutral = calculate_effective_gravity(0.0, 1.0, &config);

    // トップスピンは重力増加
    assert!(
        g_topspin > g_neutral,
        "Topspin gravity {} should be > neutral {}",
        g_topspin,
        g_neutral
    );
}

/// TST-30605-012: 有効重力テスト（スライス）
/// @spec 30605_trajectory_calculation_spec.md#req-30605-020
#[test]
fn test_effective_gravity_slice() {
    let config = make_test_config();

    let g_slice = calculate_effective_gravity(-0.5, 1.0, &config);
    let g_neutral = calculate_effective_gravity(0.0, 1.0, &config);

    // スライスは重力減少
    assert!(
        g_slice < g_neutral,
        "Slice gravity {} should be < neutral {}",
        g_slice,
        g_neutral
    );
}

/// TST-30605-020: 球種初速係数テスト（フラット）
/// @spec 30605_trajectory_calculation_spec.md#req-30605-031
#[test]
fn test_spin_speed_factor_flat() {
    let config = make_test_config();
    let factor = calculate_speed_factors(0.0, 5.0, 16.0, &config.trajectory);

    // フラット係数 × 距離係数
    let expected_min = config.trajectory.spin_speed_flat * config.trajectory.distance_speed_min;
    let expected_max = config.trajectory.spin_speed_flat * config.trajectory.distance_speed_max;

    assert!(
        factor >= expected_min && factor <= expected_max,
        "Factor {} out of range [{}, {}]",
        factor,
        expected_min,
        expected_max
    );
}

/// TST-30605-021: 球種初速係数テスト（トップスピン）
/// @spec 30605_trajectory_calculation_spec.md#req-30605-031
#[test]
fn test_spin_speed_factor_topspin() {
    let config = make_test_config();
    let factor_topspin = calculate_speed_factors(0.5, 5.0, 16.0, &config.trajectory);
    let factor_flat = calculate_speed_factors(0.0, 5.0, 16.0, &config.trajectory);

    // トップスピンは遅い
    assert!(
        factor_topspin < factor_flat,
        "Topspin factor {} should be < flat {}",
        factor_topspin,
        factor_flat
    );
}

/// TST-30605-022: 距離初速係数テスト
/// @spec 30605_trajectory_calculation_spec.md#req-30605-032
#[test]
fn test_distance_speed_factor() {
    let config = make_test_config();

    let factor_near = calculate_speed_factors(0.0, 2.0, 16.0, &config.trajectory);
    let factor_far = calculate_speed_factors(0.0, 14.0, 16.0, &config.trajectory);

    // 遠距離は速い
    assert!(
        factor_far > factor_near,
        "Far factor {} should be > near {}",
        factor_far,
        factor_near
    );
}

/// TST-30605-030: ズレ計算テスト
/// @spec 30605_trajectory_calculation_spec.md#req-30605-040
#[test]
fn test_landing_deviation_perfect_accuracy() {
    let config = make_test_config();
    let target = Vec3::new(5.0, 0.0, 2.0);

    let result = apply_landing_deviation(target, 1.0, &config.trajectory);

    // 精度100%ではズレなし
    assert!(
        (result.x - target.x).abs() < 0.001,
        "X should not deviate with perfect accuracy"
    );
    assert!(
        (result.z - target.z).abs() < 0.001,
        "Z should not deviate with perfect accuracy"
    );
}

/// TST-30605-040: 速度調整後のネット通過角度再計算テスト
/// 速度が調整された場合でもネットを超えられることを検証
#[test]
fn test_net_clearance_with_adjusted_speed() {
    let config = make_test_config();
    let trajectory_config = &config.trajectory;
    let court_config = &config.court;

    // 低い打点からの打球（ネットを超えにくい条件）
    let start = Vec3::new(-6.0, 0.5, 0.0); // 打点が低い
    let target = Vec3::new(5.0, 0.0, 0.0); // 遠い着地点
    let base_speed = 12.0; // 中程度の速度
    let effective_gravity = 9.8;

    let (angle, speed, _landing) = calculate_launch_angle(
        start,
        target,
        base_speed,
        effective_gravity,
        trajectory_config,
        court_config.net_x,
        court_config.net_height,
    );

    // ネット通過時の高さを計算して検証
    let dist_to_net = (court_config.net_x - start.x).abs();
    let angle_rad = angle.to_radians();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    if cos_a.abs() > 0.001 {
        let t_net = dist_to_net / (speed * cos_a);
        let height_at_net =
            start.y + speed * sin_a * t_net - 0.5 * effective_gravity * t_net * t_net;

        // ネット高さ（1.0m）を超えていること
        assert!(
            height_at_net > court_config.net_height,
            "Ball should clear net: height_at_net={:.3} > net_height={:.1}, angle={:.1}, speed={:.1}",
            height_at_net, court_config.net_height, angle, speed
        );
    }
}

/// TST-30605-041: 極端に遠い着地点でのネット通過テスト
/// 着地点が遠い場合でも角度が適切に計算されることを検証
#[test]
fn test_net_clearance_with_far_target() {
    let config = make_test_config();
    let trajectory_config = &config.trajectory;
    let court_config = &config.court;

    // ベースライン付近から相手ベースライン付近への打球
    let start = Vec3::new(-7.0, 1.0, 0.0);
    let target = Vec3::new(7.0, 0.0, 0.0); // 非常に遠い
    let base_speed = 15.0;
    let effective_gravity = 9.8;

    let (angle, speed, _landing) = calculate_launch_angle(
        start,
        target,
        base_speed,
        effective_gravity,
        trajectory_config,
        court_config.net_x,
        court_config.net_height,
    );

    // 角度が有効範囲内
    assert!(
        angle >= -90.0 && angle <= trajectory_config.max_launch_angle,
        "Angle {} should be in valid range",
        angle
    );

    // 速度が正
    assert!(speed > 0.0, "Speed should be positive: {}", speed);

    // ネット通過時の高さを検証
    let dist_to_net = (court_config.net_x - start.x).abs();
    let angle_rad = angle.to_radians();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    if cos_a.abs() > 0.001 {
        let t_net = dist_to_net / (speed * cos_a);
        let height_at_net =
            start.y + speed * sin_a * t_net - 0.5 * effective_gravity * t_net * t_net;

        assert!(
            height_at_net > court_config.net_height,
            "Ball should clear net even for far target: height={:.3}",
            height_at_net
        );
    }
}
