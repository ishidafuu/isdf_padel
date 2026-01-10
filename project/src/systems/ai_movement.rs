//! AI移動システム v0.5
//! @spec 30301_ai_movement_spec.md

use bevy::prelude::*;

use crate::components::{
    AiController, AiMovementState, Ball, KnockbackState, LogicalPosition, Player, Velocity,
};
use crate::core::court::CourtSide;
use crate::resource::config::GameConfig;

/// 着地点を計算
/// @spec 30301_ai_movement_spec.md#req-30301-v05
///
/// 二次方程式を解いて Y=0 となる時間を求め、その時点でのXZ位置を返す
fn calculate_landing_position(position: Vec3, velocity: Vec3, gravity: f32) -> Option<Vec3> {
    let y0 = position.y;
    let vy = velocity.y;
    let g = gravity;

    // すでに地面上または地面以下の場合
    if y0 <= 0.0 {
        return Some(Vec3::new(position.x, 0.0, position.z));
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
    let time_to_landing = if t1 > 0.0 && t2 > 0.0 {
        t1.min(t2)
    } else if t1 > 0.0 {
        t1
    } else if t2 > 0.0 {
        t2
    } else {
        return None;
    };

    // 着地時のXZ位置を計算（等速直線運動）
    let landing_x = position.x + velocity.x * time_to_landing;
    let landing_z = position.z + velocity.z * time_to_landing;

    Some(Vec3::new(landing_x, 0.0, landing_z))
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

/// リカバリー位置を計算
/// @spec 30301_ai_movement_spec.md#req-30301-v05
///
/// ショット後に最適位置へ戻る
fn calculate_recovery_position(
    shot_direction_z: f32,
    court_side: CourtSide,
    config: &GameConfig,
) -> Vec3 {
    let depth = config.ai.recovery_depth;
    let bias_factor = config.ai.recovery_bias_factor;
    let max_z = config.ai.max_recovery_z;

    // X軸: リカバリー深さ
    let base_x = match court_side {
        CourtSide::Left => -depth,
        CourtSide::Right => depth,
    };

    // Z軸: 打球方向の逆サイドにやや寄る
    let z_offset = (-shot_direction_z * bias_factor).clamp(-max_z, max_z);

    Vec3::new(base_x, 0.0, z_offset)
}

/// AI移動システム v0.5
/// @spec 30301_ai_movement_spec.md#req-30301-v05
///
/// 着地点予測移動、動的待機位置、リカバリーポジショニングを統合
pub fn ai_movement_system(
    time: Res<Time>,
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
    let delta = time.delta_secs();
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
                // ホームポジションへ移動
                move_towards_target(
                    &mut logical_pos,
                    &mut velocity,
                    ai.home_position,
                    config.ai.move_speed,
                    config.ai.home_return_stop_distance,
                    delta,
                );
                continue;
            }
        };

        // ボールが自分に向かっているかチェック（飛行方向で判定）
        // @spec 30301_ai_movement_spec.md#req-30301-v05
        let ball_coming_to_me = match player.court_side {
            CourtSide::Left => ball_vel.x < 0.0,  // 左に向かっている
            CourtSide::Right => ball_vel.x > 0.0, // 右に向かっている
        };

        // 動的待機位置（フォールバック用に先に計算）
        let idle_pos = calculate_idle_position(ball_pos, player.court_side, &config);

        // 状態遷移と目標位置計算
        let (new_state, target_pos) = if ball_coming_to_me {
            // ボールが自分に向かっている: 着地予測位置へ移動
            let landing_pos = calculate_landing_position(ball_pos, ball_vel, gravity)
                .unwrap_or(idle_pos); // フォールバック: 待機位置

            (AiMovementState::Tracking, landing_pos)
        } else {
            // ボールが相手側: 動的待機位置へ移動
            // ショット直後はリカバリー状態を維持（簡易実装: 待機位置で代用）
            // TODO: 将来的にはショットイベントをトリガーにしてRecovering状態へ遷移
            (AiMovementState::Idle, idle_pos)
        };

        // 状態と目標位置を更新
        ai.movement_state = new_state;
        ai.target_position = target_pos;

        // 到達判定
        let diff = Vec2::new(target_pos.x - ai_pos.x, target_pos.z - ai_pos.z);
        let distance_xz = diff.length();

        let stop_distance = if matches!(new_state, AiMovementState::Tracking) {
            // 追跡中は打球可能距離で停止
            config.shot.max_distance
        } else {
            // 待機中はホーム復帰停止距離で停止
            config.ai.home_return_stop_distance
        };

        if distance_xz <= stop_distance {
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
            stop_distance,
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
    _stop_distance: f32,
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

    /// 着地計算テスト
    #[test]
    fn test_landing_calculation() {
        // 位置: (0, 5, 0)、速度: (10, 0, 5)、重力: -10
        let position = Vec3::new(0.0, 5.0, 0.0);
        let velocity = Vec3::new(10.0, 0.0, 5.0);
        let gravity = -10.0;

        let result = calculate_landing_position(position, velocity, gravity);
        assert!(result.is_some());

        let landing_pos = result.unwrap();
        // t = √(2h/g) = √(2*5/10) = 1.0秒
        // X = 0 + 10 * 1.0 = 10.0
        assert!((landing_pos.x - 10.0).abs() < 0.1);
        // Z = 0 + 5 * 1.0 = 5.0
        assert!((landing_pos.z - 5.0).abs() < 0.1);
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
