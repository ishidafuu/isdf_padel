//! ボール軌道システム
//! @spec 30401_trajectory_spec.md
//! @spec 30402_reflection_spec.md

mod bounce;
mod net_collision;
mod physics;

#[cfg(test)]
mod tests;

use bevy::prelude::*;

pub use bounce::{ball_ground_bounce_system, ball_out_of_bounds_system, ball_wall_reflection_system};
pub use net_collision::ball_net_collision_system;
pub use physics::{ball_air_drag_system, ball_gravity_system, ball_position_update_system, ball_spin_decay_system};

/// ボール軌道プラグイン
/// @spec 30401_trajectory_spec.md
pub struct BallTrajectoryPlugin;

impl Plugin for BallTrajectoryPlugin {
    fn build(&self, app: &mut App) {
        use crate::core::events::{BallOutOfBoundsEvent, GroundBounceEvent, NetHitEvent, WallReflectionEvent};

        app.add_message::<BallOutOfBoundsEvent>()
            .add_message::<GroundBounceEvent>()
            .add_message::<NetHitEvent>()
            .add_message::<WallReflectionEvent>()
            .add_systems(
                Update,
                (
                    // スピン減衰を最初に適用（重力計算前にスピン値を更新）
                    ball_spin_decay_system,
                    // 重力適用（スピンによる変動含む）
                    ball_gravity_system,
                    // 空気抵抗適用（スピンによる追加抵抗含む）
                    ball_air_drag_system,
                    // 位置更新
                    ball_position_update_system,
                    // ネット衝突判定（位置更新後、バウンド判定前）
                    ball_net_collision_system,
                    // バウンド・反射（スピンによるバウンド変動含む）
                    ball_ground_bounce_system,
                    ball_wall_reflection_system,
                    ball_out_of_bounds_system,
                )
                    .chain(),
            );
    }
}
