//! 通常ショット処理
//! @spec 30602_shot_direction_spec.md#req-30602-032

use bevy::prelude::*;

use crate::components::InputMode;
use crate::components::{
    Ball, BallSpin, BounceCount, BounceState, InputState, LastShooter, LogicalPosition, Player,
    Velocity,
};
use crate::core::events::{ShotAttributesCalculatedEvent, ShotEvent, ShotExecutedEvent};
use crate::resource::config::GameConfig;
use crate::resource::debug::LastShotDebugInfo;
use crate::systems::shot::attributes::{
    build_shot_context_from_input_state, calculate_shot_attributes_detail,
};
use crate::systems::trajectory_calculator::{calculate_trajectory, TrajectoryContext};

use super::utils::{calculate_stability_power_factor, get_player_info, update_shot_debug_info};
use super::{NormalShotContext, NormalShotResult};

/// 通常ショット処理
/// @spec 30602_shot_direction_spec.md#req-30602-032
#[allow(clippy::type_complexity)]
pub(super) fn handle_normal_shot(
    event: &ShotEvent,
    config: &GameConfig,
    ball_query: &mut Query<
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
    player_query: &Query<(&Player, &LogicalPosition, &Velocity, &InputState), Without<Ball>>,
    shot_executed_writer: &mut MessageWriter<ShotExecutedEvent>,
    shot_attrs_writer: &mut MessageWriter<ShotAttributesCalculatedEvent>,
    debug_info: &mut LastShotDebugInfo,
) {
    // ボールを取得
    let Ok((
        _,
        mut ball_velocity,
        mut bounce_count,
        mut last_shooter,
        ball_pos,
        bounce_state,
        mut ball_spin,
    )) = ball_query.single_mut()
    else {
        warn!("No ball found for shot direction calculation");
        return;
    };

    // プレイヤー情報を取得
    let Some((player_pos, player_velocity, hold_time)) =
        get_player_info(player_query, event.player_id)
    else {
        warn!("Player {} not found", event.player_id);
        return;
    };

    // 最後にショットを打ったプレイヤーを記録
    last_shooter.record(event.court_side);

    // ショット計算コンテキストを構築
    let ctx = NormalShotContext {
        player_id: event.player_id,
        court_side: event.court_side,
        direction: event.direction,
        jump_height: event.jump_height,
        hold_time,
        player_pos,
        player_velocity,
        ball_pos: ball_pos.value,
        bounce_state: *bounce_state,
    };

    // ショット計算を実行
    let result = calculate_normal_shot(&ctx, config);

    // 結果をボールに適用
    ball_velocity.value = result.shot_velocity;
    bounce_count.reset();
    ball_spin.value = result.spin;

    // デバッグ情報を更新
    update_shot_debug_info(
        debug_info,
        ctx.player_id,
        ctx.ball_pos,
        ctx.direction,
        ctx.court_side,
        result.effective_power,
        result.spin,
        result.accuracy,
        &result.trajectory_result,
        result.shot_velocity,
        config,
    );

    // イベント発行
    shot_executed_writer.write(ShotExecutedEvent {
        player_id: ctx.player_id,
        shot_velocity: result.shot_velocity,
        is_jump_shot: result.is_jump_shot,
    });

    // ショット属性詳細イベント発行（トレース用）
    let detail = &result.shot_attrs_detail;
    shot_attrs_writer.write(ShotAttributesCalculatedEvent {
        player_id: ctx.player_id,
        input_mode: match detail.input_mode {
            InputMode::Push => "Push".to_string(),
            InputMode::Hold => "Hold".to_string(),
        },
        hit_height: detail.hit_height,
        bounce_elapsed: detail.bounce_elapsed,
        approach_dot: detail.approach_dot,
        ball_distance: detail.ball_distance,
        height_factors: detail.height_factors,
        timing_factors: detail.timing_factors,
        approach_factors: detail.approach_factors,
        distance_factors: detail.distance_factors,
        final_power: detail.attributes.power,
        final_stability: detail.attributes.stability,
        final_angle: detail.attributes.angle,
        final_spin: detail.attributes.spin,
        final_accuracy: detail.attributes.accuracy,
    });

    info!(
        "Player {} shot executed: power={:.1}, angle={:.1}, stability={:.2}, accuracy={:.2}, spin={:.2}, landing=({:.1}, {:.1})",
        ctx.player_id,
        result.effective_power,
        result.trajectory_result.launch_angle,
        result.stability,
        result.accuracy,
        result.spin,
        result.trajectory_result.landing_position.x,
        result.trajectory_result.landing_position.z
    );
}

/// 通常ショットの弾道を計算
/// @spec 30602_shot_direction_spec.md#req-30602-002
/// @spec 30602_shot_direction_spec.md#req-30602-003
/// @spec 30602_shot_direction_spec.md#req-30602-004
/// @spec 30602_shot_direction_spec.md#req-30602-005
/// @spec 30604_shot_attributes_spec.md#req-30604-068
/// @spec 30604_shot_attributes_spec.md#req-30604-069
/// @spec 30604_shot_attributes_spec.md#req-30604-070
/// @spec 30605_trajectory_calculation_spec.md - 着地点逆算型弾道システム
fn calculate_normal_shot(ctx: &NormalShotContext, config: &GameConfig) -> NormalShotResult {
    // ショット属性計算（詳細版）
    let shot_context = build_shot_context_from_input_state(
        ctx.hold_time,
        ctx.player_pos,
        ctx.player_velocity,
        ctx.ball_pos,
        &ctx.bounce_state,
        &config.shot_attributes,
    );
    let shot_attrs_detail =
        calculate_shot_attributes_detail(&shot_context, &config.shot_attributes);
    let shot_attrs = &shot_attrs_detail.attributes;

    // 安定性による威力減衰
    let stability_factor =
        calculate_stability_power_factor(shot_attrs.stability, &config.shot_attributes);
    let effective_power = shot_attrs.power * stability_factor;

    // 弾道計算
    let trajectory_ctx = TrajectoryContext {
        input: ctx.direction,
        court_side: ctx.court_side,
        ball_position: ctx.ball_pos,
        spin: shot_attrs.spin,
        base_speed: effective_power,
        accuracy: shot_attrs.accuracy,
    };
    let trajectory_result = calculate_trajectory(&trajectory_ctx, config);
    let shot_velocity = trajectory_result.direction * trajectory_result.final_speed;

    // ジャンプショット判定
    let is_jump_shot = ctx.jump_height > config.shot.jump_threshold;

    info!(
        "shot_direction(v0.4): landing={:?}, angle={:.1}, speed={:.1}, stability_factor={:.2}, velocity={:?}",
        trajectory_result.landing_position,
        trajectory_result.launch_angle,
        trajectory_result.final_speed,
        stability_factor,
        shot_velocity
    );

    NormalShotResult {
        shot_velocity,
        trajectory_result,
        effective_power,
        spin: shot_attrs.spin,
        accuracy: shot_attrs.accuracy,
        stability: shot_attrs.stability,
        is_jump_shot,
        shot_attrs_detail,
    }
}
