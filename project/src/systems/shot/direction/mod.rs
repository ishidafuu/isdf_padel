//! ショット方向計算システム
//! @spec 30602_shot_direction_spec.md
//! @spec 30603_jump_shot_spec.md
//! @spec 30604_shot_attributes_spec.md
//! @spec 30605_trajectory_calculation_spec.md

mod normal_shot;
mod serve_shot;
mod utils;

#[cfg(test)]
mod tests;

use bevy::prelude::*;

use crate::components::{
    Ball, BallSpin, BounceCount, BounceState, InputState, LastShooter, LogicalPosition, Player,
    Velocity,
};
use crate::core::events::{ShotAttributesCalculatedEvent, ShotEvent, ShotExecutedEvent};
use crate::core::CourtSide;
use crate::resource::config::GameConfig;
use crate::resource::debug::LastShotDebugInfo;
use crate::resource::scoring::MatchScore;

use normal_shot::handle_normal_shot;
use serve_shot::handle_serve_shot;

/// 通常ショット処理用コンテキスト
/// @spec 30602_shot_direction_spec.md#req-30602-032
pub(super) struct NormalShotContext {
    pub player_id: u8,
    pub court_side: CourtSide,
    pub direction: Vec2,
    pub jump_height: f32,
    pub hold_time: f32,
    pub player_pos: Vec3,
    pub player_velocity: Vec3,
    pub ball_pos: Vec3,
    pub bounce_state: BounceState,
}

/// 通常ショット計算結果
/// @spec 30602_shot_direction_spec.md#req-30602-032
pub(super) struct NormalShotResult {
    pub shot_velocity: Vec3,
    pub trajectory_result: crate::systems::trajectory_calculator::TrajectoryResult,
    pub effective_power: f32,
    pub spin: f32,
    pub accuracy: f32,
    pub stability: f32,
    pub is_jump_shot: bool,
    /// ショット属性計算詳細（トレース用）
    pub shot_attrs_detail: crate::systems::shot::attributes::ShotAttributesDetail,
}

/// ショット方向計算システム
/// ShotEvent を受信してボールの速度を設定する
/// @spec 30602_shot_direction_spec.md#req-30602-001
/// @spec 30602_shot_direction_spec.md#req-30602-031 - サーブ処理分岐
/// @spec 30602_shot_direction_spec.md#req-30602-032 - 通常ショット処理
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn shot_direction_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    mut shot_events: MessageReader<ShotEvent>,
    mut ball_query: Query<
        (
            Entity,
            &mut Velocity,
            &mut BounceCount,
            &mut LastShooter,
            &LogicalPosition,
            &BounceState,
            &mut BallSpin,
        ),
        With<Ball>,
    >,
    player_query: Query<(&Player, &LogicalPosition, &Velocity, &InputState), Without<Ball>>,
    mut shot_executed_writer: MessageWriter<ShotExecutedEvent>,
    mut shot_attrs_writer: MessageWriter<ShotAttributesCalculatedEvent>,
    mut debug_info: ResMut<LastShotDebugInfo>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<ColorMaterial>>>,
) {
    for event in shot_events.read() {
        // サーブ処理分岐
        if event.is_serve {
            handle_serve_shot(
                &mut commands,
                &config,
                &match_score,
                event,
                &mut shot_executed_writer,
                &mut meshes,
                &mut materials,
            );
            continue;
        }

        // 通常ショット処理
        handle_normal_shot(
            event,
            &config,
            &mut ball_query,
            &player_query,
            &mut shot_executed_writer,
            &mut shot_attrs_writer,
            &mut debug_info,
        );
    }
}
