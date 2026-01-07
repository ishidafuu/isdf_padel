//! ショット入力システム
//! @spec 30601_shot_input_spec.md

use bevy::prelude::*;

use crate::components::{Ball, KnockbackState, LogicalPosition, Player, ShotState};
use crate::core::events::ShotEvent;
use crate::resource::config::GameConfig;
use crate::systems::MovementInput;

/// ショット入力リソース（各プレイヤーのショット入力を保持）
/// @spec 30601_shot_input_spec.md#req-30601-001
#[derive(Resource, Default)]
pub struct ShotInput {
    /// Player 1 のショット入力（押された瞬間のみtrue）
    pub player1_pressed: bool,
    /// Player 2 のショット入力（押された瞬間のみtrue）
    pub player2_pressed: bool,
}

/// ショット入力読み取りシステム
/// @spec 30601_shot_input_spec.md#req-30601-001
/// NOTE: テスト用に P1/P2 を同じキーで操作（F または Enter）
pub fn read_shot_input_system(keyboard: Res<ButtonInput<KeyCode>>, mut input: ResMut<ShotInput>) {
    // 共通ショット入力: F または Enter
    let shot_pressed = keyboard.just_pressed(KeyCode::KeyF) || keyboard.just_pressed(KeyCode::Enter);

    // 両プレイヤーに同じ入力を設定
    input.player1_pressed = shot_pressed;
    input.player2_pressed = shot_pressed;
}

/// ショット入力処理システム
/// @spec 30601_shot_input_spec.md#req-30601-001
/// @spec 30601_shot_input_spec.md#req-30601-002
/// @spec 30601_shot_input_spec.md#req-30601-003
/// @spec 30601_shot_input_spec.md#req-30601-004
/// @spec 30601_shot_input_spec.md#req-30601-005
/// @spec 30601_shot_input_spec.md#req-30601-006
pub fn shot_input_system(
    config: Res<GameConfig>,
    shot_input: Res<ShotInput>,
    movement_input: Res<MovementInput>,
    mut player_query: Query<(
        &Player,
        &LogicalPosition,
        &mut ShotState,
        &KnockbackState,
    )>,
    ball_query: Query<&LogicalPosition, With<Ball>>,
    mut event_writer: MessageWriter<ShotEvent>,
) {
    // ボールの位置を取得（存在しない場合はショット不可）
    let ball_logical_pos = match ball_query.iter().next() {
        Some(t) => t,
        None => return, // ボールがない場合は何もしない
    };
    let ball_pos = ball_logical_pos.value;

    for (player, player_logical_pos, mut shot_state, knockback) in player_query.iter_mut() {
        // プレイヤーごとのショット入力を取得
        let shot_pressed = match player.id {
            1 => shot_input.player1_pressed,
            2 => shot_input.player2_pressed,
            _ => false,
        };

        if !shot_pressed {
            continue;
        }

        // REQ-30601-005: ふっとばし中はショット禁止
        if knockback.is_knockback_active() {
            info!(
                "Player {} shot ignored: knockback active",
                player.id
            );
            continue;
        }

        // REQ-30601-004: クールダウン中はショット禁止
        if shot_state.is_on_cooldown() {
            info!(
                "Player {} shot ignored: cooldown (remaining: {:.2}s)",
                player.id, shot_state.cooldown_timer
            );
            continue;
        }

        let player_pos = player_logical_pos.value;

        // REQ-30601-002: 距離判定
        let distance_2d = distance_2d(player_pos, ball_pos);
        if distance_2d > config.shot.max_distance {
            info!(
                "Player {} shot ignored: too far (distance: {:.2}, max: {:.2})",
                player.id, distance_2d, config.shot.max_distance
            );
            continue;
        }

        // REQ-30601-003: 高さ差判定
        let height_diff = (player_pos.y - ball_pos.y).abs();
        if height_diff > config.shot.max_height_diff {
            info!(
                "Player {} shot ignored: height diff too large ({:.2}, max: {:.2})",
                player.id, height_diff, config.shot.max_height_diff
            );
            continue;
        }

        // REQ-30601-006: ショット条件を満たした場合、ShotEvent を発行

        // 入力方向を取得（正規化）
        let raw_direction = match player.id {
            1 => movement_input.player1,
            2 => movement_input.player2,
            _ => Vec2::ZERO,
        };
        let direction = if raw_direction.length() > 0.0 {
            raw_direction.normalize()
        } else {
            // 入力がない場合は前方向（Player1は+Z、Player2は-Z）
            match player.id {
                1 => Vec2::new(0.0, 1.0),
                2 => Vec2::new(0.0, -1.0),
                _ => Vec2::ZERO,
            }
        };

        // REQ-30601-004: クールダウン開始
        shot_state.start_cooldown(config.shot.cooldown_time);

        // ShotEvent を発行
        event_writer.write(ShotEvent {
            player_id: player.id,
            direction,
            jump_height: player_pos.y,
        });

        info!(
            "Player {} shot! direction: {:?}, height: {:.2}",
            player.id, direction, player_pos.y
        );
    }
}

/// クールダウンタイマー更新システム
/// @spec 30601_shot_input_spec.md#req-30601-004
pub fn shot_cooldown_system(time: Res<Time>, mut query: Query<&mut ShotState, With<Player>>) {
    let delta = time.delta_secs();

    for mut shot_state in query.iter_mut() {
        shot_state.update_cooldown(delta);
    }
}

/// 2D距離計算（XZ平面）
/// @spec 30601_shot_input_spec.md#req-30601-002
#[inline]
fn distance_2d(a: Vec3, b: Vec3) -> f32 {
    let dx = a.x - b.x;
    let dz = a.z - b.z;
    (dx * dx + dz * dz).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30604-002: 距離判定テスト
    #[test]
    fn test_distance_2d() {
        let player_pos = Vec3::new(0.0, 0.0, 0.0);
        let ball_pos = Vec3::new(1.0, 2.0, 0.0); // Y軸は無視される

        let distance = distance_2d(player_pos, ball_pos);
        assert!((distance - 1.0).abs() < 0.001);
    }

    /// TST-30604-002: 距離判定テスト（XZ両方に距離がある場合）
    #[test]
    fn test_distance_2d_diagonal() {
        let player_pos = Vec3::new(0.0, 0.0, 0.0);
        let ball_pos = Vec3::new(1.0, 0.0, 1.0);

        let distance = distance_2d(player_pos, ball_pos);
        let expected = (2.0_f32).sqrt();
        assert!((distance - expected).abs() < 0.001);
    }

    /// TST-30604-004: クールダウンテスト
    #[test]
    fn test_cooldown() {
        let mut shot_state = ShotState::default();
        assert!(!shot_state.is_on_cooldown());

        shot_state.start_cooldown(0.5);
        assert!(shot_state.is_on_cooldown());
        assert!((shot_state.cooldown_timer - 0.5).abs() < 0.001);

        shot_state.update_cooldown(0.3);
        assert!(shot_state.is_on_cooldown());
        assert!((shot_state.cooldown_timer - 0.2).abs() < 0.001);

        shot_state.update_cooldown(0.3);
        assert!(!shot_state.is_on_cooldown());
        assert!(shot_state.cooldown_timer == 0.0);
    }
}
