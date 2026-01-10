//! 視覚パラメータ（影、フィードバック）
//! @data 80101_game_constants.md#shadow-config

use serde::Deserialize;

/// 影パラメータ
/// @data 80101_game_constants.md#shadow-config
#[derive(Deserialize, Clone, Debug, Default)]
pub struct ShadowConfig {
    /// プレイヤー影のサイズ（幅, 高さ）
    #[serde(default = "default_player_shadow_size")]
    pub player_size: (f32, f32),
    /// プレイヤー影の透明度
    #[serde(default = "default_player_shadow_alpha")]
    pub player_alpha: f32,
    /// プレイヤー影のY方向オフセット（足元に表示するため）
    #[serde(default = "default_player_shadow_y_offset")]
    pub player_y_offset: f32,

    /// ボール影のサイズ（幅, 高さ）
    #[serde(default = "default_ball_shadow_size")]
    pub ball_size: (f32, f32),
    /// ボール影の透明度
    #[serde(default = "default_ball_shadow_alpha")]
    pub ball_alpha: f32,
    /// ボール影のY方向オフセット
    #[serde(default = "default_ball_shadow_y_offset")]
    pub ball_y_offset: f32,

    /// 影のZ深度（背面に表示）
    #[serde(default = "default_shadow_z_layer")]
    pub z_layer: f32,
}

fn default_player_shadow_size() -> (f32, f32) {
    (50.0, 20.0)
}
fn default_player_shadow_alpha() -> f32 {
    0.6
}
fn default_player_shadow_y_offset() -> f32 {
    30.0
}
fn default_ball_shadow_size() -> (f32, f32) {
    (25.0, 10.0)
}
fn default_ball_shadow_alpha() -> f32 {
    0.5
}
fn default_ball_shadow_y_offset() -> f32 {
    0.0
}
fn default_shadow_z_layer() -> f32 {
    -0.5
}

/// 視覚フィードバックパラメータ
/// @spec 30802_visual_feedback_spec.md
/// @data 80101_game_constants.md#visual-feedback-config
#[derive(Deserialize, Clone, Debug)]
pub struct VisualFeedbackConfig {
    /// ホールド中のプレイヤー色（RGBA）
    /// @spec 30802_visual_feedback_spec.md#req-30802-001
    #[serde(default = "default_hold_color")]
    pub hold_color: (f32, f32, f32, f32),
    /// トップスピン時のボール色（RGBA）
    /// @spec 30802_visual_feedback_spec.md#req-30802-003
    #[serde(default = "default_ball_color_topspin")]
    pub ball_color_topspin: (f32, f32, f32, f32),
    /// ニュートラル時のボール色（RGBA）
    /// @spec 30802_visual_feedback_spec.md#req-30802-003
    #[serde(default = "default_ball_color_neutral")]
    pub ball_color_neutral: (f32, f32, f32, f32),
    /// スライス時のボール色（RGBA）
    /// @spec 30802_visual_feedback_spec.md#req-30802-003
    #[serde(default = "default_ball_color_slice")]
    pub ball_color_slice: (f32, f32, f32, f32),
}

impl Default for VisualFeedbackConfig {
    fn default() -> Self {
        Self {
            hold_color: default_hold_color(),
            ball_color_topspin: default_ball_color_topspin(),
            ball_color_neutral: default_ball_color_neutral(),
            ball_color_slice: default_ball_color_slice(),
        }
    }
}

fn default_hold_color() -> (f32, f32, f32, f32) {
    (1.0, 0.5, 0.0, 1.0) // オレンジ
}
fn default_ball_color_topspin() -> (f32, f32, f32, f32) {
    (1.0, 0.2, 0.2, 1.0) // 赤
}
fn default_ball_color_neutral() -> (f32, f32, f32, f32) {
    (0.9, 0.9, 0.2, 1.0) // 黄色
}
fn default_ball_color_slice() -> (f32, f32, f32, f32) {
    (0.2, 0.4, 1.0, 1.0) // 青
}
