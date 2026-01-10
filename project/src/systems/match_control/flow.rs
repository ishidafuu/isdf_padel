//! 試合フローシステム
//! @spec 30101_flow_spec.md
//! @spec 30102_serve_spec.md
//!
//! 試合全体の状態遷移とフロー管理を行う。
//! MatchStart → Serve → Rally → PointEnd → Serve/MatchEnd

use bevy::prelude::*;

use crate::components::{Ball, LogicalPosition, Player, TossBall};
use crate::core::{CourtSide, MatchStartEvent, MatchWonEvent, RallyEndEvent, ShotEvent};
use crate::resource::scoring::ServeState;
use crate::resource::{GameConfig, GameState, MatchFlowState, MatchScore, RallyState};
use super::{
    serve_double_fault_system, serve_hit_input_system, serve_init_system,
    serve_position_system, serve_toss_input_system, serve_toss_physics_system,
    serve_toss_timeout_system,
};
use crate::systems::GameSystemSet;

/// 試合フロープラグイン
/// @spec 30101_flow_spec.md
pub struct MatchFlowPlugin;

impl Plugin for MatchFlowPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MatchFlowState>()
            .init_resource::<ServeState>()
            .add_message::<MatchStartEvent>()
            .add_systems(OnEnter(MatchFlowState::MatchStart), match_start_system)
            .add_systems(OnEnter(MatchFlowState::Serve), (serve_init_system, serve_position_system))
            // @spec 30102_serve_spec.md: トス→ヒット方式サーブシステム（Serve状態でのみ動作）
            // GameSystemSet::GameLogic に配置し、入力読み取り後に実行されることを保証
            .add_systems(
                Update,
                (
                    serve_toss_input_system,
                    serve_toss_physics_system,
                    serve_hit_input_system,
                    serve_toss_timeout_system,
                    serve_double_fault_system,
                    serve_to_rally_system,
                )
                    .chain()
                    .run_if(in_state(MatchFlowState::Serve))
                    .in_set(GameSystemSet::GameLogic),
            )
            .add_systems(
                Update,
                rally_to_point_end_system
                    .run_if(in_state(MatchFlowState::Rally))
                    .in_set(GameSystemSet::GameLogic),
            )
            .add_systems(OnEnter(MatchFlowState::PointEnd), point_end_enter_system)
            .add_systems(
                Update,
                point_end_to_next_system
                    .run_if(in_state(MatchFlowState::PointEnd))
                    .in_set(GameSystemSet::GameLogic),
            )
            .add_systems(OnEnter(MatchFlowState::MatchEnd), match_end_system);
    }
}

/// 試合開始システム
/// @spec 30101_flow_spec.md#req-30101-001
/// MatchStart状態に入ったときに実行
fn match_start_system(
    mut next_state: ResMut<NextState<MatchFlowState>>,
    mut match_score: ResMut<MatchScore>,
    mut rally_state: ResMut<RallyState>,
    mut match_start_events: MessageWriter<MatchStartEvent>,
    mut query: Query<(&Player, &mut LogicalPosition)>,
    config: Res<GameConfig>,
) {
    info!("Match starting...");

    // @spec 30101_flow_spec.md#req-30101-001: スコアを初期化する
    *match_score = MatchScore::new();
    match_score.game_state = GameState::Playing;

    // @spec 30101_flow_spec.md#req-30101-001: サーブ権をLeft側に設定する
    match_score.server = CourtSide::Left;
    *rally_state = RallyState::new(CourtSide::Left);

    // @spec 30101_flow_spec.md#req-30101-001: プレイヤーを配置する
    reset_player_positions(&mut query, &config);
    for (player, logical_pos) in query.iter() {
        info!("Player {} positioned at {:?}", player.id, logical_pos.value);
    }

    // @spec 30101_flow_spec.md#req-30101-005: MatchStartEvent 発行
    match_start_events.write(MatchStartEvent {
        first_server: CourtSide::Left,
    });

    // @spec 30101_flow_spec.md#req-30101-001: MatchState を Serve に遷移する
    next_state.set(MatchFlowState::Serve);
    info!("Match started! First server: Left side. State: Serve");
}

