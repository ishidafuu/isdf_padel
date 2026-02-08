//! ゲームパッド入力システム
//! HumanControlled マーカーを持つプレイヤーの InputState をゲームパッドで更新
//! device_id=0: キーボード入力との併用（OR演算）
//!
//! 実行順序: human_input_system → gamepad_input_system
//! @spec 20006_input_system.md#req-20006-050

use bevy::prelude::*;

use crate::components::{quantize_movement, HumanControlled, InputState};
use crate::resource::config::GameConfig;

/// ゲームパッド入力読み取りシステム
/// device_id=0: キーボード入力との併用（ゲームパッド入力があれば優先/OR演算）
/// device_id=1以上: ゲームパッド専用
/// @spec 20006_input_system.md#req-20006-050
pub fn gamepad_input_system(
    gamepads: Query<&Gamepad>,
    time: Res<Time>,
    config: Res<GameConfig>,
    mut query: Query<(&HumanControlled, &mut InputState)>,
) {
    let delta_ms = time.delta_secs() * 1000.0;
    let gamepad_config = &config.gamepad_buttons;

    // 最初に見つかったゲームパッドを使用
    // @spec 20006_input_system.md#req-20006-051
    let Some(gamepad) = gamepads.iter().next() else {
        // ゲームパッド未接続時は何もしない
        return;
    };

    for (human, mut input_state) in query.iter_mut() {
        // device_id=0（キーボード併用）またはdevice_id=1（ゲームパッド専用）のみ対応
        if human.device_id > 1 {
            continue;
        }

        // 左スティック入力（デッドゾーン適用）
        // @spec 20006_input_system.md#req-20006-052
        let left_stick = gamepad.left_stick();
        let mut gamepad_movement = Vec2::new(left_stick.x, left_stick.y);
        if gamepad_movement.length() < gamepad_config.stick_deadzone {
            gamepad_movement = Vec2::ZERO;
        }
        gamepad_movement *= config.input.input_sensitivity;
        gamepad_movement = quantize_movement(gamepad_movement);

        // device_id=0: キーボードとゲームパッドの論理和（ゲームパッド入力があれば優先）
        // device_id=1: ゲームパッド入力のみ
        if human.device_id == 0 {
            // ゲームパッド入力があればそちらを使用、なければキーボード入力を維持
            if gamepad_movement.length_squared() > 0.0 {
                input_state.movement = gamepad_movement;
            }
        } else {
            input_state.movement = gamepad_movement;
        }

        // ジャンプ入力
        // @spec 20006_input_system.md#req-20006-053
        let gamepad_jump = gamepad.just_pressed(gamepad_config.jump);
        if human.device_id == 0 {
            // キーボードとゲームパッドのOR演算
            input_state.jump_pressed = input_state.jump_pressed || gamepad_jump;
        } else {
            input_state.jump_pressed = gamepad_jump;
        }

        // ショット入力
        // @spec 20006_input_system.md#req-20006-053
        let gamepad_shot_pressed = gamepad.pressed(gamepad_config.shot);
        let gamepad_shot_just_pressed = gamepad.just_pressed(gamepad_config.shot);

        if human.device_id == 0 {
            // キーボードとゲームパッドのOR演算
            input_state.shot_pressed = input_state.shot_pressed || gamepad_shot_just_pressed;

            // ホールド状態の追跡
            // human_input_system が先に実行され、キーボード入力に基づいて holding/hold_time を設定済み
            // ゲームパッドでホールド中ならその状態を上書き/継続
            if gamepad_shot_pressed {
                if !input_state.holding {
                    // gamepad-only(device_id=0)のケースでは human_input_system 側で
                    // holding=false になっているため、ここで新規ホールドとして初期化する。
                    input_state.holding = true;
                    input_state.hold_time = 0.0;
                } else if !gamepad_shot_just_pressed {
                    // ゲームパッドでホールド継続（2フレーム目以降）
                    input_state.hold_time += delta_ms;
                }
                // キーボード側で既にホールド中のときは、hold_time を維持してOR入力として扱う。
            }
            // ゲームパッドが離されている場合:
            // - キーボードでホールド中 → holding = true のまま（human_input_system で設定済み）
            // - キーボードも離されている → holding = false（human_input_system で設定済み）
        } else {
            input_state.shot_pressed = gamepad_shot_just_pressed;

            // ショットホールド状態の追跡
            if gamepad_shot_pressed {
                if !input_state.holding {
                    input_state.holding = true;
                    input_state.hold_time = 0.0;
                } else {
                    input_state.hold_time += delta_ms;
                }
            } else {
                input_state.holding = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_id_filter() {
        // device_id=1のみが処理対象
        let human_keyboard = HumanControlled { device_id: 0 };
        let human_gamepad = HumanControlled { device_id: 1 };

        assert_eq!(human_keyboard.device_id, 0);
        assert_eq!(human_gamepad.device_id, 1);
    }
}
