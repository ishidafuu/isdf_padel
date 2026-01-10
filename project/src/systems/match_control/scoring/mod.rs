//! スコアリングシステム
//! @spec 30701_point_spec.md
//! @spec 30702_game_spec.md
//! @spec 30703_set_spec.md
//! @spec 30903_serve_authority_spec.md

mod display;
mod game_set;
mod rally;

#[cfg(test)]
mod tests;

use bevy::prelude::*;

use crate::core::{
    GameWonEvent, MatchWonEvent, PointScoredEvent, RallyEndEvent, SetWonEvent,
};
use crate::resource::MatchScore;

pub use display::score_display_system;
pub use rally::{point_scored_system, rally_end_system};

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
