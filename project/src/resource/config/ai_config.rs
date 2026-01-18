//! AIパラメータ
//! @spec 30301_ai_movement_spec.md
//! @spec 30302_ai_shot_spec.md
//! @spec 30102_serve_spec.md#req-30102-070
//! @spec 30303_ai_tactics_spec.md

use serde::Deserialize;

/// AIパラメータ
/// @spec 30301_ai_movement_spec.md
/// @spec 30302_ai_shot_spec.md
/// @spec 30102_serve_spec.md#req-30102-070
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct AiConfig {
    /// AI移動速度（m/s）
    /// @spec 30301_ai_movement_spec.md#req-30301-001
    pub move_speed: f32,
    /// ホームポジションX座標（m、打ち合い方向）
    /// @spec 30301_ai_movement_spec.md#req-30301-005
    pub home_position_x: f32,
    /// AIショットクールダウン（秒）
    /// @spec 30302_ai_shot_spec.md#req-30302-002
    /// @spec 30302_ai_shot_spec.md#req-30302-004
    pub shot_cooldown: f32,
    /// ホーム復帰時の停止距離（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-005
    pub home_return_stop_distance: f32,
    /// AIサーブまでの待機時間下限（秒）
    /// @spec 30102_serve_spec.md#req-30102-070
    pub serve_delay_min: f32,
    /// AIサーブまでの待機時間上限（秒）
    /// @spec 30102_serve_spec.md#req-30102-070
    pub serve_delay_max: f32,
    /// AIサーブ方向バリエーション（Z軸）
    /// @spec 30102_serve_spec.md#req-30102-071
    pub serve_direction_variance: f32,
    /// 待機時のX軸深さ（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    pub optimal_depth: f32,
    /// Z軸調整係数（ボール位置に応じた横移動）
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    pub coverage_bias_factor: f32,
    /// Z軸移動の最大値（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    pub max_z_offset: f32,
    /// リカバリー時のX軸深さ（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    pub recovery_depth: f32,
    /// 打球逆サイドへの寄り係数
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    pub recovery_bias_factor: f32,
    /// リカバリーZ軸の最大値（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-v05
    pub max_recovery_z: f32,
    /// 予測精度（0.0～1.0、1.0で完璧）
    /// @spec 30301_ai_movement_spec.md#req-30301-052
    pub prediction_accuracy: f32,
    /// 最大予測誤差（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-052
    pub prediction_error: f32,
    /// 打球方向のランダムブレ（度）
    /// @spec 30302_ai_shot_spec.md#req-30302-055
    pub direction_variance: f32,
    /// 反応遅延（秒）
    /// @spec 30301_ai_movement_spec.md#req-30301-053
    pub reaction_delay: f32,
    // === 戦術パラメータ ===
    // @spec 30303_ai_tactics_spec.md
    /// 攻め可能な最適距離（m）- この距離以内なら攻め可能
    /// @spec 30303_ai_tactics_spec.md#req-30303-010
    pub optimal_distance: f32,
    /// 攻め確率（0.0～1.0）- 条件を満たした時に攻める確率
    /// @spec 30303_ai_tactics_spec.md#req-30303-011
    pub offensive_probability: f32,
    /// 攻めマージン（m）- ライン際からの内側マージン
    /// @spec 30303_ai_tactics_spec.md#req-30303-022
    pub offensive_margin: f32,
    /// サーブ攻め確率（0.0～1.0）
    /// @spec 30303_ai_tactics_spec.md#req-30303-030
    pub serve_offensive_probability: f32,
    /// サーブ攻めマージン（m）- サービスエリア端からのマージン
    /// @spec 30303_ai_tactics_spec.md#req-30303-032
    pub serve_offensive_margin: f32,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            move_speed: 5.0,
            home_position_x: 5.0,
            shot_cooldown: 0.5,
            home_return_stop_distance: 0.3,
            serve_delay_min: 0.5,
            serve_delay_max: 1.5,
            serve_direction_variance: 0.5,
            optimal_depth: 5.0,
            coverage_bias_factor: 0.3,
            max_z_offset: 3.0,
            recovery_depth: 4.0,
            recovery_bias_factor: 0.5,
            max_recovery_z: 2.5,
            prediction_accuracy: 0.7,
            prediction_error: 0.5,
            direction_variance: 10.0,
            reaction_delay: 0.15,
            // 戦術パラメータ
            optimal_distance: 1.2,
            offensive_probability: 0.6,
            offensive_margin: 0.8,
            serve_offensive_probability: 0.5,
            serve_offensive_margin: 0.3,
        }
    }
}
