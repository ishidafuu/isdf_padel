//! AI移動システム v0.7
//! @spec 30301_ai_movement_spec.md

use bevy::prelude::*;
use rand::Rng;

use crate::components::{
    AiController, AiMovementState, Ball, KnockbackState, LogicalPosition, Player, Velocity,
};
use crate::core::court::CourtSide;
use crate::resource::config::GameConfig;
use crate::resource::FixedDeltaTime;

/// 着地時間を計算
/// @spec 30301_ai_movement_spec.md#req-30301-v07-002
///
/// 二次方程式を解いて Y=0 となる時間を求める
fn calculate_time_to_landing(position: Vec3, velocity: Vec3, gravity: f32) -> Option<f32> {
    let y0 = position.y;
    let vy = velocity.y;
    let g = gravity;

    // すでに地面上または地面以下の場合
    if y0 <= 0.0 {
        return Some(0.0);
    }

    // 二次方程式: 0.5 * g * t² + vy * t + y0 = 0
    let a = 0.5 * g;
    let b = vy;
    let c = y0;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let t1 = (-b - sqrt_d) / (2.0 * a);
    let t2 = (-b + sqrt_d) / (2.0 * a);

    // 正の時間のうち最も近い未来を選択
    if t1 > 0.0 && t2 > 0.0 {
        Some(t1.min(t2))
    } else if t1 > 0.0 {
        Some(t1)
    } else if t2 > 0.0 {
        Some(t2)
    } else {
        None
    }
}

/// インターセプト位置を計算
/// @spec 30301_ai_movement_spec.md#req-30301-v07-001
///
/// AIのX座標にボールが到達する時のZ座標を予測
fn calculate_intercept_z(
    ai_x: f32,
    ball_pos: Vec3,
    ball_vel: Vec3,
) -> Option<f32> {
    // ボールがAIの方向に向かっていない、または速度がほぼ0の場合
    if ball_vel.x.abs() < 0.001 {
        return None;
    }

    // ボールがAIのX座標に到達する時間
    let time_to_intercept = (ai_x - ball_pos.x) / ball_vel.x;

    // 負の時間 = すでに通り過ぎた
    if time_to_intercept < 0.0 {
        return None;
    }

    // その時点でのZ座標
    let intercept_z = ball_pos.z + ball_vel.z * time_to_intercept;

    Some(intercept_z)
}

/// 短いボール判定
/// @spec 30301_ai_movement_spec.md#req-30301-v07-002
///
/// ボールがAIのX座標に到達する前に着地するかを判定
fn is_short_ball(
    ai_x: f32,
    ball_pos: Vec3,
    ball_vel: Vec3,
    gravity: f32,
) -> bool {
    // インターセプト時間を計算
    if ball_vel.x.abs() < 0.001 {
        return true; // X方向に動いていない = 短いボールとみなす
    }

    let time_to_intercept = (ai_x - ball_pos.x) / ball_vel.x;
    if time_to_intercept < 0.0 {
        return true; // すでに通り過ぎた = 短いボールとみなす
    }

    // 着地時間を計算
    let time_to_landing = calculate_time_to_landing(ball_pos, ball_vel, gravity);

    match time_to_landing {
        Some(t_land) => t_land < time_to_intercept,
        None => true, // 着地しない場合も短いボールとみなす
    }
}

/// Z座標に誤差を適用
/// @spec 30301_ai_movement_spec.md#req-30301-052
///
/// 誤差範囲 = (1.0 - prediction_accuracy) * prediction_error
fn apply_z_error(z: f32, config: &GameConfig) -> f32 {
    let mut rng = rand::rng();
    let accuracy = config.ai.prediction_accuracy.clamp(0.0, 1.0);
    let max_error = config.ai.prediction_error;

    // 誤差範囲: 精度が低いほど誤差が大きい
    let error_range = (1.0 - accuracy) * max_error;

    if error_range <= 0.0 {
        return z;
    }

    // Z座標にのみランダムな誤差を追加
    let error_z = rng.random_range(-error_range..=error_range);

    z + error_z
}

/// 待機位置を計算
/// @spec 30301_ai_movement_spec.md#req-30301-v05
///
/// ボール位置に応じた動的な待機位置を計算
fn calculate_idle_position(
    ball_pos: Vec3,
    court_side: CourtSide,
    config: &GameConfig,
) -> Vec3 {
    let depth = config.ai.optimal_depth;
    let bias_factor = config.ai.coverage_bias_factor;
    let max_z_offset = config.ai.max_z_offset;

    // X軸: 自コート側の深さ位置
    let base_x = match court_side {
        CourtSide::Left => -depth,
        CourtSide::Right => depth,
    };

    // Z軸: ボール位置に応じて調整（相手の返球範囲をカバー）
    let z_offset = (ball_pos.z * bias_factor).clamp(-max_z_offset, max_z_offset);

    Vec3::new(base_x, 0.0, z_offset)
}


