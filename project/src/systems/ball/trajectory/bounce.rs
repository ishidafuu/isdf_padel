//! ボールバウンド・反射システム
//! @spec 30402_reflection_spec.md

use bevy::prelude::*;

use crate::components::{Ball, BallSpin, BallSpinExt, LogicalPosition, Velocity};
use crate::core::events::{BallOutOfBoundsEvent, GroundBounceEvent, WallReflectionEvent};
use crate::core::WallReflection;
use crate::resource::config::GameConfig;
use crate::resource::debug::LastShotDebugInfo;
use crate::systems::court_factory::create_outer_wall_bounds;

/// 地面バウンドシステム
/// @spec 30402_reflection_spec.md#req-30402-001
/// @spec 30402_reflection_spec.md#req-30402-002
/// @spec 30402_reflection_spec.md#req-30402-100
pub fn ball_ground_bounce_system(
    config: Res<GameConfig>,
    mut query: Query<
        (
            Entity,
            &mut Velocity,
            &mut LogicalPosition,
            Option<&BallSpin>,
        ),
        With<Ball>,
    >,
    mut event_writer: MessageWriter<GroundBounceEvent>,
    mut debug_info: ResMut<LastShotDebugInfo>,
) {
    let base_bounce_factor = config.ball.bounce_factor;
    let min_bounce_velocity = config.ball.min_bounce_velocity;
    let net_x = config.court.net_x;
    let h_factor = config.spin_physics.bounce_spin_horizontal_factor;
    let v_factor = config.spin_physics.bounce_spin_vertical_factor;

    for (entity, mut velocity, mut logical_pos, ball_spin) in query.iter_mut() {
        let pos = logical_pos.value;

        // REQ-30402-001: ボールが地面（Y <= 0）に接触し、下向きまたは静止中の場合
        // Y速度が0の場合もバウンドさせる（プレイヤー衝突で水平に跳ね返った場合対応）
        if pos.y <= 0.0 && velocity.value.y <= 0.0 {
            // REQ-30402-100: スピンによるバウンド挙動変化
            let spin_value = ball_spin.value_or_default();

            // 水平方向（X, Z）: velocity *= base_bounce * (1.0 + spin * h_factor)
            // トップスピン（spin > 0）: 水平維持率上昇 → 低く伸びる
            // スライス（spin < 0）: 水平維持率低下 → 高く止まる
            let horizontal_bounce = base_bounce_factor * (1.0 + spin_value * h_factor);
            velocity.value.x *= horizontal_bounce;
            velocity.value.z *= horizontal_bounce;

            // 垂直方向（Y）: velocity.y = -velocity.y * base_bounce * (1.0 - spin * v_factor)
            // トップスピン（spin > 0）: 垂直維持率低下 → 低く伸びる
            // スライス（spin < 0）: 垂直維持率上昇 → 高く止まる
            let vertical_bounce = base_bounce_factor * (1.0 - spin_value * v_factor);
            let bounced_y = -velocity.value.y * vertical_bounce;
            // 最小バウンド速度を保証（Y速度が0でも軽く跳ねる）
            velocity.value.y = bounced_y.max(min_bounce_velocity);

            // 位置を地面に補正（めり込み防止）
            logical_pos.value.y = 0.0;

            // REQ-30402-002: GroundBounceEvent 発行
            // 新座標系: X=打ち合い方向、ボールのX位置でコートサイド判定
            let court_side = crate::core::determine_court_side(pos.x, net_x);
            event_writer.write(GroundBounceEvent {
                ball: entity,
                bounce_point: Vec3::new(pos.x, 0.0, pos.z),
                court_side,
            });

            // デバッグマーカー用: バウンド後はデバッグ情報を無効化
            debug_info.is_valid = false;
        }
    }
}

/// 壁・天井反射システム（外壁接触で即アウト判定用イベント発行）
/// @spec 30503_boundary_behavior.md#beh-30503-008
/// @spec 30402_reflection_spec.md#req-30402-003
/// @spec 30402_reflection_spec.md#req-30402-004
/// @spec 30402_reflection_spec.md#req-30402-005
/// @spec 30402_reflection_spec.md#req-30402-006
/// @spec 30402_reflection_spec.md#req-30402-007
pub fn ball_wall_reflection_system(
    config: Res<GameConfig>,
    mut query: Query<(Entity, &mut Velocity, &mut LogicalPosition), With<Ball>>,
    mut event_writer: MessageWriter<WallReflectionEvent>,
) {
    // 外壁位置で判定（コートラインではなく）
    let bounds = create_outer_wall_bounds(&config.court);
    let bounce_factor = config.ball.wall_bounce_factor;

    for (entity, mut velocity, mut logical_pos) in query.iter_mut() {
        let pos = logical_pos.value;
        let vel = velocity.value;

        // 壁・天井との接触チェックと反射計算
        if let Some(result) = WallReflection::check_and_reflect(pos, vel, &bounds, bounce_factor) {
            // 速度を反射後の値に更新
            velocity.value = result.reflected_velocity;

            // REQ-30402-007: 位置を境界内に補正（めり込み防止）
            logical_pos.value.x = bounds.clamp_x(pos.x);
            logical_pos.value.y = bounds.clamp_y(pos.y);
            logical_pos.value.z = bounds.clamp_z(pos.z);

            // REQ-30402-004: WallReflectionEvent 発行
            event_writer.write(WallReflectionEvent {
                ball: entity,
                wall_type: result.wall_type,
                contact_point: result.contact_point,
                incident_velocity: vel,
                reflected_velocity: result.reflected_velocity,
            });
        }
    }
}

/// ボールアウトオブバウンズ検出システム（コート外着地判定）
/// @spec 30901_point_judgment_spec.md#req-30901-001
///
/// ボールがコート境界外に着地（Y <= 0 かつ コート外）した場合にアウトイベントを発行。
/// テニスルールでは、打ったボールがコート外に着地 = アウト（打った側の失点）。
/// 通常は壁に当たった時点でアウト（wall_hit_judgment_system）だが、
/// 壁を超えた場合のフォールバックとして機能。
pub fn ball_out_of_bounds_system(
    config: Res<GameConfig>,
    query: Query<(Entity, &LogicalPosition), With<Ball>>,
    mut event_writer: MessageWriter<BallOutOfBoundsEvent>,
) {
    let half_width = config.court.width / 2.0;
    let half_depth = config.court.depth / 2.0;

    for (entity, logical_pos) in query.iter() {
        let pos = logical_pos.value;

        // @spec 30901_point_judgment_spec.md#req-30901-001
        // ボールが地面に着地（Y <= 0）かつコート境界外の場合のみアウト
        // コート内の着地は GroundBounceEvent（ball_ground_bounce_system）で処理
        if pos.y <= 0.0 {
            // X軸は打ち合い方向（depth）、Z軸は左右幅方向（width）
            let out_of_bounds_x = pos.x.abs() > half_depth;
            let out_of_bounds_z = pos.z.abs() > half_width;

            if out_of_bounds_x || out_of_bounds_z {
                info!(
                    "Ball out of bounds at {:?} (half_width: {}, half_depth: {})",
                    pos, half_width, half_depth
                );
                event_writer.write(BallOutOfBoundsEvent {
                    ball: entity,
                    final_position: pos,
                });
            }
        }
    }
}
