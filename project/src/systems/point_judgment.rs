//! ポイント判定システム
//! @spec 30901_point_judgment_spec.md
//!
//! 得点条件（ツーバウンド、アウト、レット）を判定する。

use bevy::prelude::*;

use crate::components::{Ball, BounceCount};
use crate::core::events::{BallOutOfBoundsEvent, GroundBounceEvent, NetHitEvent, RallyEndEvent, RallyEndReason};
use crate::core::CourtSide;
use crate::resource::config::GameConfig;
use crate::resource::{GameState, MatchScore, RallyState, RallyPhase};

/// ポイント判定プラグイン
/// @spec 30901_point_judgment_spec.md
pub struct PointJudgmentPlugin;

impl Plugin for PointJudgmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RallyState>()
            .add_systems(
                Update,
                (
                    bounce_count_update_system,
                    double_bounce_judgment_system,
                    out_of_bounds_judgment_system,
                    let_judgment_system,
                )
                    .chain(),
            );
    }
}

/// バウンス回数更新システム
/// @spec 30901_point_judgment_spec.md#req-30901-002
/// GroundBounceEvent を受信して BounceCount を更新
pub fn bounce_count_update_system(
    mut bounce_events: MessageReader<GroundBounceEvent>,
    mut query: Query<&mut BounceCount, With<Ball>>,
) {
    for event in bounce_events.read() {
        if let Ok(mut bounce_count) = query.get_mut(event.ball) {
            // @spec 30901_point_judgment_spec.md#req-30901-002
            // バウンドしたコート側を記録
            bounce_count.record_bounce(event.court_side);

            info!(
                "Ball bounced on {:?} court, count: {}",
                event.court_side, bounce_count.count
            );
        }
    }
}

/// ツーバウンド判定システム
/// @spec 30901_point_judgment_spec.md#req-30901-002
/// BounceCount >= 2 でラリー終了（該当プレイヤーが失点）
pub fn double_bounce_judgment_system(
    query: Query<(Entity, &BounceCount), With<Ball>>,
    match_score: Res<MatchScore>,
    mut rally_events: MessageWriter<RallyEndEvent>,
) {
    // ゲーム進行中でなければスキップ
    if match_score.game_state != GameState::Playing {
        return;
    }

    for (_entity, bounce_count) in query.iter() {
        // @spec 30901_point_judgment_spec.md#req-30901-002
        // ツーバウンド判定: 同じコート側で2回以上バウンド
        if bounce_count.count >= 2 {
            if let Some(court_side) = bounce_count.last_court_side {
                // バウンドしたコート側のプレイヤーが失点
                // つまり、相手側が得点
                let winner = court_side.opponent();

                info!(
                    "Double bounce on {:?} court! {:?} wins the point.",
                    court_side, winner
                );

                rally_events.write(RallyEndEvent {
                    winner,
                    reason: RallyEndReason::DoubleBounce,
                });
            }
        }
    }
}

/// アウト判定システム
/// @spec 30901_point_judgment_spec.md#req-30901-001
/// ボールがコート外に出た場合（主にデバッグ/安全弁）
///
/// NOTE: パデルでは壁で囲まれているため通常アウトは発生しない。
/// このシステムはバグや例外的な状況に対する安全弁として機能する。
pub fn out_of_bounds_judgment_system(
    mut out_events: MessageReader<BallOutOfBoundsEvent>,
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    mut rally_events: MessageWriter<RallyEndEvent>,
) {
    // ゲーム進行中でなければスキップ
    if match_score.game_state != GameState::Playing {
        return;
    }

    let net_z = config.court.net_z;

    for event in out_events.read() {
        let pos = event.final_position;

        // @spec 30901_point_judgment_spec.md#req-30901-001
        // ボールの最終位置からどちら側でアウトになったか判定
        let court_side = crate::core::determine_court_side(pos.z, net_z);

        // アウトになったコート側のプレイヤーが失点
        // （自陣でアウトになった = 自分のミス）
        let winner = court_side.opponent();

        warn!(
            "Ball out of bounds at {:?}! {:?} side loses.",
            pos, court_side
        );

        rally_events.write(RallyEndEvent {
            winner,
            reason: RallyEndReason::Out,
        });
    }
}

