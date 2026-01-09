//! ショット方向計算システム
//! @spec 30602_shot_direction_spec.md
//! @spec 30603_jump_shot_spec.md
//! @spec 30604_shot_attributes_spec.md
//! @spec 30605_trajectory_calculation_spec.md

use bevy::prelude::*;
use rand::Rng;

use crate::components::{
    Ball, BallSpin, BounceCount, BounceState, InputState, LastShooter, LogicalPosition, Player,
    Velocity,
};
use crate::core::events::{ShotEvent, ShotExecutedEvent};
use crate::resource::config::GameConfig;
use crate::systems::shot_attributes::{build_shot_context_from_input_state, calculate_shot_attributes};
use crate::systems::trajectory_calculator::{TrajectoryContext, calculate_trajectory};

/// ショット方向計算システム
/// ShotEvent を受信してボールの速度を設定する
/// @spec 30602_shot_direction_spec.md#req-30602-001
/// @spec 30602_shot_direction_spec.md#req-30602-002
/// @spec 30602_shot_direction_spec.md#req-30602-003
/// @spec 30602_shot_direction_spec.md#req-30602-004
/// @spec 30602_shot_direction_spec.md#req-30602-005
/// @spec 30602_shot_direction_spec.md#req-30602-007
/// @spec 30604_shot_attributes_spec.md#req-30604-068
/// @spec 30604_shot_attributes_spec.md#req-30604-069
/// @spec 30604_shot_attributes_spec.md#req-30604-070
/// @spec 30605_trajectory_calculation_spec.md - 着地点逆算型弾道システム
pub fn shot_direction_system(
    config: Res<GameConfig>,
    mut shot_events: MessageReader<ShotEvent>,
    mut ball_query: Query<
        (
            &mut Velocity,
            &mut BounceCount,
            &mut LastShooter,
            &LogicalPosition,
            &BounceState,
            &mut BallSpin,
        ),
        With<Ball>,
    >,
    player_query: Query<(&Player, &LogicalPosition, &Velocity, &InputState), Without<Ball>>,
    mut shot_executed_writer: MessageWriter<ShotExecutedEvent>,
) {
    for event in shot_events.read() {
        // ボールを取得
        let Ok((
            mut ball_velocity,
            mut bounce_count,
            mut last_shooter,
            ball_pos,
            bounce_state,
            mut ball_spin,
        )) = ball_query.single_mut()
        else {
            warn!("No ball found for shot direction calculation");
            continue;
        };

        // 最後にショットを打ったプレイヤーを記録（自己衝突回避のため）
        last_shooter.record(event.court_side);

        // プレイヤー情報を取得（InputState も含む）
        let player_info = player_query
            .iter()
            .find(|(p, _, _, _)| p.id == event.player_id);
        let (player_pos, player_velocity, hold_time) = match player_info {
            Some((_, pos, vel, input_state)) => (pos.value, vel.value, input_state.hold_time),
            None => {
                warn!("Player {} not found", event.player_id);
                continue;
            }
        };

        // === ショット属性計算（v0.2新機能） ===
        // @spec 30604_shot_attributes_spec.md
        // @spec 20006_input_system.md - InputState 対応
        let shot_context = build_shot_context_from_input_state(
            hold_time,
            player_pos,
            player_velocity,
            ball_pos.value,
            bounce_state,
            &config.shot_attributes,
        );

        let shot_attrs = calculate_shot_attributes(&shot_context, &config.shot_attributes);

        // REQ-30604-069: 安定性によるミスショット判定
        let (is_miss_shot, miss_direction_offset) =
            check_miss_shot(shot_attrs.stability, &config.shot_attributes);

        // === 着地点逆算型弾道計算（v0.4新機能） ===
        // @spec 30605_trajectory_calculation_spec.md

        // 精度の計算（ミスショット時は精度を下げる）
        let effective_accuracy = if is_miss_shot {
            shot_attrs.accuracy * 0.3 // ミスショット時は精度30%
        } else {
            shot_attrs.accuracy
        };

        // 弾道計算コンテキストを構築
        let trajectory_ctx = TrajectoryContext {
            input: event.direction, // X=左右, Y=前後
            court_side: event.court_side,
            ball_position: ball_pos.value,
            spin: shot_attrs.spin,
            base_speed: shot_attrs.power,
            accuracy: effective_accuracy,
        };

        // 弾道を計算
        let trajectory_result = calculate_trajectory(&trajectory_ctx, &config);

        // REQ-30604-070: 精度によるコースブレ（方向に追加のブレを適用）
        let direction_error =
            calculate_direction_error(shot_attrs.accuracy, &config.shot_attributes);

        // ミスショット時は大きなブレを追加
        let total_direction_offset = if is_miss_shot {
            miss_direction_offset
        } else {
            direction_error
        };

        // 方向にブレを適用
        let adjusted_direction =
            apply_direction_offset(trajectory_result.direction, total_direction_offset);

        // 最終的な打球ベクトルを計算
        let shot_velocity = adjusted_direction * trajectory_result.final_speed;

        // REQ-30602-005: ボール速度の設定
        info!(
            "shot_direction(v0.4): landing={:?}, angle={:.1}, speed={:.1}, miss={}, velocity={:?}",
            trajectory_result.landing_position,
            trajectory_result.launch_angle,
            trajectory_result.final_speed,
            is_miss_shot,
            shot_velocity
        );
        ball_velocity.value = shot_velocity;

        // バウンスカウントをリセット（新しいショット開始）
        bounce_count.reset();

        // REQ-30802-004: スピン属性をボールに設定
        ball_spin.value = shot_attrs.spin;

        // REQ-30603-001: ジャンプショット判定（ログ用）
        let is_jump_shot = event.jump_height > config.shot.jump_threshold;

        // REQ-30602-007: ShotExecutedEvent の発行
        shot_executed_writer.write(ShotExecutedEvent {
            player_id: event.player_id,
            shot_velocity,
            is_jump_shot,
        });

        info!(
            "Player {} shot executed: power={:.1}, angle={:.1}, stability={:.2}, accuracy={:.2}, spin={:.2}, miss={}, landing=({:.1}, {:.1})",
            event.player_id,
            shot_attrs.power,
            trajectory_result.launch_angle,
            shot_attrs.stability,
            shot_attrs.accuracy,
            shot_attrs.spin,
            is_miss_shot,
            trajectory_result.landing_position.x,
            trajectory_result.landing_position.z
        );
    }
}

