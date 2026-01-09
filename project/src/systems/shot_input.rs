//! ショット入力システム
//! @spec 30601_shot_input_spec.md
//! @spec 20006_input_system.md

use bevy::prelude::*;

use crate::components::{
    Ball, BounceCount, InputState, KnockbackState, LastShooter, LogicalPosition, Player, ShotState,
};
use crate::core::events::ShotEvent;
use crate::resource::config::GameConfig;
use crate::resource::{MatchScore, RallyPhase, RallyState};

/// ショット入力処理システム
/// @spec 30601_shot_input_spec.md#req-30601-001
/// @spec 30601_shot_input_spec.md#req-30601-002
/// @spec 30601_shot_input_spec.md#req-30601-003
/// @spec 30601_shot_input_spec.md#req-30601-004
/// @spec 30601_shot_input_spec.md#req-30601-005
/// @spec 30601_shot_input_spec.md#req-30601-006
/// @spec 20006_input_system.md - InputState 対応
pub fn shot_input_system(
    config: Res<GameConfig>,
    rally_state: Res<RallyState>,
    match_score: Res<MatchScore>,
    mut player_query: Query<(
        &Player,
        &LogicalPosition,
        &mut ShotState,
        &KnockbackState,
        &InputState,
    )>,
    ball_query: Query<(&LogicalPosition, &LastShooter, &BounceCount), With<Ball>>,
    mut event_writer: MessageWriter<ShotEvent>,
) {
    // ボールの位置とLastShooterとBounceCountを取得（存在しない場合はショット不可）
    let (ball_logical_pos, last_shooter, bounce_count) = match ball_query.iter().next() {
        Some(t) => t,
        None => return, // ボールがない場合は何もしない
    };
    let ball_pos = ball_logical_pos.value;

    for (player, player_logical_pos, mut shot_state, knockback, input_state) in
        player_query.iter_mut()
    {
        // InputState からショット入力を取得
        if !input_state.shot_pressed {
            continue;
        }

        // REQ-30601-005: ふっとばし中はショット禁止
        if knockback.is_knockback_active() {
            info!("Player {} shot ignored: knockback active", player.id);
            continue;
        }

        // サーブ中でボールがまだバウンドしていない場合、リターナーはショット禁止
        // パデルルール: サーブは必ず1バウンドしてからリターンする
        if rally_state.phase == RallyPhase::Serving && bounce_count.count == 0 {
            // リターナー（サーバーの相手側）のみブロック
            if player.court_side != match_score.server {
                info!(
                    "Player {} shot ignored: serve must bounce first (phase={:?}, bounces={})",
                    player.id, rally_state.phase, bounce_count.count
                );
                continue;
            }
        }

        // 自分が打ったボールは打てない（相手が打ち返すまで待つ）
        if last_shooter.side == Some(player.court_side) {
            info!(
                "Player {} shot ignored: own ball (last_shooter={:?})",
                player.id, last_shooter.side
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

        // REQ-30602-001: ショット方向の決定
        // X軸（左右）: 入力で調整可能
        // Z軸（前後）: 常に相手コート方向に固定（shot_direction.rs で処理）
        let raw_direction = input_state.movement;
        // X軸入力のみを使用（上下入力は無視）
        // Y成分は shot_direction.rs でプレイヤーIDに応じて決定される
        let direction = Vec2::new(raw_direction.x, 0.0);

        // REQ-30601-004: クールダウン開始
        shot_state.start_cooldown(config.shot.cooldown_time);

        // ShotEvent を発行（通常ショット: is_serve = false）
        // @spec 30602_shot_direction_spec.md#req-30602-032
        event_writer.write(ShotEvent {
            player_id: player.id,
            court_side: player.court_side,
            direction,
            jump_height: player_pos.y,
            is_serve: false,
            hit_position: None,
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
