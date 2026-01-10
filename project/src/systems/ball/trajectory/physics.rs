//! ボール物理システム
//! @spec 30401_trajectory_spec.md

use bevy::prelude::*;

use crate::components::{Ball, BallSpin, BallSpinExt, LogicalPosition, Velocity};
use crate::resource::config::GameConfig;
use crate::resource::FixedDeltaTime;

/// ボール重力適用システム
/// @spec 30401_trajectory_spec.md#req-30401-001
/// @spec 30401_trajectory_spec.md#req-30401-004
/// @spec 30401_trajectory_spec.md#req-30401-100
pub fn ball_gravity_system(
    fixed_dt: Res<FixedDeltaTime>,
    config: Res<GameConfig>,
    mut query: Query<(&mut Velocity, &LogicalPosition, Option<&BallSpin>), With<Ball>>,
) {
    let delta = fixed_dt.delta_secs();
    let base_gravity = config.physics.gravity;
    let gravity_spin_factor = config.spin_physics.gravity_spin_factor;

    for (mut velocity, logical_pos, ball_spin) in query.iter_mut() {
        // REQ-30401-001: ボールが空中にある場合のみ重力を適用
        // Y > 0 のときは空中とみなす
        if logical_pos.value.y > 0.0 {
            // REQ-30401-100: スピンによる重力変動
            // effective_gravity = base_gravity * (1.0 + spin * gravity_spin_factor)
            // トップスピン（spin > 0）: 重力増加 → 早く落ちる
            // スライス（spin < 0）: 重力減少 → 浮く
            let spin_value = ball_spin.value_or_default();
            let effective_gravity = base_gravity * (1.0 + spin_value * gravity_spin_factor);

            // REQ-30401-004: 速度更新（重力適用）
            velocity.value.y += effective_gravity * delta;
        }
    }
}

/// ボール位置更新システム
/// @spec 30401_trajectory_spec.md#req-30401-003
pub fn ball_position_update_system(
    fixed_dt: Res<FixedDeltaTime>,
    mut query: Query<(&Velocity, &mut LogicalPosition), With<Ball>>,
) {
    let delta = fixed_dt.delta_secs();

    for (velocity, mut logical_pos) in query.iter_mut() {
        // REQ-30401-003: Position += Velocity * deltaTime
        logical_pos.value += velocity.value * delta;
    }
}

/// ボール空気抵抗システム
/// @spec 30401_trajectory_spec.md#req-30401-102
///
/// スピン絶対値に応じて空気抵抗を増加させる。
/// drag = base_air_drag + spin.abs() * spin_drag_factor
/// 速度減衰: velocity *= (1.0 - drag * delta).max(0.9)
pub fn ball_air_drag_system(
    fixed_dt: Res<FixedDeltaTime>,
    config: Res<GameConfig>,
    mut query: Query<(&mut Velocity, &LogicalPosition, Option<&BallSpin>), With<Ball>>,
) {
    let delta = fixed_dt.delta_secs();
    let base_air_drag = config.spin_physics.base_air_drag;
    let spin_drag_factor = config.spin_physics.spin_drag_factor;

    for (mut velocity, logical_pos, ball_spin) in query.iter_mut() {
        // 空中にある場合のみ適用
        if logical_pos.value.y > 0.0 {
            let spin_value = ball_spin.value_or_default();
            let drag = base_air_drag + spin_value.abs() * spin_drag_factor;

            // 速度減衰（最低0.9を保証して極端な減速を防ぐ）
            let decay_factor = (1.0 - drag * delta).max(0.9);
            velocity.value.x *= decay_factor;
            velocity.value.z *= decay_factor;
            // Y速度は重力で制御されるため、空気抵抗は水平方向のみ適用
        }
    }
}

/// ボールスピン時間減衰システム
/// @spec 30401_trajectory_spec.md#req-30401-101
///
/// 飛行中にスピン効果を時間経過で減衰させる。
/// ball_spin.value *= (1.0 - spin_decay_rate * delta).max(0.0)
pub fn ball_spin_decay_system(
    fixed_dt: Res<FixedDeltaTime>,
    config: Res<GameConfig>,
    mut query: Query<(&mut BallSpin, &LogicalPosition), With<Ball>>,
) {
    let delta = fixed_dt.delta_secs();
    let spin_decay_rate = config.spin_physics.spin_decay_rate;

    for (mut ball_spin, logical_pos) in query.iter_mut() {
        // 空中にある場合のみ減衰
        if logical_pos.value.y > 0.0 {
            ball_spin.value *= (1.0 - spin_decay_rate * delta).max(0.0);
        }
    }
}