/// ミスショット判定
/// @spec 30604_shot_attributes_spec.md#req-30604-069
fn check_miss_shot(
    stability: f32,
    config: &crate::resource::config::ShotAttributesConfig,
) -> (bool, f32) {
    if stability >= config.stability_threshold {
        return (false, 0.0);
    }

    // ミス確率 = (閾値 - 安定性) / 閾値
    let miss_probability =
        (config.stability_threshold - stability) / config.stability_threshold;

    let mut rng = rand::rng();
    if rng.random::<f32>() < miss_probability {
        // ミスショット：ランダムな大きなブレを追加
        let miss_offset = rng.random_range(-45.0..45.0_f32);
        (true, miss_offset)
    } else {
        (false, 0.0)
    }
}

/// 精度によるコースブレ計算
/// @spec 30604_shot_attributes_spec.md#req-30604-070
fn calculate_direction_error(
    accuracy: f32,
    config: &crate::resource::config::ShotAttributesConfig,
) -> f32 {
    // direction_error = (1.0 - accuracy) × max_direction_error × random(-1, 1)
    let mut rng = rand::rng();
    let random_factor = rng.random_range(-1.0..1.0_f32);
    (1.0 - accuracy.clamp(0.0, 1.0)) * config.max_direction_error * random_factor
}

