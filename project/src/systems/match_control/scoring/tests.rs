//! スコアリングシステムテスト
//! @spec 30701_point_spec.md
//! @spec 30702_game_spec.md
//! @spec 30703_set_spec.md

use crate::core::CourtSide;
use crate::resource::MatchScore;

/// TST-30705-001: ポイント初期化テスト
/// @spec 30701_point_spec.md#req-30701-001
#[test]
fn test_point_initialization() {
    let match_score = MatchScore::new();
    assert_eq!(match_score.get_point(CourtSide::Left).index, 0);
    assert_eq!(match_score.get_point(CourtSide::Right).index, 0);
}

/// TST-30705-002: ポイント加算テスト
/// @spec 30701_point_spec.md#req-30701-002
#[test]
fn test_point_progression() {
    let mut match_score = MatchScore::new();
    let point_values = vec![0, 15, 30, 40];

    // 0 -> 15
    match_score.add_point(CourtSide::Left);
    assert_eq!(match_score.get_point_index(CourtSide::Left), 1);
    assert_eq!(
        match_score.get_point_display(CourtSide::Left, &point_values),
        "15"
    );

    // 15 -> 30
    match_score.add_point(CourtSide::Left);
    assert_eq!(match_score.get_point_index(CourtSide::Left), 2);
    assert_eq!(
        match_score.get_point_display(CourtSide::Left, &point_values),
        "30"
    );

    // 30 -> 40
    match_score.add_point(CourtSide::Left);
    assert_eq!(match_score.get_point_index(CourtSide::Left), 3);
    assert_eq!(
        match_score.get_point_display(CourtSide::Left, &point_values),
        "40"
    );
}

/// TST-30705-003: ゲーム勝利判定テスト
/// @spec 30701_point_spec.md#req-30701-003
#[test]
fn test_game_win_detection() {
    let mut match_score = MatchScore::new();
    let win_index = 4; // point_values.len() = 4

    // Left側 が 40 に到達
    match_score.add_point(CourtSide::Left); // 15
    match_score.add_point(CourtSide::Left); // 30
    match_score.add_point(CourtSide::Left); // 40

    // まだ勝利ではない
    assert!(!match_score.check_game_win(CourtSide::Left, win_index));

    // 40 -> Game
    match_score.add_point(CourtSide::Left);
    assert!(match_score.check_game_win(CourtSide::Left, win_index));
}

/// TST-30705-004: ポイント表示テスト
/// @spec 30701_point_spec.md#req-30701-004
#[test]
fn test_point_display() {
    let mut match_score = MatchScore::new();
    let point_values = vec![0, 15, 30, 40];

    assert_eq!(
        match_score.get_point_display(CourtSide::Left, &point_values),
        "0"
    );

    match_score.add_point(CourtSide::Left);
    assert_eq!(
        match_score.get_point_display(CourtSide::Left, &point_values),
        "15"
    );

    // Game表示（インデックス超過時）
    match_score.add_point(CourtSide::Left); // 30
    match_score.add_point(CourtSide::Left); // 40
    match_score.add_point(CourtSide::Left); // Game
    assert_eq!(
        match_score.get_point_display(CourtSide::Left, &point_values),
        "Game"
    );
}

/// TST-30705-005: ポイントリセットテスト
/// @spec 30701_point_spec.md#req-30701-005
#[test]
fn test_point_reset() {
    let mut match_score = MatchScore::new();

    // ポイントを獲得
    match_score.add_point(CourtSide::Left);
    match_score.add_point(CourtSide::Right);

    // リセット
    match_score.reset_points();

    assert_eq!(match_score.get_point(CourtSide::Left).index, 0);
    assert_eq!(match_score.get_point(CourtSide::Right).index, 0);
}

// ========================================
// 30702: ゲームカウント管理テスト
// ========================================

/// TST-30706-001: ゲームカウント初期化テスト
/// @spec 30702_game_spec.md#req-30702-001
#[test]
fn test_game_count_initialization() {
    let match_score = MatchScore::new();
    assert_eq!(match_score.get_score(CourtSide::Left).games, 0);
    assert_eq!(match_score.get_score(CourtSide::Right).games, 0);
    assert_eq!(match_score.get_score(CourtSide::Left).sets, 0);
    assert_eq!(match_score.get_score(CourtSide::Right).sets, 0);
}

/// TST-30706-002: ゲームカウント加算テスト
/// @spec 30702_game_spec.md#req-30702-002
#[test]
fn test_game_count_increment() {
    let mut match_score = MatchScore::new();

    // Left側がゲームを獲得
    match_score.win_game(CourtSide::Left);
    assert_eq!(match_score.get_score(CourtSide::Left).games, 1);
    assert_eq!(match_score.get_score(CourtSide::Right).games, 0);

    // Right側がゲームを獲得
    match_score.win_game(CourtSide::Right);
    assert_eq!(match_score.get_score(CourtSide::Left).games, 1);
    assert_eq!(match_score.get_score(CourtSide::Right).games, 1);

    // Left側が再度ゲームを獲得
    match_score.win_game(CourtSide::Left);
    assert_eq!(match_score.get_score(CourtSide::Left).games, 2);
}

