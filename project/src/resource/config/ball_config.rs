//! ボールパラメータ
//! @data 80101_game_constants.md#ball-config

use serde::Deserialize;

/// ボールパラメータ
/// @data 80101_game_constants.md#ball-config
#[derive(Deserialize, Clone, Debug)]
pub struct BallConfig {
    /// TODO: v0.4着地点逆算型弾道システムで使用予定
    #[allow(dead_code)]
    #[serde(default = "default_normal_shot_speed")]
    pub normal_shot_speed: f32,
    /// TODO: v0.2ショット属性システムで使用予定
    #[allow(dead_code)]
    #[serde(default = "default_power_shot_speed")]
    pub power_shot_speed: f32,
    #[serde(default = "default_bounce_factor")]
    pub bounce_factor: f32,
    #[serde(default = "default_ball_radius")]
    pub radius: f32,
    /// 最小バウンド速度（Y速度が0の場合に適用）
    #[serde(default = "default_min_bounce_velocity")]
    pub min_bounce_velocity: f32,
    /// 壁反射係数（壁バウンド時の速度減衰）
    #[serde(default = "default_wall_bounce_factor")]
    pub wall_bounce_factor: f32,
}

fn default_normal_shot_speed() -> f32 {
    10.0
}
fn default_power_shot_speed() -> f32 {
    15.0
}
fn default_bounce_factor() -> f32 {
    0.8
}
fn default_ball_radius() -> f32 {
    0.2
}
fn default_min_bounce_velocity() -> f32 {
    1.0
}
fn default_wall_bounce_factor() -> f32 {
    0.8
}