/// レット判定システム
/// @spec 30901_point_judgment_spec.md#req-30901-003
/// サーブ時にネットに触れて相手コートに入った場合はレット（再サーブ）
pub fn let_judgment_system(
    mut net_events: MessageReader<NetHitEvent>,
    rally_state: Res<RallyState>,
    config: Res<GameConfig>,
    query: Query<&Transform, With<Ball>>,
    mut rally_events: MessageWriter<RallyEndEvent>,
) {
    // サーブ中でなければスキップ
    if rally_state.phase != RallyPhase::Serving {
        return;
    }

    let net_z = config.court.net_z;

    for event in net_events.read() {
        // ネットに触れた後のボール位置を確認
        if let Ok(transform) = query.get(event.ball) {
            let ball_z = transform.translation.z;

            // @spec 30901_point_judgment_spec.md#req-30901-003
            // サーブ側からネットを超えて相手コートに入ったかを判定
            let server_side = rally_state.server;
            let in_opponent_court = match server_side {
                CourtSide::Player1 => ball_z > net_z, // 1Pサーブ → 2P側に入った
                CourtSide::Player2 => ball_z < net_z, // 2Pサーブ → 1P側に入った
            };

            if in_opponent_court {
                // レット（再サーブ）
                info!(
                    "Let! Ball touched net and landed in opponent's court. Server: {:?}",
                    server_side
                );

                // レット時は NetFault を使用（再サーブ処理は scoring システムで行う）
                rally_events.write(RallyEndEvent {
                    winner: server_side, // レットなので失点なし、サーバーは次のサーブへ
                    reason: RallyEndReason::NetFault,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30904-001: ツーバウンド判定テスト
    /// @spec 30901_point_judgment_spec.md#req-30901-002
    #[test]
    fn test_req_30901_002_double_bounce() {
        let mut bounce_count = BounceCount::default();

        // 1回目のバウンド
        bounce_count.record_bounce(CourtSide::Player1);
        assert_eq!(bounce_count.count, 1);
        assert_eq!(bounce_count.last_court_side, Some(CourtSide::Player1));

        // 2回目のバウンド（同じコート）
        bounce_count.record_bounce(CourtSide::Player1);
        assert_eq!(bounce_count.count, 2);

        // ツーバウンド判定
        assert!(bounce_count.count >= 2);
    }

    /// TST-30904-002: バウンドリセットテスト（コート変更時）
    /// @spec 30901_point_judgment_spec.md#req-30901-002
    #[test]
    fn test_bounce_count_reset_on_court_change() {
        let mut bounce_count = BounceCount::default();

        // 1Pコートで1回バウンド
        bounce_count.record_bounce(CourtSide::Player1);
        assert_eq!(bounce_count.count, 1);

        // 2Pコートでバウンド（カウントリセット）
        bounce_count.record_bounce(CourtSide::Player2);
        assert_eq!(bounce_count.count, 1);
        assert_eq!(bounce_count.last_court_side, Some(CourtSide::Player2));
    }

    /// TST-30904-003: アウト判定テスト
    /// @spec 30901_point_judgment_spec.md#req-30901-001
    #[test]
    fn test_req_30901_001_out_judgment() {
        let net_z = 0.0;

        // 1Pコート側でアウト
        let pos_1p = Vec3::new(0.0, -1.0, -2.0);
        let court_side_1p = crate::core::determine_court_side(pos_1p.z, net_z);
        assert_eq!(court_side_1p, CourtSide::Player1);

        // 2Pコート側でアウト
        let pos_2p = Vec3::new(0.0, -1.0, 2.0);
        let court_side_2p = crate::core::determine_court_side(pos_2p.z, net_z);
        assert_eq!(court_side_2p, CourtSide::Player2);
    }

    /// TST-30904-004: コート境界取得テスト
    /// @spec 30901_point_judgment_spec.md#req-30901-004
    #[test]
    fn test_req_30901_004_court_bounds() {
        use crate::core::CourtBounds;
        use crate::resource::CourtConfig;

        let config = CourtConfig {
            width: 10.0,
            depth: 6.0,
            ceiling_height: 5.0,
            max_jump_height: 5.0,
            net_height: 1.0,
            net_z: 0.0,
        };

        let bounds = CourtBounds::from_config(&config);

        // 境界座標の確認
        assert_eq!(bounds.left, -5.0);
        assert_eq!(bounds.right, 5.0);
        assert_eq!(bounds.back_1p, -3.0);
        assert_eq!(bounds.back_2p, 3.0);
    }
}