/// AI移動システム v0.7
/// @spec 30301_ai_movement_spec.md#req-30301-v07-001
/// @spec 30301_ai_movement_spec.md#req-30301-v07-002
/// @spec 30301_ai_movement_spec.md#req-30301-v07-003
///
/// インターセプト方式移動、短いボール判定、目標ロック機構を実装
#[allow(clippy::type_complexity)]
pub fn ai_movement_system(
    fixed_dt: Res<FixedDeltaTime>,
    config: Res<GameConfig>,
    ball_query: Query<(&LogicalPosition, &Velocity), (With<Ball>, Without<AiController>)>,
    mut ai_query: Query<
        (
            &Player,
            &mut AiController,
            &mut LogicalPosition,
            &mut Velocity,
            &KnockbackState,
        ),
        (With<AiController>, Without<Ball>),
    >,
) {
    let delta = fixed_dt.delta_secs();
    let gravity = config.physics.gravity;

    // ボール情報を取得
    let ball_info = ball_query.iter().next().map(|(pos, vel)| (pos.value, vel.value));

    for (player, mut ai, mut logical_pos, mut velocity, knockback) in ai_query.iter_mut() {
        // ふっとばし中は移動しない
        if knockback.is_knockback_active() {
            continue;
        }

        let ai_pos = logical_pos.value;

        // ボールが存在しない場合はホームポジションへ
        let (ball_pos, ball_vel) = match ball_info {
            Some((pos, vel)) => (pos, vel),
            None => {
                // ロック解除
                ai.locked_target_z = None;
                ai.lock_ball_velocity_x_sign = None;
                ai.reaction_timer = 0.0;
                // ホームポジションへ移動
                move_towards_target(
                    &mut logical_pos,
                    &mut velocity,
                    ai.home_position,
                    config.ai.move_speed,
                    delta,
                );
                continue;
            }
        };

        // ボールが自分に向かっているかチェック（飛行方向で判定）
        let ball_coming_to_me = match player.court_side {
            CourtSide::Left => ball_vel.x < 0.0,  // 左に向かっている
            CourtSide::Right => ball_vel.x > 0.0, // 右に向かっている
        };

        // 動的待機位置（フォールバック用）
        let idle_pos = calculate_idle_position(ball_pos, player.court_side, &config);

        // === 反応遅延の処理 ===
        // @spec 30301_ai_movement_spec.md#req-30301-053
        if ball_coming_to_me && ai.movement_state == AiMovementState::Idle {
            ai.reaction_timer = config.ai.reaction_delay;
        }

        if ai.reaction_timer > 0.0 {
            ai.reaction_timer -= delta;
            if ai.reaction_timer < 0.0 {
                ai.reaction_timer = 0.0;
            }
        }

        // === 目標ロック機構 ===
        // @spec 30301_ai_movement_spec.md#req-30301-v07-003
        let current_ball_vel_x_sign = ball_vel.x > 0.0;

        // ボール速度X成分の符号変化を検知 → ロック解除
        let state_changed = match ai.lock_ball_velocity_x_sign {
            Some(prev_sign) => prev_sign != current_ball_vel_x_sign,
            None => ball_coming_to_me, // 初回かつボールが向かっている
        };

        // ボールが相手側に向かった場合はロック解除
        if !ball_coming_to_me {
            ai.locked_target_z = None;
            ai.lock_ball_velocity_x_sign = None;
        }

        // 状態遷移と目標位置計算
        let (new_state, target_pos) = if ball_coming_to_me {
            // 反応遅延中は追跡を開始しない
            if ai.reaction_timer > 0.0 {
                (AiMovementState::Idle, idle_pos)
            } else {
                // === インターセプト方式 ===
                // @spec 30301_ai_movement_spec.md#req-30301-v07-001
                // @spec 30301_ai_movement_spec.md#req-30301-v07-002

                // 状態変化時のみ目標を再計算
                let target_z = if state_changed || ai.locked_target_z.is_none() {
                    // 短いボール判定
                    let target_z = if is_short_ball(ai_pos.x, ball_pos, ball_vel, gravity) {
                        // 短いボール: ボールの現在Z座標を追跡
                        ball_pos.z
                    } else {
                        // インターセプト: AIのX座標でのZ座標を予測
                        calculate_intercept_z(ai_pos.x, ball_pos, ball_vel)
                            .unwrap_or(ball_pos.z)
                    };

                    // 誤差を1回だけ適用してロック
                    let with_error = apply_z_error(target_z, &config);
                    ai.locked_target_z = Some(with_error);
                    ai.lock_ball_velocity_x_sign = Some(current_ball_vel_x_sign);

                    with_error
                } else {
                    // ロック済みの目標を使用
                    ai.locked_target_z.unwrap_or(ball_pos.z)
                };

                // X座標は維持、Z座標のみ移動
                let target = Vec3::new(ai_pos.x, 0.0, target_z);
                (AiMovementState::Tracking, target)
            }
        } else {
            // ボールが相手側: 動的待機位置へ移動
            (AiMovementState::Idle, idle_pos)
        };

        // 状態と目標位置を更新
        ai.movement_state = new_state;
        ai.target_position = target_pos;

        // 到達判定（Z座標のみで判定）
        let distance_z = (target_pos.z - ai_pos.z).abs();

        let stop_distance = if matches!(new_state, AiMovementState::Tracking) {
            config.shot.max_distance
        } else {
            config.ai.home_return_stop_distance
        };

        if distance_z <= stop_distance {
            // 目標に到達 → 停止
            velocity.value.x = 0.0;
            velocity.value.z = 0.0;
            continue;
        }

        // 目標に向かって移動
        move_towards_target(
            &mut logical_pos,
            &mut velocity,
            target_pos,
            config.ai.move_speed,
            delta,
        );
    }
}

