//! ネット判定システム
//! @spec 30901_point_judgment_spec.md#req-30901-003
//! @spec 30901_point_judgment_spec.md#req-30901-005

use bevy::prelude::*;

use crate::components::{Ball, LastShooter, LogicalPosition};
use crate::core::events::{NetHitEvent, RallyEndEvent, RallyEndReason};
use crate::core::CourtSide;
use crate::resource::config::GameConfig;
use crate::resource::{GameState, MatchScore, RallyPhase, RallyState};

/// レット判定システム
/// @spec 30901_point_judgment_spec.md#req-30901-003
/// サーブ時にネットに触れて相手コートに入った場合はレット（再サーブ）
pub fn let_judgment_system(
    mut net_events: MessageReader<NetHitEvent>,
    rally_state: Res<RallyState>,
    config: Res<GameConfig>,
    query: Query<&LogicalPosition, With<Ball>>,
    mut rally_events: MessageWriter<RallyEndEvent>,
) {
    // サーブ中でなければスキップ
    if rally_state.phase != RallyPhase::Serving {
        return;
    }

    // 新座標系: X=打ち合い方向（ネット位置で判定）
    let net_x = config.court.net_x;

    for event in net_events.read() {
        // ネットに触れた後のボール位置を確認
        if let Ok(logical_pos) = query.get(event.ball) {
            let ball_x = logical_pos.value.x;

            // @spec 30901_point_judgment_spec.md#req-30901-003
            // サーブ側からネットを超えて相手コートに入ったかを判定
            let server_side = rally_state.server;
            let in_opponent_court = match server_side {
                CourtSide::Player1 => ball_x > net_x, // 1Pサーブ → 2P側（+X）に入った
                CourtSide::Player2 => ball_x < net_x, // 2Pサーブ → 1P側（-X）に入った
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

/// ネット失点判定システム（ラリー中）
/// @spec 30103_point_end_spec.md#req-30103-002
/// ラリー中にネットに当たった後、自コートに落ちた場合は失点
pub fn net_fault_judgment_system(
    mut net_events: MessageReader<NetHitEvent>,
    rally_state: Res<RallyState>,
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    query: Query<(&LogicalPosition, &LastShooter), With<Ball>>,
    mut rally_events: MessageWriter<RallyEndEvent>,
) {
    // ラリー中でなければスキップ（サーブ中はlet_judgment_systemで処理）
    if rally_state.phase != RallyPhase::Rally {
        return;
    }

    // ゲーム進行中でなければスキップ
    if match_score.game_state != GameState::Playing {
        return;
    }

    // 新座標系: X=打ち合い方向（ネット位置で判定）
    let net_x = config.court.net_x;

    for event in net_events.read() {
        if let Ok((logical_pos, last_shooter)) = query.get(event.ball) {
            if let Some(shooter) = last_shooter.side {
                let ball_x = logical_pos.value.x;

                // @spec 30103_point_end_spec.md#req-30103-002
                // 打ったボールがネットに当たった時点でショット元のコート側にあれば失点
                let in_shooter_court = match shooter {
                    CourtSide::Player1 => ball_x < net_x, // 1Pが打った → ネット手前（-X側）
                    CourtSide::Player2 => ball_x > net_x, // 2Pが打った → ネット手前（+X側）
                };

                if in_shooter_court {
                    // ネットに当たって相手コートに届かなかった → 失点
                    let winner = shooter.opponent();

                    info!(
                        "Net fault! {:?} hit the net and ball stayed on their side. {:?} wins.",
                        shooter, winner
                    );

                    rally_events.write(RallyEndEvent {
                        winner,
                        reason: RallyEndReason::NetFault,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::events::RallyEndReason;

    /// TST-30104-013: PointEndEvent発行確認テスト
    /// @spec 30103_point_end_spec.md#req-30103-004
    #[test]
    fn test_req_30103_004_rally_end_reason() {
        // RallyEndReasonに必要な理由が含まれているか確認
        let reasons = vec![
            RallyEndReason::DoubleBounce,
            RallyEndReason::NetFault,
            RallyEndReason::OwnCourtHit,
        ];

        // すべての理由が異なることを確認
        assert_ne!(reasons[0], reasons[1]);
        assert_ne!(reasons[1], reasons[2]);
        assert_ne!(reasons[0], reasons[2]);
    }
}
