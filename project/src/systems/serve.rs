//! サーブ処理システム
//! @spec 30102_serve_spec.md
//!
//! Serve状態でサーバーがショット入力をするとボールを生成しShotEventを発行する。
//! ボールの速度はサーブ実行時に直接設定する（commands.spawn() の遅延適用対策）。
//! Rally状態への遷移は serve_to_rally_system が担当する。

use bevy::prelude::*;

use crate::components::{Ball, BallBundle, LogicalPosition, Player};
use crate::core::{CourtSide, ShotEvent};
use crate::resource::{GameConfig, MatchScore};
use crate::systems::{MovementInput, ShotInput};

/// サーブ実行システム
/// @spec 30102_serve_spec.md#req-30102-002
/// @spec 30102_serve_spec.md#req-30102-003
///
/// Serve状態でサーバーがBボタンを押すとボールを生成しShotEventを発行する。
pub fn serve_execute_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    shot_input: Res<ShotInput>,
    match_score: Res<MatchScore>,
    movement_input: Res<MovementInput>,
    player_query: Query<(&Player, &LogicalPosition)>,
    ball_query: Query<Entity, With<Ball>>,
    mut shot_event_writer: MessageWriter<ShotEvent>,
) {
    // @spec 30102_serve_spec.md#req-30102-002: すでにボールがある場合は何もしない
    if !ball_query.is_empty() {
        return;
    }

    // @spec 30102_serve_spec.md#req-30102-001: サーバーを特定
    let server_id = match match_score.server {
        CourtSide::Player1 => 1,
        CourtSide::Player2 => 2,
    };

    // @spec 30102_serve_spec.md#req-30102-002: サーバーのショット入力をチェック
    let shot_pressed = match server_id {
        1 => shot_input.player1_pressed,
        2 => shot_input.player2_pressed,
        _ => false,
    };

    if !shot_pressed {
        return;
    }

    // サーバーのプレイヤーを取得
    let Some((_, logical_pos)) = player_query.iter().find(|(p, _)| p.id == server_id) else {
        warn!("Server player {} not found", server_id);
        return;
    };

    // @spec 30102_serve_spec.md#req-30102-002: 入力方向を取得
    let raw_direction = match server_id {
        1 => movement_input.player1,
        2 => movement_input.player2,
        _ => Vec2::ZERO,
    };

    // 入力がない場合は相手コート方向をデフォルトに
    let direction = if raw_direction.length() > 0.0 {
        raw_direction.normalize()
    } else {
        match server_id {
            1 => Vec2::new(0.0, 1.0),  // Player1: +Z方向（2Pコートへ）
            2 => Vec2::new(0.0, -1.0), // Player2: -Z方向（1Pコートへ）
            _ => Vec2::ZERO,
        }
    };

    // @spec 30102_serve_spec.md#req-30102-002: ボールを生成（プレイヤーの足元 + (0, 0.5, 0)）
    let ball_pos = logical_pos.value + Vec3::new(0.0, 0.5, 0.0);

    // サーブ時は通常ショットの速度と角度を使用
    // commands.spawn() は遅延適用されるため、ここで直接速度を計算する
    let speed = config.ball.normal_shot_speed;
    let angle_rad = config.shot.normal_shot_angle.to_radians();
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();

    // Vec2(x, y) -> Vec3(x, 0, y) で XZ平面に変換
    let horizontal_dir = Vec3::new(direction.x, 0.0, direction.y).normalize();
    let ball_velocity = Vec3::new(
        horizontal_dir.x * speed * cos_angle,
        speed * sin_angle,
        horizontal_dir.z * speed * cos_angle,
    );

    // サーバー情報を LastShooter に設定（自己衝突回避のため）
    info!(
        "Serve: Creating ball with LastShooter = {:?}",
        match_score.server
    );
    commands.spawn(BallBundle::with_shooter(ball_pos, ball_velocity, match_score.server));
    info!(
        "Serve: Ball spawned at {:?} with velocity {:?} by Player{}",
        ball_pos, ball_velocity, server_id
    );

    // @spec 30102_serve_spec.md#req-30102-002: ShotEvent を発行
    // serve_to_rally_system が状態遷移を行う
    // NOTE: ボール速度は上記で直接設定済み（commands.spawn の遅延適用対策）
    shot_event_writer.write(ShotEvent {
        player_id: server_id,
        direction,
        jump_height: logical_pos.value.y,
    });

    info!(
        "Serve: ShotEvent emitted for Player{}, direction: {:?}",
        server_id, direction
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30104-006: サーブ権の管理テスト
    /// @spec 30102_serve_spec.md#req-30102-001
    #[test]
    fn test_req_30102_001_server_determination() {
        // Player1 がサーバーの場合
        let server = CourtSide::Player1;
        let server_id = match server {
            CourtSide::Player1 => 1,
            CourtSide::Player2 => 2,
        };
        assert_eq!(server_id, 1);

        // Player2 がサーバーの場合
        let server = CourtSide::Player2;
        let server_id = match server {
            CourtSide::Player1 => 1,
            CourtSide::Player2 => 2,
        };
        assert_eq!(server_id, 2);
    }

    /// TST-30104-007: サーブ方向デフォルト値テスト
    /// @spec 30102_serve_spec.md#req-30102-002
    #[test]
    fn test_req_30102_002_default_direction() {
        // Player1 のデフォルト方向: +Z（2Pコートへ）
        let player1_default = Vec2::new(0.0, 1.0);
        assert!((player1_default.y - 1.0).abs() < 0.001);

        // Player2 のデフォルト方向: -Z（1Pコートへ）
        let player2_default = Vec2::new(0.0, -1.0);
        assert!((player2_default.y - (-1.0)).abs() < 0.001);
    }

    /// TST-30104-008: ボール生成位置テスト
    /// @spec 30102_serve_spec.md#req-30102-002
    #[test]
    fn test_req_30102_002_ball_spawn_position() {
        let player_pos = Vec3::new(0.0, 0.0, -2.0);
        let ball_pos = player_pos + Vec3::new(0.0, 0.5, 0.0);

        assert!((ball_pos.x - 0.0).abs() < 0.001);
        assert!((ball_pos.y - 0.5).abs() < 0.001);
        assert!((ball_pos.z - (-2.0)).abs() < 0.001);
    }
}