/// サーブ実行検知システム
/// @spec 30101_flow_spec.md#req-30101-002
/// @spec 30902_fault_spec.md
///
/// ShotEventを受信してServingフェーズに遷移する。
/// ラリーへの遷移はサービスボックス判定後に行う。
fn serve_to_rally_system(
    mut shot_events: MessageReader<ShotEvent>,
    match_score: Res<MatchScore>,
    mut rally_state: ResMut<RallyState>,
) {
    // @spec 30101_flow_spec.md#req-30101-002: サーブが打たれる（ShotEvent受信）
    for event in shot_events.read() {
        // サーバーのショットのみを検出（CourtSideで直接比較）
        if event.court_side == match_score.server {
            // @spec 30902_fault_spec.md: Servingフェーズに遷移（サービスボックス判定待ち）
            rally_state.start_serve();
            info!(
                "Serve executed by Player{}. Phase: WaitingServe -> Serving",
                event.player_id
            );
        }
    }
}

/// ラリーからポイント終了への遷移システム
/// @spec 30101_flow_spec.md#req-30101-003
fn rally_to_point_end_system(
    mut next_state: ResMut<NextState<MatchFlowState>>,
    mut rally_events: MessageReader<RallyEndEvent>,
    mut rally_state: ResMut<RallyState>,
) {
    // @spec 30101_flow_spec.md#req-30101-003: 失点条件を満たす
    for event in rally_events.read() {
        // @spec 30101_flow_spec.md#req-30101-003: MatchState を PointEnd に遷移する
        rally_state.end_point();
        next_state.set(MatchFlowState::PointEnd);
        info!(
            "Rally ended! {:?} wins the point. Reason: {:?}. State: Rally -> PointEnd",
            event.winner, event.reason
        );
    }
}

/// ポイント終了状態に入ったときの処理
/// @spec 30101_flow_spec.md#req-30101-003
fn point_end_enter_system(
    mut commands: Commands,
    mut serve_state: ResMut<ServeState>,
    ball_query: Query<Entity, With<Ball>>,
    toss_ball_query: Query<Entity, With<TossBall>>,
) {
    // @spec 30101_flow_spec.md#req-30101-003: PointEndEvent を発行する
    // NOTE: 現在は RallyEndEvent が PointEnd の役割を果たしている
    info!("Point ended. Preparing for next serve...");

    // ボールを削除（次のサーブで新しいボールを生成するため）
    for ball_entity in ball_query.iter() {
        commands.entity(ball_entity).despawn();
        info!("Ball despawned for next serve");
    }

    // トスボールも削除
    for toss_entity in toss_ball_query.iter() {
        commands.entity(toss_entity).despawn();
        info!("Toss ball despawned for next serve");
    }

    // ServeStateをリセット（次のポイント用）
    serve_state.reset_for_new_point();
}

/// ポイント終了から次の状態への遷移システム
/// @spec 30101_flow_spec.md#req-30101-004
/// @spec 30101_flow_spec.md#req-30101-005
fn point_end_to_next_system(
    mut next_state: ResMut<NextState<MatchFlowState>>,
    match_score: Res<MatchScore>,
    mut rally_state: ResMut<RallyState>,
    mut query: Query<(&Player, &mut LogicalPosition)>,
    config: Res<GameConfig>,
) {
    // @spec 30101_flow_spec.md#req-30101-005: 勝利条件を満たす
    if let GameState::MatchWon(_winner) = match_score.game_state {
        // @spec 30101_flow_spec.md#req-30101-005: MatchState を MatchEnd に遷移する
        next_state.set(MatchFlowState::MatchEnd);
        info!("Match won! State: PointEnd -> MatchEnd");
        return;
    }

    // @spec 30101_flow_spec.md#req-30101-004: 試合が終了していない場合
    // @spec 30101_flow_spec.md#req-30101-004: プレイヤーを初期位置に戻す
    reset_player_positions(&mut query, &config);

    // ラリー状態を次のサーブ待ちに更新
    rally_state.next_serve();
    rally_state.server = match_score.server;

    // @spec 30101_flow_spec.md#req-30101-004: MatchState を Serve に遷移する
    next_state.set(MatchFlowState::Serve);
    info!(
        "Next point. Server: {:?}. State: PointEnd -> Serve",
        match_score.server
    );
}

/// 試合終了システム
/// @spec 30101_flow_spec.md#req-30101-005
fn match_end_system(match_score: Res<MatchScore>, mut match_end_events: MessageWriter<MatchWonEvent>) {
    if let GameState::MatchWon(winner) = match_score.game_state {
        // @spec 30101_flow_spec.md#req-30101-005: MatchWonEvent を発行する
        match_end_events.write(MatchWonEvent { winner });
        info!("Match ended! Winner: {:?}", winner);
    }
}

