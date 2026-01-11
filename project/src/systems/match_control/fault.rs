//! フォールト判定システム
//! @spec 30902_fault_spec.md
//!
//! サーブ時のフォールト判定（サービスボックス外、ダブルフォルト）を行う。

use bevy::prelude::*;

use crate::core::events::{
    DoubleFaultEvent, FaultEvent, FaultReason, GroundBounceEvent, RallyEndEvent, RallyEndReason,
};
use crate::core::CourtSide;
use crate::resource::config::{GameConfig, ServeSide};
use crate::resource::{MatchFlowState, PointEndTimer, RallyPhase, RallyState};

/// フォールト判定プラグイン
/// @spec 30902_fault_spec.md
pub struct FaultJudgmentPlugin;

impl Plugin for FaultJudgmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FaultEvent>()
            .add_message::<DoubleFaultEvent>()
            .add_systems(
                Update,
                (
                    serve_landing_judgment_system,
                    fault_processing_system,
                    double_fault_processing_system,
                )
                    .chain(),
            );
    }
}

/// サービスボックス境界
/// @spec 30902_fault_spec.md#req-30902-001
#[derive(Debug, Clone, Copy)]
pub struct ServiceBox {
    /// X軸の最小値（ネット側、打ち合い方向）
    pub x_min: f32,
    /// X軸の最大値（サービスライン側、打ち合い方向）
    pub x_max: f32,
    /// Z軸の最小値（コート幅方向）
    pub z_min: f32,
    /// Z軸の最大値（コート幅方向）
    pub z_max: f32,
}

impl ServiceBox {
    /// 指定位置がサービスボックス内かどうかを判定
    /// @spec 30902_fault_spec.md#req-30902-001
    #[inline]
    pub fn contains(&self, x: f32, z: f32) -> bool {
        x >= self.x_min && x <= self.x_max && z >= self.z_min && z <= self.z_max
    }
}

/// サーバー側とサーブサイドからサービスボックスを取得
/// @spec 30902_fault_spec.md#req-30902-001
///
/// 論理座標系: X=打ち合い方向, Y=高さ, Z=コート幅
///
/// テニスルール: サーブはクロス（対角線）に打つ
/// - サーバーがデュースサイド（Z > 0）からサーブ → 相手のデュースサイドサービスボックス（Z < 0）
/// - サーバーがアドサイド（Z < 0）からサーブ → 相手のアドサイドサービスボックス（Z > 0）
///
/// 結果: サービスボックスのZ座標はサーバーのZ座標と逆符号になる
pub fn get_service_box(
    server: CourtSide,
    serve_side: ServeSide,
    config: &GameConfig,
) -> ServiceBox {
    let half_width = config.court.width / 2.0;
    let net_x = config.court.net_x;
    let service_box_depth = config.court.service_box_depth;

    // X座標: サーバー側のみに依存
    // Left(1P)がサーブ → 2Pコート側（X > 0）
    // Right(2P)がサーブ → 1Pコート側（X < 0）
    let (x_min, x_max) = match server {
        CourtSide::Left => (net_x, net_x + service_box_depth),
        CourtSide::Right => (net_x - service_box_depth, net_x),
    };

    // Z座標: クロスサーブの法則
    // (Left, Deuce) と (Right, Ad) → Z < 0 側
    // (Left, Ad) と (Right, Deuce) → Z > 0 側
    let is_negative_z_side =
        (server == CourtSide::Left) == (serve_side == ServeSide::Deuce);

    let (z_min, z_max) = if is_negative_z_side {
        (-half_width, 0.0)
    } else {
        (0.0, half_width)
    };

    ServiceBox {
        x_min,
        x_max,
        z_min,
        z_max,
    }
}

