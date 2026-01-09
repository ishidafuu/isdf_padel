//! 弾道計算モジュール
//! @spec 30605_trajectory_calculation_spec.md
//!
//! 入力から着地地点を決定し、放物線公式で発射角度を逆算するシステム

use bevy::prelude::*;
use rand::Rng;

use crate::core::CourtSide;
use crate::resource::config::{CourtConfig, GameConfig, TrajectoryConfig};

/// 弾道計算結果
/// @spec 30605_trajectory_calculation_spec.md
#[derive(Debug, Clone)]
pub struct TrajectoryResult {
    /// 発射角度（度）
    pub launch_angle: f32,
    /// 最終初速
    pub final_speed: f32,
    /// 発射方向ベクトル（正規化済み）
    pub direction: Vec3,
    /// 着地予定地点
    pub landing_position: Vec3,
}

/// 弾道計算コンテキスト
/// 計算に必要な入力パラメータをまとめる
#[derive(Debug, Clone)]
pub struct TrajectoryContext {
    /// 入力方向（X=左右, Y=前後）
    pub input: Vec2,
    /// コートサイド
    pub court_side: CourtSide,
    /// ボールの現在位置
    pub ball_position: Vec3,
    /// スピン値（-1.0〜+1.0）
    pub spin: f32,
    /// 基準初速（ショット属性から）
    pub base_speed: f32,
    /// 精度（ショット属性から）
    pub accuracy: f32,
}

/// 着地地点を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-010
/// @spec 30605_trajectory_calculation_spec.md#req-30605-011
/// @spec 30605_trajectory_calculation_spec.md#req-30605-012
/// @spec 30605_trajectory_calculation_spec.md#req-30605-013
pub fn calculate_landing_position(
    ctx: &TrajectoryContext,
    court_config: &CourtConfig,
    trajectory_config: &TrajectoryConfig,
) -> Vec3 {
    let margin = trajectory_config.landing_margin;
    let half_width = court_config.width / 2.0;
    let half_depth = court_config.depth / 2.0;

    // ネット位置
    let net_x = court_config.net_x;

    // コートサイドに応じた座標変換
    let (baseline_x, _net_side_x) = match ctx.court_side {
        CourtSide::Left => {
            // Left側は+X方向に打つ → 相手側ベースライン = +half_depth
            (half_depth, net_x)
        }
        CourtSide::Right => {
            // Right側は-X方向に打つ → 相手側ベースライン = -half_depth
            (-half_depth, net_x)
        }
    };

    // REQ-30605-010, REQ-30605-011: 前後入力による深さ調整
    // input.y: -1.0=ネット際, 0.0=サービスライン付近, +1.0=ベースライン際
    let target_x = if ctx.input.y.abs() < 0.01 {
        // ニュートラル: デフォルト着地深さを使用
        let depth = trajectory_config.default_landing_depth;
        match ctx.court_side {
            CourtSide::Left => net_x + depth,
            CourtSide::Right => net_x - depth,
        }
    } else {
        // 入力あり: 線形補間
        // Left側の場合: input.y=-1 → ネット際, input.y=+1 → ベースライン際
        let near = net_x + margin * ctx.court_side.sign();
        let far = baseline_x - margin * ctx.court_side.sign();
        let t = (ctx.input.y + 1.0) / 2.0; // -1..1 → 0..1
        lerp(near, far, t)
    };

    // REQ-30605-012: 左右入力によるコース調整
    // input.x: -1.0=左サイド, 0.0=中央, +1.0=右サイド
    let target_z = ctx.input.x * (half_width - margin);

    Vec3::new(target_x, 0.0, target_z)
}

/// 有効重力を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-020
pub fn calculate_effective_gravity(
    spin: f32,
    initial_height: f32,
    config: &GameConfig,
) -> f32 {
    let gravity = config.physics.gravity.abs();
    let spin_config = &config.spin_physics;

    // 飛行時間推定（簡易計算）
    let estimated_flight_time = if initial_height > 0.1 {
        2.0 * (initial_height / gravity).sqrt()
    } else {
        1.0 // デフォルト
    };

    // 平均スピン効果
    let avg_spin = spin * (1.0 - spin_config.spin_decay_rate * estimated_flight_time / 2.0);

    // 有効重力 = 基本重力 × (1 + スピン効果)
    // トップスピン(+) → 重力増加（落ちやすい）
    // スライス(-) → 重力減少（浮きやすい）
    gravity * (1.0 + avg_spin * spin_config.gravity_spin_factor)
}

