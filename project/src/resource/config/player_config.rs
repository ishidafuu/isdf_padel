//! プレイヤー移動・ビジュアルパラメータ
//! @data 80101_game_constants.md#player-config

use serde::Deserialize;

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
    /// TODO: v0.2で移動システム改善として使用予定
    #[allow(dead_code)]
    #[serde(default = "default_friction")]
    pub friction: f32,
    /// TODO: v0.2で空中制御として使用予定
    #[allow(dead_code)]
    #[serde(default = "default_air_control")]
    pub air_control_factor: f32,
    #[serde(default = "default_x_min")]
    pub x_min: f32,
    #[serde(default = "default_x_max")]
    pub x_max: f32,
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
fn default_x_min() -> f32 {
    -3.0
}
fn default_x_max() -> f32 {
    3.0
}

/// プレイヤービジュアル設定
/// @data 80101_game_constants.md#player-visual-config
#[derive(Deserialize, Clone, Debug)]
pub struct PlayerVisualConfig {
    /// Left側（画面左）の色（RGB）
    #[serde(default = "default_player1_color")]
    pub player1_color: (f32, f32, f32),
    /// Right側（画面右）の色（RGB）
    #[serde(default = "default_player2_color")]
    pub player2_color: (f32, f32, f32),
    /// プレイヤーのサイズ（幅, 高さ）ピクセル
    /// ArticulatedCharacterではパーツサイズを使用するため未使用
    #[serde(default = "default_player_size")]
    #[allow(dead_code)]
    pub size: (f32, f32),
}

impl Default for PlayerVisualConfig {
    fn default() -> Self {
        Self {
            player1_color: default_player1_color(),
            player2_color: default_player2_color(),
            size: default_player_size(),
        }
    }
}

fn default_player1_color() -> (f32, f32, f32) {
    (0.2, 0.4, 0.8) // 青
}

fn default_player2_color() -> (f32, f32, f32) {
    (0.8, 0.2, 0.2) // 赤
}

fn default_player_size() -> (f32, f32) {
    (40.0, 60.0)
}
