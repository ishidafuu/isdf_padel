//! ボールパラメータ
//! @data 80101_game_constants.md#ball-config

use serde::Deserialize;

/// ボールパラメータ
/// @data 80101_game_constants.md#ball-config
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct BallConfig {
    /// TODO: v0.4着地点逆算型弾道システムで使用予定
    #[allow(dead_code)]
    pub normal_shot_speed: f32,
    /// TODO: v0.2ショット属性システムで使用予定
    #[allow(dead_code)]
    pub power_shot_speed: f32,
    pub bounce_factor: f32,
    pub radius: f32,
    /// 最小バウンド速度（Y速度が0の場合に適用）
    pub min_bounce_velocity: f32,
    /// 壁反射係数（壁バウンド時の速度減衰）
    pub wall_bounce_factor: f32,
}

impl Default for BallConfig {
    fn default() -> Self {
        Self {
            normal_shot_speed: 10.0,
            power_shot_speed: 15.0,
            bounce_factor: 0.8,
            radius: 0.2,
            min_bounce_velocity: 1.0,
            wall_bounce_factor: 0.8,
        }
    }
}