/// 発射角度を逆算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-021
/// @spec 30605_trajectory_calculation_spec.md#req-30605-022
/// @spec 30605_trajectory_calculation_spec.md#req-30605-024
pub fn calculate_launch_angle(
    start_pos: Vec3,
    target_pos: Vec3,
    base_speed: f32,
    effective_gravity: f32,
    trajectory_config: &TrajectoryConfig,
) -> (f32, f32) {
    let dx = target_pos.x - start_pos.x;
    let dz = target_pos.z - start_pos.z;
    let horizontal_distance = (dx * dx + dz * dz).sqrt();

    // 高さの差（打点高さ - 着地高さ）
    let h = start_pos.y - target_pos.y;

    // 基準初速
    let mut v = base_speed;
    let g = effective_gravity;

    // 放物線公式で角度を逆算（最大3回まで初速調整）
    for _ in 0..3 {
        let d = horizontal_distance;
        let v2 = v * v;
        let v4 = v2 * v2;

        // 判別式: v^4 - g(g*d^2 + 2*h*v^2)
        let discriminant = v4 - g * (g * d * d + 2.0 * h * v2);

        if discriminant >= 0.0 {
            // tan(θ) = (v² ± sqrt(discriminant)) / (g * d)
            let sqrt_disc = discriminant.sqrt();
            let tan_theta_1 = (v2 - sqrt_disc) / (g * d);
            let tan_theta_2 = (v2 + sqrt_disc) / (g * d);

            // 角度が低い方を採用（テニス的な軌道）
            let angle_1 = tan_theta_1.atan().to_degrees();
            let angle_2 = tan_theta_2.atan().to_degrees();

            let angle = if angle_1.abs() < angle_2.abs() {
                angle_1
            } else {
                angle_2
            };

            // 範囲制限
            let clamped_angle = angle.clamp(
                trajectory_config.min_launch_angle,
                trajectory_config.max_launch_angle,
            );

            return (clamped_angle, v);
        }

        // 解が得られない場合は初速を10%増加して再計算
        v *= 1.1;
    }

    // 3回試行しても解が得られない場合は最大角度を使用
    (trajectory_config.max_launch_angle, v)
}

/// 球種・距離による初速係数を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-031
/// @spec 30605_trajectory_calculation_spec.md#req-30605-032
/// @spec 30605_trajectory_calculation_spec.md#req-30605-033
pub fn calculate_speed_factors(
    spin: f32,
    horizontal_distance: f32,
    max_court_distance: f32,
    trajectory_config: &TrajectoryConfig,
) -> f32 {
    // 球種係数
    let spin_factor = if spin > 0.1 {
        trajectory_config.spin_speed_topspin
    } else if spin < -0.1 {
        trajectory_config.spin_speed_slice
    } else {
        trajectory_config.spin_speed_flat
    };

    // 距離係数
    let distance_ratio = (horizontal_distance / max_court_distance).clamp(0.0, 1.0);
    let distance_factor = lerp(
        trajectory_config.distance_speed_min,
        trajectory_config.distance_speed_max,
        distance_ratio,
    );

    spin_factor * distance_factor
}

/// 精度によるズレを適用
/// @spec 30605_trajectory_calculation_spec.md#req-30605-040
pub fn apply_landing_deviation(
    target: Vec3,
    accuracy: f32,
    trajectory_config: &TrajectoryConfig,
) -> Vec3 {
    let deviation = (1.0 - accuracy.clamp(0.0, 1.0)) * trajectory_config.max_landing_deviation;

    if deviation < 0.001 {
        return target;
    }

    let mut rng = rand::rng();
    let offset_x = rng.random_range(-1.0..1.0) * deviation;
    let offset_z = rng.random_range(-1.0..1.0) * deviation;

    Vec3::new(target.x + offset_x, target.y, target.z + offset_z)
}

