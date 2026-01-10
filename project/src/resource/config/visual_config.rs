//! 視覚パラメータ（影、フィードバック）
//! @data 80101_game_constants.md#shadow-config

use serde::Deserialize;

/// 影パラメータ
/// @data 80101_game_constants.md#shadow-config
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct ShadowConfig {
    /// プレイヤー影のサイズ（幅, 高さ）
    pub player_size: (f32, f32),
    /// プレイヤー影の透明度
    pub player_alpha: f32,
    /// プレイヤー影のY方向オフセット（足元に表示するため）
    pub player_y_offset: f32,
    /// ボール影のサイズ（幅, 高さ）
    pub ball_size: (f32, f32),
    /// ボール影の透明度
    pub ball_alpha: f32,
    /// ボール影のY方向オフセット
    pub ball_y_offset: f32,
    /// 影のZ深度（背面に表示）
    pub z_layer: f32,
}

impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            player_size: (50.0, 20.0),
            player_alpha: 0.6,
            player_y_offset: 30.0,
            ball_size: (25.0, 10.0),
            ball_alpha: 0.5,
            ball_y_offset: 0.0,
            z_layer: -0.5,
        }
    }
}

/// 視覚フィードバックパラメータ
/// @spec 30802_visual_feedback_spec.md
/// @data 80101_game_constants.md#visual-feedback-config
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct VisualFeedbackConfig {
    /// ホールド中のプレイヤー色（RGBA）
    /// @spec 30802_visual_feedback_spec.md#req-30802-001
    pub hold_color: (f32, f32, f32, f32),
    /// トップスピン時のボール色（RGBA）
    /// @spec 30802_visual_feedback_spec.md#req-30802-003
    pub ball_color_topspin: (f32, f32, f32, f32),
    /// ニュートラル時のボール色（RGBA）
    /// @spec 30802_visual_feedback_spec.md#req-30802-003
    pub ball_color_neutral: (f32, f32, f32, f32),
    /// スライス時のボール色（RGBA）
    /// @spec 30802_visual_feedback_spec.md#req-30802-003
    pub ball_color_slice: (f32, f32, f32, f32),
}

impl Default for VisualFeedbackConfig {
    fn default() -> Self {
        Self {
            hold_color: (1.0, 0.5, 0.0, 1.0),
            ball_color_topspin: (1.0, 0.2, 0.2, 1.0),
            ball_color_neutral: (0.9, 0.9, 0.2, 1.0),
            ball_color_slice: (0.2, 0.4, 1.0, 1.0),
        }
    }
}
