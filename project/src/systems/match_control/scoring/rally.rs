//! ラリー・ポイント処理
//! @spec 30701_point_spec.md
//! @spec 30903_serve_authority_spec.md

use bevy::prelude::*;

use crate::core::{GameWonEvent, MatchWonEvent, PointScoredEvent, RallyEndEvent, SetWonEvent};
use crate::resource::{GameConfig, GameState, MatchScore, RallyState};

use super::game_set::{handle_game_win, handle_point_scored};

/// ラリー終了イベントを処理してポイントを加算
/// @spec 30701_point_spec.md#req-30701-002
/// @spec 30903_serve_authority_spec.md#req-30903-002
/// @spec 30903_serve_authority_spec.md#req-30903-003
///
/// 重複防止:
/// - 同一ポイント内で既にスコア加算済みならスキップ（フレームをまたいだ重複防止）
/// - 1フレームにつき最初のイベントのみ処理
#[allow(clippy::too_many_arguments)]
pub fn rally_end_system(
    mut rally_events: MessageReader<RallyEndEvent>,
    mut point_events: MessageWriter<PointScoredEvent>,
    mut game_events: MessageWriter<GameWonEvent>,
    mut set_events: MessageWriter<SetWonEvent>,
    mut match_events: MessageWriter<MatchWonEvent>,
    mut match_score: ResMut<MatchScore>,
    mut rally_state: ResMut<RallyState>,
    config: Res<GameConfig>,
) {
    for event in rally_events.read() {
        // このポイントで既にスコア加算済みならスキップ（フレームをまたいだ重複防止）
        if rally_state.point_scored_this_rally {
            warn!("Skipping duplicate RallyEndEvent (already scored this rally): {:?}", event.reason);
            continue;
        }

        // ゲーム進行中でなければスキップ
        if match_score.game_state != GameState::Playing {
            continue;
        }

        let scorer = event.winner;
        let point_values = &config.scoring.point_values;
        let win_index = point_values.len();

        // ポイント加算
        // @spec 30701_point_spec.md#req-30701-002
        match_score.add_point(scorer);

        // 重複加算防止フラグを設定
        rally_state.point_scored_this_rally = true;

        let new_index = match_score.get_point_index(scorer);

        // ゲーム勝利判定
        // @spec 30701_point_spec.md#req-30701-003
        if new_index >= win_index {
            handle_game_win(
                scorer,
                &mut match_score,
                &mut rally_state,
                &config,
                &mut game_events,
                &mut set_events,
                &mut match_events,
            );
        } else {
            handle_point_scored(
                scorer,
                new_index,
                point_values,
                &match_score,
                &mut rally_state,
                &mut point_events,
            );
        }
    }
}

/// ポイント獲得イベントを処理（将来のUI更新などに使用）
/// @spec 30701_point_spec.md#req-30701-004
pub fn point_scored_system(mut point_events: MessageReader<PointScoredEvent>) {
    for event in point_events.read() {
        // 現在はログ出力のみ
        // 将来はUI更新などを行う
        debug!(
            "PointScoredEvent: {:?} scored, new value: {}",
            event.scorer, event.new_point_value
        );
    }
}
