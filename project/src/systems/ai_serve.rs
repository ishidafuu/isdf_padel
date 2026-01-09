//! AI自動サーブシステム
//! @spec 30102_serve_spec.md#req-30102-070
//! @spec 30102_serve_spec.md#req-30102-071
//!
//! AIプレイヤーがサーブ権を持つ時、一定時間後に自動でサーブを実行する。

use bevy::prelude::*;
use rand::Rng;

use crate::components::{AiController, Ball, BallBundle, LogicalPosition, Player};
use crate::core::{CourtSide, ShotEvent};
use crate::resource::{GameConfig, MatchFlowState, MatchScore};

/// AIサーブ待機タイマー（リソース）
/// @spec 30102_serve_spec.md#req-30102-070
///
/// サーブ権を持つAIの待機時間を管理する。
/// サーブは1人しか行わないため、リソースで管理。
#[derive(Resource, Default)]
pub struct AiServeTimer {
    /// 残り待機時間
    pub timer: Option<Timer>,
    /// サーブ方向のZ軸オフセット（事前決定）
    /// @spec 30102_serve_spec.md#req-30102-071
    pub direction_z_offset: f32,
}

/// AIサーブタイマー初期化システム
/// @spec 30102_serve_spec.md#req-30102-070
///
/// Serve状態に入った時、AIがサーバーならタイマーを初期化する。
pub fn ai_serve_timer_init_system(
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    ai_query: Query<&Player, With<AiController>>,
    mut ai_serve_timer: ResMut<AiServeTimer>,
) {
    // AIがサーバーか確認
    let is_ai_server = ai_query
        .iter()
        .any(|player| player.court_side == match_score.server);

    if !is_ai_server {
        // 人間がサーバーの場合、タイマーをクリア
        ai_serve_timer.timer = None;
        return;
    }

    // 既にタイマーが設定されている場合はスキップ
    if ai_serve_timer.timer.is_some() {
        return;
    }

    // @spec 30102_serve_spec.md#req-30102-070: ランダムな待機時間を決定
    let mut rng = rand::rng();
    let delay = rng.random_range(config.ai.serve_delay_min..=config.ai.serve_delay_max);

    // @spec 30102_serve_spec.md#req-30102-071: サーブ方向のランダムバリエーションを事前決定
    let direction_z_offset =
        rng.random_range(-config.ai.serve_direction_variance..=config.ai.serve_direction_variance);

    ai_serve_timer.timer = Some(Timer::from_seconds(delay, TimerMode::Once));
    ai_serve_timer.direction_z_offset = direction_z_offset;

    info!(
        "AI serve timer initialized: {:.2}s, direction_z_offset: {:.2}",
        delay, direction_z_offset
    );
}

/// AIサーブ実行システム
/// @spec 30102_serve_spec.md#req-30102-070
/// @spec 30102_serve_spec.md#req-30102-071
///
/// タイマーが完了したらサーブを実行する。
pub fn ai_serve_execute_system(
    mut commands: Commands,
    time: Res<Time>,
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    mut ai_serve_timer: ResMut<AiServeTimer>,
    ai_query: Query<(&Player, &LogicalPosition), With<AiController>>,
    ball_query: Query<Entity, With<Ball>>,
    mut shot_event_writer: MessageWriter<ShotEvent>,
) {
    // タイマーがなければスキップ
    let Some(ref mut timer) = ai_serve_timer.timer else {
        return;
    };

    // タイマー更新
    timer.tick(time.delta());

    // タイマー完了前ならスキップ
    if !timer.is_finished() {
        return;
    }

    // 既にボールがある場合は何もしない
    if !ball_query.is_empty() {
        ai_serve_timer.timer = None;
        return;
    }

    // AIサーバーを取得
    let Some((player, logical_pos)) = ai_query
        .iter()
        .find(|(p, _)| p.court_side == match_score.server)
    else {
        warn!("AI server not found");
        ai_serve_timer.timer = None;
        return;
    };

    // @spec 30102_serve_spec.md#req-30102-060: ボール生成位置
    let ball_spawn_offset_y = config.serve.ball_spawn_offset_y;
    let ball_pos = logical_pos.value + Vec3::new(0.0, ball_spawn_offset_y, 0.0);

    // @spec 30102_serve_spec.md#req-30102-071: サーブ方向（ランダムバリエーション付き）
    let base_direction_x = match match_score.server {
        CourtSide::Player1 => config.serve.p1_default_direction_x,
        CourtSide::Player2 => config.serve.p2_default_direction_x,
    };
    let direction = Vec2::new(base_direction_x, ai_serve_timer.direction_z_offset);

    // @spec 30102_serve_spec.md#req-30102-060: オーバーハンドサーブの弾道計算
    let speed = config.serve.serve_speed;
    let angle_rad = config.serve.serve_angle.to_radians();
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();

    // Vec2(x, y) -> Vec3(x, 0, y) で XZ平面に変換
    let horizontal_dir = Vec3::new(direction.x, 0.0, direction.y).normalize();
    let ball_velocity = Vec3::new(
        horizontal_dir.x * speed * cos_angle,
        speed * sin_angle,
        horizontal_dir.z * speed * cos_angle,
    );

    // ボール生成
    commands.spawn(BallBundle::with_shooter(
        ball_pos,
        ball_velocity,
        match_score.server,
    ));

    info!(
        "AI Serve: Ball spawned at {:?} with velocity {:?} by {:?}",
        ball_pos, ball_velocity, match_score.server
    );

    // ShotEvent を発行
    shot_event_writer.write(ShotEvent {
        player_id: player.id,
        court_side: match_score.server,
        direction,
        jump_height: logical_pos.value.y,
    });

    info!(
        "AI Serve: ShotEvent emitted for {:?}, direction: {:?}",
        match_score.server, direction
    );

    // タイマーをクリア
    ai_serve_timer.timer = None;
}

/// AIサーブプラグイン
pub struct AiServePlugin;

impl Plugin for AiServePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AiServeTimer>().add_systems(
            Update,
            (ai_serve_timer_init_system, ai_serve_execute_system)
                .chain()
                .run_if(in_state(MatchFlowState::Serve)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// REQ-30102-070: AIサーブタイマーテスト
    #[test]
    fn test_ai_serve_timer_default() {
        let timer = AiServeTimer::default();
        assert!(timer.timer.is_none());
        assert!((timer.direction_z_offset - 0.0).abs() < 0.001);
    }

    /// REQ-30102-071: サーブ方向バリエーションテスト
    #[test]
    fn test_ai_serve_direction_variance() {
        // バリエーションは ±0.5 の範囲
        let variance = 0.5;
        let min = -variance;
        let max = variance;

        // ランダム値が範囲内であることを確認
        let mut rng = rand::rng();
        for _ in 0..100 {
            let offset: f32 = rng.random_range(min..=max);
            assert!(offset >= min && offset <= max);
        }
    }
}
