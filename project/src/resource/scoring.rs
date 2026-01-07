//! スコアリングリソース
//! @spec 30701_point_spec.md
//! @spec 30702_game_spec.md
//! @spec 30703_set_spec.md
//! @spec 30101_flow_spec.md
//! @spec 30903_serve_authority_spec.md

use bevy::prelude::*;

use crate::core::CourtSide;
use crate::resource::config::ServeSide;

/// 試合フロー状態
/// @spec 30101_flow_spec.md#MatchStateType
#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum MatchFlowState {
    /// 試合開始
    #[default]
    MatchStart,
    /// サーブ待機
    Serve,
    /// ラリー中
    Rally,
    /// ポイント終了
    PointEnd,
    /// 試合終了
    MatchEnd,
}

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
    /// @spec 30703_set_spec.md#req-30703-002
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

    /// セット獲得判定（6ゲーム先取でセット獲得）
    /// @spec 30703_set_spec.md#req-30703-002
    pub fn check_set_win(&self, winner: CourtSide, games_to_win: u32) -> bool {
        let games = match winner {
            CourtSide::Player1 => self.player1_score.games,
            CourtSide::Player2 => self.player2_score.games,
        };
        games >= games_to_win
    }

    /// セット獲得処理
    /// @spec 30703_set_spec.md#req-30703-002
    pub fn win_set(&mut self, winner: CourtSide) {
        match winner {
            CourtSide::Player1 => self.player1_score.win_set(),
            CourtSide::Player2 => self.player2_score.win_set(),
        }
    }

    /// マッチ勝利判定（1セット制: 1セット先取でマッチ勝利）
    /// @spec 30703_set_spec.md#req-30703-003
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

/// ラリーフェーズ
/// @spec 30901_point_judgment_spec.md#req-30901-003
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RallyPhase {
    /// サーブ待機中
    #[default]
    WaitingServe,
    /// サーブ中（ボールが打たれた後、最初の着地まで）
    Serving,
    /// ラリー中（サーブが有効に入った後）
    Rally,
    /// ポイント終了（次のサーブ待ち）
    PointEnded,
}

/// ラリー状態リソース
/// @spec 30901_point_judgment_spec.md
/// @spec 30903_serve_authority_spec.md
#[derive(Resource, Debug, Clone)]
pub struct RallyState {
    /// 現在のラリーフェーズ
    pub phase: RallyPhase,
    /// 現在のサーバー
    pub server: CourtSide,
    /// 現在のサーブサイド
    /// @spec 30903_serve_authority_spec.md#req-30903-003
    pub serve_side: ServeSide,
    /// サーブのファウル回数（0 or 1）
    pub fault_count: u32,
}

impl Default for RallyState {
    fn default() -> Self {
        Self {
            phase: RallyPhase::WaitingServe,
            server: CourtSide::Player1,
            serve_side: ServeSide::Deuce,
            fault_count: 0,
        }
    }
}

impl RallyState {
    /// 新規ラリー状態を作成
    /// @spec 30903_serve_authority_spec.md#req-30903-001
    pub fn new(server: CourtSide) -> Self {
        Self {
            phase: RallyPhase::WaitingServe,
            server,
            serve_side: ServeSide::Deuce,
            fault_count: 0,
        }
    }

    /// サーブサイドを更新（ポイント合計から判定）
    /// @spec 30903_serve_authority_spec.md#req-30903-003
    pub fn update_serve_side(&mut self, server_points: usize, receiver_points: usize) {
        let total = server_points + receiver_points;
        self.serve_side = ServeSide::from_point_total(total);
    }

    /// サーブ開始
    pub fn start_serve(&mut self) {
        self.phase = RallyPhase::Serving;
    }

    /// ラリー開始（サーブが有効に入った）
    pub fn start_rally(&mut self) {
        self.phase = RallyPhase::Rally;
    }

    /// ポイント終了
    pub fn end_point(&mut self) {
        self.phase = RallyPhase::PointEnded;
        self.fault_count = 0;
    }

    /// 次のサーブへ（サーバー変更なし）
    pub fn next_serve(&mut self) {
        self.phase = RallyPhase::WaitingServe;
    }

    /// ファウル記録
    pub fn record_fault(&mut self) {
        self.fault_count += 1;
    }

    /// ダブルフォルトか判定
    pub fn is_double_fault(&self) -> bool {
        self.fault_count >= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================
    // 30903: サーブ権管理テスト
    // ========================================

    /// TST-30904-020: サーブ権初期化テスト
    /// @spec 30903_serve_authority_spec.md#req-30903-001
    #[test]
    fn test_req_30903_001_serve_authority_init() {
        let rally_state = RallyState::new(CourtSide::Player1);

        // 初期サーバーはPlayer1
        assert_eq!(rally_state.server, CourtSide::Player1);
        // 初期サーブサイドはデュース
        assert_eq!(rally_state.serve_side, ServeSide::Deuce);
    }

    /// TST-30904-021: サーブ権交代テスト（ゲーム終了時）
    /// @spec 30903_serve_authority_spec.md#req-30903-002
    #[test]
    fn test_req_30903_002_server_switch_on_game_end() {
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

    /// TST-30904-022: デュースサイド/アドサイド判定テスト
    /// @spec 30903_serve_authority_spec.md#req-30903-003
    #[test]
    fn test_req_30903_003_serve_side_determination() {
        // ポイント合計0（偶数）→ デュースサイド
        assert_eq!(ServeSide::from_point_total(0), ServeSide::Deuce);

        // ポイント合計1（奇数）→ アドサイド
        assert_eq!(ServeSide::from_point_total(1), ServeSide::Ad);

        // ポイント合計2（偶数）→ デュースサイド
        assert_eq!(ServeSide::from_point_total(2), ServeSide::Deuce);

        // ポイント合計3（奇数）→ アドサイド
        assert_eq!(ServeSide::from_point_total(3), ServeSide::Ad);
    }

    /// TST-30904-023: RallyStateサーブサイド更新テスト
    /// @spec 30903_serve_authority_spec.md#req-30903-003
    #[test]
    fn test_rally_state_update_serve_side() {
        let mut rally_state = RallyState::new(CourtSide::Player1);

        // 初期状態: 0-0 → デュース
        assert_eq!(rally_state.serve_side, ServeSide::Deuce);

        // サーバー1ポイント: 1-0 → アド（合計1）
        rally_state.update_serve_side(1, 0);
        assert_eq!(rally_state.serve_side, ServeSide::Ad);

        // レシーバー1ポイント: 1-1 → デュース（合計2）
        rally_state.update_serve_side(1, 1);
        assert_eq!(rally_state.serve_side, ServeSide::Deuce);

        // サーバー2ポイント: 2-1 → アド（合計3）
        rally_state.update_serve_side(2, 1);
        assert_eq!(rally_state.serve_side, ServeSide::Ad);

        // サーバー3ポイント: 3-1 → デュース（合計4）
        rally_state.update_serve_side(3, 1);
        assert_eq!(rally_state.serve_side, ServeSide::Deuce);
    }
}
