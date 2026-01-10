//! スコアリングパラメータ
//! @data 80101_game_constants.md#scoring-config
//! @data 80701_point_config.md
//! @data 80703_set_config.md

use serde::Deserialize;

/// スコアリングパラメータ
/// @data 80101_game_constants.md#scoring-config
/// @data 80701_point_config.md
/// @data 80703_set_config.md
#[derive(Deserialize, Clone, Debug)]
pub struct ScoringConfig {
    /// ポイント進行値 [0, 15, 30, 40]
    /// @spec 30701_point_spec.md#req-30701-001
    #[serde(default = "default_point_values")]
    pub point_values: Vec<u32>,
    /// 勝利に必要なゲーム数（6ゲーム先取でセット獲得）
    /// @data 80703_set_config.md#games_to_win_set
    #[serde(default = "default_games_to_win_set")]
    pub games_to_win_set: u32,
    /// 勝利に必要なセット数（1セット先取でマッチ勝利）
    /// @data 80703_set_config.md#sets_to_win_match
    #[serde(default = "default_sets_to_win_match")]
    pub sets_to_win_match: u32,
}

fn default_point_values() -> Vec<u32> {
    vec![0, 15, 30, 40]
}
fn default_games_to_win_set() -> u32 {
    6
}
fn default_sets_to_win_match() -> u32 {
    1
}
