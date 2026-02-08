//! スコア表示システム
//! @spec 30701_point_spec.md#req-30701-004

use bevy::prelude::*;

use crate::core::CourtSide;
use crate::resource::{GameConfig, MatchScore};

/// スコア表示システム（デバッグ用）
/// @spec 30701_point_spec.md#req-30701-004
pub fn score_display_system(
    match_score: Res<MatchScore>,
    config: Res<GameConfig>,
    mut last_display: Local<Option<String>>,
) {
    let point_values = &config.scoring.point_values;
    let p1_point = match_score.get_point_display(CourtSide::Left, point_values);
    let p2_point = match_score.get_point_display(CourtSide::Right, point_values);

    let p1_score = match_score.get_score(CourtSide::Left);
    let p2_score = match_score.get_score(CourtSide::Right);

    let score_text = format!(
        "P1: {} (G:{} S:{}) - P2: {} (G:{} S:{})",
        p1_point, p1_score.games, p1_score.sets, p2_point, p2_score.games, p2_score.sets,
    );

    // 変更があった場合のみ表示
    if last_display.as_ref() != Some(&score_text) {
        info!("Score: {}", score_text);
        *last_display = Some(score_text);
    }
}
