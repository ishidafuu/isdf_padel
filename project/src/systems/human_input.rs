//! 人間入力システム
//! HumanControlled マーカーを持つプレイヤーの InputState を更新

use bevy::prelude::*;

use crate::components::{HumanControlled, InputState};
use crate::resource::config::GameConfig;

/// 人間入力読み取りシステム
/// HumanControlled を持つプレイヤーの InputState を更新
/// キーバインドは GameConfig.input_keys から取得
pub fn human_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    config: Res<GameConfig>,
    mut query: Query<(&HumanControlled, &mut InputState)>,
) {
    let delta_ms = time.delta_secs() * 1000.0;
    let keys = &config.input_keys;

    for (human, mut input_state) in query.iter_mut() {
        // デバイスIDに基づいて入力を読み取る
        // 現在はキーボード（device_id=0）のみ対応
        if human.device_id != 0 {
            continue;
        }

        // 移動入力（横向きコート用マッピング）
        let mut movement = Vec2::ZERO;

        // W/S → 論理X（画面上下）
        if keyboard.pressed(keys.move_up) || keyboard.pressed(keys.move_up_alt) {
            movement.x += 1.0;
        }
        if keyboard.pressed(keys.move_down) || keyboard.pressed(keys.move_down_alt) {
            movement.x -= 1.0;
        }

        // A/D → 論理Z（画面左右＝打ち合い方向）
        if keyboard.pressed(keys.move_left) || keyboard.pressed(keys.move_left_alt) {
            movement.y -= 1.0;
        }
        if keyboard.pressed(keys.move_right) || keyboard.pressed(keys.move_right_alt) {
            movement.y += 1.0;
        }

        input_state.movement = movement;

        // ジャンプ入力
        input_state.jump_pressed = keyboard.just_pressed(keys.jump);

        // ショット入力
        let shot_key_pressed = keyboard.pressed(keys.shot);
        let shot_key_just_pressed = keyboard.just_pressed(keys.shot);

        input_state.shot_pressed = shot_key_just_pressed;

        // ショットホールド状態の追跡
        if shot_key_pressed {
            if !input_state.holding {
                // 押し始め
                input_state.holding = true;
                input_state.hold_time = 0.0;
            } else {
                // 押し続けている
                input_state.hold_time += delta_ms;
            }
        } else {
            // 離した
            input_state.holding = false;
            // hold_time はショット実行時に参照されるのでリセットしない
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_state_default() {
        let state = InputState::default();
        assert_eq!(state.movement, Vec2::ZERO);
        assert!(!state.jump_pressed);
        assert!(!state.shot_pressed);
        assert!(!state.holding);
        assert_eq!(state.hold_time, 0.0);
    }
}
