//! 着地位置計算
//! @spec 30605_trajectory_calculation_spec.md

use bevy::prelude::*;

use crate::core::CourtSide;
use crate::resource::config::{CourtConfig, TrajectoryConfig};

use super::physics_utils::{lerp, CourtSideExt};
use super::types::TrajectoryContext;

/// 着地地点を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-010
/// @spec 30605_trajectory_calculation_spec.md#req-30605-011
/// @spec 30605_trajectory_calculation_spec.md#req-30605-012
/// @spec 30605_trajectory_calculation_spec.md#req-30605-013
pub fn calculate_landing_position(
    ctx: &TrajectoryContext,
    court_config: &CourtConfig,
    trajectory_config: &TrajectoryConfig,
) -> Vec3 {
    let margin = trajectory_config.landing_margin;
    let half_width = court_config.width / 2.0;
    let half_depth = court_config.depth / 2.0;

    // ネット位置
    let net_x = court_config.net_x;

    // コートサイドに応じた座標変換
    let (baseline_x, _net_side_x) = match ctx.court_side {
        CourtSide::Left => {
            // Left側は+X方向に打つ → 相手側ベースライン = +half_depth
            (half_depth, net_x)
        }
        CourtSide::Right => {
            // Right側は-X方向に打つ → 相手側ベースライン = -half_depth
            (-half_depth, net_x)
        }
    };

    // REQ-30605-010, REQ-30605-011: 前後入力による深さ調整
    // input.y: -1.0=ネット際, 0.0=サービスライン付近, +1.0=ベースライン際
    let target_x = if ctx.input.y.abs() < 0.01 {
        // ニュートラル: デフォルト着地深さを使用
        let depth = trajectory_config.default_landing_depth;
        match ctx.court_side {
            CourtSide::Left => net_x + depth,
            CourtSide::Right => net_x - depth,
        }
    } else {
        // 入力あり: 線形補間
        // Left側の場合: input.y=-1 → ネット際, input.y=+1 → ベースライン際
        let near = net_x + margin * ctx.court_side.sign();
        let far = baseline_x - margin * ctx.court_side.sign();
        let t = (ctx.input.y + 1.0) / 2.0; // -1..1 → 0..1
        lerp(near, far, t)
    };

    // REQ-30605-012: 左右入力によるコース調整
    // input.x: -1.0=左サイド, 0.0=中央, +1.0=右サイド
    let target_z = ctx.input.x * (half_width - margin);

    Vec3::new(target_x, 0.0, target_z)
}

/// 精度による着地位置の収束を適用（決定的）
/// @spec 30605_trajectory_calculation_spec.md#req-30605-040
/// @spec 30604_shot_attributes_spec.md#req-30604-070
/// ランダム性なし: 精度が低いほどコート中央（X軸方向のネット側）に収束
pub fn apply_landing_deviation(
    target: Vec3,
    accuracy: f32,
    _trajectory_config: &TrajectoryConfig,
) -> Vec3 {
    // 精度が低いほど狙った位置に打てない = 中央寄りに収束
    let convergence = (1.0 - accuracy.clamp(0.0, 1.0)) * 0.3;

    if convergence < 0.001 {
        return target;
    }

    // X軸: ネット方向（X=0）に向かって収束
    // Z軸: コート中央（Z=0）に向かって収束
    let converged_x = target.x * (1.0 - convergence);
    let converged_z = target.z * (1.0 - convergence);

    Vec3::new(converged_x, target.y, converged_z)
}

/// 着地点を短縮してネットを越える位置に調整
/// @spec 30605_trajectory_calculation_spec.md#req-30605-022
pub fn shorten_target_position(
    start_pos: Vec3,
    target_pos: Vec3,
    dx: f32,
    dz: f32,
    horizontal_distance: f32,
    max_distance: f32,
    trajectory_config: &TrajectoryConfig,
) -> Vec3 {
    // 着地点を短縮
    let scale = (max_distance / horizontal_distance).min(0.95); // 95%を上限として安全マージン
    let mut new_x = start_pos.x + dx * scale;
    let new_z = start_pos.z + dz * scale;

    // ネットを越える最低位置を保証
    let net_margin = trajectory_config.landing_margin;
    let target_crosses_net = if dx > 0.0 {
        target_pos.x > net_margin
    } else {
        target_pos.x < -net_margin
    };

    if target_crosses_net {
        if dx > 0.0 && new_x < net_margin {
            new_x = net_margin;
        } else if dx < 0.0 && new_x > -net_margin {
            new_x = -net_margin;
        }
    }

    Vec3::new(new_x, target_pos.y, new_z)
}