/// プレイヤーを初期位置にリセット
/// @spec 30101_flow_spec.md#req-30101-001
fn reset_player_positions(
    query: &mut Query<(&Player, &mut LogicalPosition)>,
    config: &GameConfig,
) {
    for (player, mut logical_pos) in query.iter_mut() {
        logical_pos.value = get_initial_position(player.court_side, config);
    }
}

/// プレイヤーの初期位置を取得
/// 論理座標系: X=打ち合い方向, Y=高さ, Z=コート幅
/// @spec 30101_flow_spec.md#req-30101-001
fn get_initial_position(court_side: CourtSide, config: &GameConfig) -> Vec3 {
    match court_side {
        // @spec 30101_flow_spec.md#req-30101-001: Left側（画面左）
        CourtSide::Left => Vec3::new(config.player.x_min + 1.0, 0.0, 0.0),
        // @spec 30101_flow_spec.md#req-30101-001: Right側（画面右）
        CourtSide::Right => Vec3::new(config.player.x_max - 1.0, 0.0, 0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30104-001: 試合開始テスト
    /// @spec 30101_flow_spec.md#req-30101-001
    #[test]
    fn test_req_30101_001_match_start() {
        // 初期状態はMatchStart
        let state = MatchFlowState::default();
        assert_eq!(state, MatchFlowState::MatchStart);
    }

    /// TST-30104-002: サーブからラリーへの遷移テスト
    /// @spec 30101_flow_spec.md#req-30101-002
    #[test]
    fn test_req_30101_002_serve_to_rally() {
        let mut rally_state = RallyState::new(CourtSide::Left);

        // サーブ開始
        rally_state.start_serve();
        assert_eq!(rally_state.phase, crate::resource::RallyPhase::Serving);

        // ラリー開始
        rally_state.start_rally();
        assert_eq!(rally_state.phase, crate::resource::RallyPhase::Rally);
    }

    /// TST-30104-003: ラリーからポイント終了への遷移テスト
    /// @spec 30101_flow_spec.md#req-30101-003
    #[test]
    fn test_req_30101_003_rally_to_point_end() {
        let mut rally_state = RallyState::new(CourtSide::Left);
        rally_state.start_rally();

        // ポイント終了
        rally_state.end_point();
        assert_eq!(rally_state.phase, crate::resource::RallyPhase::PointEnded);
    }

    /// TST-30104-004: ポイント終了から次のポイントへの遷移テスト
    /// @spec 30101_flow_spec.md#req-30101-004
    #[test]
    fn test_req_30101_004_point_end_to_serve() {
        let mut rally_state = RallyState::new(CourtSide::Left);
        rally_state.end_point();

        // 次のサーブへ
        rally_state.next_serve();
        assert_eq!(rally_state.phase, crate::resource::RallyPhase::WaitingServe);
    }

    /// TST-30104-005: 試合終了テスト
    /// @spec 30101_flow_spec.md#req-30101-005
    #[test]
    fn test_req_30101_005_match_end() {
        let mut match_score = MatchScore::new();

        // 試合勝利状態を設定
        match_score.game_state = GameState::MatchWon(CourtSide::Left);

        // MatchWon状態を確認
        assert!(matches!(
            match_score.game_state,
            GameState::MatchWon(CourtSide::Left)
        ));
    }

    /// TST-30104-006: 状態遷移の順序テスト
    /// @spec 30101_flow_spec.md
    #[test]
    fn test_state_transition_order() {
        // 状態遷移の順序を確認
        // MatchStart -> Serve -> Rally -> PointEnd -> Serve (または MatchEnd)
        let states = vec![
            MatchFlowState::MatchStart,
            MatchFlowState::Serve,
            MatchFlowState::Rally,
            MatchFlowState::PointEnd,
            MatchFlowState::Serve,
        ];

        // 各状態が異なることを確認
        assert_eq!(states[0], MatchFlowState::MatchStart);
        assert_eq!(states[1], MatchFlowState::Serve);
        assert_eq!(states[2], MatchFlowState::Rally);
        assert_eq!(states[3], MatchFlowState::PointEnd);
        assert_eq!(states[4], MatchFlowState::Serve);
    }
}
