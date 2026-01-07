//! ショット方向計算システム
//! @spec 30602_shot_direction_spec.md
//! @spec 30603_jump_shot_spec.md

use bevy::prelude::*;

use crate::components::{Ball, BounceCount, Velocity};
use crate::core::events::{ShotEvent, ShotExecutedEvent};
use crate::resource::config::GameConfig;

/// ショット方向計算システム
/// ShotEvent を受信してボールの速度を設定する
/// @spec 30602_shot_direction_spec.md#req-30602-001
/// @spec 30602_shot_direction_spec.md#req-30602-002
/// @spec 30602_shot_direction_spec.md#req-30602-003
/// @spec 30602_shot_direction_spec.md#req-30602-004
/// @spec 30602_shot_direction_spec.md#req-30602-005
/// @spec 30602_shot_direction_spec.md#req-30602-007
pub fn shot_direction_system(
    config: Res<GameConfig>,
    mut shot_events: MessageReader<ShotEvent>,
    mut ball_query: Query<(&mut Velocity, &mut BounceCount), With<Ball>>,
    mut shot_executed_writer: MessageWriter<ShotExecutedEvent>,
) {
    for event in shot_events.read() {
        // ボールを取得
        let Ok((mut velocity, mut bounce_count)) = ball_query.single_mut() else {
            warn!("No ball found for shot direction calculation");
            continue;
        };

        // REQ-30602-001: 水平方向の計算
        let horizontal_dir = calculate_horizontal_direction(event.direction);

        // REQ-30602-002, REQ-30602-003: ジャンプショット判定と速度決定
        // REQ-30603-001: ジャンプ判定（Position.Y > JumpThreshold）
        // REQ-30603-002: ジャンプショット速度増加（power_shot_speed）
        // REQ-30603-003: ジャンプショット角度変化（jump_shot_angle）
        let is_jump_shot = event.jump_height > config.shot.jump_threshold;
        let (speed, angle_deg) = if is_jump_shot {
            (config.ball.power_shot_speed, config.shot.jump_shot_angle)
        } else {
            (config.ball.normal_shot_speed, config.shot.normal_shot_angle)
        };

        // REQ-30602-004: 打球ベクトルの計算
        let shot_velocity = calculate_shot_velocity(horizontal_dir, speed, angle_deg);

        // REQ-30602-005: ボール速度の設定
        velocity.value = shot_velocity;

        // バウンスカウントをリセット（新しいショット開始）
        bounce_count.reset();

        // REQ-30602-007: ShotExecutedEvent の発行
        shot_executed_writer.write(ShotExecutedEvent {
            player_id: event.player_id,
            shot_velocity,
            is_jump_shot,
        });

        info!(
            "Player {} shot executed: velocity={:?}, is_jump_shot={}",
            event.player_id, shot_velocity, is_jump_shot
        );
    }
}

/// 水平方向を計算
/// @spec 30602_shot_direction_spec.md#req-30602-001
#[inline]
fn calculate_horizontal_direction(direction: Vec2) -> Vec3 {
    // Vec2(x, y) -> Vec3(x, 0, y) で XZ平面に変換
    // y は Z軸方向（奥行き）に対応
    if direction.length_squared() > f32::EPSILON {
        Vec3::new(direction.x, 0.0, direction.y).normalize()
    } else {
        // 入力がない場合: 相手コート方向（Z軸正方向）
        // Note: shot_input.rs で既にデフォルト方向が設定されているが、
        // 万が一のフォールバック
        Vec3::new(0.0, 0.0, 1.0)
    }
}

