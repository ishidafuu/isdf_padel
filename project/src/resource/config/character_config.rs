//! パーツ分離キャラクター設定
//! @spec 31001_parts_spec.md
//! @spec 31002_animation_spec.md

use serde::Deserialize;

/// パーツ分離キャラクター設定
/// @spec 31001_parts_spec.md
/// @spec 31002_animation_spec.md
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct CharacterConfig {
    /// アニメーションファイルパス
    pub animation_file_path: String,
    /// Z優先度オフセット（向きによる前後関係調整用）
    /// @spec 31001_parts_spec.md#req-31001-007
    pub z_priority_offset: f32,
    /// 歩行判定の速度閾値
    /// @spec 31002_animation_spec.md#req-31002-051
    pub walk_velocity_threshold: f32,
    /// ジャンプ判定の速度閾値
    /// @spec 31002_animation_spec.md#req-31002-051
    pub jump_velocity_threshold: f32,
}

impl Default for CharacterConfig {
    fn default() -> Self {
        Self {
            animation_file_path: "assets/animations/character_animations.anim.ron".to_string(),
            z_priority_offset: 0.5,
            walk_velocity_threshold: 0.5,
            jump_velocity_threshold: 0.1,
        }
    }
}
