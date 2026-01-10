//! 発射角度計算
//! @spec 30605_trajectory_calculation_spec.md

use bevy::prelude::*;

use crate::resource::config::TrajectoryConfig;

use super::landing_position::shorten_target_position;
use super::physics_utils::{calculate_max_reachable_distance, calculate_speed_for_target};

/// 発射角度を逆算（着地点も調整）
/// @spec 30605_trajectory_calculation_spec.md#req-30605-021
/// @spec 30605_trajectory_calculation_spec.md#req-30605-022
/// @spec 30605_trajectory_calculation_spec.md#req-30605-024
/// 戻り値: (角度, 速度, 調整後の着地点)
pub fn calculate_launch_angle(
    start_pos: Vec3,
    target_pos: Vec3,
    base_speed: f32,
    effective_gravity: f32,
    trajectory_config: &TrajectoryConfig,
    net_x: f32,
    net_height: f32,
) -> (f32, f32, Vec3) {
    let dx = target_pos.x - start_pos.x;
    let dz = target_pos.z - start_pos.z;
    let horizontal_distance = (dx * dx + dz * dz).sqrt();

    // 高さの差（着地高さ - 打点高さ）※発射点基準
    let h = target_pos.y - start_pos.y;

    let v = base_speed;
    let g = effective_gravity;
    let d = horizontal_distance;
    let v2 = v * v;
    let v4 = v2 * v2;

    // 判別式: v^4 - g(g*d^2 + 2*h*v^2)
    let discriminant = v4 - g * (g * d * d + 2.0 * h * v2);

    // === 解がある場合: 目標地点に到達可能 ===
    if discriminant >= 0.0 {
        return calculate_angle_when_reachable(
            start_pos, target_pos, v, g, d, h, discriminant,
            trajectory_config, net_x, net_height,
        );
    }

    // === 解がない場合: パワー不足で目標地点に届かない ===
    let max_distance = calculate_max_reachable_distance(base_speed, effective_gravity, h);

    if max_distance < 0.1 || horizontal_distance < 0.001 {
        return (trajectory_config.max_launch_angle, v, target_pos);
    }

    // 着地点を短縮
    let new_target = shorten_target_position(
        start_pos, target_pos, dx, dz, horizontal_distance, max_distance, trajectory_config,
    );

    // 短縮した着地点で角度を再計算
    let new_dx = new_target.x - start_pos.x;
    let new_dz = new_target.z - start_pos.z;
    let new_d = (new_dx * new_dx + new_dz * new_dz).sqrt();
    let new_discriminant = v4 - g * (g * new_d * new_d + 2.0 * h * v2);

    if new_discriminant >= 0.0 {
        return calculate_angle_when_reachable(
            start_pos, new_target, v, g, new_d, h, new_discriminant,
            trajectory_config, net_x, net_height,
        );
    }

    // === フォールバック: 最大角度で打つ ===
    calculate_max_angle_fallback(
        start_pos, target_pos, dx, dz, horizontal_distance,
        v, g, h, trajectory_config.max_launch_angle,
    )
}

/// 解がある場合（目標地点に到達可能）の角度計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-021
#[allow(clippy::too_many_arguments)]
fn calculate_angle_when_reachable(
    start_pos: Vec3,
    target_pos: Vec3,
    v: f32,
    g: f32,
    d: f32,
    h: f32,
    discriminant: f32,
    trajectory_config: &TrajectoryConfig,
    net_x: f32,
    net_height: f32,
) -> (f32, f32, Vec3) {
    let v2 = v * v;
    let sqrt_disc = discriminant.sqrt();
    let tan_theta_1 = (v2 - sqrt_disc) / (g * d);
    let tan_theta_2 = (v2 + sqrt_disc) / (g * d);

    // 角度が低い方を採用（テニス的な軌道）
    let angle_1 = tan_theta_1.atan().to_degrees();
    let angle_2 = tan_theta_2.atan().to_degrees();

    let angle = if angle_1.abs() < angle_2.abs() {
        angle_1
    } else {
        angle_2
    };

    // ネット通過に必要な最小角度を計算
    let min_net_angle = calculate_min_angle_for_net_clearance(
        start_pos, target_pos, v, g, net_x, net_height,
    );

    // ネット通過角度と計算角度の大きい方を採用
    let final_angle = angle.max(min_net_angle);

    // 上限のみ制限
    let clamped_angle = final_angle.min(trajectory_config.max_launch_angle);

    // 角度が変更された場合、目標着地点に到達するように速度を調整
    if (clamped_angle - angle).abs() > 0.1 {
        let adjusted_speed = calculate_speed_for_target(clamped_angle, d, g, h);
        if adjusted_speed > 0.0 {
            return (clamped_angle, adjusted_speed, target_pos);
        }
    }

    (clamped_angle, v, target_pos)
}

