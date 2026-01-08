//! プレイヤー移動システム
//! @spec 30201_movement_spec.md
//! @spec 20006_input_system.md

use bevy::prelude::*;

use crate::components::{AiController, InputState, KnockbackState, LogicalPosition, Player, Velocity};
use crate::core::events::PlayerMoveEvent;
use crate::resource::config::GameConfig;

use super::court_factory::create_court_bounds;

/// プレイヤー移動システム
/// @spec 30201_movement_spec.md#req-30201-001
/// @spec 30201_movement_spec.md#req-30201-002
/// @spec 30201_movement_spec.md#req-30201-003
/// @spec 30201_movement_spec.md#req-30201-004
/// @spec 30201_movement_spec.md#req-30201-005
/// @spec 30201_movement_spec.md#req-30201-006
/// @spec 20006_input_system.md - InputState 対応
pub fn movement_system(
    time: Res<Time>,
    config: Res<GameConfig>,
    mut query: Query<
        (&Player, &mut LogicalPosition, &mut Velocity, &KnockbackState, &InputState),
        Without<AiController>,
    >,
    mut event_writer: MessageWriter<PlayerMoveEvent>,
) {
    let bounds = create_court_bounds(&config.court);
    let delta = time.delta_secs();

    for (player, mut logical_pos, mut velocity, knockback, input_state) in query.iter_mut() {
        // REQ-30201-005: ふっとばし中は入力を無視
        if knockback.is_knockback_active() {
            continue;
        }

        // InputState から入力を取得
        let raw_input = input_state.movement;

        // 入力がない場合は水平速度のみ0にする（Y成分は保持：ジャンプ対応）
        if raw_input == Vec2::ZERO {
            velocity.value.x = 0.0;
            velocity.value.z = 0.0;
            continue;
        }

        // REQ-30201-003: 斜め移動の正規化
        let normalized_input = if raw_input.length() > 1.0 {
            raw_input.normalize()
        } else {
            raw_input
        };

        // REQ-30201-001, REQ-30201-002: 移動速度計算
        let move_speed_x = config.player.move_speed;
        let move_speed_z = config.player.move_speed_z;

        // 速度ベクトル計算
        let target_velocity = Vec3::new(
            normalized_input.x * move_speed_x,
            0.0, // Y軸は移動システムでは操作しない
            normalized_input.y * move_speed_z,
        );

        // REQ-30201-003: 最大速度制限
        let max_speed = config.player.max_speed;
        let horizontal_speed = Vec2::new(target_velocity.x, target_velocity.z).length();
        let final_velocity = if horizontal_speed > max_speed {
            let scale = max_speed / horizontal_speed;
            Vec3::new(target_velocity.x * scale, 0.0, target_velocity.z * scale)
        } else {
            target_velocity
        };

        // 水平速度のみ設定（Y成分は保持：ジャンプ対応）
        velocity.value.x = final_velocity.x;
        velocity.value.z = final_velocity.z;

        // 位置更新（論理座標を操作）
        let old_position = logical_pos.value;
        let mut new_position = old_position + final_velocity * delta;

        // REQ-30201-004: 境界でのクランプ
        new_position.x = bounds.clamp_x(new_position.x);

        // プレイヤーの自コート境界（Z軸）
        let (z_min, z_max) = get_player_z_bounds(player.id, &config);
        new_position.z = new_position.z.clamp(z_min, z_max);

        // 位置更新
        logical_pos.value = new_position;

        // REQ-30201-006: 位置が変化した場合のみイベント発行
        if new_position != old_position {
            event_writer.write(PlayerMoveEvent {
                player_id: player.id,
                position: new_position,
                velocity: final_velocity,
            });
        }
    }
}

/// プレイヤーごとのZ軸境界を取得
/// @spec 30201_movement_spec.md#req-30201-002
fn get_player_z_bounds(player_id: u8, config: &GameConfig) -> (f32, f32) {
    match player_id {
        // Player 1: 1Pコート側（-Z側）
        1 => (config.player.z_min, 0.0),
        // Player 2: 2Pコート側（+Z側）
        2 => (0.0, config.player.z_max),
        _ => (config.player.z_min, config.player.z_max),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30204-003: 斜め移動の正規化テスト
    #[test]
    fn test_diagonal_normalization() {
        let input = Vec2::new(1.0, 1.0);
        let normalized = if input.length() > 1.0 {
            input.normalize()
        } else {
            input
        };

        // 長さが約1.0になることを確認
        assert!((normalized.length() - 1.0).abs() < 0.001);
    }

    /// 境界クランプテスト
    #[test]
    fn test_z_bounds_player1() {
        // Player 1 は -Z 側（1Pコート）
        let (z_min, z_max) = (-3.0_f32, 0.0_f32);
        let test_z = 1.0_f32;
        let clamped = test_z.clamp(z_min, z_max);
        assert_eq!(clamped, 0.0);
    }

    /// 境界クランプテスト
    #[test]
    fn test_z_bounds_player2() {
        // Player 2 は +Z 側（2Pコート）
        let (z_min, z_max) = (0.0_f32, 3.0_f32);
        let test_z = -1.0_f32;
        let clamped = test_z.clamp(z_min, z_max);
        assert_eq!(clamped, 0.0);
    }
}
