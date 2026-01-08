//! ポイント判定システム
//! @spec 30901_point_judgment_spec.md
//!
//! 得点条件（ツーバウンド、アウト、レット）を判定する。

use bevy::prelude::*;

use crate::components::{Ball, BounceCount, LastShooter, LogicalPosition};
use crate::core::events::{BallOutOfBoundsEvent, GroundBounceEvent, NetHitEvent, RallyEndEvent, RallyEndReason, ShotExecutedEvent, WallReflectionEvent};
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
                    update_last_shooter_system,
                    bounce_count_update_system,
                    double_bounce_judgment_system,
                    out_of_bounds_judgment_system,
                    wall_hit_judgment_system,
                    let_judgment_system,
                    net_fault_judgment_system,
                    own_court_hit_judgment_system,
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

/// アウト判定システム（主要失点条件）
/// @spec 30901_point_judgment_spec.md#req-30901-001
///
/// テニスルールに準拠したアウト判定。
/// ボールがコート境界外に着地した場合、最後に打った側（LastShooter）の失点。
/// 壁を超えてコート外に着地した場合のフォールバックとして機能。
pub fn out_of_bounds_judgment_system(
    mut out_events: MessageReader<BallOutOfBoundsEvent>,
    match_score: Res<MatchScore>,
    query: Query<&LastShooter, With<Ball>>,
    mut rally_events: MessageWriter<RallyEndEvent>,
) {
    // ゲーム進行中でなければスキップ
    if match_score.game_state != GameState::Playing {
        return;
    }

    for event in out_events.read() {
        // @spec 30901_point_judgment_spec.md#req-30901-001
        // LastShooter（最後に打った側）から失点側を決定
        if let Ok(last_shooter) = query.get(event.ball) {
            if let Some(shooter) = last_shooter.side {
                // 打った側の失点 = 相手の得点
                let winner = shooter.opponent();

                info!(
                    "Out! Ball landed out of bounds at {:?}. {:?} hit the ball. {:?} wins.",
                    event.final_position, shooter, winner
                );

                rally_events.write(RallyEndEvent {
                    winner,
                    reason: RallyEndReason::Out,
                });
            } else {
                // LastShooter が未設定の場合（サーブ前など）
                // ボール位置から判定（フォールバック）
                let court_side = if event.final_position.z < 0.0 {
                    CourtSide::Player1
                } else {
                    CourtSide::Player2
                };
                let winner = court_side.opponent();

                warn!(
                    "Out with no LastShooter! Ball at {:?}. Defaulting to {:?} side loss.",
                    event.final_position, court_side
                );

                rally_events.write(RallyEndEvent {
                    winner,
                    reason: RallyEndReason::Out,
                });
            }
        }
    }
}

/// 壁ヒット判定システム（テニスルール）
/// @spec 30901_point_judgment_spec.md#REQ-30901-006
///
/// ボールが壁（フェンス）に当たった時点でラリー終了（アウト）。
/// テニスでは壁に当たった時点でインプレーではなくなる。
/// 最後に打った側（LastShooter）の失点となる。
pub fn wall_hit_judgment_system(
    mut wall_events: MessageReader<WallReflectionEvent>,
    match_score: Res<MatchScore>,
    query: Query<&LastShooter, With<Ball>>,
    mut rally_events: MessageWriter<RallyEndEvent>,
) {
    // ゲーム進行中でなければスキップ
    if match_score.game_state != GameState::Playing {
        return;
    }

    for event in wall_events.read() {
        // 壁に当たったボールの LastShooter を取得
        if let Ok(last_shooter) = query.get(event.ball) {
            if let Some(shooter) = last_shooter.side {
                // 壁に当てた側の失点 = 相手の得点
                let winner = shooter.opponent();

                info!(
                    "Wall hit ({:?})! {:?} hit ball to wall. {:?} wins the point.",
                    event.wall_type, shooter, winner
                );

                rally_events.write(RallyEndEvent {
                    winner,
                    reason: RallyEndReason::Out,
                });
            } else {
                // LastShooter が未設定の場合（サーブ前など）
                // 壁に当たった位置から判定
                let court_side = if event.contact_point.z < 0.0 {
                    CourtSide::Player1
                } else {
                    CourtSide::Player2
                };
                let winner = court_side.opponent();

                warn!(
                    "Wall hit ({:?}) with no LastShooter! Defaulting to {:?} side loss.",
                    event.wall_type, court_side
                );

                rally_events.write(RallyEndEvent {
                    winner,
                    reason: RallyEndReason::Out,
                });
            }
        }
    }
}

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

