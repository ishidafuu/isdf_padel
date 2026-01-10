//! 物理演算パラメータ
//! @data 80101_game_constants.md#physics-config

use serde::Deserialize;

/// 物理演算パラメータ
/// @data 80101_game_constants.md#physics-config
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct PhysicsConfig {
    pub gravity: f32,
    pub max_fall_speed: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: -9.8,
            max_fall_speed: -20.0,
        }
    }
}

/// スピン物理パラメータ
/// @spec 30401_trajectory_spec.md#req-30401-100
/// @spec 30401_trajectory_spec.md#req-30401-101
/// @spec 30401_trajectory_spec.md#req-30401-102
/// @spec 30402_reflection_spec.md#req-30402-100
/// @data 80101_game_constants.md#spin-physics-config
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct SpinPhysicsConfig {
    /// 重力に対するスピンの影響度（±30%時 = 0.3）
    pub gravity_spin_factor: f32,
    /// バウンド時の水平方向へのスピンの影響度
    pub bounce_spin_horizontal_factor: f32,
    /// バウンド時の垂直方向へのスピンの影響度
    pub bounce_spin_vertical_factor: f32,
    /// ベース空気抵抗（スピンなしでも適用）
    pub base_air_drag: f32,
    /// スピンによる追加空気抵抗係数
    pub spin_drag_factor: f32,
    /// スピン時間減衰率（1秒あたり）
    pub spin_decay_rate: f32,
}

impl Default for SpinPhysicsConfig {
    fn default() -> Self {
        Self {
            gravity_spin_factor: 0.3,
            bounce_spin_horizontal_factor: 0.3,
            bounce_spin_vertical_factor: 0.2,
            base_air_drag: 0.0,
            spin_drag_factor: 0.3,
            spin_decay_rate: 0.5,
        }
    }
}