/// サーブ着地判定システム
/// @spec 30902_fault_spec.md#req-30902-001
///
/// サーブ中（Serving状態）にボールが最初に着地した位置を判定し、
/// サービスボックス外であればフォールトイベントを発行する。
/// サービスボックス内であればラリーフェーズに遷移する。
pub fn serve_landing_judgment_system(
    mut bounce_events: MessageReader<GroundBounceEvent>,
    mut rally_state: ResMut<RallyState>,
    mut next_state: ResMut<NextState<crate::resource::MatchFlowState>>,
    config: Res<GameConfig>,
    mut fault_events: MessageWriter<FaultEvent>,
) {
    // サーブ中でなければスキップ
    if rally_state.phase != RallyPhase::Serving {
        return;
    }

    for event in bounce_events.read() {
        // ボールの着地位置を取得
        let ball_pos = event.bounce_point;

        // サービスボックスを取得
        let service_box = get_service_box(rally_state.server, rally_state.serve_side, &config);

        // @spec 30902_fault_spec.md#req-30902-001: サービスボックス判定
        if !service_box.contains(ball_pos.x, ball_pos.z) {
            // サービスボックス外 → フォールト
            let new_fault_count = rally_state.fault_count + 1;

            info!(
                "Fault! Ball landed at ({:.2}, {:.2}) outside service box. Fault count: {}",
                ball_pos.x, ball_pos.z, new_fault_count
            );

            fault_events.write(FaultEvent {
                server: rally_state.server,
                fault_count: new_fault_count,
                reason: FaultReason::OutOfServiceBox,
            });
        } else {
            // サービスボックス内 → 有効なサーブ（ラリー開始）
            info!(
                "Valid serve! Ball landed at ({:.2}, {:.2}) inside service box. Starting rally.",
                ball_pos.x, ball_pos.z
            );

            // @spec 30101_flow_spec.md#req-30101-002: ラリーフェーズに遷移
            rally_state.start_rally();
            next_state.set(crate::resource::MatchFlowState::Rally);
        }
    }
}

/// フォールト処理システム
/// @spec 30902_fault_spec.md#req-30902-003
///
/// FaultEventを受信してfault_countを更新し、
/// ダブルフォルトの場合はDoubleFaultEventを発行する。
/// 1回目のフォールトの場合はPointEnd経由でディレイ後に次のサーブに戻る。
pub fn fault_processing_system(
    mut fault_events: MessageReader<FaultEvent>,
    mut rally_state: ResMut<RallyState>,
    mut double_fault_events: MessageWriter<DoubleFaultEvent>,
    mut next_state: ResMut<NextState<MatchFlowState>>,
    mut point_end_timer: ResMut<PointEndTimer>,
    config: Res<GameConfig>,
) {
    for event in fault_events.read() {
        // @spec 30902_fault_spec.md#req-30902-003: Faultカウンタを更新
        rally_state.record_fault();

        info!(
            "Fault recorded. Server: {:?}, Fault count: {}",
            event.server, rally_state.fault_count
        );

        // ボール削除は PointEnd で行う

        // @spec 30902_fault_spec.md#req-30902-002: ダブルフォルト判定
        if rally_state.is_double_fault() {
            info!("Double fault! Server {:?} loses the point.", event.server);

            double_fault_events.write(DoubleFaultEvent {
                server: event.server,
            });
        } else {
            // 1回目のフォールト → PointEnd 経由でディレイ後に次のサーブへ
            point_end_timer.remaining = config.scoring.point_end_delay;
            point_end_timer.is_fault_delay = true;
            next_state.set(MatchFlowState::PointEnd);
            info!(
                "First fault. Entering PointEnd for delay. Server: {:?}, Fault count: {}",
                event.server, rally_state.fault_count
            );
        }
    }
}