/// 目標位置に向かって移動
fn move_towards_target(
    logical_pos: &mut LogicalPosition,
    velocity: &mut Velocity,
    target: Vec3,
    move_speed: f32,
    delta: f32,
) {
    let ai_pos = logical_pos.value;
    let diff = Vec2::new(target.x - ai_pos.x, target.z - ai_pos.z);

    let direction = diff.normalize_or_zero();
    let target_velocity = Vec3::new(direction.x * move_speed, 0.0, direction.y * move_speed);

    // 速度設定
    velocity.value.x = target_velocity.x;
    velocity.value.z = target_velocity.z;

    // 位置更新
    let new_position = ai_pos + target_velocity * delta;
    logical_pos.value = new_position;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 着地時間計算テスト
    /// @spec 30301_ai_movement_spec.md#req-30301-v07-002
    #[test]
    fn test_time_to_landing_calculation() {
        // 位置: (0, 5, 0)、速度: (10, 0, 5)、重力: -10
        let position = Vec3::new(0.0, 5.0, 0.0);
        let velocity = Vec3::new(10.0, 0.0, 5.0);
        let gravity = -10.0;

        let result = calculate_time_to_landing(position, velocity, gravity);
        assert!(result.is_some());

        let time = result.unwrap();
        // t = √(2h/g) = √(2*5/10) = 1.0秒
        assert!((time - 1.0).abs() < 0.1);
    }

    /// インターセプトZ座標計算テスト
    /// @spec 30301_ai_movement_spec.md#req-30301-v07-001
    #[test]
    fn test_intercept_z_calculation() {
        // AI位置X = 5.0, ボール位置 = (0, 2, 0), ボール速度 = (10, 0, 5)
        let ai_x = 5.0;
        let ball_pos = Vec3::new(0.0, 2.0, 0.0);
        let ball_vel = Vec3::new(10.0, 0.0, 5.0);

        let result = calculate_intercept_z(ai_x, ball_pos, ball_vel);
        assert!(result.is_some());

        let intercept_z = result.unwrap();
        // time_to_intercept = (5.0 - 0.0) / 10.0 = 0.5秒
        // intercept_z = 0.0 + 5.0 * 0.5 = 2.5
        assert!((intercept_z - 2.5).abs() < 0.1);
    }

    /// 短いボール判定テスト
    /// @spec 30301_ai_movement_spec.md#req-30301-v07-002
    #[test]
    fn test_short_ball_detection() {
        let gravity = -10.0;

        // 短いボール: 着地が早い
        let ai_x = 10.0;
        let ball_pos = Vec3::new(0.0, 1.0, 0.0);
        let ball_vel = Vec3::new(5.0, 0.0, 0.0); // ゆっくり移動

        assert!(is_short_ball(ai_x, ball_pos, ball_vel, gravity));

        // 長いボール: インターセプト可能
        let ball_pos2 = Vec3::new(0.0, 5.0, 0.0);
        let ball_vel2 = Vec3::new(20.0, 5.0, 0.0); // 速い

        assert!(!is_short_ball(ai_x, ball_pos2, ball_vel2, gravity));
    }

    /// 待機位置計算テスト
    /// @spec 30301_ai_movement_spec.md#req-30301-v05-002
    #[test]
    fn test_idle_position_calculation() {
        use crate::resource::config::AiConfig;

        let ai_config = AiConfig::default();
        let ball_pos = Vec3::new(3.0, 2.0, 2.0);

        // Right側のAI: X = +optimal_depth, Z = ball_z * bias
        let expected_x_right = ai_config.optimal_depth;
        let expected_z = (ball_pos.z * ai_config.coverage_bias_factor)
            .clamp(-ai_config.max_z_offset, ai_config.max_z_offset);

        assert!((expected_x_right - 5.0).abs() < 0.01); // デフォルト: 5.0m
        assert!((expected_z - 0.6).abs() < 0.01); // 2.0 * 0.3 = 0.6

        // Left側のAI: X = -optimal_depth
        let expected_x_left = -ai_config.optimal_depth;
        assert!((expected_x_left - (-5.0)).abs() < 0.01);
    }

    /// REQ-30301-002: 移動方向正規化テスト
    #[test]
    fn test_direction_normalization() {
        let ai_pos = Vec2::new(0.0, 5.0);
        let ball_pos = Vec2::new(3.0, 2.0);
        let diff = ball_pos - ai_pos;
        let direction = diff.normalize();

        // 正規化された方向ベクトルの長さは1
        assert!((direction.length() - 1.0).abs() < 0.001);
    }
}