/// 方向にオフセットを適用
fn apply_direction_offset(horizontal_dir: Vec3, offset_deg: f32) -> Vec3 {
    if offset_deg.abs() < f32::EPSILON {
        return horizontal_dir;
    }

    // XZ平面での回転
    let offset_rad = offset_deg.to_radians();
    let cos_offset = offset_rad.cos();
    let sin_offset = offset_rad.sin();

    let new_x = horizontal_dir.x * cos_offset - horizontal_dir.z * sin_offset;
    let new_z = horizontal_dir.x * sin_offset + horizontal_dir.z * cos_offset;

    Vec3::new(new_x, horizontal_dir.y, new_z).normalize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::CourtSide;

    /// 水平方向を計算（テスト用）
    /// @spec 30602_shot_direction_spec.md#req-30602-001
    /// 新座標系: X=打ち合い方向, Z=コート幅
    /// X軸方向（打ち合い）: コートサイドに応じて常に相手コート方向に固定
    /// Z軸方向（左右）: 入力で調整可能
    #[inline]
    fn calculate_horizontal_direction(direction: Vec2, court_side: CourtSide) -> Vec3 {
        // X軸方向: コートサイドに応じて固定（常に相手コートへ）
        // Player1側（X < net_x）にいる場合: +X方向（相手コート）
        // Player2側（X > net_x）にいる場合: -X方向（相手コート）
        let x_direction = match court_side {
            CourtSide::Left => 1.0,
            CourtSide::Right => -1.0,
        };

        // Z軸方向: 入力X値を使用（コート幅方向の打ち分け）
        // shot_input.rs から direction.x に横入力（左右）が入る
        let z_direction = direction.x;

        // 正規化して返す
        Vec3::new(x_direction, 0.0, z_direction).normalize()
    }

    /// 打球ベクトルを計算（テスト用）
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

    /// TST-30604-007: 水平方向計算テスト（Player1側コート、入力なし）
    /// 新座標系: X=打ち合い方向（固定）, Z=コート幅（入力）
    #[test]
    fn test_calculate_horizontal_direction_player1_side_no_input() {
        // Player1側コート: 入力なし -> +X方向（相手コート方向のみ）
        let direction = Vec2::new(0.0, 0.0);
        let result = calculate_horizontal_direction(direction, CourtSide::Left);

        assert!((result.x - 1.0).abs() < 0.001);  // +X方向（2Pコートへ）
        assert!((result.y - 0.0).abs() < 0.001);
        assert!((result.z - 0.0).abs() < 0.001);  // 横方向なし
    }

    /// TST-30604-007: 水平方向計算テスト（Player2側コート、入力なし）
    /// 新座標系: X=打ち合い方向（固定）, Z=コート幅（入力）
    #[test]
    fn test_calculate_horizontal_direction_player2_side_no_input() {
        // Player2側コート: 入力なし -> -X方向（相手コート方向のみ）
        let direction = Vec2::new(0.0, 0.0);
        let result = calculate_horizontal_direction(direction, CourtSide::Right);

        assert!((result.x - -1.0).abs() < 0.001); // -X方向（1Pコートへ）
        assert!((result.y - 0.0).abs() < 0.001);
        assert!((result.z - 0.0).abs() < 0.001);  // 横方向なし
    }

    /// TST-30604-007: 水平方向計算テスト（Player1側コート、右入力）
    /// 新座標系: X=打ち合い方向（固定）, Z=コート幅（入力）
    #[test]
    fn test_calculate_horizontal_direction_player1_side_right() {
        // Player1側コート: 右入力(+Z) -> 右前方向
        let direction = Vec2::new(1.0, 0.0);  // X入力 → Z方向
        let result = calculate_horizontal_direction(direction, CourtSide::Left);

        let expected = 1.0 / 2.0_f32.sqrt();
        assert!((result.x - expected).abs() < 0.001);  // +X方向
        assert!((result.y - 0.0).abs() < 0.001);
        assert!((result.z - expected).abs() < 0.001);  // +Z方向（右）
    }

    /// TST-30604-007: 水平方向計算テスト（Player2側コート、右入力）
    /// 新座標系: X=打ち合い方向（固定）, Z=コート幅（入力）
    #[test]
    fn test_calculate_horizontal_direction_player2_side_right() {
        // Player2側コート: 右入力(+Z) -> 右後方向
        let direction = Vec2::new(1.0, 0.0);  // X入力 → Z方向
        let result = calculate_horizontal_direction(direction, CourtSide::Right);

        let expected = 1.0 / 2.0_f32.sqrt();
        assert!((result.x - -expected).abs() < 0.001); // -X方向（1Pコートへ）
        assert!((result.y - 0.0).abs() < 0.001);
        assert!((result.z - expected).abs() < 0.001);  // +Z方向（右）
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

    // ========================================================================
    // ショット属性の軌道反映テスト（v0.2 新機能）
    // @spec 30604_shot_attributes_spec.md
    // ========================================================================

    /// TST-30604-068: ミスショット判定テスト（安定性が閾値以上）
    /// @spec 30604_shot_attributes_spec.md#req-30604-069
    #[test]
    fn test_check_miss_shot_stable() {
        let config = crate::resource::config::ShotAttributesConfig::default();
        let stability = 1.0; // 閾値(0.3)以上

        let (is_miss, _offset) = check_miss_shot(stability, &config);
        assert!(!is_miss, "高い安定性はミスショットを発生させない");
    }

    /// TST-30604-069: 方向オフセット適用テスト（オフセットなし）
    #[test]
    fn test_apply_direction_offset_zero() {
        let dir = Vec3::new(0.0, 0.0, 1.0);
        let result = apply_direction_offset(dir, 0.0);

        assert!((result.x - dir.x).abs() < 0.001);
        assert!((result.z - dir.z).abs() < 0.001);
    }

    /// TST-30604-070: 方向オフセット適用テスト（90度回転）
    #[test]
    fn test_apply_direction_offset_90_degrees() {
        let dir = Vec3::new(0.0, 0.0, 1.0);
        let result = apply_direction_offset(dir, 90.0);

        // Z方向から反時計回りに90度回転 → -X方向
        // (標準的な2D回転行列: 正の角度 = 反時計回り)
        assert!((result.x - (-1.0)).abs() < 0.001);
        assert!((result.z - 0.0).abs() < 0.001);
    }

    /// TST-30604-071: 方向オフセット適用テスト（-45度回転）
    #[test]
    fn test_apply_direction_offset_minus_45_degrees() {
        let dir = Vec3::new(0.0, 0.0, 1.0);
        let result = apply_direction_offset(dir, -45.0);

        let expected = 1.0 / 2.0_f32.sqrt();
        // -45度回転（時計回り）で右前方向
        assert!((result.x - expected).abs() < 0.001);
        assert!((result.z - expected).abs() < 0.001);
    }

    /// TST-30604-072: 精度によるコースブレ範囲テスト
    /// @spec 30604_shot_attributes_spec.md#req-30604-070
    #[test]
    fn test_calculate_direction_error_perfect_accuracy() {
        let config = crate::resource::config::ShotAttributesConfig::default();
        let accuracy = 1.0; // 完璧な精度

        // 精度1.0では(1.0 - 1.0) * max_error * random = 0
        let error = calculate_direction_error(accuracy, &config);
        assert!(
            error.abs() < 0.001,
            "完璧な精度ではコースブレが発生しない"
        );
    }
}