/// ダブルフォルト処理システム
/// @spec 30902_fault_spec.md#req-30902-002
///
/// DoubleFaultEventを受信してレシーバーにポイントを与え、
/// PointEnd状態に遷移する。
pub fn double_fault_processing_system(
    mut double_fault_events: MessageReader<DoubleFaultEvent>,
    mut rally_state: ResMut<RallyState>,
    mut next_state: ResMut<NextState<crate::resource::MatchFlowState>>,
    mut rally_events: MessageWriter<RallyEndEvent>,
) {
    for event in double_fault_events.read() {
        // @spec 30902_fault_spec.md#req-30902-002: レシーバーがポイントを獲得
        let winner = event.server.opponent();

        info!(
            "Double fault! {:?} wins the point.",
            winner
        );

        // ポイント終了処理
        rally_state.end_point();

        // RallyEndEventを発行してスコア処理に移行
        rally_events.write(RallyEndEvent {
            winner,
            reason: RallyEndReason::DoubleFault,
        });

        // MatchFlowState::PointEndに遷移
        next_state.set(crate::resource::MatchFlowState::PointEnd);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> GameConfig {
        GameConfig {
            physics: crate::resource::config::PhysicsConfig {
                gravity: -9.8,
                max_fall_speed: -20.0,
            },
            court: crate::resource::config::CourtConfig {
                width: 10.0,
                depth: 6.0,
                ceiling_height: 5.0,
                max_jump_height: 5.0,
                net_height: 1.0,
                net_x: 0.0,
                service_box_depth: 1.5,
                outer_wall_z: 8.0,
                outer_wall_x: 10.0,
            },
            player: crate::resource::config::PlayerConfig {
                move_speed: 5.0,
                move_speed_z: 4.0,
                max_speed: 10.0,
                jump_force: 8.0,
                friction: 0.9,
                air_control_factor: 0.5,
                x_min: -3.0,
                x_max: 3.0,
            },
            ball: crate::resource::config::BallConfig {
                normal_shot_speed: 10.0,
                power_shot_speed: 15.0,
                bounce_factor: 0.8,
                radius: 0.2,
                min_bounce_velocity: 1.0,
                wall_bounce_factor: 0.8,
            },
            collision: crate::resource::config::CollisionConfig {
                character_radius: 0.5,
                z_tolerance: 0.3,
            },
            knockback: crate::resource::config::KnockbackConfig {
                enabled: true,
                duration: 0.5,
                speed_multiplier: 0.5,
                invincibility_time: 1.0,
            },
            shot: crate::resource::config::ShotConfig {
                max_distance: 1.5,
                max_height_diff: 2.0,
                cooldown_time: 0.5,
                normal_shot_angle: 45.0,
                jump_shot_angle: 30.0,
                jump_threshold: 0.5,
            },
            scoring: crate::resource::config::ScoringConfig {
                point_values: vec![0, 15, 30, 40],
                games_to_win_set: 6,
                sets_to_win_match: 1,
            },
            input: crate::resource::config::InputConfig {
                jump_buffer_time: 0.1,
                shot_buffer_time: 0.05,
                normalization_threshold: 1.0,
                input_sensitivity: 1.0,
            },
            input_keys: crate::resource::config::InputKeysConfig::default(),
            gamepad_buttons: crate::resource::config::GamepadButtonsConfig::default(),
            shadow: crate::resource::config::ShadowConfig::default(),
            shot_attributes: crate::resource::config::ShotAttributesConfig::default(),
            ai: crate::resource::config::AiConfig::default(),
            visual_feedback: crate::resource::config::VisualFeedbackConfig::default(),
            player_visual: crate::resource::config::PlayerVisualConfig::default(),
            serve: crate::resource::config::ServeConfig::default(),
            spin_physics: crate::resource::config::SpinPhysicsConfig::default(),
            trajectory: crate::resource::config::TrajectoryConfig::default(),
            character: crate::resource::config::CharacterConfig::default(),
        }
    }

    /// TST-30904-010: サービスボックス判定テスト
    /// @spec 30902_fault_spec.md#req-30902-001
    /// 新座標系: X=打ち合い方向, Z=コート幅
    /// クロスサーブ: サーバーの対角線上にサービスボックス
    #[test]
    fn test_req_30902_001_service_box_judgment() {
        let config = test_config();

        // 1Pがサーブ、デュースサイド（Z>0）→ クロスで2Pコート左半分（X>0, Z<0）
        let service_box = get_service_box(CourtSide::Left, ServeSide::Deuce, &config);
        assert_eq!(service_box.x_min, 0.0);    // ネット位置
        assert_eq!(service_box.x_max, 1.5);    // サービスライン位置
        assert_eq!(service_box.z_min, -5.0);   // コート左側（-Z）クロス
        assert_eq!(service_box.z_max, 0.0);    // コート中央

        // サービスボックス内（X=0.75, Z=-2.5）
        assert!(service_box.contains(0.75, -2.5));
        // サービスボックス外（右半分：Z > 0）
        assert!(!service_box.contains(0.75, 2.5));
        // サービスボックス外（サービスラインより奥：X > 1.5）
        assert!(!service_box.contains(2.0, -2.5));
    }

    /// TST-30904-010: サービスボックス判定テスト（アドサイド）
    /// @spec 30902_fault_spec.md#req-30902-001
    /// 新座標系: X=打ち合い方向, Z=コート幅
    /// クロスサーブ: サーバーの対角線上にサービスボックス
    #[test]
    fn test_req_30902_001_service_box_ad_side() {
        let config = test_config();

        // 1Pがサーブ、アドサイド（Z<0）→ クロスで2Pコート右半分（X>0, Z>0）
        let service_box = get_service_box(CourtSide::Left, ServeSide::Ad, &config);
        assert_eq!(service_box.x_min, 0.0);    // ネット位置
        assert_eq!(service_box.x_max, 1.5);    // サービスライン位置
        assert_eq!(service_box.z_min, 0.0);    // コート中央
        assert_eq!(service_box.z_max, 5.0);    // コート右側（+Z）クロス

        // サービスボックス内（X=0.75, Z=2.5）
        assert!(service_box.contains(0.75, 2.5));
        // サービスボックス外（左半分：Z < 0）
        assert!(!service_box.contains(0.75, -2.5));
    }

    /// TST-30904-010: サービスボックス判定テスト（2Pサーブ）
    /// @spec 30902_fault_spec.md#req-30902-001
    /// 新座標系: X=打ち合い方向, Z=コート幅
    /// クロスサーブ: サーバーの対角線上にサービスボックス
    #[test]
    fn test_req_30902_001_service_box_player2_serve() {
        let config = test_config();

        // 2Pがサーブ、デュースサイド（Z<0）→ クロスで1Pコート右半分（X<0, Z>0）
        let service_box = get_service_box(CourtSide::Right, ServeSide::Deuce, &config);
        assert_eq!(service_box.x_min, -1.5);   // サービスライン位置
        assert_eq!(service_box.x_max, 0.0);    // ネット位置
        assert_eq!(service_box.z_min, 0.0);    // コート中央
        assert_eq!(service_box.z_max, 5.0);    // コート右側（+Z）クロス

        // サービスボックス内（X=-0.75, Z=2.5）
        assert!(service_box.contains(-0.75, 2.5));
        // サービスボックス外（左半分：Z < 0）
        assert!(!service_box.contains(-0.75, -2.5));
    }

    /// TST-30904-011: ダブルフォルト判定テスト
    /// @spec 30902_fault_spec.md#req-30902-002
    #[test]
    fn test_req_30902_002_double_fault() {
        let mut rally_state = RallyState::new(CourtSide::Left);

        // 初期状態: fault_count = 0
        assert_eq!(rally_state.fault_count, 0);
        assert!(!rally_state.is_double_fault());

        // 1回目のフォールト
        rally_state.record_fault();
        assert_eq!(rally_state.fault_count, 1);
        assert!(!rally_state.is_double_fault());

        // 2回目のフォールト → ダブルフォルト
        rally_state.record_fault();
        assert_eq!(rally_state.fault_count, 2);
        assert!(rally_state.is_double_fault());
    }

    /// TST-30904-012: Faultカウンタ管理テスト
    /// @spec 30902_fault_spec.md#req-30902-003
    #[test]
    fn test_req_30902_003_fault_counter_management() {
        let mut rally_state = RallyState::new(CourtSide::Left);

        // 初期状態: fault_count = 0
        assert_eq!(rally_state.fault_count, 0);

        // フォールト記録
        rally_state.record_fault();
        assert_eq!(rally_state.fault_count, 1);

        // ポイント終了でリセット
        rally_state.end_point();
        assert_eq!(rally_state.fault_count, 0);
    }
}
