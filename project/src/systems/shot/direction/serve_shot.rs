//! サーブショット処理
//! @spec 30602_shot_direction_spec.md#req-30602-031

use bevy::prelude::*;

use crate::components::BallBundle;
use crate::core::events::{ShotEvent, ShotExecutedEvent};
use crate::resource::config::{GameConfig, ServeSide};
use crate::resource::scoring::MatchScore;
use crate::systems::trajectory_calculator::{calculate_serve_trajectory, ServeTrajectoryContext};

/// サーブショット処理
/// @spec 30602_shot_direction_spec.md#req-30602-031
/// @spec 30605_trajectory_calculation_spec.md#req-30605-050
pub(super) fn handle_serve_shot(
    commands: &mut Commands,
    config: &GameConfig,
    match_score: &MatchScore,
    event: &ShotEvent,
    shot_executed_writer: &mut MessageWriter<ShotExecutedEvent>,
) {
    // 打点位置を取得（サーブ時は必須）
    let hit_position = match event.hit_position {
        Some(pos) => pos,
        None => {
            warn!("Serve shot event missing hit_position");
            return;
        }
    };

    // サーブサイドをポイント合計から計算
    // @spec 30903_serve_authority_spec.md#req-30903-003
    let server_points = match_score.get_point_index(event.court_side);
    let receiver_points = match_score.get_point_index(event.court_side.opponent());
    let total_points = server_points + receiver_points;
    let serve_side = ServeSide::from_point_total(total_points);

    // サーブ弾道計算
    let serve_ctx = ServeTrajectoryContext {
        input: event.direction,
        server: event.court_side,
        serve_side,
        hit_position,
        base_speed: config.serve.serve_speed,
    };

    let trajectory_result = calculate_serve_trajectory(&serve_ctx, config);

    // 最終的な打球ベクトルを計算
    let shot_velocity = trajectory_result.direction * trajectory_result.final_speed;

    // ボールを新規生成
    // @spec 30602_shot_direction_spec.md#req-30602-031
    commands.spawn(BallBundle::with_shooter(
        hit_position,
        shot_velocity,
        event.court_side,
    ));

    // ShotExecutedEvent の発行
    shot_executed_writer.write(ShotExecutedEvent {
        player_id: event.player_id,
        shot_velocity,
        is_jump_shot: false, // サーブはジャンプショットではない
    });

    info!(
        "Serve shot executed: player={}, landing=({:.1}, {:.1}), angle={:.1}, speed={:.1}",
        event.player_id,
        trajectory_result.landing_position.x,
        trajectory_result.landing_position.z,
        trajectory_result.launch_angle,
        trajectory_result.final_speed
    );
}
