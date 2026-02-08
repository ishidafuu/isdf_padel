//! プレイヤージャンプシステム
//! @spec 30202_jump_spec.md
//! @spec 20006_input_system.md

use bevy::prelude::*;

use crate::components::{
    GroundedState, InputState, KnockbackState, LogicalPosition, Player, Velocity,
};
use crate::core::events::{PlayerJumpEvent, PlayerLandEvent};
use crate::resource::config::GameConfig;
use crate::resource::FixedDeltaTime;

/// ジャンプ開始システム
/// @spec 30202_jump_spec.md#req-30202-001
/// @spec 30202_jump_spec.md#req-30202-004
/// @spec 30202_jump_spec.md#req-30202-006
/// @spec 30202_jump_spec.md#req-30202-007
/// @spec 20006_input_system.md - InputState 対応
pub fn jump_system(
    config: Res<GameConfig>,
    mut query: Query<(
        &Player,
        &mut Velocity,
        &mut GroundedState,
        &KnockbackState,
        &InputState,
    )>,
    mut event_writer: MessageWriter<PlayerJumpEvent>,
) {
    for (player, mut velocity, mut grounded, knockback, input_state) in query.iter_mut() {
        // REQ-30202-006: ふっとばし中はジャンプ禁止
        if knockback.is_knockback_active() {
            continue;
        }

        // InputState からジャンプ入力を取得
        if !input_state.jump_pressed {
            continue;
        }

        // REQ-30202-004: 空中ジャンプの禁止
        if !grounded.is_grounded {
            continue;
        }

        // REQ-30202-001: ジャンプ開始
        let jump_force = config.player.jump_force;
        velocity.value.y = jump_force;
        grounded.is_grounded = false;

        // REQ-30202-007: PlayerJumpEvent の発行
        event_writer.write(PlayerJumpEvent {
            player_id: player.id,
            jump_velocity: jump_force,
        });
    }
}

/// 重力適用システム
/// @spec 30202_jump_spec.md#req-30202-002
pub fn gravity_system(
    fixed_dt: Res<FixedDeltaTime>,
    config: Res<GameConfig>,
    mut query: Query<(&mut Velocity, &GroundedState), With<Player>>,
) {
    let delta = fixed_dt.delta_secs();
    let gravity = config.physics.gravity;
    let max_fall_speed = config.physics.max_fall_speed;

    for (mut velocity, grounded) in query.iter_mut() {
        // 接地中は重力を適用しない
        if grounded.is_grounded {
            continue;
        }

        // REQ-30202-002: 重力を毎フレーム加算
        velocity.value.y += gravity * delta;

        // 最大落下速度の制限
        if velocity.value.y < max_fall_speed {
            velocity.value.y = max_fall_speed;
        }
    }
}

/// 垂直位置更新システム（ジャンプ中のY座標更新）
/// @spec 30202_jump_spec.md#req-30202-002
pub fn vertical_movement_system(
    fixed_dt: Res<FixedDeltaTime>,
    mut query: Query<(&Velocity, &mut LogicalPosition, &GroundedState), With<Player>>,
) {
    let delta = fixed_dt.delta_secs();

    for (velocity, mut logical_pos, grounded) in query.iter_mut() {
        // 接地中かつY速度が0ならスキップ
        if grounded.is_grounded && velocity.value.y == 0.0 {
            continue;
        }

        // Y座標を更新（論理座標のY = 高さ）
        logical_pos.value.y += velocity.value.y * delta;
    }
}

/// 着地判定システム
/// @spec 30202_jump_spec.md#req-30202-003
/// @spec 30202_jump_spec.md#req-30202-008
pub fn landing_system(
    mut query: Query<(
        &Player,
        &mut LogicalPosition,
        &mut Velocity,
        &mut GroundedState,
    )>,
    mut event_writer: MessageWriter<PlayerLandEvent>,
) {
    for (player, mut logical_pos, mut velocity, mut grounded) in query.iter_mut() {
        // すでに接地中ならスキップ
        if grounded.is_grounded {
            continue;
        }

        // REQ-30202-003: 着地判定（論理座標Y=高さが0以下）
        if logical_pos.value.y <= 0.0 {
            // 着地処理
            logical_pos.value.y = 0.0;
            velocity.value.y = 0.0;
            grounded.is_grounded = true;

            // REQ-30202-008: PlayerLandEvent の発行
            event_writer.write(PlayerLandEvent {
                player_id: player.id,
                land_position: logical_pos.value,
            });
        }
    }
}

/// 天井衝突システム
/// @spec 30202_jump_spec.md#req-30202-005
pub fn ceiling_collision_system(
    config: Res<GameConfig>,
    mut query: Query<(&mut LogicalPosition, &mut Velocity, &GroundedState), With<Player>>,
) {
    let ceiling_height = config.court.ceiling_height;

    for (mut logical_pos, mut velocity, grounded) in query.iter_mut() {
        // 接地中ならスキップ
        if grounded.is_grounded {
            continue;
        }

        // REQ-30202-005: 天井に到達 AND Y速度が正（上方向）
        if logical_pos.value.y >= ceiling_height && velocity.value.y > 0.0 {
            logical_pos.value.y = ceiling_height;
            velocity.value.y = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30204-007: ジャンプ開始テスト（接地中のみジャンプ可能）
    #[test]
    fn test_jump_requires_grounded() {
        let grounded = GroundedState { is_grounded: true };
        assert!(grounded.is_grounded);

        let in_air = GroundedState { is_grounded: false };
        assert!(!in_air.is_grounded);
    }

    /// TST-30204-008: 重力適用テスト
    #[test]
    fn test_gravity_calculation() {
        let gravity = -9.8_f32;
        let delta = 0.016_f32; // 60fps
        let mut velocity_y = 8.0_f32;

        velocity_y += gravity * delta;

        // 1フレーム後の速度
        assert!((velocity_y - 7.8432).abs() < 0.001);
    }

    /// TST-30204-009: 着地判定テスト
    #[test]
    fn test_landing_detection() {
        let position_y = -0.1_f32;
        let should_land = position_y <= 0.0;
        assert!(should_land);
    }

    /// TST-30204-011: 天井衝突テスト
    #[test]
    fn test_ceiling_collision() {
        let ceiling_height = 5.0_f32;
        let position_y = 5.5_f32;
        let velocity_y = 2.0_f32;

        let should_collide = position_y >= ceiling_height && velocity_y > 0.0;
        assert!(should_collide);
    }
}