/// TST-30706-003: セット勝利判定テスト（6ゲーム先取）
/// @spec 30702_game_spec.md#req-30702-003
#[test]
fn test_set_win_at_six_games() {
    let mut match_score = MatchScore::new();
    let games_to_win = 6;

    // Left側が5ゲーム獲得（まだセット勝利ではない）
    for _ in 0..5 {
        match_score.win_game(CourtSide::Left);
    }
    assert!(!match_score.check_set_win(CourtSide::Left, games_to_win));

    // Left側が6ゲーム目を獲得（セット勝利）
    match_score.win_game(CourtSide::Left);
    assert!(match_score.check_set_win(CourtSide::Left, games_to_win));
}

/// TST-30706-004: ポイントリセット確認（ゲーム獲得後）
/// @spec 30702_game_spec.md#req-30702-002
#[test]
fn test_points_reset_after_game_win() {
    let mut match_score = MatchScore::new();

    // ポイントを獲得
    match_score.add_point(CourtSide::Left);
    match_score.add_point(CourtSide::Left);
    assert_eq!(match_score.get_point_index(CourtSide::Left), 2);

    // ゲーム獲得（ポイントもリセットされる）
    match_score.win_game(CourtSide::Left);

    // ポイントがリセットされていることを確認
    assert_eq!(match_score.get_point(CourtSide::Left).index, 0);
    assert_eq!(match_score.get_point(CourtSide::Right).index, 0);
    // ゲームカウントが増えていることを確認
    assert_eq!(match_score.get_score(CourtSide::Left).games, 1);
}

/// TST-30706-005: ゲームカウント表示テスト
/// @spec 30702_game_spec.md#req-30702-005
#[test]
fn test_game_count_display() {
    let mut match_score = MatchScore::new();

    // 初期状態
    assert_eq!(match_score.get_score(CourtSide::Left).games, 0);
    assert_eq!(match_score.get_score(CourtSide::Right).games, 0);

    // ゲーム獲得後の表示
    match_score.win_game(CourtSide::Left);
    match_score.win_game(CourtSide::Left);
    match_score.win_game(CourtSide::Right);

    assert_eq!(match_score.get_score(CourtSide::Left).games, 2);
    assert_eq!(match_score.get_score(CourtSide::Right).games, 1);
}

/// TST-30706-006: サーバー交代テスト（ゲーム獲得後）
/// @spec 30702_game_spec.md
#[test]
fn test_server_switch_after_game() {
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

/// TST-30706-007: セット獲得後のゲームカウントリセットテスト
/// @spec 30702_game_spec.md
#[test]
fn test_game_count_reset_after_set_win() {
    let mut match_score = MatchScore::new();

    // Left側が6ゲーム獲得
    for _ in 0..6 {
        match_score.win_game(CourtSide::Left);
    }
    assert_eq!(match_score.get_score(CourtSide::Left).games, 6);

    // セット獲得
    match_score.win_set(CourtSide::Left);

    // ゲームカウントがリセットされ、セット数が増加
    assert_eq!(match_score.get_score(CourtSide::Left).games, 0);
    assert_eq!(match_score.get_score(CourtSide::Left).sets, 1);
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
    assert_eq!(match_score.get_score(CourtSide::Left).sets, 0);
    assert_eq!(match_score.get_score(CourtSide::Right).sets, 0);
}

/// TST-30707-002: セットカウント加算テスト
/// @spec 30703_set_spec.md#req-30703-002
#[test]
fn test_set_count_increment() {
    let mut match_score = MatchScore::new();

    // Left側がセットを獲得
    match_score.win_set(CourtSide::Left);
    assert_eq!(match_score.get_score(CourtSide::Left).sets, 1);
    assert_eq!(match_score.get_score(CourtSide::Right).sets, 0);

    // Right側がセットを獲得
    match_score.win_set(CourtSide::Right);
    assert_eq!(match_score.get_score(CourtSide::Left).sets, 1);
    assert_eq!(match_score.get_score(CourtSide::Right).sets, 1);
}

/// TST-30707-003: マッチ勝利判定テスト（1セット制）
/// @spec 30703_set_spec.md#req-30703-003
#[test]
fn test_match_win_one_set() {
    let mut match_score = MatchScore::new();
    let sets_to_win = 1; // 1セット制

    // セット獲得前は勝利ではない
    assert!(!match_score.check_match_win(CourtSide::Left, sets_to_win));

    // Left側がセットを獲得
    match_score.win_set(CourtSide::Left);

    // 1セット獲得でマッチ勝利
    assert!(match_score.check_match_win(CourtSide::Left, sets_to_win));
    // 相手はまだ勝利ではない
    assert!(!match_score.check_match_win(CourtSide::Right, sets_to_win));
}

/// TST-30707-004: セットカウント表示テスト
/// @spec 30703_set_spec.md#req-30703-004
#[test]
fn test_set_count_display() {
    let mut match_score = MatchScore::new();

    // 初期状態: 0-0
    assert_eq!(match_score.get_score(CourtSide::Left).sets, 0);
    assert_eq!(match_score.get_score(CourtSide::Right).sets, 0);

    // セット獲得後
    match_score.win_set(CourtSide::Left);
    // 表示: "1" - "0"
    assert_eq!(match_score.get_score(CourtSide::Left).sets, 1);
    assert_eq!(match_score.get_score(CourtSide::Right).sets, 0);
}