/// 方向ベクトルを計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-023
pub fn calculate_direction_vector(
    start_pos: Vec3,
    target_pos: Vec3,
    launch_angle: f32,
) -> Vec3 {
    let dx = target_pos.x - start_pos.x;
    let dz = target_pos.z - start_pos.z;
    let horizontal_distance = (dx * dx + dz * dz).sqrt();

    if horizontal_distance < 0.001 {
        // ほぼ同じ位置の場合はデフォルト方向
        return Vec3::new(1.0, launch_angle.to_radians().sin(), 0.0).normalize();
    }

    // 水平方向の単位ベクトル
    let horizontal_dir_x = dx / horizontal_distance;
    let horizontal_dir_z = dz / horizontal_distance;

    let angle_rad = launch_angle.to_radians();
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();

    Vec3::new(
        horizontal_dir_x * cos_angle,
        sin_angle,
        horizontal_dir_z * cos_angle,
    )
}

/// 弾道を計算（メイン関数）
/// @spec 30605_trajectory_calculation_spec.md
pub fn calculate_trajectory(ctx: &TrajectoryContext, config: &GameConfig) -> TrajectoryResult {
    let court_config = &config.court;
    let trajectory_config = &config.trajectory;

    // 1. 着地地点を決定
    let raw_landing = calculate_landing_position(ctx, court_config, trajectory_config);

    // 2. 精度によるズレを適用
    let landing_position = apply_landing_deviation(raw_landing, ctx.accuracy, trajectory_config);

    // 3. 有効重力を計算
    let effective_gravity = calculate_effective_gravity(ctx.spin, ctx.ball_position.y, config);

    // 4. 発射角度と調整後初速を計算
    let (launch_angle, adjusted_speed) = calculate_launch_angle(
        ctx.ball_position,
        landing_position,
        ctx.base_speed,
        effective_gravity,
        trajectory_config,
    );

    // 5. 初速係数を計算
    let dx = landing_position.x - ctx.ball_position.x;
    let dz = landing_position.z - ctx.ball_position.z;
    let horizontal_distance = (dx * dx + dz * dz).sqrt();
    let max_court_distance = court_config.depth; // コート全長

    let speed_factor =
        calculate_speed_factors(ctx.spin, horizontal_distance, max_court_distance, trajectory_config);

    // 6. 最終初速
    let final_speed = adjusted_speed * speed_factor;

    // 7. 方向ベクトルを計算
    let direction = calculate_direction_vector(ctx.ball_position, landing_position, launch_angle);

    TrajectoryResult {
        launch_angle,
        final_speed,
        direction,
        landing_position,
    }
}

/// 線形補間
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// CourtSide の符号を取得（計算用ヘルパー）
trait CourtSideExt {
    fn sign(&self) -> f32;
}

impl CourtSideExt for CourtSide {
    fn sign(&self) -> f32 {
        match self {
            CourtSide::Left => 1.0,
            CourtSide::Right => -1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_config() -> GameConfig {
        // テスト用の最小限の設定
        use crate::resource::config::*;
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
            input: Vec2::new(0.0, -1.0), // ネット際
            court_side: CourtSide::Left,
            ball_position: Vec3::new(-5.0, 1.0, 0.0),
            spin: 0.0,
            base_speed: 15.0,
            accuracy: 1.0,
        };

        let landing = calculate_landing_position(&ctx, &config.court, &config.trajectory);

        // ネット際: X = net_x + margin = 0 + 0.5 = 0.5
        assert!(
            landing.x < 2.0,
            "Expected X near net, got {}",
            landing.x
        );
    }

    /// TST-30605-003: 後入力（ベースライン際）着地テスト
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-011
    #[test]
    fn test_backward_landing_position() {
        let config = make_test_config();
        let ctx = TrajectoryContext {
            input: Vec2::new(0.0, 1.0), // ベースライン際
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

        // 右入力
        let ctx_right = TrajectoryContext {
            input: Vec2::new(1.0, 0.0),
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

        // 左入力
        let ctx_left = TrajectoryContext {
            input: Vec2::new(-1.0, 0.0),
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

        let start = Vec3::new(-5.0, 1.0, 0.0);
        let target = Vec3::new(5.0, 0.0, 0.0);
        let base_speed = 15.0;
        let effective_gravity = 9.8;

        let (angle, _speed) =
            calculate_launch_angle(start, target, base_speed, effective_gravity, trajectory_config);

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
}
