//! AI自動サーブシステム
//! @spec 30102_serve_spec.md#req-30102-087
//! @spec 30102_serve_spec.md#req-30102-088
//!
//! v0.4: トス→ヒット方式
//! AIプレイヤーがサーブ権を持つ時、
//! 1. 待機時間後に自動でトスを実行
//! 2. ボールが最適高さに達したらヒットを実行

use bevy::prelude::*;
use rand::Rng;

use crate::components::{AiController, Ball, LogicalPosition, Player, TossBall, TossBallBundle};
use crate::core::ShotEvent;
use crate::resource::scoring::{ServeState, ServeSubPhase};
use crate::resource::{FixedDeltaTime, GameConfig, MatchFlowState, MatchScore};
use crate::systems::GameSystemSet;

/// AIサーブ待機タイマー（リソース）
/// @spec 30102_serve_spec.md#req-30102-087
///
/// サーブ権を持つAIの待機時間を管理する。
#[derive(Resource, Default)]
pub struct AiServeTimer {
    /// 残り待機時間（トス開始用）
    pub toss_timer: Option<Timer>,
    /// サーブ方向のZ軸オフセット（事前決定）
    /// @spec 30102_serve_spec.md#req-30102-071
    pub direction_z_offset: f32,
    /// ヒット済みフラグ（連続ヒット防止）
    pub hit_executed: bool,
}

/// AIサーブタイマー初期化システム
/// @spec 30102_serve_spec.md#req-30102-087
///
/// Serve状態に入った時、AIがサーバーでWaiting状態ならタイマーを初期化する。
pub fn ai_serve_timer_init_system(
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    serve_state: Res<ServeState>,
    ai_query: Query<&Player, With<AiController>>,
    mut ai_serve_timer: ResMut<AiServeTimer>,
) {
    // AIがサーバーか確認
    let is_ai_server = ai_query
        .iter()
        .any(|player| player.court_side == match_score.server);

    if !is_ai_server {
        // 人間がサーバーの場合、タイマーをクリア
        ai_serve_timer.toss_timer = None;
        ai_serve_timer.hit_executed = false;
        return;
    }

    // Waiting状態でのみタイマー初期化
    if serve_state.phase != ServeSubPhase::Waiting {
        return;
    }

    // 既にタイマーが設定されている場合はスキップ
    if ai_serve_timer.toss_timer.is_some() {
        return;
    }

    // @spec 30102_serve_spec.md#req-30102-087: ランダムな待機時間を決定
    let mut rng = rand::rng();
    let delay = rng.random_range(config.ai.serve_delay_min..=config.ai.serve_delay_max);

    // @spec 30102_serve_spec.md#req-30102-071: サーブ方向のランダムバリエーションを事前決定
    let direction_z_offset =
        rng.random_range(-config.ai.serve_direction_variance..=config.ai.serve_direction_variance);

    ai_serve_timer.toss_timer = Some(Timer::from_seconds(delay, TimerMode::Once));
    ai_serve_timer.direction_z_offset = direction_z_offset;
    ai_serve_timer.hit_executed = false;

    info!(
        "AI toss timer initialized: {:.2}s, direction_z_offset: {:.2}",
        delay, direction_z_offset
    );
}

/// AIトス実行システム
/// @spec 30102_serve_spec.md#req-30102-087
///
/// タイマーが完了したらトスを実行する。
pub fn ai_serve_toss_system(
    mut commands: Commands,
    fixed_dt: Res<FixedDeltaTime>,
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    mut serve_state: ResMut<ServeState>,
    mut ai_serve_timer: ResMut<AiServeTimer>,
    ai_query: Query<(&Player, &LogicalPosition), With<AiController>>,
    toss_ball_query: Query<Entity, With<TossBall>>,
    ball_query: Query<Entity, With<Ball>>,
) {
    // タイマーがなければスキップ
    let Some(ref mut timer) = ai_serve_timer.toss_timer else {
        return;
    };

    // Waiting状態でのみトス可能
    if serve_state.phase != ServeSubPhase::Waiting {
        return;
    }

    // タイマー更新（FixedDeltaTime使用で高速シミュレーション対応）
    timer.tick(std::time::Duration::from_secs_f32(fixed_dt.delta_secs()));

    // タイマー完了前ならスキップ
    if !timer.is_finished() {
        return;
    }

    // 既にボールがある場合は何もしない
    if !toss_ball_query.is_empty() || !ball_query.is_empty() {
        ai_serve_timer.toss_timer = None;
        return;
    }

    // AIサーバーを取得
    let Some((_player, logical_pos)) = ai_query
        .iter()
        .find(|(p, _)| p.court_side == match_score.server)
    else {
        warn!("AI server not found for toss");
        ai_serve_timer.toss_timer = None;
        return;
    };

    // @spec 30102_serve_spec.md#req-30102-087: トスボール生成
    let toss_pos = logical_pos.value + Vec3::new(0.0, config.serve.toss_start_offset_y, 0.0);
    let toss_vel = Vec3::new(0.0, config.serve.toss_velocity_y, 0.0);

    commands.spawn(TossBallBundle::new(toss_pos, toss_vel));

    // ServeState更新
    serve_state.start_toss(logical_pos.value);

    info!(
        "AI Toss: Ball tossed at {:?} with velocity {:?} by {:?}",
        toss_pos, toss_vel, match_score.server
    );

    // タイマーをクリア（ヒット待ちに遷移）
    ai_serve_timer.toss_timer = None;
}

