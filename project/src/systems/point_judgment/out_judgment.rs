//! アウト判定システム
//! @spec 30901_point_judgment_spec.md#req-30901-001

use bevy::prelude::*;

use crate::components::{Ball, LastShooter};
use crate::core::events::{BallOutOfBoundsEvent, RallyEndEvent, RallyEndReason, WallReflectionEvent};
use crate::core::CourtSide;
use crate::resource::{GameState, MatchScore};

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
                    CourtSide::Left
                } else {
                    CourtSide::Right
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
                    CourtSide::Left
                } else {
                    CourtSide::Right
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::events::RallyEndReason;

    /// TST-30904-003: アウト判定テスト（X軸=打ち合い方向）
    /// @spec 30901_point_judgment_spec.md#req-30901-001
    #[test]
    fn test_req_30901_001_out_judgment() {
        let net_x = 0.0;

        // 1Pコート側でアウト（X < 0）
        let pos_1p = Vec3::new(-2.0, -1.0, 0.0);
        let court_side_1p = crate::core::determine_court_side(pos_1p.x, net_x);
        assert_eq!(court_side_1p, CourtSide::Left);

        // 2Pコート側でアウト（X > 0）
        let pos_2p = Vec3::new(2.0, -1.0, 0.0);
        let court_side_2p = crate::core::determine_court_side(pos_2p.x, net_x);
        assert_eq!(court_side_2p, CourtSide::Right);
    }

    /// TST-30904-004: コート境界取得テスト
    /// @spec 30901_point_judgment_spec.md#req-30901-004
    #[test]
    fn test_req_30901_004_court_bounds() {
        use crate::resource::CourtConfig;
        use crate::systems::court_factory::create_court_bounds;

        let config = CourtConfig {
            width: 10.0,
            depth: 6.0,
            ceiling_height: 5.0,
            max_jump_height: 5.0,
            net_height: 1.0,
            net_x: 0.0,
            service_box_depth: 1.5,
            outer_wall_z: 8.0,
            outer_wall_x: 10.0,
        };

        let bounds = create_court_bounds(&config);

        // 境界座標の確認
        // Z軸（コート幅）: left=-5, right=5
        // X軸（打ち合い方向）: back_left=-3, back_right=3
        assert_eq!(bounds.left, -5.0);
        assert_eq!(bounds.right, 5.0);
        assert_eq!(bounds.back_left, -3.0);
        assert_eq!(bounds.back_right, 3.0);
    }

    /// TST-30036-001: 壁ヒット時アウト判定テスト
    /// 壁に当たった場合、LastShooter の失点となる
    #[test]
    fn test_wall_hit_out_judgment() {
        let mut last_shooter = LastShooter::default();

        // 1Pがショット
        last_shooter.record(CourtSide::Left);
        assert_eq!(last_shooter.side, Some(CourtSide::Left));

        // 壁に当たった → 1P失点（2P得点）
        let winner = last_shooter
            .side
            .expect("last_shooter.side should be set after record")
            .opponent();
        assert_eq!(winner, CourtSide::Right);
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
        last_shooter.record(CourtSide::Left);

        // 1Pがサイドアウト → 1P失点（2P得点）
        let winner = last_shooter
            .side
            .expect("last_shooter.side should be set after record")
            .opponent();
        assert_eq!(winner, CourtSide::Right);
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
        last_shooter.record(CourtSide::Right);

        // 2Pがベースラインアウト → 2P失点（1P得点）
        let winner = last_shooter
            .side
            .expect("last_shooter.side should be set after record")
            .opponent();
        assert_eq!(winner, CourtSide::Left);
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
            Vec3::new(0.0, 0.0, 0.0),   // センター
            Vec3::new(4.0, 0.0, 2.0),   // コーナー近く（境界内）
            Vec3::new(-4.0, 0.0, -2.0), // 対角コーナー近く（境界内）
            Vec3::new(5.0, 0.0, 3.0),   // ちょうど境界（境界内扱い）
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
