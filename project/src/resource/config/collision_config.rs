//! 当たり判定・ふっとばしパラメータ
//! @data 80101_game_constants.md#collision-config

use serde::Deserialize;

/// 当たり判定パラメータ
/// @data 80101_game_constants.md#collision-config
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct CollisionConfig {
    pub character_radius: f32,
    pub z_tolerance: f32,
}

impl Default for CollisionConfig {
    fn default() -> Self {
        Self {
            character_radius: 0.5,
            z_tolerance: 0.3,
        }
    }
}

/// ふっとばしパラメータ
/// @data 80101_game_constants.md#knockback-config
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct KnockbackConfig {
    /// ふっとばし機能の有効/無効
    pub enabled: bool,
    pub duration: f32,
    pub speed_multiplier: f32,
    pub invincibility_time: f32,
}

impl Default for KnockbackConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration: 0.5,
            speed_multiplier: 0.5,
            invincibility_time: 1.0,
        }
    }
}
