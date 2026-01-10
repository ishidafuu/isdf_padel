//! 入力関連コンポーネント
//! @spec 20006_input_system.md

use bevy::prelude::*;

/// 入力状態コンポーネント
/// 各プレイヤーエンティティに付与される入力状態
/// @spec 20006_input_system.md
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct InputState {
    /// 移動入力（-1.0 〜 1.0）
    pub movement: Vec2,
    /// ジャンプボタンが押されたか（今フレーム）
    pub jump_pressed: bool,
    /// ショットボタンが押されたか（今フレーム）
    pub shot_pressed: bool,
    /// ショットボタンを保持中か
    pub holding: bool,
    /// ホールド継続時間（秒）
    pub hold_time: f32,
}
