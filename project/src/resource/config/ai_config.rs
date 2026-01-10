//! AIパラメータ
//! @spec 30301_ai_movement_spec.md
//! @spec 30302_ai_shot_spec.md
//! @spec 30102_serve_spec.md#req-30102-070

use serde::Deserialize;

/// AIパラメータ
/// @spec 30301_ai_movement_spec.md
/// @spec 30302_ai_shot_spec.md
/// @spec 30102_serve_spec.md#req-30102-070
#[derive(Deserialize, Clone, Debug)]
pub struct AiConfig {
    /// AI移動速度（m/s）
    /// @spec 30301_ai_movement_spec.md#req-30301-001
    #[serde(default = "default_ai_move_speed")]
    pub move_speed: f32,
    /// ホームポジションX座標（m、打ち合い方向）
    /// @spec 30301_ai_movement_spec.md#req-30301-005
    #[serde(default = "default_ai_home_position_x")]
    pub home_position_x: f32,
    /// AIショットクールダウン（秒）
    /// @spec 30302_ai_shot_spec.md#req-30302-002
    /// @spec 30302_ai_shot_spec.md#req-30302-004
    #[serde(default = "default_ai_shot_cooldown")]
    pub shot_cooldown: f32,
    /// ホーム復帰時の停止距離（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-005
    #[serde(default = "default_ai_home_return_stop_distance")]
    pub home_return_stop_distance: f32,
    /// AIサーブまでの待機時間下限（秒）
    /// @spec 30102_serve_spec.md#req-30102-070
    #[serde(default = "default_ai_serve_delay_min")]
    pub serve_delay_min: f32,
    /// AIサーブまでの待機時間上限（秒）
    /// @spec 30102_serve_spec.md#req-30102-070
    #[serde(default = "default_ai_serve_delay_max")]
    pub serve_delay_max: f32,
    /// AIサーブ方向バリエーション（Z軸）
    /// @spec 30102_serve_spec.md#req-30102-071
    #[serde(default = "default_ai_serve_direction_variance")]
    pub serve_direction_variance: f32,

    // === v0.5 追加パラメータ ===

    /// 待機時のX軸深さ（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    #[serde(default = "default_ai_optimal_depth")]
    pub optimal_depth: f32,
    /// Z軸調整係数（ボール位置に応じた横移動）
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    #[serde(default = "default_ai_coverage_bias_factor")]
    pub coverage_bias_factor: f32,
    /// Z軸移動の最大値（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    #[serde(default = "default_ai_max_z_offset")]
    pub max_z_offset: f32,
    /// リカバリー時のX軸深さ（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    #[serde(default = "default_ai_recovery_depth")]
    pub recovery_depth: f32,
    /// 打球逆サイドへの寄り係数
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    #[serde(default = "default_ai_recovery_bias_factor")]
    pub recovery_bias_factor: f32,
    /// リカバリーZ軸の最大値（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    #[serde(default = "default_ai_max_recovery_z")]
    pub max_recovery_z: f32,

    // === v0.6 AI不完全性パラメータ ===

    /// 予測精度（0.0～1.0、1.0で完璧）
    /// @spec 30301_ai_movement_spec.md#req-30301-052
    #[serde(default = "default_ai_prediction_accuracy")]
    pub prediction_accuracy: f32,
    /// 最大予測誤差（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-052
    #[serde(default = "default_ai_prediction_error")]
    pub prediction_error: f32,
    /// 打球方向のランダムブレ（度）
    /// @spec 30302_ai_shot_spec.md#req-30302-055
    #[serde(default = "default_ai_direction_variance")]
    pub direction_variance: f32,
    /// 反応遅延（秒）
    /// @spec 30301_ai_movement_spec.md#req-30301-053
    #[serde(default = "default_ai_reaction_delay")]
    pub reaction_delay: f32,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            move_speed: default_ai_move_speed(),
            home_position_x: default_ai_home_position_x(),
            shot_cooldown: default_ai_shot_cooldown(),
            home_return_stop_distance: default_ai_home_return_stop_distance(),
            serve_delay_min: default_ai_serve_delay_min(),
            serve_delay_max: default_ai_serve_delay_max(),
            serve_direction_variance: default_ai_serve_direction_variance(),
            // v0.5 追加パラメータ
            optimal_depth: default_ai_optimal_depth(),
            coverage_bias_factor: default_ai_coverage_bias_factor(),
            max_z_offset: default_ai_max_z_offset(),
            recovery_depth: default_ai_recovery_depth(),
            recovery_bias_factor: default_ai_recovery_bias_factor(),
            max_recovery_z: default_ai_max_recovery_z(),
            // v0.6 AI不完全性パラメータ
            prediction_accuracy: default_ai_prediction_accuracy(),
            prediction_error: default_ai_prediction_error(),
            direction_variance: default_ai_direction_variance(),
            reaction_delay: default_ai_reaction_delay(),
        }
    }
}

fn default_ai_move_speed() -> f32 {
    5.0
}
fn default_ai_home_position_x() -> f32 {
    5.0 // 2Pコート側（+X方向）
}
/// @spec 30302_ai_shot_spec.md#req-30302-002
fn default_ai_shot_cooldown() -> f32 {
    0.5 // デフォルト: 0.5秒
}
fn default_ai_home_return_stop_distance() -> f32 {
    0.3
}
fn default_ai_serve_delay_min() -> f32 {
    0.5 // 秒
}
fn default_ai_serve_delay_max() -> f32 {
    1.5 // 秒
}
fn default_ai_serve_direction_variance() -> f32 {
    0.5 // Z軸方向のバリエーション
}

// === v0.5 追加パラメータ ===

fn default_ai_optimal_depth() -> f32 {
    5.0 // 待機時のX軸深さ（m）
}
fn default_ai_coverage_bias_factor() -> f32 {
    0.3 // Z軸調整係数
}
fn default_ai_max_z_offset() -> f32 {
    3.0 // Z軸移動の最大値（m）
}
fn default_ai_recovery_depth() -> f32 {
    4.0 // リカバリー時のX軸深さ（m）
}
fn default_ai_recovery_bias_factor() -> f32 {
    0.5 // 打球逆サイドへの寄り係数
}
fn default_ai_max_recovery_z() -> f32 {
    2.5 // リカバリーZ軸の最大値（m）
}

// === v0.6 AI不完全性パラメータ ===

/// 予測精度（Normal難易度相当: 0.7）
/// @spec 30301_ai_movement_spec.md#req-30301-052
fn default_ai_prediction_accuracy() -> f32 {
    0.7
}
/// 最大予測誤差（Normal難易度相当: 0.5m）
/// @spec 30301_ai_movement_spec.md#req-30301-052
fn default_ai_prediction_error() -> f32 {
    0.5
}
/// 打球方向のランダムブレ（Normal難易度相当: 10度）
/// @spec 30302_ai_shot_spec.md#req-30302-055
fn default_ai_direction_variance() -> f32 {
    10.0
}
/// 反応遅延（Normal難易度相当: 150ms）
/// @spec 30301_ai_movement_spec.md#req-30301-053
fn default_ai_reaction_delay() -> f32 {
    0.15
}
