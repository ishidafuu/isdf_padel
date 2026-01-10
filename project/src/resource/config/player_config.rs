//! プレイヤー移動・ビジュアルパラメータ
//! @data 80101_game_constants.md#player-config

use serde::Deserialize;

/// プレイヤー移動パラメータ
/// @data 80101_game_constants.md#player-config
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct PlayerConfig {
    pub move_speed: f32,
    pub move_speed_z: f32,
    pub max_speed: f32,
    pub jump_force: f32,
    /// TODO: v0.2で移動システム改善として使用予定
    #[allow(dead_code)]
    pub friction: f32,
    /// TODO: v0.2で空中制御として使用予定
    #[allow(dead_code)]
    pub air_control_factor: f32,
    pub x_min: f32,
    pub x_max: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            move_speed: 5.0,
            move_speed_z: 4.0,
            max_speed: 10.0,
            jump_force: 8.0,
            friction: 0.9,
            air_control_factor: 0.5,
            x_min: -3.0,
            x_max: 3.0,
        }
    }
}

/// プレイヤービジュアル設定
/// @data 80101_game_constants.md#player-visual-config
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct PlayerVisualConfig {
    /// Left側（画面左）の色（RGB）
    pub player1_color: (f32, f32, f32),
    /// Right側（画面右）の色（RGB）
    pub player2_color: (f32, f32, f32),
    /// プレイヤーのサイズ（幅, 高さ）ピクセル
    #[allow(dead_code)]
    pub size: (f32, f32),
}

impl Default for PlayerVisualConfig {
    fn default() -> Self {
        Self {
            player1_color: (0.2, 0.4, 0.8),
            player2_color: (0.8, 0.2, 0.2),
            size: (40.0, 60.0),
        }
    }
}
