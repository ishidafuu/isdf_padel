//! パーツ分離キャラクター設定
//! @spec 31001_parts_spec.md
//! @spec 31002_animation_spec.md

use serde::Deserialize;

/// パーツ分離キャラクター設定
/// @spec 31001_parts_spec.md
/// @spec 31002_animation_spec.md
#[derive(Deserialize, Clone, Debug)]
pub struct CharacterConfig {
    /// アニメーションファイルパス
    #[serde(default = "default_animation_file_path")]
    pub animation_file_path: String,
    /// Z優先度オフセット（向きによる前後関係調整用）
    /// @spec 31001_parts_spec.md#req-31001-007
    #[serde(default = "default_z_priority_offset")]
    pub z_priority_offset: f32,
    /// 歩行判定の速度閾値
    /// @spec 31002_animation_spec.md#req-31002-051
    #[serde(default = "default_walk_velocity_threshold")]
    pub walk_velocity_threshold: f32,
    /// ジャンプ判定の速度閾値
    /// @spec 31002_animation_spec.md#req-31002-051
    #[serde(default = "default_jump_velocity_threshold")]
    pub jump_velocity_threshold: f32,
}

impl Default for CharacterConfig {
    fn default() -> Self {
        Self {
            animation_file_path: default_animation_file_path(),
            z_priority_offset: default_z_priority_offset(),
            walk_velocity_threshold: default_walk_velocity_threshold(),
            jump_velocity_threshold: default_jump_velocity_threshold(),
        }
    }
}

fn default_animation_file_path() -> String {
    "assets/animations/character_animations.anim.ron".to_string()
}

fn default_z_priority_offset() -> f32 {
    0.5
}

fn default_walk_velocity_threshold() -> f32 {
    0.5
}

fn default_jump_velocity_threshold() -> f32 {
    0.1
}
