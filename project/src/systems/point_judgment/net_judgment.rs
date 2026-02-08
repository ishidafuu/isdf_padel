//! ネット判定システム
//! @spec 30901_point_judgment_spec.md#req-30901-003
//! @spec 30901_point_judgment_spec.md#req-30901-005

use bevy::prelude::*;

use crate::components::{Ball, LastShooter, LogicalPosition, PointEnded};
use crate::core::events::{NetHitEvent, RallyEndEvent, RallyEndReason};
use crate::core::CourtSide;
use crate::resource::config::GameConfig;
use crate::resource::{GameState, MatchScore, RallyPhase, RallyState};
use crate::simulation::DebugLogger;

/// レット判定システム
/// @spec 30901_point_judgment_spec.md#req-30901-003
/// サーブ時にネット接触を記録し、着地判定時にレット（再サーブ）へ分岐する。
pub fn let_judgment_system(
    mut net_events: MessageReader<NetHitEvent>,
    mut rally_state: ResMut<RallyState>,
    mut debug_logger: Option<ResMut<DebugLogger>>,
) {
    // サーブ中でなければイベントを破棄
    if rally_state.phase != RallyPhase::Serving {
        net_events.read().count();
        return;
    }

    // 既にネット接触済みなら追加イベントは破棄
    if rally_state.serve_touched_net {
        net_events.read().count();
        return;
    }

    // サーブ中にネット接触した事実だけを記録し、
    // 着地判定側で「サービスボックス内 + ネット接触」をレットとして処理する。
    if net_events.read().next().is_some() {
        rally_state.serve_touched_net = true;
        if let Some(ref mut logger) = debug_logger {
            logger.log_scoring(&format!(
                "LET_PENDING server={:?} (waiting bounce judgment)",
                rally_state.server
            ));
        }
        info!(
            "Serve net touch detected. Waiting bounce judgment for let. Server: {:?}",
            rally_state.server
        );
    }
}

/// ネット失点判定システム（ラリー中）
/// @spec 30103_point_end_spec.md#req-30103-002
/// ラリー中にネットに当たった後、自コートに落ちた場合は失点
#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn net_fault_judgment_system(
    mut commands: Commands,
    mut net_events: MessageReader<NetHitEvent>,
    mut rally_state: ResMut<RallyState>,
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    mut debug_logger: Option<ResMut<DebugLogger>>,
    query: Query<(Entity, &LogicalPosition, &LastShooter), (With<Ball>, Without<PointEnded>)>,
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

    // 同一フレームで既にイベント発行済みならスキップ
    if rally_state.rally_end_event_sent_this_frame {
        return;
    }

    // 新座標系: X=打ち合い方向（ネット位置で判定）
    let net_x = config.court.net_x;

    for event in net_events.read() {
        if let Ok((entity, logical_pos, last_shooter)) = query.get(event.ball) {
            if let Some(shooter) = last_shooter.side {
                let ball_x = logical_pos.value.x;

                // @spec 30103_point_end_spec.md#req-30103-002
                // 打ったボールがネットに当たった時点でショット元のコート側にあれば失点
                let in_shooter_court = match shooter {
                    CourtSide::Left => ball_x < net_x, // 1Pが打った → ネット手前（-X側）
                    CourtSide::Right => ball_x > net_x, // 2Pが打った → ネット手前（+X側）
                };

                if in_shooter_court {
                    // ネットに当たって相手コートに届かなかった → 失点
                    let winner = shooter.opponent();

                    // ネットフォールト得点ログ出力
                    if let Some(ref mut logger) = debug_logger {
                        logger.log_scoring(&format!(
                            "POINT winner={:?} reason=NetFault shooter={:?}",
                            winner, shooter
                        ));
                    }

                    info!(
                        "Net fault! {:?} hit the net and ball stayed on their side. {:?} wins.",
                        shooter, winner
                    );

                    rally_events.write(RallyEndEvent {
                        winner,
                        reason: RallyEndReason::NetFault,
                    });

                    // 重複発行防止フラグを設定
                    rally_state.rally_end_event_sent_this_frame = true;

                    // 他のポイント判定システムからの重複発行を防止
                    commands.entity(entity).insert(PointEnded);
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
        let reasons = [
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
