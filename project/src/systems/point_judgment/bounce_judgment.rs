//! バウンド判定システム
//! @spec 30901_point_judgment_spec.md#req-30901-002

use bevy::prelude::*;

use crate::components::{Ball, BounceCount, LastShooter};
use crate::core::events::{GroundBounceEvent, RallyEndEvent, RallyEndReason};
use crate::resource::{GameState, MatchScore, RallyPhase, RallyState};
use crate::simulation::DebugLogger;

/// バウンス回数更新システム
/// @spec 30901_point_judgment_spec.md#req-30901-002
/// GroundBounceEvent を受信して BounceCount を更新
pub fn bounce_count_update_system(
    mut bounce_events: MessageReader<GroundBounceEvent>,
    mut debug_logger: Option<ResMut<DebugLogger>>,
    mut query: Query<&mut BounceCount, With<Ball>>,
) {
    for event in bounce_events.read() {
        if let Ok(mut bounce_count) = query.get_mut(event.ball) {
            // @spec 30901_point_judgment_spec.md#req-30901-002
            // バウンドしたコート側を記録
            bounce_count.record_bounce(event.court_side);

            // バウンスログ出力
            if let Some(ref mut logger) = debug_logger {
                logger.log_physics(&format!(
                    "BOUNCE court={:?} count={} pos=({:.2},{:.2},{:.2})",
                    event.court_side, bounce_count.count,
                    event.bounce_point.x, event.bounce_point.y, event.bounce_point.z
                ));
            }

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
    mut debug_logger: Option<ResMut<DebugLogger>>,
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

                // ツーバウンス得点ログ出力
                if let Some(ref mut logger) = debug_logger {
                    logger.log_scoring(&format!(
                        "POINT winner={:?} reason=DoubleBounce court={:?}",
                        winner, court_side
                    ));
                }

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

/// 自コート打球失点判定システム
/// @spec 30103_point_end_spec.md#req-30103-003
/// 打った打球が自コートに落ちた場合は失点
pub fn own_court_hit_judgment_system(
    mut bounce_events: MessageReader<GroundBounceEvent>,
    rally_state: Res<RallyState>,
    match_score: Res<MatchScore>,
    mut debug_logger: Option<ResMut<DebugLogger>>,
    query: Query<(&BounceCount, &LastShooter), With<Ball>>,
    mut rally_events: MessageWriter<RallyEndEvent>,
) {
    // ラリー中でなければスキップ
    if rally_state.phase != RallyPhase::Rally {
        return;
    }

    // ゲーム進行中でなければスキップ
    if match_score.game_state != GameState::Playing {
        return;
    }

    for event in bounce_events.read() {
        if let Ok((bounce_count, last_shooter)) = query.get(event.ball) {
            if let Some(shooter) = last_shooter.side {
                // @spec 30103_point_end_spec.md#req-30103-003
                // 最初のバウンドで、バウンドしたコート側が打った側と同じ場合
                // つまり、ネットを超える前に自コートでバウンドした
                if bounce_count.count == 1 && event.court_side == shooter {
                    let winner = shooter.opponent();

                    // 自コート打球得点ログ出力
                    if let Some(ref mut logger) = debug_logger {
                        logger.log_scoring(&format!(
                            "POINT winner={:?} reason=OwnCourtHit shooter={:?}",
                            winner, shooter
                        ));
                    }

                    info!(
                        "Own court hit! {:?} hit ball landed on their own court. {:?} wins.",
                        shooter, winner
                    );

                    rally_events.write(RallyEndEvent {
                        winner,
                        reason: RallyEndReason::OwnCourtHit,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::CourtSide;

    /// TST-30904-001: ツーバウンド判定テスト
    /// @spec 30901_point_judgment_spec.md#req-30901-002
    #[test]
    fn test_req_30901_002_double_bounce() {
        let mut bounce_count = BounceCount::default();

        // 1回目のバウンド
        bounce_count.record_bounce(CourtSide::Left);
        assert_eq!(bounce_count.count, 1);
        assert_eq!(bounce_count.last_court_side, Some(CourtSide::Left));

        // 2回目のバウンド（同じコート）
        bounce_count.record_bounce(CourtSide::Left);
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
        bounce_count.record_bounce(CourtSide::Left);
        assert_eq!(bounce_count.count, 1);

        // 2Pコートでバウンド（カウントリセット）
        bounce_count.record_bounce(CourtSide::Right);
        assert_eq!(bounce_count.count, 1);
        assert_eq!(bounce_count.last_court_side, Some(CourtSide::Right));
    }

    /// TST-30104-010: ツーバウンス失点テスト
    /// @spec 30103_point_end_spec.md#req-30103-001
    #[test]
    fn test_req_30103_001_two_bounce_loss() {
        let mut bounce_count = BounceCount::default();

        // 1P側で2回バウンド → 1P失点（2P得点）
        bounce_count.record_bounce(CourtSide::Left);
        bounce_count.record_bounce(CourtSide::Left);

        assert!(bounce_count.count >= 2);
        assert_eq!(bounce_count.last_court_side, Some(CourtSide::Left));
        // 勝者は相手側
        let winner = bounce_count
            .last_court_side
            .expect("last_court_side should be set after record_bounce")
            .opponent();
        assert_eq!(winner, CourtSide::Right);
    }

    /// TST-30104-012: 自コート打球失点テスト
    /// @spec 30103_point_end_spec.md#req-30103-003
    #[test]
    fn test_req_30103_003_own_court_hit() {
        let mut last_shooter = LastShooter::default();
        let mut bounce_count = BounceCount::default();

        // 1Pがショット
        last_shooter.record(CourtSide::Left);
        assert_eq!(last_shooter.side, Some(CourtSide::Left));

        // 1Pコートでバウンド（自コート打球）
        bounce_count.record_bounce(CourtSide::Left);

        // 条件: 最初のバウンドで、バウンドしたコート側が打った側と同じ
        assert_eq!(bounce_count.count, 1);
        assert_eq!(last_shooter.side, bounce_count.last_court_side);

        // 1Pの自コート打球 → 1P失点（2P得点）
        let winner = last_shooter
            .side
            .expect("last_shooter.side should be set after record")
            .opponent();
        assert_eq!(winner, CourtSide::Right);
    }
}
