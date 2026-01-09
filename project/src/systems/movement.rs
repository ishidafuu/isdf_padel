//! プレイヤー移動システム
//! @spec 30201_movement_spec.md
//! @spec 20006_input_system.md
//! @spec 30102_serve_spec.md

use bevy::prelude::*;

use crate::components::{AiController, InputState, KnockbackState, LogicalPosition, Player, Velocity};
use crate::core::events::PlayerMoveEvent;
use crate::resource::config::GameConfig;
use crate::resource::scoring::{MatchFlowState, ServeState, ServeSubPhase};
use crate::resource::MatchScore;

/// プレイヤー移動システム
/// @spec 30201_movement_spec.md#req-30201-001
/// @spec 30201_movement_spec.md#req-30201-002
/// @spec 30201_movement_spec.md#req-30201-003
/// @spec 30201_movement_spec.md#req-30201-005
/// @spec 30201_movement_spec.md#req-30201-006
/// @spec 30102_serve_spec.md#req-30102-085
/// @spec 30102_serve_spec.md#req-30102-086
/// @spec 20006_input_system.md - InputState 対応
/// NOTE: B30201-002 で境界制限(REQ-30201-004)を削除（コート外移動許可）
pub fn movement_system(
    time: Res<Time>,
    config: Res<GameConfig>,
    match_state: Res<State<MatchFlowState>>,
    match_score: Res<MatchScore>,
    serve_state: Res<ServeState>,
    mut query: Query<
        (&Player, &mut LogicalPosition, &mut Velocity, &KnockbackState, &InputState),
        Without<AiController>,
    >,
    mut event_writer: MessageWriter<PlayerMoveEvent>,
) {
    let delta = time.delta_secs();
    let is_serve_state = *match_state.get() == MatchFlowState::Serve;

    for (player, mut logical_pos, mut velocity, knockback, input_state) in query.iter_mut() {
        // REQ-30201-005: ふっとばし中は入力を無視
        if knockback.is_knockback_active() {
            continue;
        }

        // @spec 30102_serve_spec.md#req-30102-085: トス中は完全に移動禁止
        let is_server = player.court_side == match_score.server;
        if is_serve_state && is_server && serve_state.phase == ServeSubPhase::Tossing {
            velocity.value.x = 0.0;
            velocity.value.z = 0.0;
            continue;
        }

        // InputState から入力を取得
        let raw_input = input_state.movement;

        // 入力がない場合は水平速度のみ0にする（Y成分は保持：ジャンプ対応）
        if raw_input == Vec2::ZERO {
            velocity.value.x = 0.0;
            velocity.value.z = 0.0;
            continue;
        }

        // REQ-30201-003: 斜め移動の正規化
        let normalization_threshold = config.input.normalization_threshold;
        let normalized_input = if raw_input.length() > normalization_threshold {
            raw_input.normalize()
        } else {
            raw_input
        };

        // REQ-30201-001, REQ-30201-002: 移動速度計算
        let move_speed_x = config.player.move_speed;
        let move_speed_z = config.player.move_speed_z;

        // @spec 30102_serve_spec.md#req-30102-086: ベースライン制限（サーブ待機中のサーバーはX方向移動禁止）
        let x_movement_allowed = !(is_serve_state && is_server && serve_state.phase == ServeSubPhase::Waiting);

        // 速度ベクトル計算
        let target_velocity = Vec3::new(
            if x_movement_allowed { normalized_input.x * move_speed_x } else { 0.0 },
            0.0, // Y軸は移動システムでは操作しない
            normalized_input.y * move_speed_z,
        );

        // REQ-30201-003: 最大速度制限
        let max_speed = config.player.max_speed;
        let horizontal_speed = Vec2::new(target_velocity.x, target_velocity.z).length();
        let final_velocity = if horizontal_speed > max_speed {
            let scale = max_speed / horizontal_speed;
            Vec3::new(target_velocity.x * scale, 0.0, target_velocity.z * scale)
        } else {
            target_velocity
        };

        // 水平速度のみ設定（Y成分は保持：ジャンプ対応）
        velocity.value.x = final_velocity.x;
        velocity.value.z = final_velocity.z;

        // 位置更新（論理座標を操作）
        // NOTE: B30201-002 で境界制限を削除（コート外移動許可）
        let old_position = logical_pos.value;
        let mut new_position = old_position + final_velocity * delta;

        // @spec 30102_serve_spec.md#req-30102-086: サーブ待機中のサーバーはX座標をベースラインに固定
        if is_serve_state && is_server && serve_state.phase == ServeSubPhase::Waiting {
            let baseline_x = match player.court_side {
                crate::core::CourtSide::Player1 => config.serve.serve_baseline_x_p1,
                crate::core::CourtSide::Player2 => config.serve.serve_baseline_x_p2,
            };
            new_position.x = baseline_x;
        }

        logical_pos.value = new_position;

        // REQ-30201-006: 位置が変化した場合のみイベント発行
        if new_position != old_position {
            event_writer.write(PlayerMoveEvent {
                player_id: player.id,
                position: new_position,
                velocity: final_velocity,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30204-003: 斜め移動の正規化テスト
    #[test]
    fn test_diagonal_normalization() {
        let input = Vec2::new(1.0, 1.0);
        let normalized = if input.length() > 1.0 {
            input.normalize()
        } else {
            input
        };

        // 長さが約1.0になることを確認
        assert!((normalized.length() - 1.0).abs() < 0.001);
    }

    // NOTE: B30201-002 で境界クランプテストを削除（コート外移動許可）
}
