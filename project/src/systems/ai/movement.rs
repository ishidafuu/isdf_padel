//! AI移動システム v0.7
//! @spec 30301_ai_movement_spec.md

use bevy::prelude::*;

use crate::components::{
    AiController, AiMovementState, Ball, KnockbackState, LogicalPosition, Player, Velocity,
};
use crate::core::court::CourtSide;
use crate::resource::config::GameConfig;
use crate::resource::{FixedDeltaTime, GameRng};
use crate::simulation::DebugLogger;

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

/// 着地地点を計算
/// @spec 30301_ai_movement_spec.md#req-30301-v08-001
///
/// 放物線軌道から着地時のX, Z座標を計算
fn calculate_landing_position(ball_pos: Vec3, ball_vel: Vec3, gravity: f32) -> Option<Vec3> {
    let time_to_land = calculate_time_to_landing(ball_pos, ball_vel, gravity)?;
    Some(Vec3::new(
        ball_pos.x + ball_vel.x * time_to_land,
        0.0,
        ball_pos.z + ball_vel.z * time_to_land,
    ))
}

/// 軌道ライン上のZ座標を計算
/// @spec 30301_ai_movement_spec.md#req-30301-v08-002
///
/// ボール現在位置と着地点を結ぶ線上で、AIのX座標におけるZ座標を計算
fn calculate_trajectory_line_z(
    ai_x: f32,
    ball_pos: Vec3,
    ball_vel: Vec3,
    gravity: f32,
) -> Option<f32> {
    let landing_pos = calculate_landing_position(ball_pos, ball_vel, gravity)?;

    // ボール位置と着地点を結ぶ線上のZ座標を計算
    let dx = landing_pos.x - ball_pos.x;
    if dx.abs() < 0.001 {
        return Some(ball_pos.z);
    }

    // AIのX座標が軌道線上のどこにあるかを計算
    let t = (ai_x - ball_pos.x) / dx;

    // tが0-1の範囲外の場合でも線形補間を続ける（軌道延長線上）
    let trajectory_z = ball_pos.z + (landing_pos.z - ball_pos.z) * t;
    Some(trajectory_z)
}

