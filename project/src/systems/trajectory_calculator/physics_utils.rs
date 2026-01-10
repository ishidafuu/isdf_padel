//! 物理計算ユーティリティ
//! @spec 30605_trajectory_calculation_spec.md

use bevy::prelude::*;

use crate::core::CourtSide;
use crate::resource::config::{GameConfig, TrajectoryConfig};

/// CourtSide の符号を取得（計算用ヘルパー）
pub trait CourtSideExt {
    fn sign(&self) -> f32;
}

impl CourtSideExt for CourtSide {
    fn sign(&self) -> f32 {
        match self {
            CourtSide::Left => 1.0,
            CourtSide::Right => -1.0,
        }
    }
}

/// 有効重力を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-020
pub fn calculate_effective_gravity(
    spin: f32,
    initial_height: f32,
    config: &GameConfig,
) -> f32 {
    let gravity = config.physics.gravity.abs();
    let spin_config = &config.spin_physics;

    // 飛行時間推定（簡易計算）
    let estimated_flight_time = if initial_height > 0.1 {
        2.0 * (initial_height / gravity).sqrt()
    } else {
        1.0 // デフォルト
    };

    // 平均スピン効果
    let avg_spin = spin * (1.0 - spin_config.spin_decay_rate * estimated_flight_time / 2.0);

    // 有効重力 = 基本重力 × (1 + スピン効果)
    // トップスピン(+) → 重力増加（落ちやすい）
    // スライス(-) → 重力減少（浮きやすい）
    gravity * (1.0 + avg_spin * spin_config.gravity_spin_factor)
}

/// 方向ベクトルを計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-023
pub fn calculate_direction_vector(
    start_pos: Vec3,
    target_pos: Vec3,
    launch_angle: f32,
) -> Vec3 {
    let dx = target_pos.x - start_pos.x;
    let dz = target_pos.z - start_pos.z;
    let horizontal_distance = (dx * dx + dz * dz).sqrt();

    if horizontal_distance < 0.001 {
        // ほぼ同じ位置の場合はデフォルト方向
        return Vec3::new(1.0, launch_angle.to_radians().sin(), 0.0).normalize();
    }

    // 水平方向の単位ベクトル
    let horizontal_dir_x = dx / horizontal_distance;
    let horizontal_dir_z = dz / horizontal_distance;

    let angle_rad = launch_angle.to_radians();
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();

    Vec3::new(
        horizontal_dir_x * cos_angle,
        sin_angle,
        horizontal_dir_z * cos_angle,
    )
}

/// 指定した角度で目標地点に到達するために必要な初速を計算
/// 公式: v = √[ g·d² / (2·cos²(θ)·(d·tan(θ) - h)) ]
pub fn calculate_speed_for_target(
    angle_deg: f32,
    horizontal_distance: f32,
    gravity: f32,
    height_diff: f32, // 着地高さ - 発射高さ（負の値 = 着地点が低い）
) -> f32 {
    let angle_rad = angle_deg.to_radians();
    let cos_a = angle_rad.cos();
    let tan_a = angle_rad.tan();

    if cos_a.abs() < 0.001 {
        return 0.0; // ほぼ真上に打つ場合
    }

    // 解の存在条件: d·tan(θ) - h > 0
    let denominator = horizontal_distance * tan_a - height_diff;

    if denominator <= 0.0 {
        return 0.0; // その角度では到達不可能
    }

    let v_squared = gravity * horizontal_distance * horizontal_distance
        / (2.0 * cos_a * cos_a * denominator);

    if v_squared <= 0.0 {
        return 0.0;
    }

    v_squared.sqrt()
}

/// 与えられた初速と重力で到達可能な最大水平距離を計算
/// 判別式 v⁴ - g(g*d² + 2h*v²) >= 0 を満たす最大の d を求める
/// d_max = sqrt((v⁴ - 2gh*v²) / g²) = (v²/g) * sqrt(1 - 2gh/v²)
pub fn calculate_max_reachable_distance(base_speed: f32, effective_gravity: f32, height_diff: f32) -> f32 {
    let v2 = base_speed * base_speed;
    let g = effective_gravity;
    let h = height_diff;

    // v² > 2gh が必要（そうでないと到達不可能）
    let discriminant_factor = 1.0 - 2.0 * g * h / v2;

    if discriminant_factor <= 0.0 {
        // 初速が低すぎて上に打っても落ちてくる（到達距離ほぼ0）
        return 0.1; // 最小値
    }

    (v2 / g) * discriminant_factor.sqrt()
}

/// 指定した角度・初速・重力での水平飛距離を計算
/// h: 着地高さ - 発射高さ（負の値 = 着地点が低い）
#[allow(dead_code)]
pub fn calculate_landing_distance_for_angle(angle_deg: f32, speed: f32, gravity: f32, h: f32) -> f32 {
    let angle_rad = angle_deg.to_radians();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    if cos_a.abs() < 0.001 {
        return 0.0; // ほぼ真上に打つ場合
    }

    let v_horizontal = speed * cos_a;
    let v_vertical = speed * sin_a;

    // 飛行時間を計算: y(t) = v_y*t - 0.5*g*t² + h = 0
    // 0.5*g*t² - v_y*t - h = 0
    // t = (v_y + sqrt(v_y² + 2*g*h)) / g (hが負なら +2gh は正)
    let discriminant = v_vertical * v_vertical + 2.0 * gravity * (-h);

    if discriminant < 0.0 {
        return 0.0; // 到達不可能
    }

    let flight_time = (v_vertical + discriminant.sqrt()) / gravity;

    if flight_time < 0.0 {
        return 0.0;
    }

    v_horizontal * flight_time
}

/// 球種・距離による初速係数を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-031
/// @spec 30605_trajectory_calculation_spec.md#req-30605-032
/// @spec 30605_trajectory_calculation_spec.md#req-30605-033
#[allow(dead_code)]
pub fn calculate_speed_factors(
    spin: f32,
    horizontal_distance: f32,
    max_court_distance: f32,
    trajectory_config: &TrajectoryConfig,
) -> f32 {
    // 球種係数
    let spin_factor = if spin > 0.1 {
        trajectory_config.spin_speed_topspin
    } else if spin < -0.1 {
        trajectory_config.spin_speed_slice
    } else {
        trajectory_config.spin_speed_flat
    };

    // 距離係数
    let distance_ratio = (horizontal_distance / max_court_distance).clamp(0.0, 1.0);
    let distance_factor = lerp(
        trajectory_config.distance_speed_min,
        trajectory_config.distance_speed_max,
        distance_ratio,
    );

    spin_factor * distance_factor
}

/// 線形補間
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