/// ネット通過に必要な最小角度を計算
/// 打点位置、速度、重力からネットを越えるために必要な最小発射角度を計算
fn calculate_min_angle_for_net_clearance(
    start_pos: Vec3,
    target_pos: Vec3,
    speed: f32,
    gravity: f32,
    net_x: f32,
    net_height: f32,
) -> f32 {
    let dx = target_pos.x - start_pos.x;

    // ネットを越えない方向の場合は制限不要
    let crosses_net = (dx > 0.0 && start_pos.x < net_x && target_pos.x > net_x)
        || (dx < 0.0 && start_pos.x > net_x && target_pos.x < net_x);

    if !crosses_net {
        return 0.0; // ネットを越えない場合は制限なし
    }

    // ネットまでの水平距離
    let dist_to_net = (net_x - start_pos.x).abs();

    // 打点とネット上端の高さの差（マージン込み）
    let net_clearance_margin = 0.3; // ネット上端からのマージン
    let required_height = net_height + net_clearance_margin - start_pos.y;

    // 打点がネットより十分高い場合は制限緩和
    if required_height < 0.0 {
        // 打点がネット上端より高い場合、低い角度でも通過可能
        // ただし、落下を考慮して最低限の角度は必要
        return -5.0; // 少し下向きでもOK
    }

    // 二分探索でネットを越える最小角度を求める
    let mut low = 0.0_f32;
    let mut high = 60.0_f32;

    for _ in 0..20 {
        let mid = (low + high) / 2.0;
        let mid_rad = mid.to_radians();
        let cos_a = mid_rad.cos();
        let sin_a = mid_rad.sin();

        if cos_a.abs() < 0.001 {
            low = mid;
            continue;
        }

        // ネット到達時刻
        let t_net = dist_to_net / (speed * cos_a);

        // ネット到達時の高さ
        let height_at_net = start_pos.y + speed * sin_a * t_net - 0.5 * gravity * t_net * t_net;

        if height_at_net >= net_height + net_clearance_margin {
            high = mid; // この角度で通過できる、もっと低い角度を試す
        } else {
            low = mid; // 通過できない、もっと高い角度が必要
        }
    }

    high // 安全側（高い方）を返す
}

/// 最大角度でのフォールバック軌道を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-024
#[allow(clippy::too_many_arguments)]
fn calculate_max_angle_fallback(
    start_pos: Vec3,
    target_pos: Vec3,
    dx: f32,
    dz: f32,
    horizontal_distance: f32,
    v: f32,
    g: f32,
    h: f32,
    max_launch_angle: f32,
) -> (f32, f32, Vec3) {
    let max_angle_rad = max_launch_angle.to_radians();
    let cos_angle = max_angle_rad.cos();
    let sin_angle = max_angle_rad.sin();

    let v_horizontal = v * cos_angle;
    let v_vertical = v * sin_angle;

    // 飛行時間を計算
    let flight_time = (v_vertical + (v_vertical * v_vertical + 2.0 * g * h.abs()).sqrt()) / g;

    // 実際の水平到達距離
    let actual_distance = v_horizontal * flight_time;

    // 着地点を実際の到達距離に基づいて更新
    let actual_scale = if horizontal_distance > 0.001 {
        (actual_distance / horizontal_distance).min(1.0)
    } else {
        1.0
    };

    let actual_target = Vec3::new(
        start_pos.x + dx * actual_scale,
        target_pos.y,
        start_pos.z + dz * actual_scale,
    );

    (max_launch_angle, v, actual_target)
}