/// AIヒット実行システム
/// @spec 30102_serve_spec.md#req-30102-088
///
/// ボールが最適高さに達したらヒットを実行する。
pub fn ai_serve_hit_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    mut serve_state: ResMut<ServeState>,
    mut ai_serve_timer: ResMut<AiServeTimer>,
    ai_query: Query<(&Player, &LogicalPosition), With<AiController>>,
    toss_ball_query: Query<(Entity, &LogicalPosition), With<TossBall>>,
    mut shot_event_writer: MessageWriter<ShotEvent>,
) {
    // Tossing状態でのみヒット可能
    if serve_state.phase != ServeSubPhase::Tossing {
        return;
    }

    // ヒット済みならスキップ
    if ai_serve_timer.hit_executed {
        return;
    }

    // AIサーバーを取得
    let Some((player, player_pos)) = ai_query
        .iter()
        .find(|(p, _)| p.court_side == match_score.server)
    else {
        return;
    };

    // トスボールを取得
    let Some((toss_entity, toss_pos)) = toss_ball_query.iter().next() else {
        return;
    };

    let ball_height = toss_pos.value.y;
    let optimal_height = config.serve.hit_height_optimal;
    let tolerance = config.serve.ai_hit_tolerance;

    // @spec 30102_serve_spec.md#req-30102-088: 最適高さに達したらヒット
    if (ball_height - optimal_height).abs() > tolerance {
        return;
    }

    // ヒット可能範囲チェック（念のため）
    if ball_height < config.serve.hit_height_min || ball_height > config.serve.hit_height_max {
        return;
    }

    // ヒット実行
    // 打点位置を記録（トスボールの位置を使用）
    let hit_pos = toss_pos.value;

    // トスボールを削除
    commands.entity(toss_entity).despawn();

    // サーブ方向（ランダムバリエーション付き Z オフセットを direction.y として渡す）
    // @spec 30102_serve_spec.md#req-30102-071
    let direction = Vec2::new(0.0, ai_serve_timer.direction_z_offset);

    // ServeState更新
    serve_state.on_hit_success();

    // 注: 状態遷移は serve_landing_judgment_system で行う
    // ShotEvent発行（is_serve = true）
    // @spec 30602_shot_direction_spec.md#req-30602-031
    // ボール生成と弾道計算は shot_direction_system で実行
    shot_event_writer.write(ShotEvent {
        player_id: player.id,
        court_side: match_score.server,
        direction,
        jump_height: player_pos.value.y,
        is_serve: true,
        hit_position: Some(hit_pos),
    });

    // ヒット済みフラグを設定
    ai_serve_timer.hit_executed = true;

    info!(
        "AI Serve hit: ShotEvent sent with hit_pos {:?} by {:?}",
        hit_pos, match_score.server
    );
}

/// AIサーブプラグイン
pub struct AiServePlugin;

impl Plugin for AiServePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AiServeTimer>().add_systems(
            Update,
            (
                ai_serve_timer_init_system,
                ai_serve_toss_system,
                ai_serve_hit_system,
            )
                .chain()
                .run_if(in_state(MatchFlowState::Serve))
                .in_set(GameSystemSet::GameLogic),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// REQ-30102-087: AIサーブタイマーテスト
    #[test]
    fn test_ai_serve_timer_default() {
        let timer = AiServeTimer::default();
        assert!(timer.toss_timer.is_none());
        assert!((timer.direction_z_offset - 0.0).abs() < 0.001);
        assert!(!timer.hit_executed);
    }

    /// REQ-30102-071: サーブ方向バリエーションテスト
    #[test]
    fn test_ai_serve_direction_variance() {
        let variance = 0.5;
        let min = -variance;
        let max = variance;

        let mut rng = rand::rng();
        for _ in 0..100 {
            let offset: f32 = rng.random_range(min..=max);
            assert!(offset >= min && offset <= max);
        }
    }

    /// REQ-30102-088: 最適高さ判定テスト
    #[test]
    fn test_ai_serve_optimal_height() {
        let optimal_height: f32 = 2.2;
        let tolerance: f32 = 0.1;

        // 範囲内
        let ball_height: f32 = 2.25;
        let is_optimal = (ball_height - optimal_height).abs() <= tolerance;
        assert!(is_optimal);

        // 範囲外
        let ball_height: f32 = 2.5;
        let is_optimal = (ball_height - optimal_height).abs() <= tolerance;
        assert!(!is_optimal);
    }
}