/// 短いボール判定
/// @spec 30301_ai_movement_spec.md#req-30301-v07-002
///
/// ボールがAIのX座標に到達する前に着地するかを判定
fn is_short_ball(ai_x: f32, ball_pos: Vec3, ball_vel: Vec3, gravity: f32) -> bool {
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
fn apply_z_error(z: f32, config: &GameConfig, game_rng: &mut GameRng) -> f32 {
    let accuracy = config.ai.prediction_accuracy.clamp(0.0, 1.0);
    let max_error = config.ai.prediction_error;

    // 誤差範囲: 精度が低いほど誤差が大きい
    let error_range = (1.0 - accuracy) * max_error;

    if error_range <= 0.0 {
        return z;
    }

    // Z座標にのみランダムな誤差を追加
    let error_z = game_rng.random_range(-error_range..=error_range);

    z + error_z
}

/// 待機位置を計算
/// @spec 30301_ai_movement_spec.md#req-30301-v05
///
/// ボール位置に応じた動的な待機位置を計算
fn calculate_idle_position(ball_pos: Vec3, court_side: CourtSide, config: &GameConfig) -> Vec3 {
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

/// 反応遅延タイマーを更新
/// @spec 30301_ai_movement_spec.md#req-30301-053
fn update_reaction_timer(
    ai: &mut AiController,
    ball_coming_to_me: bool,
    reaction_delay: f32,
    delta: f32,
) {
    // 反応遅延はボールが初めて向かってきた時のみ設定
    if ball_coming_to_me && ai.movement_state == AiMovementState::Idle && ai.reaction_timer <= 0.0 {
        ai.reaction_timer = reaction_delay;
    }

    if ai.reaction_timer > 0.0 {
        ai.reaction_timer -= delta;
        if ai.reaction_timer < 0.0 {
            ai.reaction_timer = 0.0;
        }
    }
}

/// 目標ロックの状態変化を検出
/// @spec 30301_ai_movement_spec.md#req-30301-v07-003
fn detect_lock_state_change(ai: &AiController, ball_vel_x: f32, ball_coming_to_me: bool) -> bool {
    let current_sign = ball_vel_x > 0.0;
    match ai.lock_ball_velocity_x_sign {
        Some(prev_sign) => prev_sign != current_sign,
        None => ball_coming_to_me,
    }
}

/// 追跡目標位置を計算
/// @spec 30301_ai_movement_spec.md#req-30301-v07-001
/// @spec 30301_ai_movement_spec.md#req-30301-v07-002
#[allow(clippy::too_many_arguments)]
fn calculate_tracking_target(
    ai: &mut AiController,
    ai_pos: Vec3,
    ball_pos: Vec3,
    ball_vel: Vec3,
    gravity: f32,
    state_changed: bool,
    config: &GameConfig,
    game_rng: &mut GameRng,
) -> Vec3 {
    let is_short = is_short_ball(ai_pos.x, ball_pos, ball_vel, gravity);
    let landing_pos = calculate_landing_position(ball_pos, ball_vel, gravity);
    let current_ball_vel_x_sign = ball_vel.x > 0.0;

    let (target_x, target_z) = if state_changed || ai.locked_target_z.is_none() {
        // 軌道ライン追跡
        let (target_x, target_z) = if is_short {
            // 短いボール: 着地地点に向かって移動
            match landing_pos {
                Some(pos) => (pos.x, pos.z),
                None => (ball_pos.x, ball_pos.z),
            }
        } else {
            // 軌道ライン: AIのX座標での軌道ライン上Z座標を予測
            let z = calculate_trajectory_line_z(ai_pos.x, ball_pos, ball_vel, gravity)
                .unwrap_or(ball_pos.z);
            (ai_pos.x, z)
        };

        // 誤差を1回だけ適用してロック
        let with_error = apply_z_error(target_z, config, game_rng);
        ai.locked_target_z = Some(with_error);
        ai.lock_ball_velocity_x_sign = Some(current_ball_vel_x_sign);

        (target_x, with_error)
    } else {
        // ロック済みの目標を使用
        let x = if is_short {
            landing_pos.map(|pos| pos.x).unwrap_or(ball_pos.x)
        } else {
            ai_pos.x
        };
        (x, ai.locked_target_z.unwrap_or(ball_pos.z))
    };

    Vec3::new(target_x, 0.0, target_z)
}

/// 到達距離を計算
/// @spec 30301_ai_movement_spec.md#req-30301-v07-001
fn calculate_arrival_distance(
    ai_pos: Vec3,
    target_pos: Vec3,
    ball_pos: Vec3,
    ball_vel: Vec3,
    gravity: f32,
) -> f32 {
    let is_short = is_short_ball(ai_pos.x, ball_pos, ball_vel, gravity);
    if is_short {
        // 短いボール: XZ平面の距離で判定
        let dx = target_pos.x - ai_pos.x;
        let dz = target_pos.z - ai_pos.z;
        (dx * dx + dz * dz).sqrt()
    } else {
        // 通常: Z座標のみで判定
        (target_pos.z - ai_pos.z).abs()
    }
}

/// ボール不在時のAI状態をリセットしてホームポジションへ移動
fn handle_no_ball_state(
    ai: &mut AiController,
    logical_pos: &mut LogicalPosition,
    velocity: &mut Velocity,
    move_speed: f32,
    delta: f32,
) {
    ai.locked_target_z = None;
    ai.lock_ball_velocity_x_sign = None;
    ai.reaction_timer = 0.0;
    move_towards_target(logical_pos, velocity, ai.home_position, move_speed, delta);
}

/// ボールがAIに向かっているかを判定
fn is_ball_coming_to_ai(court_side: CourtSide, ball_vel_x: f32) -> bool {
    match court_side {
        CourtSide::Left => ball_vel_x < 0.0,
        CourtSide::Right => ball_vel_x > 0.0,
    }
}

/// AIの状態遷移と目標位置を決定
#[allow(clippy::too_many_arguments)]
fn determine_ai_target(
    ai: &mut AiController,
    ai_pos: Vec3,
    ball_pos: Vec3,
    ball_vel: Vec3,
    gravity: f32,
    ball_coming_to_me: bool,
    idle_pos: Vec3,
    state_changed: bool,
    config: &GameConfig,
    game_rng: &mut GameRng,
) -> (AiMovementState, Vec3) {
    if ball_coming_to_me {
        if ai.reaction_timer > 0.0 {
            // @spec 30301_ai_movement_spec.md#req-30301-053
            // 反応遅延中はその場で停止（待機位置に移動しない）
            (AiMovementState::Idle, ai_pos)
        } else {
            let target = calculate_tracking_target(
                ai,
                ai_pos,
                ball_pos,
                ball_vel,
                gravity,
                state_changed,
                config,
                game_rng,
            );
            (AiMovementState::Tracking, target)
        }
    } else {
        (AiMovementState::Idle, idle_pos)
    }
}

/// AI移動のデバッグログを出力
fn log_ai_movement_debug(
    logger: &mut DebugLogger,
    player_id: u8,
    target_pos: Vec3,
    ball_pos: Vec3,
    new_state: AiMovementState,
    prev_state: AiMovementState,
    state_changed: bool,
) {
    if prev_state != new_state || state_changed {
        let reason = match new_state {
            AiMovementState::Tracking => "Tracking",
            AiMovementState::Idle => "Idle",
            AiMovementState::Recovering => "Recovering",
        };
        logger.log_ai(&format!(
            "P{} target=({:.2},{:.2},{:.2}) state={} ball_pos=({:.2},{:.2},{:.2})",
            player_id,
            target_pos.x,
            target_pos.y,
            target_pos.z,
            reason,
            ball_pos.x,
            ball_pos.y,
            ball_pos.z
        ));
    }
}

/// AI移動を実行（到達判定付き）
#[allow(clippy::too_many_arguments)]
fn execute_ai_movement(
    logical_pos: &mut LogicalPosition,
    velocity: &mut Velocity,
    ai_pos: Vec3,
    target_pos: Vec3,
    ball_pos: Vec3,
    ball_vel: Vec3,
    gravity: f32,
    new_state: AiMovementState,
    config: &GameConfig,
    delta: f32,
) {
    let distance = calculate_arrival_distance(ai_pos, target_pos, ball_pos, ball_vel, gravity);
    let stop_distance = if matches!(new_state, AiMovementState::Tracking) {
        config.shot.max_distance
    } else {
        config.ai.home_return_stop_distance
    };

    if distance <= stop_distance {
        velocity.value.x = 0.0;
        velocity.value.z = 0.0;
    } else {
        move_towards_target(
            logical_pos,
            velocity,
            target_pos,
            config.ai.move_speed,
            delta,
        );
    }
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
    mut game_rng: ResMut<GameRng>,
    mut debug_logger: Option<ResMut<DebugLogger>>,
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
    let ball_info = ball_query
        .iter()
        .next()
        .map(|(pos, vel)| (pos.value, vel.value));

    for (player, mut ai, mut logical_pos, mut velocity, knockback) in ai_query.iter_mut() {
        if knockback.is_knockback_active() {
            continue;
        }

        let ai_pos = logical_pos.value;

        // ボール不在時はホームポジションへ
        let Some((ball_pos, ball_vel)) = ball_info else {
            handle_no_ball_state(
                &mut ai,
                &mut logical_pos,
                &mut velocity,
                config.ai.move_speed,
                delta,
            );
            continue;
        };

        let ball_coming_to_me = is_ball_coming_to_ai(player.court_side, ball_vel.x);
        let idle_pos = calculate_idle_position(ball_pos, player.court_side, &config);

        update_reaction_timer(&mut ai, ball_coming_to_me, config.ai.reaction_delay, delta);
        let state_changed = detect_lock_state_change(&ai, ball_vel.x, ball_coming_to_me);

        if !ball_coming_to_me {
            ai.locked_target_z = None;
            ai.lock_ball_velocity_x_sign = None;
        }

        let prev_state = ai.movement_state;
        let (new_state, target_pos) = determine_ai_target(
            &mut ai,
            ai_pos,
            ball_pos,
            ball_vel,
            gravity,
            ball_coming_to_me,
            idle_pos,
            state_changed,
            &config,
            &mut game_rng,
        );

        ai.movement_state = new_state;
        ai.target_position = target_pos;

        if let Some(ref mut logger) = debug_logger {
            log_ai_movement_debug(
                logger,
                player.id,
                target_pos,
                ball_pos,
                new_state,
                prev_state,
                state_changed,
            );
        }

        execute_ai_movement(
            &mut logical_pos,
            &mut velocity,
            ai_pos,
            target_pos,
            ball_pos,
            ball_vel,
            gravity,
            new_state,
            &config,
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

    /// 軌道ラインZ座標計算テスト
    /// @spec 30301_ai_movement_spec.md#req-30301-v08-002
    #[test]
    fn test_trajectory_line_z_calculation() {
        // AI位置X = 5.0, ボール位置 = (0, 5, 0), ボール速度 = (10, 0, 5), 重力 = -10
        // 着地時間 = √(2*5/10) = 1.0秒
        // 着地位置X = 0 + 10 * 1.0 = 10.0
        // 着地位置Z = 0 + 5 * 1.0 = 5.0
        // AIのX=5.0は軌道の中間点（t=0.5）
        // trajectory_z = 0 + (5 - 0) * 0.5 = 2.5
        let ai_x = 5.0;
        let ball_pos = Vec3::new(0.0, 5.0, 0.0);
        let ball_vel = Vec3::new(10.0, 0.0, 5.0);
        let gravity = -10.0;

        let result = calculate_trajectory_line_z(ai_x, ball_pos, ball_vel, gravity);
        assert!(result.is_some());

        let trajectory_z = result.unwrap();
        // 軌道ラインの中間点なので Z = 2.5
        assert!((trajectory_z - 2.5).abs() < 0.1);
    }

    /// 着地地点計算テスト
    /// @spec 30301_ai_movement_spec.md#req-30301-v08-001
    #[test]
    fn test_landing_position_calculation() {
        // ボール位置 = (0, 5, 0), ボール速度 = (10, 0, 5), 重力 = -10
        // 着地時間 = √(2*5/10) = 1.0秒
        // 着地位置 = (10, 0, 5)
        let ball_pos = Vec3::new(0.0, 5.0, 0.0);
        let ball_vel = Vec3::new(10.0, 0.0, 5.0);
        let gravity = -10.0;

        let result = calculate_landing_position(ball_pos, ball_vel, gravity);
        assert!(result.is_some());

        let landing_pos = result.unwrap();
        assert!((landing_pos.x - 10.0).abs() < 0.1);
        assert!((landing_pos.z - 5.0).abs() < 0.1);
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