/// 打球ベクトルを計算
/// @spec 30602_shot_direction_spec.md#req-30602-004
#[inline]
fn calculate_shot_velocity(horizontal_dir: Vec3, speed: f32, angle_deg: f32) -> Vec3 {
    let angle_rad = angle_deg.to_radians();
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();

    Vec3::new(
        horizontal_dir.x * speed * cos_angle,
        speed * sin_angle,
        horizontal_dir.z * speed * cos_angle,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30604-007: 水平方向計算テスト
    #[test]
    fn test_calculate_horizontal_direction_forward() {
        // 前方向入力
        let direction = Vec2::new(0.0, 1.0);
        let result = calculate_horizontal_direction(direction);

        assert!((result.x - 0.0).abs() < 0.001);
        assert!((result.y - 0.0).abs() < 0.001);
        assert!((result.z - 1.0).abs() < 0.001);
    }

    /// TST-30604-007: 水平方向計算テスト（斜め入力）
    #[test]
    fn test_calculate_horizontal_direction_diagonal() {
        // 右前入力
        let direction = Vec2::new(1.0, 1.0).normalize();
        let result = calculate_horizontal_direction(direction);

        let expected = 1.0 / 2.0_f32.sqrt();
        assert!((result.x - expected).abs() < 0.001);
        assert!((result.y - 0.0).abs() < 0.001);
        assert!((result.z - expected).abs() < 0.001);
    }

    /// TST-30604-007: 水平方向計算テスト（入力なし）
    #[test]
    fn test_calculate_horizontal_direction_no_input() {
        // 入力なし -> デフォルトで前方向
        let direction = Vec2::ZERO;
        let result = calculate_horizontal_direction(direction);

        assert!((result.x - 0.0).abs() < 0.001);
        assert!((result.y - 0.0).abs() < 0.001);
        assert!((result.z - 1.0).abs() < 0.001);
    }

    /// TST-30604-008: 通常ショット速度テスト
    #[test]
    fn test_calculate_shot_velocity_normal() {
        let horizontal_dir = Vec3::new(0.0, 0.0, 1.0);
        let speed = 10.0;
        let angle_deg = 45.0;

        let result = calculate_shot_velocity(horizontal_dir, speed, angle_deg);

        // 45度の場合: cos(45) = sin(45) ≈ 0.707
        let expected_horizontal = speed * 45.0_f32.to_radians().cos();
        let expected_vertical = speed * 45.0_f32.to_radians().sin();

        assert!((result.x - 0.0).abs() < 0.001);
        assert!((result.y - expected_vertical).abs() < 0.001);
        assert!((result.z - expected_horizontal).abs() < 0.001);
    }

    /// TST-30604-009: ジャンプショット速度テスト
    #[test]
    fn test_calculate_shot_velocity_jump_shot() {
        let horizontal_dir = Vec3::new(0.0, 0.0, 1.0);
        let speed = 15.0;
        let angle_deg = 30.0;

        let result = calculate_shot_velocity(horizontal_dir, speed, angle_deg);

        // 30度の場合
        let expected_horizontal = speed * 30.0_f32.to_radians().cos();
        let expected_vertical = speed * 30.0_f32.to_radians().sin();

        assert!((result.x - 0.0).abs() < 0.001);
        assert!((result.y - expected_vertical).abs() < 0.001);
        assert!((result.z - expected_horizontal).abs() < 0.001);
    }

    /// TST-30604-010: 斜め打球ベクトルテスト
    #[test]
    fn test_calculate_shot_velocity_diagonal() {
        let horizontal_dir = Vec3::new(1.0, 0.0, 1.0).normalize();
        let speed = 10.0;
        let angle_deg = 45.0;

        let result = calculate_shot_velocity(horizontal_dir, speed, angle_deg);

        let cos_angle = 45.0_f32.to_radians().cos();
        let sin_angle = 45.0_f32.to_radians().sin();
        let horizontal_component = horizontal_dir.x * speed * cos_angle;

        // X と Z は同じ値（45度方向）
        assert!((result.x - horizontal_component).abs() < 0.001);
        assert!((result.y - speed * sin_angle).abs() < 0.001);
        assert!((result.z - horizontal_component).abs() < 0.001);
    }
}
