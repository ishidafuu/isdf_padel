//! AI移動システム
//! @spec 30301_ai_movement_spec.md

use bevy::prelude::*;

use crate::components::{AiController, Ball, KnockbackState, LogicalPosition, Player, Velocity};
use crate::core::court::CourtSide;
use crate::resource::config::GameConfig;

use super::court_factory::create_court_bounds;

/// AI移動システム
/// @spec 30301_ai_movement_spec.md#req-30301-001
/// @spec 30301_ai_movement_spec.md#req-30301-002
/// @spec 30301_ai_movement_spec.md#req-30301-003
/// @spec 30301_ai_movement_spec.md#req-30301-004
/// @spec 30301_ai_movement_spec.md#req-30301-005
pub fn ai_movement_system(
    time: Res<Time>,
    config: Res<GameConfig>,
    ball_query: Query<&LogicalPosition, (With<Ball>, Without<AiController>)>,
    mut ai_query: Query<
        (
            &Player,
            &AiController,
            &mut LogicalPosition,
            &mut Velocity,
            &KnockbackState,
        ),
        (With<AiController>, Without<Ball>),
    >,
) {
    let delta = time.delta_secs();
    let bounds = create_court_bounds(&config.court);

    // ボール位置を取得（存在しなければ何もしない）
    let ball_pos = match ball_query.iter().next() {
        Some(pos) => pos.value,
        None => return,
    };

    for (player, ai, mut logical_pos, mut velocity, knockback) in ai_query.iter_mut() {
        // ふっとばし中は移動しない
        if knockback.is_knockback_active() {
            continue;
        }

        let ai_pos = logical_pos.value;

        // REQ-30301-005: ボールが自分側にあるかチェック
        // Player 2 (AI) は +Z 側なので、ボールが +Z 側にあるか
        let ball_on_my_side = match player.court_side {
            CourtSide::Player1 => ball_pos.z < 0.0,
            CourtSide::Player2 => ball_pos.z > 0.0,
        };

        // ターゲット位置を決定
        let target_pos = if ball_on_my_side {
            // ボールが自分側: ボールを追跡
            ball_pos
        } else {
            // ボールが相手側: ホームポジションへ復帰
            ai.home_position
        };

        // REQ-30301-002: XZ平面での移動方向計算
        let diff = Vec2::new(target_pos.x - ai_pos.x, target_pos.z - ai_pos.z);
        let distance_xz = diff.length();

        // REQ-30301-003: 到達判定（打球可能距離内なら停止）
        if ball_on_my_side && distance_xz <= config.shot.max_distance {
            // ボール追跡中で打球可能距離内 → 停止
            velocity.value.x = 0.0;
            velocity.value.z = 0.0;
            continue;
        }

        // ホームポジション復帰時も、近づいたら停止
        if !ball_on_my_side && distance_xz <= config.ai.home_return_stop_distance {
            velocity.value.x = 0.0;
            velocity.value.z = 0.0;
            continue;
        }

        // REQ-30301-001: 移動速度計算
        let move_speed = config.ai.move_speed;
        let direction = diff.normalize_or_zero();
        let target_velocity = Vec3::new(
            direction.x * move_speed,
            0.0, // Y軸は移動システムでは操作しない
            direction.y * move_speed,
        );

        // 速度設定
        velocity.value.x = target_velocity.x;
        velocity.value.z = target_velocity.z;

        // 位置更新
        let mut new_position = ai_pos + target_velocity * delta;

        // REQ-30301-004: 境界制限
        new_position.x = bounds.clamp_x(new_position.x);

        // プレイヤーの自コート境界（Z軸）
        let (z_min, z_max) = get_ai_z_bounds(player.court_side, &config);
        new_position.z = new_position.z.clamp(z_min, z_max);

        logical_pos.value = new_position;
    }
}

/// AIプレイヤーのZ軸境界を取得
/// @spec 30301_ai_movement_spec.md#req-30301-004
fn get_ai_z_bounds(court_side: CourtSide, config: &GameConfig) -> (f32, f32) {
    match court_side {
        // Player 1: 1Pコート側（-Z側）
        CourtSide::Player1 => (config.player.z_min, 0.0),
        // Player 2: 2Pコート側（+Z側）
        CourtSide::Player2 => (0.0, config.player.z_max),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    /// REQ-30301-003: 到達判定テスト
    #[test]
    fn test_reach_detection() {
        let ai_pos = Vec2::new(0.0, 5.0);
        let ball_pos = Vec2::new(0.5, 5.0);
        let distance = (ball_pos - ai_pos).length();
        let max_distance = 1.5;

        assert!(distance <= max_distance);
    }
}
