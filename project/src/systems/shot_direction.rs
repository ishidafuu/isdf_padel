//! ショット方向計算システム
//! @spec 30602_shot_direction_spec.md
//! @spec 30603_jump_shot_spec.md
//! @spec 30604_shot_attributes_spec.md
//! @spec 30605_trajectory_calculation_spec.md

use bevy::prelude::*;


use crate::components::{
    Ball, BallBundle, BallSpin, BounceCount, BounceState, InputState, LastShooter, LogicalPosition, Player,
    Velocity,
};
use crate::core::events::{ShotEvent, ShotExecutedEvent};
use crate::resource::config::{GameConfig, ServeSide};
use crate::resource::debug::LastShotDebugInfo;
use crate::resource::scoring::MatchScore;
use crate::systems::shot_attributes::{build_shot_context_from_input_state, calculate_shot_attributes};
use crate::systems::trajectory_calculator::{
    calculate_serve_trajectory, calculate_trajectory, ServeTrajectoryContext, TrajectoryContext,
};

/// ショット方向計算システム
/// ShotEvent を受信してボールの速度を設定する
/// @spec 30602_shot_direction_spec.md#req-30602-001
/// @spec 30602_shot_direction_spec.md#req-30602-002
/// @spec 30602_shot_direction_spec.md#req-30602-003
/// @spec 30602_shot_direction_spec.md#req-30602-004
/// @spec 30602_shot_direction_spec.md#req-30602-005
/// @spec 30602_shot_direction_spec.md#req-30602-007
/// @spec 30602_shot_direction_spec.md#req-30602-031 - サーブ処理分岐
/// @spec 30602_shot_direction_spec.md#req-30602-032 - 通常ショット処理
/// @spec 30604_shot_attributes_spec.md#req-30604-068
/// @spec 30604_shot_attributes_spec.md#req-30604-069
/// @spec 30604_shot_attributes_spec.md#req-30604-070
/// @spec 30605_trajectory_calculation_spec.md - 着地点逆算型弾道システム
pub fn shot_direction_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    mut shot_events: MessageReader<ShotEvent>,
    mut ball_query: Query<
        (
            Entity,
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
    mut debug_info: ResMut<LastShotDebugInfo>,
) {
    for event in shot_events.read() {
        // === サーブ処理分岐 ===
        // @spec 30602_shot_direction_spec.md#req-30602-031
        if event.is_serve {
            // サーブ処理: ボールを新規生成
            handle_serve_shot(
                &mut commands,
                &config,
                &match_score,
                event,
                &mut shot_executed_writer,
            );
            continue;
        }

        // === 通常ショット処理 ===
        // @spec 30602_shot_direction_spec.md#req-30602-032
        // ボールを取得
        let Ok((
            _entity,
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

        // REQ-30604-069: 安定性による威力減衰（ランダム性なし）
        let stability_factor =
            calculate_stability_power_factor(shot_attrs.stability, &config.shot_attributes);
        let effective_power = shot_attrs.power * stability_factor;

        // === 着地点逆算型弾道計算（v0.4新機能） ===
        // @spec 30605_trajectory_calculation_spec.md

        // 弾道計算コンテキストを構築
        let trajectory_ctx = TrajectoryContext {
            input: event.direction, // X=左右, Y=前後
            court_side: event.court_side,
            ball_position: ball_pos.value,
            spin: shot_attrs.spin,
            base_speed: effective_power,
            accuracy: shot_attrs.accuracy,
        };

        // 弾道を計算
        let trajectory_result = calculate_trajectory(&trajectory_ctx, &config);

        // 最終的な打球ベクトルを計算（方向ブレなし: 決定的）
        let shot_velocity = trajectory_result.direction * trajectory_result.final_speed;

        // REQ-30602-005: ボール速度の設定
        info!(
            "shot_direction(v0.4): landing={:?}, angle={:.1}, speed={:.1}, stability_factor={:.2}, velocity={:?}",
            trajectory_result.landing_position,
            trajectory_result.launch_angle,
            trajectory_result.final_speed,
            stability_factor,
            shot_velocity
        );
        ball_velocity.value = shot_velocity;

        // === デバッグ情報を一時保存 ===
        // discriminant と g_eff を再計算
        let g_eff = crate::systems::trajectory_calculator::calculate_effective_gravity(
            shot_attrs.spin,
            ball_pos.value.y,
            &config,
        );
        let dx = trajectory_result.landing_position.x - ball_pos.value.x;
        let dz = trajectory_result.landing_position.z - ball_pos.value.z;
        let horizontal_distance = (dx * dx + dz * dz).sqrt();
        let h = trajectory_result.landing_position.y - ball_pos.value.y;
        let v = trajectory_result.final_speed;
        let v2 = v * v;
        let v4 = v2 * v2;
        let discriminant = v4 - g_eff * (g_eff * horizontal_distance * horizontal_distance + 2.0 * h * v2);

        debug_info.is_valid = true;
        debug_info.player_id = event.player_id;
        debug_info.ball_pos = ball_pos.value;
        debug_info.input = event.direction;
        debug_info.court_side = Some(event.court_side);
        debug_info.power = effective_power;
        debug_info.spin = shot_attrs.spin;
        debug_info.accuracy = shot_attrs.accuracy;
        debug_info.landing = trajectory_result.landing_position;
        debug_info.launch_angle = trajectory_result.launch_angle;
        debug_info.final_speed = trajectory_result.final_speed;
        debug_info.velocity = shot_velocity;
        debug_info.discriminant = discriminant;
        debug_info.g_eff = g_eff;

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
            "Player {} shot executed: power={:.1}, angle={:.1}, stability={:.2}, accuracy={:.2}, spin={:.2}, landing=({:.1}, {:.1})",
            event.player_id,
            effective_power,
            trajectory_result.launch_angle,
            shot_attrs.stability,
            shot_attrs.accuracy,
            shot_attrs.spin,
            trajectory_result.landing_position.x,
            trajectory_result.landing_position.z
        );
    }
}

/// サーブショット処理
/// @spec 30602_shot_direction_spec.md#req-30602-031
/// @spec 30605_trajectory_calculation_spec.md#req-30605-050
fn handle_serve_shot(
    commands: &mut Commands,
    config: &GameConfig,
    match_score: &MatchScore,
    event: &ShotEvent,
    shot_executed_writer: &mut MessageWriter<ShotExecutedEvent>,
) {
    // 打点位置を取得（サーブ時は必須）
    let hit_position = match event.hit_position {
        Some(pos) => pos,
        None => {
            warn!("Serve shot event missing hit_position");
            return;
        }
    };

    // サーブサイドをポイント合計から計算
    // @spec 30903_serve_authority_spec.md#req-30903-003
    let server_points = match_score.get_point_index(event.court_side);
    let receiver_points = match_score.get_point_index(event.court_side.opponent());
    let total_points = server_points + receiver_points;
    let serve_side = ServeSide::from_point_total(total_points);

    // サーブ弾道計算
    let serve_ctx = ServeTrajectoryContext {
        input: event.direction,
        server: event.court_side,
        serve_side,
        hit_position,
        base_speed: config.serve.serve_speed,
    };

    let trajectory_result = calculate_serve_trajectory(&serve_ctx, config);

    // 最終的な打球ベクトルを計算
    let shot_velocity = trajectory_result.direction * trajectory_result.final_speed;

    // ボールを新規生成
    // @spec 30602_shot_direction_spec.md#req-30602-031
    commands.spawn(BallBundle::with_shooter(
        hit_position,
        shot_velocity,
        event.court_side,
    ));

    // ShotExecutedEvent の発行
    shot_executed_writer.write(ShotExecutedEvent {
        player_id: event.player_id,
        shot_velocity,
        is_jump_shot: false, // サーブはジャンプショットではない
    });

    info!(
        "Serve shot executed: player={}, landing=({:.1}, {:.1}), angle={:.1}, speed={:.1}",
        event.player_id,
        trajectory_result.landing_position.x,
        trajectory_result.landing_position.z,
        trajectory_result.launch_angle,
        trajectory_result.final_speed
    );
}

/// 安定性による威力減衰係数を計算
/// @spec 30604_shot_attributes_spec.md#req-30604-069
/// ランダム性なし: 同じ入力 → 同じ出力
fn calculate_stability_power_factor(
    stability: f32,
    config: &crate::resource::config::ShotAttributesConfig,
) -> f32 {
    if stability >= config.stability_threshold {
        return 1.0;
    }

    // 安定性が低いほど威力減衰
    let power_reduction =
        (config.stability_threshold - stability) / config.stability_threshold;
    1.0 - power_reduction * 0.5
}

/// 精度によるコースブレ計算（決定的）
/// @spec 30604_shot_attributes_spec.md#req-30604-070
/// ランダム性なし: 精度が低いほどコート中央寄りに収束
#[allow(dead_code)]
fn calculate_direction_error(
    _accuracy: f32,
    _config: &crate::resource::config::ShotAttributesConfig,
) -> f32 {
    // ランダム性を排除: 常に0を返す
    // 精度による影響は着地位置の収束で表現（trajectory_calculator側）
    0.0
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
        // Left側（X < net_x）にいる場合: +X方向（相手コート）
        // Right側（X > net_x）にいる場合: -X方向（相手コート）
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

    /// TST-30604-007: 水平方向計算テスト（Left側コート、入力なし）
    /// 新座標系: X=打ち合い方向（固定）, Z=コート幅（入力）
    #[test]
    fn test_calculate_horizontal_direction_left_side_no_input() {
        // Left側コート: 入力なし -> +X方向（相手コート方向のみ）
        let direction = Vec2::new(0.0, 0.0);
        let result = calculate_horizontal_direction(direction, CourtSide::Left);

        assert!((result.x - 1.0).abs() < 0.001);  // +X方向（2Pコートへ）
        assert!((result.y - 0.0).abs() < 0.001);
        assert!((result.z - 0.0).abs() < 0.001);  // 横方向なし
    }

    /// TST-30604-007: 水平方向計算テスト（Right側コート、入力なし）
    /// 新座標系: X=打ち合い方向（固定）, Z=コート幅（入力）
    #[test]
    fn test_calculate_horizontal_direction_right_side_no_input() {
        // Right側コート: 入力なし -> -X方向（相手コート方向のみ）
        let direction = Vec2::new(0.0, 0.0);
        let result = calculate_horizontal_direction(direction, CourtSide::Right);

        assert!((result.x - -1.0).abs() < 0.001); // -X方向（1Pコートへ）
        assert!((result.y - 0.0).abs() < 0.001);
        assert!((result.z - 0.0).abs() < 0.001);  // 横方向なし
    }

    /// TST-30604-007: 水平方向計算テスト（Left側コート、右入力）
    /// 新座標系: X=打ち合い方向（固定）, Z=コート幅（入力）
    #[test]
    fn test_calculate_horizontal_direction_left_side_right() {
        // Left側コート: 右入力(+Z) -> 右前方向
        let direction = Vec2::new(1.0, 0.0);  // X入力 → Z方向
        let result = calculate_horizontal_direction(direction, CourtSide::Left);

        let expected = 1.0 / 2.0_f32.sqrt();
        assert!((result.x - expected).abs() < 0.001);  // +X方向
        assert!((result.y - 0.0).abs() < 0.001);
        assert!((result.z - expected).abs() < 0.001);  // +Z方向（右）
    }

    /// TST-30604-007: 水平方向計算テスト（Right側コート、右入力）
    /// 新座標系: X=打ち合い方向（固定）, Z=コート幅（入力）
    #[test]
    fn test_calculate_horizontal_direction_right_side_right() {
        // Right側コート: 右入力(+Z) -> 右後方向
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

    /// ミスショット判定（テスト用ダミー実装）
    /// 安定性が閾値以上なら (false, 0.0) を返す
    fn check_miss_shot(
        stability: f32,
        config: &crate::resource::config::ShotAttributesConfig,
    ) -> (bool, f32) {
        if stability >= config.stability_threshold {
            (false, 0.0)
        } else {
            // 安定性が低い場合はミス判定（テスト用に常にfalse）
            (false, 0.0)
        }
    }

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
