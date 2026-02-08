//! 弾道計算モジュール
//! @spec 30605_trajectory_calculation_spec.md
//!
//! 入力から着地地点を決定し、放物線公式で発射角度を逆算するシステム

mod landing_position;
mod launch_angle;
mod main_trajectory;
mod physics_utils;
mod serve_trajectory;
#[cfg(test)]
mod tests;
mod types;

// 型の再エクスポート
pub use types::{ServeTrajectoryContext, TrajectoryContext, TrajectoryResult};

// 物理ユーティリティの再エクスポート（将来の拡張用に保持）
#[allow(unused_imports)]
pub use physics_utils::{
    calculate_direction_vector, calculate_effective_gravity, calculate_landing_distance_for_angle,
    calculate_max_reachable_distance, calculate_speed_factors, calculate_speed_for_target, lerp,
    CourtSideExt,
};

// 着地位置計算の再エクスポート（将来の拡張用に保持）
#[allow(unused_imports)]
pub use landing_position::{
    apply_landing_deviation, calculate_landing_position, shorten_target_position,
};

// 発射角度計算の再エクスポート（将来の拡張用に保持）
#[allow(unused_imports)]
pub use launch_angle::calculate_launch_angle;

// サーブ軌道計算の再エクスポート
#[allow(unused_imports)]
pub use serve_trajectory::{calculate_serve_landing_position, calculate_serve_trajectory};

// メイン軌道計算の再エクスポート
pub use main_trajectory::calculate_trajectory;
