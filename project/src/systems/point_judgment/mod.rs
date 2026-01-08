//! ポイント判定システム
//! @spec 30901_point_judgment_spec.md
//!
//! 得点条件（ツーバウンド、アウト、レット）を判定する。

mod bounce_judgment;
mod net_judgment;
mod out_judgment;

use bevy::prelude::*;

use crate::components::{Ball, LastShooter};
use crate::core::events::ShotExecutedEvent;
use crate::core::CourtSide;

pub use bounce_judgment::{
    bounce_count_update_system, double_bounce_judgment_system, own_court_hit_judgment_system,
};
pub use net_judgment::{let_judgment_system, net_fault_judgment_system};
pub use out_judgment::{out_of_bounds_judgment_system, wall_hit_judgment_system};

use crate::resource::RallyState;

/// ポイント判定プラグイン
/// @spec 30901_point_judgment_spec.md
pub struct PointJudgmentPlugin;

impl Plugin for PointJudgmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RallyState>().add_systems(
            Update,
            (
                update_last_shooter_system,
                bounce_count_update_system,
                double_bounce_judgment_system,
                out_of_bounds_judgment_system,
                wall_hit_judgment_system,
                let_judgment_system,
                net_fault_judgment_system,
                own_court_hit_judgment_system,
            )
                .chain(),
        );
    }
}

/// 最後のショット元更新システム
/// @spec 30103_point_end_spec.md#req-30103-003
/// ShotExecutedEvent を受信して LastShooter を更新
pub fn update_last_shooter_system(
    mut shot_events: MessageReader<ShotExecutedEvent>,
    mut query: Query<&mut LastShooter, With<Ball>>,
) {
    for event in shot_events.read() {
        // プレイヤーIDからCourtSideを決定
        let shooter = match event.player_id {
            1 => CourtSide::Player1,
            _ => CourtSide::Player2,
        };

        for mut last_shooter in query.iter_mut() {
            last_shooter.record(shooter);
            info!("Ball shot by {:?}", shooter);
        }
    }
}
