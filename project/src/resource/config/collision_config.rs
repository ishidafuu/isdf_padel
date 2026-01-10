//! 当たり判定・ふっとばしパラメータ
//! @data 80101_game_constants.md#collision-config

use serde::Deserialize;

/// 当たり判定パラメータ
/// @data 80101_game_constants.md#collision-config
#[derive(Deserialize, Clone, Debug)]
pub struct CollisionConfig {
    #[serde(default = "default_character_radius")]
    pub character_radius: f32,
    #[serde(default = "default_z_tolerance")]
    pub z_tolerance: f32,
}

fn default_character_radius() -> f32 {
    0.5
}
fn default_z_tolerance() -> f32 {
    0.3
}

/// ふっとばしパラメータ
/// @data 80101_game_constants.md#knockback-config
#[derive(Deserialize, Clone, Debug)]
pub struct KnockbackConfig {
    /// ふっとばし機能の有効/無効
    /// false の場合、被弾してもふっとばしが発生しない
    #[serde(default = "default_knockback_enabled")]
    pub enabled: bool,
    #[serde(default = "default_knockback_duration")]
    pub duration: f32,
    #[serde(default = "default_speed_multiplier")]
    pub speed_multiplier: f32,
    #[serde(default = "default_invincibility_time")]
    pub invincibility_time: f32,
}

fn default_knockback_enabled() -> bool {
    true // デフォルトは有効
}
fn default_knockback_duration() -> f32 {
    0.5
}
fn default_speed_multiplier() -> f32 {
    0.5
}
fn default_invincibility_time() -> f32 {
    1.0
}
