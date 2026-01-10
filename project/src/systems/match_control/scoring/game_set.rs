//! ゲーム・セット勝利処理
//! @spec 30702_game_spec.md
//! @spec 30703_set_spec.md

use bevy::prelude::*;

use crate::core::{
    CourtSide, GameWonEvent, MatchWonEvent, PointScoredEvent, SetWonEvent,
};
use crate::resource::{GameConfig, GameState, MatchScore, RallyState, ServeSide};

/// ゲーム勝利時の処理
/// @spec 30702_game_spec.md#req-30702-002
/// @spec 30703_set_spec.md#req-30703-002
/// @spec 30703_set_spec.md#req-30703-003
/// @spec 30903_serve_authority_spec.md#req-30903-002
pub(super) fn handle_game_win(
    scorer: CourtSide,
    match_score: &mut MatchScore,
    rally_state: &mut RallyState,
    config: &GameConfig,
    game_events: &mut MessageWriter<GameWonEvent>,
    set_events: &mut MessageWriter<SetWonEvent>,
    match_events: &mut MessageWriter<MatchWonEvent>,
) {
    // ゲーム獲得
    match_score.win_game(scorer);

    let games_won = match_score.get_score(scorer).games;

    info!(
        "Game won by {:?}! Games: P1={}, P2={}",
        scorer,
        match_score.get_score(CourtSide::Left).games,
        match_score.get_score(CourtSide::Right).games
    );

    game_events.write(GameWonEvent {
        winner: scorer,
        games_won,
    });

    // ゲーム終了時はデュースサイドから開始
    rally_state.serve_side = ServeSide::Deuce;

    // セット勝利判定
    if !match_score.check_set_win(scorer, config.scoring.games_to_win_set) {
        return;
    }

    handle_set_win(scorer, match_score, config, set_events, match_events);
}

/// セット勝利時の処理
/// @spec 30703_set_spec.md#req-30703-002
/// @spec 30703_set_spec.md#req-30703-003
fn handle_set_win(
    scorer: CourtSide,
    match_score: &mut MatchScore,
    config: &GameConfig,
    set_events: &mut MessageWriter<SetWonEvent>,
    match_events: &mut MessageWriter<MatchWonEvent>,
) {
    match_score.win_set(scorer);

    let sets_won = match_score.get_score(scorer).sets;

    info!(
        "Set won by {:?}! Sets: P1={}, P2={}",
        scorer,
        match_score.get_score(CourtSide::Left).sets,
        match_score.get_score(CourtSide::Right).sets
    );

    set_events.write(SetWonEvent {
        winner: scorer,
        sets_won,
    });

    // マッチ勝利判定
    if match_score.check_match_win(scorer, config.scoring.sets_to_win_match) {
        match_score.game_state = GameState::MatchWon(scorer);
        info!("Match won by {:?}!", scorer);
        match_events.write(MatchWonEvent { winner: scorer });
    }
}

/// ポイント獲得（ゲーム未終了）時の処理
/// @spec 30701_point_spec.md#req-30701-004
/// @spec 30903_serve_authority_spec.md#req-30903-003
pub(super) fn handle_point_scored(
    scorer: CourtSide,
    new_index: usize,
    point_values: &[u32],
    match_score: &MatchScore,
    rally_state: &mut RallyState,
    point_events: &mut MessageWriter<PointScoredEvent>,
) {
    let new_point_value = if new_index < point_values.len() {
        point_values[new_index]
    } else {
        0
    };

    info!(
        "Point scored by {:?}! Score: {} - {}",
        scorer,
        match_score.get_point_display(CourtSide::Left, point_values),
        match_score.get_point_display(CourtSide::Right, point_values),
    );

    point_events.write(PointScoredEvent {
        scorer,
        new_point_value,
    });

    // サーブサイドを更新
    let server_points = match_score.get_point_index(match_score.server);
    let receiver_points = match_score.get_point_index(match_score.server.opponent());
    rally_state.update_serve_side(server_points, receiver_points);

    info!(
        "Serve side: {:?} (total: {})",
        rally_state.serve_side,
        server_points + receiver_points
    );
}
