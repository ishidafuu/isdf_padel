//! 入力パラメータ
//! @data 80101_game_constants.md#input-config

use bevy::input::{gamepad::GamepadButton, keyboard::KeyCode};
use serde::Deserialize;

/// 入力パラメータ
/// @data 80101_game_constants.md#input-config
/// TODO: v0.2で入力バッファリング機能として使用予定
#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct InputConfig {
    pub jump_buffer_time: f32,
    pub shot_buffer_time: f32,
    /// 斜め移動の正規化閾値（この値を超えると正規化）
    pub normalization_threshold: f32,
    /// 入力感度（移動入力に乗算する係数、アナログ入力対応用）
    pub input_sensitivity: f32,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            jump_buffer_time: 0.1,
            shot_buffer_time: 0.05,
            normalization_threshold: 1.0,
            input_sensitivity: 1.0,
        }
    }
}

/// 入力キーバインド設定
/// @data 80101_game_constants.md#input-keys-config
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct InputKeysConfig {
    /// 上移動キー（デフォルト: W）
    pub move_up: KeyCode,
    /// 下移動キー（デフォルト: S）
    pub move_down: KeyCode,
    /// 左移動キー（デフォルト: A）
    pub move_left: KeyCode,
    /// 右移動キー（デフォルト: D）
    pub move_right: KeyCode,
    /// 上移動キー（代替: 矢印上）
    pub move_up_alt: KeyCode,
    /// 下移動キー（代替: 矢印下）
    pub move_down_alt: KeyCode,
    /// 左移動キー（代替: 矢印左）
    pub move_left_alt: KeyCode,
    /// 右移動キー（代替: 矢印右）
    pub move_right_alt: KeyCode,
    /// ジャンプキー（デフォルト: B）
    pub jump: KeyCode,
    /// ショットキー（デフォルト: V）
    pub shot: KeyCode,
}

impl Default for InputKeysConfig {
    fn default() -> Self {
        Self {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_up_alt: KeyCode::ArrowUp,
            move_down_alt: KeyCode::ArrowDown,
            move_left_alt: KeyCode::ArrowLeft,
            move_right_alt: KeyCode::ArrowRight,
            jump: KeyCode::KeyB,
            shot: KeyCode::KeyV,
        }
    }
}

/// ゲームパッドボタン設定
/// @spec 20006_input_system.md#req-20006-053
/// @data 80101_game_constants.md#gamepad-buttons-config
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct GamepadButtonsConfig {
    /// ジャンプボタン（デフォルト: South = A on Xbox, × on PlayStation）
    pub jump: GamepadButton,
    /// ショットボタン（デフォルト: East = B on Xbox, ○ on PlayStation）
    pub shot: GamepadButton,
    /// スティックデッドゾーン（入力が無視される範囲）
    pub stick_deadzone: f32,
}

impl Default for GamepadButtonsConfig {
    fn default() -> Self {
        Self {
            jump: GamepadButton::South,
            shot: GamepadButton::East,
            stick_deadzone: 0.1,
        }
    }
}
