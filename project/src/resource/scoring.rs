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
/// ECS設計原則: CourtSideベースの配列アクセス（固定識別子を排除）
#[derive(Resource, Debug, Clone)]
pub struct MatchScore {
    /// 各サイドのポイント [Left側, Right側]
    pub points: [PlayerPoint; 2],
    /// 各サイドのゲーム/セットスコア [Left側, Right側]
    pub scores: [PlayerGameScore; 2],
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
    #[allow(dead_code)]
    GameWon(CourtSide),
    /// セット勝利（勝者のCourtSide）
    #[allow(dead_code)]
    SetWon(CourtSide),
    /// マッチ勝利（勝者のCourtSide）
    MatchWon(CourtSide),
}

impl Default for MatchScore {
    fn default() -> Self {
        Self {
            points: [PlayerPoint::default(), PlayerPoint::default()],
            scores: [PlayerGameScore::default(), PlayerGameScore::default()],
            server: CourtSide::Left,
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

    /// 指定したサイドのポイントを取得
    #[inline]
    pub fn get_point(&self, side: CourtSide) -> &PlayerPoint {
        &self.points[side as usize]
    }

    /// 指定したサイドのポイントを取得（可変）
    #[inline]
    pub fn get_point_mut(&mut self, side: CourtSide) -> &mut PlayerPoint {
        &mut self.points[side as usize]
    }

    /// 指定したサイドのスコアを取得
    #[inline]
    pub fn get_score(&self, side: CourtSide) -> &PlayerGameScore {
        &self.scores[side as usize]
    }

    /// 指定したサイドのスコアを取得（可変）
    #[inline]
    pub fn get_score_mut(&mut self, side: CourtSide) -> &mut PlayerGameScore {
        &mut self.scores[side as usize]
    }

    /// 指定したプレイヤーのポイントを取得（インデックス）
    #[inline]
    pub fn get_point_index(&self, player: CourtSide) -> usize {
        self.get_point(player).index
    }

    /// 指定したプレイヤーにポイントを加算
    /// @spec 30701_point_spec.md#req-30701-002
    pub fn add_point(&mut self, scorer: CourtSide) {
        self.get_point_mut(scorer).advance();
    }

    /// ゲーム勝利判定（40から得点で勝利、相手が40未満）
    /// @spec 30701_point_spec.md#req-30701-003
    #[allow(dead_code)]
    pub fn check_game_win(&self, scorer: CourtSide, win_index: usize) -> bool {
        let scorer_index = self.get_point_index(scorer);
        // MVP v0.1: デュースなし（40から得点で即勝利）
        // win_index = 4 の場合、index 3 (40) から得点で勝利
        scorer_index >= win_index
    }

    /// ポイントをリセット（ゲーム終了後）
    /// @spec 30701_point_spec.md#req-30701-005
    pub fn reset_points(&mut self) {
        for point in &mut self.points {
            point.reset();
        }
    }

    /// ゲーム獲得処理
    /// @spec 30702_game_spec.md#req-30702-002
    pub fn win_game(&mut self, winner: CourtSide) {
        self.get_score_mut(winner).win_game();
        self.reset_points();
        // サーバー交代
        self.server = self.server.opponent();
    }

    /// セット獲得判定（6ゲーム先取でセット獲得）
    /// @spec 30703_set_spec.md#req-30703-002
    pub fn check_set_win(&self, winner: CourtSide, games_to_win: u32) -> bool {
        self.get_score(winner).games >= games_to_win
    }

    /// セット獲得処理
    /// @spec 30703_set_spec.md#req-30703-002
    pub fn win_set(&mut self, winner: CourtSide) {
        self.get_score_mut(winner).win_set();
    }

    /// マッチ勝利判定（1セット制: 1セット先取でマッチ勝利）
    /// @spec 30703_set_spec.md#req-30703-003
    pub fn check_match_win(&self, winner: CourtSide, sets_to_win: u32) -> bool {
        self.get_score(winner).sets >= sets_to_win
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

/// サーブサブフェーズ
/// @spec 30102_serve_spec.md#req-30102-080
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ServeSubPhase {
    /// トス待機中（1回目ボタン待ち）
    #[default]
    Waiting,
    /// トス中（ボールが上昇・落下中）
    Tossing,
    /// ヒット準備完了（ヒット可能高さに到達）
    /// Note: この状態は判定用で、ヒット自体はTossing中に行う
    #[allow(dead_code)]
    HitReady,
}

/// サーブ状態
/// @spec 30102_serve_spec.md#req-30102-080
#[derive(Resource, Debug, Clone)]
pub struct ServeState {
    /// 現在のサーブサブフェーズ
    pub phase: ServeSubPhase,
    /// トス開始からの経過時間（秒）
    pub toss_time: f32,
    /// トス開始位置（サーバーの位置）
    pub toss_origin: Option<Vec3>,
    /// フォルト回数（0 or 1、2でダブルフォルト）
    pub fault_count: u8,
}

impl Default for ServeState {
    fn default() -> Self {
        Self {
            phase: ServeSubPhase::default(),
            toss_time: 0.0,
            toss_origin: None,
            fault_count: 0,
        }
    }
}

impl ServeState {
    /// 新規サーブ状態を作成
    pub fn new() -> Self {
        Self::default()
    }

    /// トスを開始
    /// @spec 30102_serve_spec.md#req-30102-080
    pub fn start_toss(&mut self, origin: Vec3) {
        self.phase = ServeSubPhase::Tossing;
        self.toss_time = 0.0;
        self.toss_origin = Some(origin);
    }

    /// トス時間を更新
    pub fn update_toss_time(&mut self, delta: f32) {
        self.toss_time += delta;
    }

    /// フォルト記録
    /// @spec 30102_serve_spec.md#req-30102-084
    /// NOTE: テストコードで使用
    #[allow(dead_code)]
    pub fn record_fault(&mut self) {
        self.fault_count += 1;
        self.phase = ServeSubPhase::Waiting;
        self.toss_time = 0.0;
        self.toss_origin = None;
    }

    /// ダブルフォルト判定
    /// @spec 30102_serve_spec.md#req-30102-089
    pub fn is_double_fault(&self) -> bool {
        self.fault_count >= 2
    }

    /// トス状態のみリセット（共通処理）
    #[inline]
    fn reset_toss_state(&mut self) {
        self.phase = ServeSubPhase::Waiting;
        self.toss_time = 0.0;
        self.toss_origin = None;
    }

    /// ヒット成功時のリセット（Rallyへ遷移するため）
    pub fn on_hit_success(&mut self) {
        self.reset_toss_state();
    }

    /// 打ち直し（let）時のリセット
    /// @spec 30102_serve_spec.md#req-30102-084
    pub fn reset_for_retry(&mut self) {
        self.reset_toss_state();
    }

    /// ポイント開始時のリセット
    pub fn reset_for_new_point(&mut self) {
        self.reset_toss_state();
        self.fault_count = 0;
    }
}

/// ポイント終了時のディレイタイマー
#[derive(Resource, Default)]
pub struct PointEndTimer {
    /// 残り待機時間（秒）
    pub remaining: f32,
    /// フォルト用ディレイの場合 true（fault_count をリセットしない）
    pub is_fault_delay: bool,
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
    /// このフレームで RallyEndEvent が発行済みかどうか
    /// 同一フレーム内での重複イベント発行を防止する
    pub rally_end_event_sent_this_frame: bool,
}

impl Default for RallyState {
    fn default() -> Self {
        Self {
            phase: RallyPhase::WaitingServe,
            server: CourtSide::Left,
            serve_side: ServeSide::Deuce,
            fault_count: 0,
            rally_end_event_sent_this_frame: false,
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
            rally_end_event_sent_this_frame: false,
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
        let rally_state = RallyState::new(CourtSide::Left);

        // 初期サーバーはLeft側
        assert_eq!(rally_state.server, CourtSide::Left);
        // 初期サーブサイドはデュース
        assert_eq!(rally_state.serve_side, ServeSide::Deuce);
    }

    /// TST-30904-021: サーブ権交代テスト（ゲーム終了時）
    /// @spec 30903_serve_authority_spec.md#req-30903-002
    #[test]
    fn test_req_30903_002_server_switch_on_game_end() {
        let mut match_score = MatchScore::new();

        // 初期サーバーはLeft側
        assert_eq!(match_score.server, CourtSide::Left);

        // ゲーム獲得後はRight側がサーバー
        match_score.win_game(CourtSide::Left);
        assert_eq!(match_score.server, CourtSide::Right);

        // 次のゲーム後はLeft側がサーバー
        match_score.win_game(CourtSide::Right);
        assert_eq!(match_score.server, CourtSide::Left);
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
        let mut rally_state = RallyState::new(CourtSide::Left);

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
