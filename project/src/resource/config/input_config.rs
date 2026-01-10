//! 入力パラメータ
//! @data 80101_game_constants.md#input-config

use bevy::input::{gamepad::GamepadButton, keyboard::KeyCode};
use serde::Deserialize;

/// 入力パラメータ
/// @data 80101_game_constants.md#input-config
/// TODO: v0.2で入力バッファリング機能として使用予定
#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct InputConfig {
    #[serde(default = "default_jump_buffer_time")]
    pub jump_buffer_time: f32,
    #[serde(default = "default_shot_buffer_time")]
    pub shot_buffer_time: f32,
    /// 斜め移動の正規化閾値（この値を超えると正規化）
    #[serde(default = "default_normalization_threshold")]
    pub normalization_threshold: f32,
    /// 入力感度（移動入力に乗算する係数、アナログ入力対応用）
    #[serde(default = "default_input_sensitivity")]
    pub input_sensitivity: f32,
}

fn default_jump_buffer_time() -> f32 {
    0.1
}
fn default_shot_buffer_time() -> f32 {
    0.05
}
fn default_normalization_threshold() -> f32 {
    1.0
}
fn default_input_sensitivity() -> f32 {
    1.0
}

/// 入力キーバインド設定
/// @data 80101_game_constants.md#input-keys-config
#[derive(Deserialize, Clone, Debug)]
pub struct InputKeysConfig {
    /// 上移動キー（デフォルト: W）
    #[serde(default = "default_key_move_up")]
    pub move_up: KeyCode,
    /// 下移動キー（デフォルト: S）
    #[serde(default = "default_key_move_down")]
    pub move_down: KeyCode,
    /// 左移動キー（デフォルト: A）
    #[serde(default = "default_key_move_left")]
    pub move_left: KeyCode,
    /// 右移動キー（デフォルト: D）
    #[serde(default = "default_key_move_right")]
    pub move_right: KeyCode,
    /// 上移動キー（代替: 矢印上）
    #[serde(default = "default_key_move_up_alt")]
    pub move_up_alt: KeyCode,
    /// 下移動キー（代替: 矢印下）
    #[serde(default = "default_key_move_down_alt")]
    pub move_down_alt: KeyCode,
    /// 左移動キー（代替: 矢印左）
    #[serde(default = "default_key_move_left_alt")]
    pub move_left_alt: KeyCode,
    /// 右移動キー（代替: 矢印右）
    #[serde(default = "default_key_move_right_alt")]
    pub move_right_alt: KeyCode,
    /// ジャンプキー（デフォルト: B）
    #[serde(default = "default_key_jump")]
    pub jump: KeyCode,
    /// ショットキー（デフォルト: V）
    #[serde(default = "default_key_shot")]
    pub shot: KeyCode,
}

impl Default for InputKeysConfig {
    fn default() -> Self {
        Self {
            move_up: default_key_move_up(),
            move_down: default_key_move_down(),
            move_left: default_key_move_left(),
            move_right: default_key_move_right(),
            move_up_alt: default_key_move_up_alt(),
            move_down_alt: default_key_move_down_alt(),
            move_left_alt: default_key_move_left_alt(),
            move_right_alt: default_key_move_right_alt(),
            jump: default_key_jump(),
            shot: default_key_shot(),
        }
    }
}

fn default_key_move_up() -> KeyCode {
    KeyCode::KeyW
}
fn default_key_move_down() -> KeyCode {
    KeyCode::KeyS
}
fn default_key_move_left() -> KeyCode {
    KeyCode::KeyA
}
fn default_key_move_right() -> KeyCode {
    KeyCode::KeyD
}
fn default_key_move_up_alt() -> KeyCode {
    KeyCode::ArrowUp
}
fn default_key_move_down_alt() -> KeyCode {
    KeyCode::ArrowDown
}
fn default_key_move_left_alt() -> KeyCode {
    KeyCode::ArrowLeft
}
fn default_key_move_right_alt() -> KeyCode {
    KeyCode::ArrowRight
}
fn default_key_jump() -> KeyCode {
    KeyCode::KeyB
}
fn default_key_shot() -> KeyCode {
    KeyCode::KeyV
}

/// ゲームパッドボタン設定
/// @spec 20006_input_system.md#req-20006-053
/// @data 80101_game_constants.md#gamepad-buttons-config
#[derive(Deserialize, Clone, Debug)]
pub struct GamepadButtonsConfig {
    /// ジャンプボタン（デフォルト: South = A on Xbox, × on PlayStation）
    #[serde(default = "default_gamepad_jump")]
    pub jump: GamepadButton,
    /// ショットボタン（デフォルト: East = B on Xbox, ○ on PlayStation）
    #[serde(default = "default_gamepad_shot")]
    pub shot: GamepadButton,
    /// スティックデッドゾーン（入力が無視される範囲）
    #[serde(default = "default_stick_deadzone")]
    pub stick_deadzone: f32,
}

impl Default for GamepadButtonsConfig {
    fn default() -> Self {
        Self {
            jump: default_gamepad_jump(),
            shot: default_gamepad_shot(),
            stick_deadzone: default_stick_deadzone(),
        }
    }
}

fn default_gamepad_jump() -> GamepadButton {
    GamepadButton::South
}
fn default_gamepad_shot() -> GamepadButton {
    GamepadButton::East
}
fn default_stick_deadzone() -> f32 {
    0.1
}
