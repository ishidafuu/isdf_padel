//! 視覚フィードバックシステム
//! @spec 30802_visual_feedback_spec.md

use bevy::prelude::*;

use crate::components::{Ball, BallSpin, Player};
use crate::resource::config::GameConfig;
use crate::systems::ShotButtonState;

/// プレイヤーの元の色を保存するコンポーネント
/// @spec 30802_visual_feedback_spec.md#req-30802-001
#[derive(Component, Debug, Clone, Copy)]
pub struct OriginalColor {
    pub color: Color,
}

/// プレイヤーホールド表示システム
/// @spec 30802_visual_feedback_spec.md#req-30802-001
/// @spec 30802_visual_feedback_spec.md#req-30802-002
pub fn player_hold_visual_system(
    config: Res<GameConfig>,
    button_state: Res<ShotButtonState>,
    mut query: Query<(&Player, &mut Sprite, Option<&OriginalColor>), With<Player>>,
) {
    for (player, mut sprite, original_color) in query.iter_mut() {
        // 元の色を保存（初回のみ）
        let original = match original_color {
            Some(oc) => oc.color,
            None => {
                // OriginalColor がない場合は現在の色を保存
                let color = sprite.color;
                // Entity ID を取得するために再度クエリが必要だが、
                // ここでは commands を使って後から追加する
                sprite.color = color; // 現在の色を維持
                color
            }
        };

        // プレイヤーごとのホールド状態を取得
        let (is_holding, hold_time) = match player.id {
            1 => (button_state.player1_holding, button_state.player1_hold_time),
            2 => (button_state.player2_holding, button_state.player2_hold_time),
            _ => (false, 0.0),
        };

        if is_holding {
            // ホールド中: 色をオレンジにブレンド
            // @spec 30802_visual_feedback_spec.md#req-30802-002
            let blend_ratio: f32 =
                (hold_time / config.shot_attributes.hold_stable_time).clamp(0.0, 1.0);
            let hold_color = Color::srgba(
                config.visual_feedback.hold_color.0,
                config.visual_feedback.hold_color.1,
                config.visual_feedback.hold_color.2,
                config.visual_feedback.hold_color.3,
            );
            sprite.color = lerp_color(original, hold_color, blend_ratio);
        } else {
            // ホールドしていない: 元の色に戻す
            sprite.color = original;
        }
    }
}

/// プレイヤーの元の色を保存するシステム
/// @spec 30802_visual_feedback_spec.md#req-30802-001
pub fn save_player_original_color_system(
    query: Query<(Entity, &Sprite), (With<Player>, Without<OriginalColor>)>,
    mut commands: Commands,
) {
    for (entity, sprite) in query.iter() {
        commands.entity(entity).insert(OriginalColor {
            color: sprite.color,
        });
    }
}

/// ボールスピン色変化システム
/// @spec 30802_visual_feedback_spec.md#req-30802-003
/// @spec 30802_visual_feedback_spec.md#req-30802-005
pub fn ball_spin_color_system(
    config: Res<GameConfig>,
    mut query: Query<(&BallSpin, &mut Sprite), With<Ball>>,
) {
    for (ball_spin, mut sprite) in query.iter_mut() {
        // スピン値に応じた色を計算
        // @spec 30802_visual_feedback_spec.md#req-30802-003
        let spin = ball_spin.value;

        let topspin_color = Color::srgba(
            config.visual_feedback.ball_color_topspin.0,
            config.visual_feedback.ball_color_topspin.1,
            config.visual_feedback.ball_color_topspin.2,
            config.visual_feedback.ball_color_topspin.3,
        );
        let neutral_color = Color::srgba(
            config.visual_feedback.ball_color_neutral.0,
            config.visual_feedback.ball_color_neutral.1,
            config.visual_feedback.ball_color_neutral.2,
            config.visual_feedback.ball_color_neutral.3,
        );
        let slice_color = Color::srgba(
            config.visual_feedback.ball_color_slice.0,
            config.visual_feedback.ball_color_slice.1,
            config.visual_feedback.ball_color_slice.2,
            config.visual_feedback.ball_color_slice.3,
        );

        // スピン値に応じて色を補間
        // 正のスピン（トップ）: neutral -> topspin
        // 負のスピン（スライス）: neutral -> slice
        let new_color = if spin >= 0.0 {
            lerp_color(neutral_color, topspin_color, spin.clamp(0.0, 1.0))
        } else {
            lerp_color(neutral_color, slice_color, (-spin).clamp(0.0, 1.0))
        };

        sprite.color = new_color;
    }
}

/// 2色間を線形補間
/// @spec 30802_visual_feedback_spec.md#req-30802-002
#[inline]
fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let a_linear = a.to_linear();
    let b_linear = b.to_linear();

    let r = a_linear.red + (b_linear.red - a_linear.red) * t;
    let g = a_linear.green + (b_linear.green - a_linear.green) * t;
    let blue = a_linear.blue + (b_linear.blue - a_linear.blue) * t;
    let alpha = a_linear.alpha + (b_linear.alpha - a_linear.alpha) * t;

    Color::linear_rgba(r, g, blue, alpha)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30802-001: 色補間テスト
    #[test]
    fn test_lerp_color_midpoint() {
        let black = Color::srgba(0.0, 0.0, 0.0, 1.0);
        let white = Color::srgba(1.0, 1.0, 1.0, 1.0);

        let mid = lerp_color(black, white, 0.5);
        let mid_linear = mid.to_linear();

        // 中間値は約0.5（線形空間での補間）
        assert!((mid_linear.red - 0.5).abs() < 0.01);
        assert!((mid_linear.green - 0.5).abs() < 0.01);
        assert!((mid_linear.blue - 0.5).abs() < 0.01);
    }

    /// TST-30802-002: 色補間テスト（t=0）
    #[test]
    fn test_lerp_color_start() {
        let red = Color::srgba(1.0, 0.0, 0.0, 1.0);
        let blue = Color::srgba(0.0, 0.0, 1.0, 1.0);

        let result = lerp_color(red, blue, 0.0);
        let result_linear = result.to_linear();

        assert!((result_linear.red - 1.0).abs() < 0.01);
        assert!((result_linear.blue - 0.0).abs() < 0.01);
    }

    /// TST-30802-003: 色補間テスト（t=1）
    #[test]
    fn test_lerp_color_end() {
        let red = Color::srgba(1.0, 0.0, 0.0, 1.0);
        let blue = Color::srgba(0.0, 0.0, 1.0, 1.0);

        let result = lerp_color(red, blue, 1.0);
        let result_linear = result.to_linear();

        assert!((result_linear.red - 0.0).abs() < 0.01);
        assert!((result_linear.blue - 1.0).abs() < 0.01);
    }
}