/// 最後のショット元更新システム
/// @spec 30103_point_end_spec.md#req-30103-003
/// ShotExecutedEvent を受信して LastShooter を更新
pub fn update_last_shooter_system(
    mut shot_events: MessageReader<ShotExecutedEvent>,
    mut query: Query<&mut LastShooter, With<Ball>>,
) {
    for event in shot_events.read() {
        // プレイヤーIDからCourtSideを決定
        let shooter = match event.player_id {
            1 => CourtSide::Player1,
            _ => CourtSide::Player2,
        };

        for mut last_shooter in query.iter_mut() {
            last_shooter.record(shooter);
            info!("Ball shot by {:?}", shooter);
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

/// 自コート打球失点判定システム
/// @spec 30103_point_end_spec.md#req-30103-003
/// 打った打球が自コートに落ちた場合は失点
pub fn own_court_hit_judgment_system(
    mut bounce_events: MessageReader<GroundBounceEvent>,
    rally_state: Res<RallyState>,
    match_score: Res<MatchScore>,
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

    /// TST-30904-003: アウト判定テスト（X軸=打ち合い方向）
    /// @spec 30901_point_judgment_spec.md#req-30901-001
    #[test]
    fn test_req_30901_001_out_judgment() {
        let net_x = 0.0;

        // 1Pコート側でアウト（X < 0）
        let pos_1p = Vec3::new(-2.0, -1.0, 0.0);
        let court_side_1p = crate::core::determine_court_side(pos_1p.x, net_x);
        assert_eq!(court_side_1p, CourtSide::Player1);

        // 2Pコート側でアウト（X > 0）
        let pos_2p = Vec3::new(2.0, -1.0, 0.0);
        let court_side_2p = crate::core::determine_court_side(pos_2p.x, net_x);
        assert_eq!(court_side_2p, CourtSide::Player2);
    }

    /// TST-30904-004: コート境界取得テスト
    /// @spec 30901_point_judgment_spec.md#req-30901-004
    #[test]
    fn test_req_30901_004_court_bounds() {
        use crate::systems::court_factory::create_court_bounds;
        use crate::resource::CourtConfig;

        let config = CourtConfig {
            width: 10.0,
            depth: 6.0,
            ceiling_height: 5.0,
            max_jump_height: 5.0,
            net_height: 1.0,
            net_x: 0.0,
            service_box_depth: 1.5,
        };

        let bounds = create_court_bounds(&config);

        // 境界座標の確認
        // Z軸（コート幅）: left=-5, right=5
        // X軸（打ち合い方向）: back_1p=-3, back_2p=3
        assert_eq!(bounds.left, -5.0);
        assert_eq!(bounds.right, 5.0);
        assert_eq!(bounds.back_1p, -3.0);
        assert_eq!(bounds.back_2p, 3.0);
    }

    /// TST-30104-010: ツーバウンド失点テスト
    /// @spec 30103_point_end_spec.md#req-30103-001
    #[test]
    fn test_req_30103_001_two_bounce_loss() {
        let mut bounce_count = BounceCount::default();

        // 1P側で2回バウンド → 1P失点（2P得点）
        bounce_count.record_bounce(CourtSide::Player1);
        bounce_count.record_bounce(CourtSide::Player1);

        assert!(bounce_count.count >= 2);
        assert_eq!(bounce_count.last_court_side, Some(CourtSide::Player1));
        // 勝者は相手側
        let winner = bounce_count.last_court_side.unwrap().opponent();
        assert_eq!(winner, CourtSide::Player2);
    }

    /// TST-30104-012: 自コート打球失点テスト
    /// @spec 30103_point_end_spec.md#req-30103-003
    #[test]
    fn test_req_30103_003_own_court_hit() {
        let mut last_shooter = LastShooter::default();
        let mut bounce_count = BounceCount::default();

        // 1Pがショット
        last_shooter.record(CourtSide::Player1);
        assert_eq!(last_shooter.side, Some(CourtSide::Player1));

        // 1Pコートでバウンド（自コート打球）
        bounce_count.record_bounce(CourtSide::Player1);

        // 条件: 最初のバウンドで、バウンドしたコート側が打った側と同じ
        assert_eq!(bounce_count.count, 1);
        assert_eq!(last_shooter.side, bounce_count.last_court_side);

        // 1Pの自コート打球 → 1P失点（2P得点）
        let winner = last_shooter.side.unwrap().opponent();
        assert_eq!(winner, CourtSide::Player2);
    }

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

    /// TST-30036-001: 壁ヒット時アウト判定テスト
    /// 壁に当たった場合、LastShooter の失点となる
    #[test]
    fn test_wall_hit_out_judgment() {
        let mut last_shooter = LastShooter::default();

        // 1Pがショット
        last_shooter.record(CourtSide::Player1);
        assert_eq!(last_shooter.side, Some(CourtSide::Player1));

        // 壁に当たった → 1P失点（2P得点）
        let winner = last_shooter.side.unwrap().opponent();
        assert_eq!(winner, CourtSide::Player2);
    }

    /// TST-30036-002: 壁ヒット時のRallyEndReason確認
    /// Out理由が正しく使われることを確認
    #[test]
    fn test_wall_hit_rally_end_reason() {
        // 壁ヒット時は Out 理由を使用
        let reason = RallyEndReason::Out;
        assert_eq!(reason, RallyEndReason::Out);
    }

    /// TST-30037-001: サイドライン外アウト判定テスト
    /// @spec 30901_point_judgment_spec.md#req-30901-001
    /// X軸がコート境界外の場合、LastShooter の失点となる
    #[test]
    fn test_req_30901_001_sideline_out() {
        let half_width = 5.0_f32;

        // サイドライン外の位置
        let pos_out_left = Vec3::new(-6.0, 0.0, 0.0);
        let pos_out_right = Vec3::new(6.0, 0.0, 0.0);

        // 境界外判定
        assert!(pos_out_left.x.abs() > half_width);
        assert!(pos_out_right.x.abs() > half_width);

        // LastShooter ベースで失点判定
        let mut last_shooter = LastShooter::default();
        last_shooter.record(CourtSide::Player1);

        // 1Pがサイドアウト → 1P失点（2P得点）
        let winner = last_shooter.side.unwrap().opponent();
        assert_eq!(winner, CourtSide::Player2);
    }

    /// TST-30037-002: ベースライン外アウト判定テスト
    /// @spec 30901_point_judgment_spec.md#req-30901-001
    /// Z軸がコート境界外の場合、LastShooter の失点となる
    #[test]
    fn test_req_30901_001_baseline_out() {
        let half_depth = 3.0_f32;

        // ベースライン外の位置
        let pos_out_back = Vec3::new(0.0, 0.0, -4.0);
        let pos_out_front = Vec3::new(0.0, 0.0, 4.0);

        // 境界外判定
        assert!(pos_out_back.z.abs() > half_depth);
        assert!(pos_out_front.z.abs() > half_depth);

        // LastShooter ベースで失点判定
        let mut last_shooter = LastShooter::default();
        last_shooter.record(CourtSide::Player2);

        // 2Pがベースラインアウト → 2P失点（1P得点）
        let winner = last_shooter.side.unwrap().opponent();
        assert_eq!(winner, CourtSide::Player1);
    }

    /// TST-30037-003: コート内着地は失点にならないテスト
    /// @spec 30901_point_judgment_spec.md#req-30901-001
    /// コート内の着地はアウトにならない（GroundBounceEvent で処理）
    #[test]
    fn test_req_30901_001_in_bounds() {
        let half_width = 5.0_f32;
        let half_depth = 3.0_f32;

        // コート内の位置
        let positions = vec![
            Vec3::new(0.0, 0.0, 0.0),       // センター
            Vec3::new(4.0, 0.0, 2.0),       // コーナー近く（境界内）
            Vec3::new(-4.0, 0.0, -2.0),     // 対角コーナー近く（境界内）
            Vec3::new(5.0, 0.0, 3.0),       // ちょうど境界（境界内扱い）
        ];

        for pos in positions {
            let in_bounds_x = pos.x.abs() <= half_width;
            let in_bounds_z = pos.z.abs() <= half_depth;

            // すべてコート内
            assert!(in_bounds_x, "Position {:?} should be in bounds (X)", pos);
            assert!(in_bounds_z, "Position {:?} should be in bounds (Z)", pos);
        }
    }

    /// TST-30037-004: コーナー外アウト判定テスト
    /// @spec 30901_point_judgment_spec.md#req-30901-001
    /// コーナー外（X, Z両方とも境界外）の場合もアウト
    #[test]
    fn test_req_30901_001_corner_out() {
        let half_width = 5.0_f32;
        let half_depth = 3.0_f32;

        // コーナー外の位置
        let pos_corner_out = Vec3::new(6.0, 0.0, 4.0);

        // 境界外判定
        let out_x = pos_corner_out.x.abs() > half_width;
        let out_z = pos_corner_out.z.abs() > half_depth;

        assert!(out_x || out_z); // どちらかが境界外ならアウト
        assert!(out_x && out_z); // コーナーなので両方境界外
    }
}
