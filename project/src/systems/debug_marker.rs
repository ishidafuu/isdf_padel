//! デバッグマーカーシステム
//! 弾道計算の詳細ログを手動で出力するための機能
//!
//! Xボタン（GamepadButton::West）を押すと、直前のショット情報をログ出力する

use bevy::prelude::*;

use crate::resource::debug::LastShotDebugInfo;

/// デバッグマーカーシステム
/// Xボタン（West）を押すと LastShotDebugInfo の内容を [MARKED] 付きでログ出力
pub fn debug_marker_system(
    gamepads: Query<&Gamepad>,
    debug_info: Res<LastShotDebugInfo>,
) {
    // ゲームパッドを取得
    let Some(gamepad) = gamepads.iter().next() else {
        return;
    };

    // Xボタン（West）が押されたらログ出力
    if gamepad.just_pressed(GamepadButton::West) {
        if debug_info.is_valid {
            info!("[MARKED] ========================================");
            info!(
                "[MARKED] player={}, court_side={:?}",
                debug_info.player_id, debug_info.court_side
            );
            info!(
                "[MARKED] ball_pos=({:.2},{:.2},{:.2}), input=({:.2},{:.2})",
                debug_info.ball_pos.x,
                debug_info.ball_pos.y,
                debug_info.ball_pos.z,
                debug_info.input.x,
                debug_info.input.y
            );
            info!(
                "[MARKED] power={:.1}, spin={:.2}, accuracy={:.2}",
                debug_info.power, debug_info.spin, debug_info.accuracy
            );
            info!(
                "[MARKED] landing=({:.2},{:.2}), angle={:.1}, speed={:.1}",
                debug_info.landing.x,
                debug_info.landing.z,
                debug_info.launch_angle,
                debug_info.final_speed
            );
            info!(
                "[MARKED] velocity=({:.2},{:.2},{:.2})",
                debug_info.velocity.x, debug_info.velocity.y, debug_info.velocity.z
            );
            info!(
                "[MARKED] discriminant={:.1}, g_eff={:.2}",
                debug_info.discriminant, debug_info.g_eff
            );
            info!("[MARKED] ========================================");
        } else {
            info!("[MARKED] No valid shot data (ball has bounced or no shot yet)");
        }
    }
}
