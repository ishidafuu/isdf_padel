//! スコアリングリソース
//! @spec 30701_point_spec.md
//! @spec 30702_game_spec.md

use bevy::prelude::*;

use crate::core::CourtSide;

/// プレイヤーのポイント状態
/// @spec 30701_point_spec.md#req-30701-001
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerPoint {
    /// 現在のポイントインデックス（0=0, 1=15, 2=30, 3=40）
    pub index: usize,
}

impl PlayerPoint {
    /// ポイントを加算（次のインデックスへ進める）
    /// @spec 30701_point_spec.md#req-30701-002
    #[inline]
    pub fn advance(&mut self) {
        self.index += 1;
    }

    /// ポイントをリセット
    /// @spec 30701_point_spec.md#req-30701-005
    #[inline]
    pub fn reset(&mut self) {
        self.index = 0;
    }
}

/// プレイヤーのゲーム/セット状態
/// @spec 30702_game_spec.md#req-30702-001
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerGameScore {
    /// 現在のセットでのゲーム獲得数
    pub games: u32,
    /// 獲得セット数
    pub sets: u32,
}

impl PlayerGameScore {
    /// ゲームを獲得
    /// @spec 30702_game_spec.md#req-30702-002
    #[inline]
    pub fn win_game(&mut self) {
        self.games += 1;
    }

    /// セットを獲得（ゲーム数リセット）
    #[inline]
    pub fn win_set(&mut self) {
        self.sets += 1;
        self.games = 0;
    }
}

/// マッチ全体のスコア状態
/// @spec 30701_point_spec.md
#[derive(Resource, Debug, Clone)]
pub struct MatchScore {
    /// Player 1のポイント
    pub player1_point: PlayerPoint,
    /// Player 2のポイント
    pub player2_point: PlayerPoint,
    /// Player 1のゲーム/セットスコア
    pub player1_score: PlayerGameScore,
    /// Player 2のゲーム/セットスコア
    pub player2_score: PlayerGameScore,
    /// サーバー（サーブを打つ側）
    pub server: CourtSide,
    /// 現在のゲーム状態
    pub game_state: GameState,
}

/// ゲーム状態
/// @spec 30701_point_spec.md
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GameState {
    /// ゲーム進行中
    #[default]
    Playing,
    /// ゲーム勝利（勝者のCourtSide）
    GameWon(CourtSide),
    /// セット勝利（勝者のCourtSide）
    SetWon(CourtSide),
    /// マッチ勝利（勝者のCourtSide）
    MatchWon(CourtSide),
}

impl Default for MatchScore {
    fn default() -> Self {
        Self {
            player1_point: PlayerPoint::default(),
            player2_point: PlayerPoint::default(),
            player1_score: PlayerGameScore::default(),
            player2_score: PlayerGameScore::default(),
            server: CourtSide::Player1,
            game_state: GameState::default(),
        }
    }
}

impl MatchScore {
    /// 新規マッチを開始
    /// @spec 30701_point_spec.md#req-30701-001
    pub fn new() -> Self {
        Self::default()
    }

    /// 指定したプレイヤーのポイントを取得（インデックス）
    #[inline]
    pub fn get_point_index(&self, player: CourtSide) -> usize {
        match player {
            CourtSide::Player1 => self.player1_point.index,
            CourtSide::Player2 => self.player2_point.index,
        }
    }

    /// 指定したプレイヤーにポイントを加算
    /// @spec 30701_point_spec.md#req-30701-002
    pub fn add_point(&mut self, scorer: CourtSide) {
        match scorer {
            CourtSide::Player1 => self.player1_point.advance(),
            CourtSide::Player2 => self.player2_point.advance(),
        }
    }

    /// ゲーム勝利判定（40から得点で勝利、相手が40未満）
    /// @spec 30701_point_spec.md#req-30701-003
    pub fn check_game_win(&self, scorer: CourtSide, win_index: usize) -> bool {
        let scorer_index = self.get_point_index(scorer);
        let opponent_index = self.get_point_index(scorer.opponent());

        // MVP v0.1: デュースなし（40から得点で即勝利）
        // win_index = 4 の場合、index 3 (40) から得点で勝利
        scorer_index >= win_index && opponent_index < win_index - 1
            || scorer_index >= win_index
    }

    /// ポイントをリセット（ゲーム終了後）
    /// @spec 30701_point_spec.md#req-30701-005
    pub fn reset_points(&mut self) {
        self.player1_point.reset();
        self.player2_point.reset();
    }

    /// ゲーム獲得処理
    /// @spec 30702_game_spec.md#req-30702-002
    pub fn win_game(&mut self, winner: CourtSide) {
        match winner {
            CourtSide::Player1 => self.player1_score.win_game(),
            CourtSide::Player2 => self.player2_score.win_game(),
        }
        self.reset_points();
        // サーバー交代
        self.server = self.server.opponent();
    }

    /// セット獲得判定
    /// @spec 30702_game_spec.md#req-30702-003
    pub fn check_set_win(&self, winner: CourtSide, games_to_win: u32) -> bool {
        let games = match winner {
            CourtSide::Player1 => self.player1_score.games,
            CourtSide::Player2 => self.player2_score.games,
        };
        games >= games_to_win
    }

    /// セット獲得処理
    pub fn win_set(&mut self, winner: CourtSide) {
        match winner {
            CourtSide::Player1 => self.player1_score.win_set(),
            CourtSide::Player2 => self.player2_score.win_set(),
        }
    }

    /// マッチ勝利判定
    pub fn check_match_win(&self, winner: CourtSide, sets_to_win: u32) -> bool {
        let sets = match winner {
            CourtSide::Player1 => self.player1_score.sets,
            CourtSide::Player2 => self.player2_score.sets,
        };
        sets >= sets_to_win
    }

    /// ポイント表示用文字列を取得
    /// @spec 30701_point_spec.md#req-30701-004
    pub fn get_point_display(&self, player: CourtSide, point_values: &[u32]) -> String {
        let index = self.get_point_index(player);
        if index < point_values.len() {
            point_values[index].to_string()
        } else {
            "Game".to_string()
        }
    }
}
