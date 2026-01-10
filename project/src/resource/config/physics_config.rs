//! 物理演算パラメータ
//! @data 80101_game_constants.md#physics-config

use serde::Deserialize;

/// 物理演算パラメータ
/// @data 80101_game_constants.md#physics-config
#[derive(Deserialize, Clone, Debug)]
pub struct PhysicsConfig {
    #[serde(default = "default_gravity")]
    pub gravity: f32,
    #[serde(default = "default_max_fall_speed")]
    pub max_fall_speed: f32,
}

fn default_gravity() -> f32 {
    -9.8
}
fn default_max_fall_speed() -> f32 {
    -20.0
}

/// スピン物理パラメータ
/// @spec 30401_trajectory_spec.md#req-30401-100
/// @spec 30401_trajectory_spec.md#req-30401-101
/// @spec 30401_trajectory_spec.md#req-30401-102
/// @spec 30402_reflection_spec.md#req-30402-100
/// @data 80101_game_constants.md#spin-physics-config
#[derive(Deserialize, Clone, Debug)]
pub struct SpinPhysicsConfig {
    /// 重力に対するスピンの影響度（±30%時 = 0.3）
    #[serde(default = "default_gravity_spin_factor")]
    pub gravity_spin_factor: f32,

    /// バウンド時の水平方向へのスピンの影響度
    #[serde(default = "default_bounce_spin_horizontal_factor")]
    pub bounce_spin_horizontal_factor: f32,

    /// バウンド時の垂直方向へのスピンの影響度
    #[serde(default = "default_bounce_spin_vertical_factor")]
    pub bounce_spin_vertical_factor: f32,

    /// ベース空気抵抗（スピンなしでも適用）
    #[serde(default = "default_base_air_drag")]
    pub base_air_drag: f32,

    /// スピンによる追加空気抵抗係数
    #[serde(default = "default_spin_drag_factor")]
    pub spin_drag_factor: f32,

    /// スピン時間減衰率（1秒あたり）
    #[serde(default = "default_spin_decay_rate")]
    pub spin_decay_rate: f32,
}

impl Default for SpinPhysicsConfig {
    fn default() -> Self {
        Self {
            gravity_spin_factor: default_gravity_spin_factor(),
            bounce_spin_horizontal_factor: default_bounce_spin_horizontal_factor(),
            bounce_spin_vertical_factor: default_bounce_spin_vertical_factor(),
            base_air_drag: default_base_air_drag(),
            spin_drag_factor: default_spin_drag_factor(),
            spin_decay_rate: default_spin_decay_rate(),
        }
    }
}

fn default_gravity_spin_factor() -> f32 {
    0.3
}
fn default_bounce_spin_horizontal_factor() -> f32 {
    0.3
}
fn default_bounce_spin_vertical_factor() -> f32 {
    0.2
}
fn default_base_air_drag() -> f32 {
    0.0
}
fn default_spin_drag_factor() -> f32 {
    0.3
}
fn default_spin_decay_rate() -> f32 {
    0.5
}
