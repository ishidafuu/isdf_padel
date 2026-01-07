//! スコアリングシステム
//! @spec 30701_point_spec.md
//! @spec 30702_game_spec.md
//! @spec 30703_set_spec.md

use bevy::prelude::*;

use crate::core::{
    CourtSide, GameWonEvent, MatchWonEvent, PointScoredEvent, RallyEndEvent, SetWonEvent,
};
use crate::resource::{GameConfig, GameState, MatchScore};

/// スコアリングプラグイン
/// @spec 30701_point_spec.md
pub struct ScoringPlugin;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MatchScore>()
            .add_message::<RallyEndEvent>()
            .add_message::<PointScoredEvent>()
            .add_message::<GameWonEvent>()
            .add_message::<SetWonEvent>()
            .add_message::<MatchWonEvent>()
            .add_systems(
                Update,
                (
                    rally_end_system,
                    point_scored_system,
                    score_display_system,
                )
                    .chain(),
            );
    }
}

/// ラリー終了イベントを処理してポイントを加算
/// @spec 30701_point_spec.md#req-30701-002
pub fn rally_end_system(
    mut rally_events: MessageReader<RallyEndEvent>,
    mut point_events: MessageWriter<PointScoredEvent>,
    mut game_events: MessageWriter<GameWonEvent>,
    mut set_events: MessageWriter<SetWonEvent>,
    mut match_events: MessageWriter<MatchWonEvent>,
    mut match_score: ResMut<MatchScore>,
    config: Res<GameConfig>,
) {
    for event in rally_events.read() {
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

        let new_index = match_score.get_point_index(scorer);

        // ゲーム勝利判定
        // @spec 30701_point_spec.md#req-30701-003
        if new_index >= win_index {
            // ゲーム獲得
            // @spec 30702_game_spec.md#req-30702-002
            match_score.win_game(scorer);

            let games_won = match scorer {
                CourtSide::Player1 => match_score.player1_score.games,
                CourtSide::Player2 => match_score.player2_score.games,
            };

            info!(
                "Game won by {:?}! Games: P1={}, P2={}",
                scorer, match_score.player1_score.games, match_score.player2_score.games
            );

            game_events.write(GameWonEvent {
                winner: scorer,
                games_won,
            });

            // セット勝利判定
            // @spec 30703_set_spec.md#req-30703-002
            if match_score.check_set_win(scorer, config.scoring.games_to_win_set) {
                // @spec 30703_set_spec.md#req-30703-002
                match_score.win_set(scorer);

                let sets_won = match scorer {
                    CourtSide::Player1 => match_score.player1_score.sets,
                    CourtSide::Player2 => match_score.player2_score.sets,
                };

                info!(
                    "Set won by {:?}! Sets: P1={}, P2={}",
                    scorer, match_score.player1_score.sets, match_score.player2_score.sets
                );

                set_events.write(SetWonEvent {
                    winner: scorer,
                    sets_won,
                });

                // マッチ勝利判定
                // @spec 30703_set_spec.md#req-30703-003
                if match_score.check_match_win(scorer, config.scoring.sets_to_win_match) {
                    match_score.game_state = GameState::MatchWon(scorer);

                    info!("Match won by {:?}!", scorer);

                    match_events.write(MatchWonEvent { winner: scorer });
                }
            }
        } else {
            // ポイント獲得（ゲーム未終了）
            let new_point_value = if new_index < point_values.len() {
                point_values[new_index]
            } else {
                0
            };

            info!(
                "Point scored by {:?}! Score: {} - {}",
                scorer,
                match_score.get_point_display(CourtSide::Player1, point_values),
                match_score.get_point_display(CourtSide::Player2, point_values),
            );

            point_events.write(PointScoredEvent {
                scorer,
                new_point_value,
            });
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

/// スコア表示システム（デバッグ用）
/// @spec 30701_point_spec.md#req-30701-004
pub fn score_display_system(
    match_score: Res<MatchScore>,
    config: Res<GameConfig>,
    mut last_display: Local<Option<String>>,
) {
    let point_values = &config.scoring.point_values;
    let p1_point = match_score.get_point_display(CourtSide::Player1, point_values);
    let p2_point = match_score.get_point_display(CourtSide::Player2, point_values);

    let score_text = format!(
        "P1: {} (G:{} S:{}) - P2: {} (G:{} S:{})",
        p1_point,
        match_score.player1_score.games,
        match_score.player1_score.sets,
        p2_point,
        match_score.player2_score.games,
        match_score.player2_score.sets,
    );

    // 変更があった場合のみ表示
    if last_display.as_ref() != Some(&score_text) {
        info!("Score: {}", score_text);
        *last_display = Some(score_text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30705-001: ポイント初期化テスト
    /// @spec 30701_point_spec.md#req-30701-001
    #[test]
    fn test_point_initialization() {
        let match_score = MatchScore::new();
        assert_eq!(match_score.player1_point.index, 0);
        assert_eq!(match_score.player2_point.index, 0);
    }

    /// TST-30705-002: ポイント加算テスト
    /// @spec 30701_point_spec.md#req-30701-002
    #[test]
    fn test_point_progression() {
        let mut match_score = MatchScore::new();
        let point_values = vec![0, 15, 30, 40];

        // 0 -> 15
        match_score.add_point(CourtSide::Player1);
        assert_eq!(match_score.get_point_index(CourtSide::Player1), 1);
        assert_eq!(
            match_score.get_point_display(CourtSide::Player1, &point_values),
            "15"
        );

        // 15 -> 30
        match_score.add_point(CourtSide::Player1);
        assert_eq!(match_score.get_point_index(CourtSide::Player1), 2);
        assert_eq!(
            match_score.get_point_display(CourtSide::Player1, &point_values),
            "30"
        );

        // 30 -> 40
        match_score.add_point(CourtSide::Player1);
        assert_eq!(match_score.get_point_index(CourtSide::Player1), 3);
        assert_eq!(
            match_score.get_point_display(CourtSide::Player1, &point_values),
            "40"
        );
    }

    /// TST-30705-003: ゲーム勝利判定テスト
    /// @spec 30701_point_spec.md#req-30701-003
    #[test]
    fn test_game_win_detection() {
        let mut match_score = MatchScore::new();
        let win_index = 4; // point_values.len() = 4

        // Player1 が 40 に到達
        match_score.add_point(CourtSide::Player1); // 15
        match_score.add_point(CourtSide::Player1); // 30
        match_score.add_point(CourtSide::Player1); // 40

        // まだ勝利ではない
        assert!(!match_score.check_game_win(CourtSide::Player1, win_index));

        // 40 -> Game
        match_score.add_point(CourtSide::Player1);
        assert!(match_score.check_game_win(CourtSide::Player1, win_index));
    }

    /// TST-30705-004: ポイント表示テスト
    /// @spec 30701_point_spec.md#req-30701-004
    #[test]
    fn test_point_display() {
        let mut match_score = MatchScore::new();
        let point_values = vec![0, 15, 30, 40];

        assert_eq!(
            match_score.get_point_display(CourtSide::Player1, &point_values),
            "0"
        );

        match_score.add_point(CourtSide::Player1);
        assert_eq!(
            match_score.get_point_display(CourtSide::Player1, &point_values),
            "15"
        );

        // Game表示（インデックス超過時）
        match_score.add_point(CourtSide::Player1); // 30
        match_score.add_point(CourtSide::Player1); // 40
        match_score.add_point(CourtSide::Player1); // Game
        assert_eq!(
            match_score.get_point_display(CourtSide::Player1, &point_values),
            "Game"
        );
    }

    /// TST-30705-005: ポイントリセットテスト
    /// @spec 30701_point_spec.md#req-30701-005
    #[test]
    fn test_point_reset() {
        let mut match_score = MatchScore::new();

        // ポイントを獲得
        match_score.add_point(CourtSide::Player1);
        match_score.add_point(CourtSide::Player2);

        // リセット
        match_score.reset_points();

        assert_eq!(match_score.player1_point.index, 0);
        assert_eq!(match_score.player2_point.index, 0);
    }

    // ========================================
    // 30702: ゲームカウント管理テスト
    // ========================================

    /// TST-30706-001: ゲームカウント初期化テスト
    /// @spec 30702_game_spec.md#req-30702-001
    #[test]
    fn test_game_count_initialization() {
        let match_score = MatchScore::new();
        assert_eq!(match_score.player1_score.games, 0);
        assert_eq!(match_score.player2_score.games, 0);
        assert_eq!(match_score.player1_score.sets, 0);
        assert_eq!(match_score.player2_score.sets, 0);
    }

    /// TST-30706-002: ゲームカウント加算テスト
    /// @spec 30702_game_spec.md#req-30702-002
    #[test]
    fn test_game_count_increment() {
        let mut match_score = MatchScore::new();

        // Player1がゲームを獲得
        match_score.win_game(CourtSide::Player1);
        assert_eq!(match_score.player1_score.games, 1);
        assert_eq!(match_score.player2_score.games, 0);

        // Player2がゲームを獲得
        match_score.win_game(CourtSide::Player2);
        assert_eq!(match_score.player1_score.games, 1);
        assert_eq!(match_score.player2_score.games, 1);

        // Player1が再度ゲームを獲得
        match_score.win_game(CourtSide::Player1);
        assert_eq!(match_score.player1_score.games, 2);
    }

    /// TST-30706-003: セット勝利判定テスト（6ゲーム先取）
    /// @spec 30702_game_spec.md#req-30702-003
    #[test]
    fn test_set_win_at_six_games() {
        let mut match_score = MatchScore::new();
        let games_to_win = 6;

        // Player1が5ゲーム獲得（まだセット勝利ではない）
        for _ in 0..5 {
            match_score.win_game(CourtSide::Player1);
        }
        assert!(!match_score.check_set_win(CourtSide::Player1, games_to_win));

        // Player1が6ゲーム目を獲得（セット勝利）
        match_score.win_game(CourtSide::Player1);
        assert!(match_score.check_set_win(CourtSide::Player1, games_to_win));
    }

    /// TST-30706-004: ポイントリセット確認（ゲーム獲得後）
    /// @spec 30702_game_spec.md#req-30702-002
    #[test]
    fn test_points_reset_after_game_win() {
        let mut match_score = MatchScore::new();

        // ポイントを獲得
        match_score.add_point(CourtSide::Player1);
        match_score.add_point(CourtSide::Player1);
        assert_eq!(match_score.get_point_index(CourtSide::Player1), 2);

        // ゲーム獲得（ポイントもリセットされる）
        match_score.win_game(CourtSide::Player1);

        // ポイントがリセットされていることを確認
        assert_eq!(match_score.player1_point.index, 0);
        assert_eq!(match_score.player2_point.index, 0);
        // ゲームカウントが増えていることを確認
        assert_eq!(match_score.player1_score.games, 1);
    }

    /// TST-30706-005: ゲームカウント表示テスト
    /// @spec 30702_game_spec.md#req-30702-005
    #[test]
    fn test_game_count_display() {
        let mut match_score = MatchScore::new();

        // 初期状態
        assert_eq!(match_score.player1_score.games, 0);
        assert_eq!(match_score.player2_score.games, 0);

        // ゲーム獲得後の表示
        match_score.win_game(CourtSide::Player1);
        match_score.win_game(CourtSide::Player1);
        match_score.win_game(CourtSide::Player2);

        assert_eq!(match_score.player1_score.games, 2);
        assert_eq!(match_score.player2_score.games, 1);
    }

    /// TST-30706-006: サーバー交代テスト（ゲーム獲得後）
    /// @spec 30702_game_spec.md
    #[test]
    fn test_server_switch_after_game() {
        let mut match_score = MatchScore::new();

        // 初期サーバーはPlayer1
        assert_eq!(match_score.server, CourtSide::Player1);

        // ゲーム獲得後はPlayer2がサーバー
        match_score.win_game(CourtSide::Player1);
        assert_eq!(match_score.server, CourtSide::Player2);

        // 次のゲーム後はPlayer1がサーバー
        match_score.win_game(CourtSide::Player2);
        assert_eq!(match_score.server, CourtSide::Player1);
    }

    /// TST-30706-007: セット獲得後のゲームカウントリセットテスト
    /// @spec 30702_game_spec.md
    #[test]
    fn test_game_count_reset_after_set_win() {
        let mut match_score = MatchScore::new();

        // Player1が6ゲーム獲得
        for _ in 0..6 {
            match_score.win_game(CourtSide::Player1);
        }
        assert_eq!(match_score.player1_score.games, 6);

        // セット獲得
        match_score.win_set(CourtSide::Player1);

        // ゲームカウントがリセットされ、セット数が増加
        assert_eq!(match_score.player1_score.games, 0);
        assert_eq!(match_score.player1_score.sets, 1);
    }

    // ========================================
    // 30703: セットカウント管理テスト
    // ========================================

    /// TST-30707-001: セットカウント初期化テスト
    /// @spec 30703_set_spec.md#req-30703-001
    #[test]
    fn test_set_count_initialization() {
        let match_score = MatchScore::new();
        // @spec 30703_set_spec.md#req-30703-001: 両プレイヤーのセットカウントを0に初期化
        assert_eq!(match_score.player1_score.sets, 0);
        assert_eq!(match_score.player2_score.sets, 0);
    }

    /// TST-30707-002: セットカウント加算テスト
    /// @spec 30703_set_spec.md#req-30703-002
    #[test]
    fn test_set_count_increment() {
        let mut match_score = MatchScore::new();

        // Player1がセットを獲得
        match_score.win_set(CourtSide::Player1);
        assert_eq!(match_score.player1_score.sets, 1);
        assert_eq!(match_score.player2_score.sets, 0);

        // Player2がセットを獲得
        match_score.win_set(CourtSide::Player2);
        assert_eq!(match_score.player1_score.sets, 1);
        assert_eq!(match_score.player2_score.sets, 1);
    }

    /// TST-30707-003: マッチ勝利判定テスト（1セット制）
    /// @spec 30703_set_spec.md#req-30703-003
    #[test]
    fn test_match_win_one_set() {
        let mut match_score = MatchScore::new();
        let sets_to_win = 1; // 1セット制

        // セット獲得前は勝利ではない
        assert!(!match_score.check_match_win(CourtSide::Player1, sets_to_win));

        // Player1がセットを獲得
        match_score.win_set(CourtSide::Player1);

        // 1セット獲得でマッチ勝利
        assert!(match_score.check_match_win(CourtSide::Player1, sets_to_win));
        // 相手はまだ勝利ではない
        assert!(!match_score.check_match_win(CourtSide::Player2, sets_to_win));
    }

    /// TST-30707-004: セットカウント表示テスト
    /// @spec 30703_set_spec.md#req-30703-004
    #[test]
    fn test_set_count_display() {
        let mut match_score = MatchScore::new();

        // 初期状態: 0-0
        assert_eq!(match_score.player1_score.sets, 0);
        assert_eq!(match_score.player2_score.sets, 0);

        // セット獲得後
        match_score.win_set(CourtSide::Player1);
        // 表示: "1" - "0"
        assert_eq!(match_score.player1_score.sets, 1);
        assert_eq!(match_score.player2_score.sets, 0);
    }
}
