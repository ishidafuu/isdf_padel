//! ゲーム設定
//! @data 80101_game_constants.md

use bevy::prelude::*;
use serde::Deserialize;

/// ゲーム全体の設定
/// @data 80101_game_constants.md#gameconfig-構造
#[derive(Resource, Deserialize, Clone, Debug)]
pub struct GameConfig {
    pub physics: PhysicsConfig,
    pub court: CourtConfig,
    pub player: PlayerConfig,
    pub ball: BallConfig,
    pub collision: CollisionConfig,
    pub knockback: KnockbackConfig,
    pub shot: ShotConfig,
    pub scoring: ScoringConfig,
    pub input: InputConfig,
}

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

/// コートサイズ・範囲
/// @data 80101_game_constants.md#court-config
#[derive(Deserialize, Clone, Debug)]
pub struct CourtConfig {
    #[serde(default = "default_court_width")]
    pub width: f32,
    #[serde(default = "default_court_depth")]
    pub depth: f32,
    #[serde(default = "default_ceiling_height")]
    pub ceiling_height: f32,
    #[serde(default = "default_max_jump_height")]
    pub max_jump_height: f32,
    #[serde(default = "default_net_height")]
    pub net_height: f32,
    #[serde(default = "default_net_z")]
    pub net_z: f32,
}

fn default_court_width() -> f32 {
    10.0
}
fn default_court_depth() -> f32 {
    6.0
}
fn default_ceiling_height() -> f32 {
    5.0
}
fn default_max_jump_height() -> f32 {
    5.0
}
fn default_net_height() -> f32 {
    1.0
}
fn default_net_z() -> f32 {
    0.0
}

/// プレイヤー移動パラメータ
/// @data 80101_game_constants.md#player-config
#[derive(Deserialize, Clone, Debug)]
pub struct PlayerConfig {
    #[serde(default = "default_move_speed")]
    pub move_speed: f32,
    #[serde(default = "default_move_speed_z")]
    pub move_speed_z: f32,
    #[serde(default = "default_max_speed")]
    pub max_speed: f32,
    #[serde(default = "default_jump_force")]
    pub jump_force: f32,
    #[serde(default = "default_friction")]
    pub friction: f32,
    #[serde(default = "default_air_control")]
    pub air_control_factor: f32,
    #[serde(default = "default_z_min")]
    pub z_min: f32,
    #[serde(default = "default_z_max")]
    pub z_max: f32,
}

fn default_move_speed() -> f32 {
    5.0
}
fn default_move_speed_z() -> f32 {
    4.0
}
fn default_max_speed() -> f32 {
    10.0
}
fn default_jump_force() -> f32 {
    8.0
}
fn default_friction() -> f32 {
    0.9
}
fn default_air_control() -> f32 {
    0.5
}
fn default_z_min() -> f32 {
    -3.0
}
fn default_z_max() -> f32 {
    3.0
}

/// ボールパラメータ
/// @data 80101_game_constants.md#ball-config
#[derive(Deserialize, Clone, Debug)]
pub struct BallConfig {
    #[serde(default = "default_normal_shot_speed")]
    pub normal_shot_speed: f32,
    #[serde(default = "default_power_shot_speed")]
    pub power_shot_speed: f32,
    #[serde(default = "default_bounce_factor")]
    pub bounce_factor: f32,
    #[serde(default = "default_ball_radius")]
    pub radius: f32,
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
    #[serde(default = "default_knockback_duration")]
    pub duration: f32,
    #[serde(default = "default_speed_multiplier")]
    pub speed_multiplier: f32,
    #[serde(default = "default_invincibility_time")]
    pub invincibility_time: f32,
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

/// ショットシステムパラメータ
/// @data 80101_game_constants.md#shot-config
#[derive(Deserialize, Clone, Debug)]
pub struct ShotConfig {
    #[serde(default = "default_max_distance")]
    pub max_distance: f32,
    #[serde(default = "default_max_height_diff")]
    pub max_height_diff: f32,
    #[serde(default = "default_cooldown_time")]
    pub cooldown_time: f32,
    #[serde(default = "default_normal_shot_angle")]
    pub normal_shot_angle: f32,
    #[serde(default = "default_jump_shot_angle")]
    pub jump_shot_angle: f32,
    #[serde(default = "default_jump_threshold")]
    pub jump_threshold: f32,
}

fn default_max_distance() -> f32 {
    1.5
}
fn default_max_height_diff() -> f32 {
    2.0
}
fn default_cooldown_time() -> f32 {
    0.5
}
fn default_normal_shot_angle() -> f32 {
    45.0
}
fn default_jump_shot_angle() -> f32 {
    30.0
}
fn default_jump_threshold() -> f32 {
    0.5
}

/// スコアリングパラメータ
/// @data 80101_game_constants.md#scoring-config
/// @data 80701_point_config.md
/// @data 80703_set_config.md
#[derive(Deserialize, Clone, Debug)]
pub struct ScoringConfig {
    /// ポイント進行値 [0, 15, 30, 40]
    /// @spec 30701_point_spec.md#req-30701-001
    #[serde(default = "default_point_values")]
    pub point_values: Vec<u32>,
    /// 勝利に必要なゲーム数（6ゲーム先取でセット獲得）
    /// @data 80703_set_config.md#games_to_win_set
    #[serde(default = "default_games_to_win_set")]
    pub games_to_win_set: u32,
    /// 勝利に必要なセット数（1セット先取でマッチ勝利）
    /// @data 80703_set_config.md#sets_to_win_match
    #[serde(default = "default_sets_to_win_match")]
    pub sets_to_win_match: u32,
}

fn default_point_values() -> Vec<u32> {
    vec![0, 15, 30, 40]
}
fn default_games_to_win_set() -> u32 {
    6
}
fn default_sets_to_win_match() -> u32 {
    1
}

/// 入力パラメータ
/// @data 80101_game_constants.md#input-config
#[derive(Deserialize, Clone, Debug)]
pub struct InputConfig {
    #[serde(default = "default_jump_buffer_time")]
    pub jump_buffer_time: f32,
    #[serde(default = "default_shot_buffer_time")]
    pub shot_buffer_time: f32,
}

fn default_jump_buffer_time() -> f32 {
    0.1
}
fn default_shot_buffer_time() -> f32 {
    0.05
}

/// RONファイルからGameConfigをロード
pub fn load_game_config(path: &str) -> Result<GameConfig, String> {
    let config_str =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;
    ron::from_str(&config_str).map_err(|e| format!("Failed to parse config: {}", e))
}
