//! サーブ処理システム
//! @spec 30102_serve_spec.md
//!
//! v0.4: トス→ヒット方式
//! 1回目ボタン: トス開始（ボールを上に投げる）
//! 2回目ボタン: ヒット（ボールを打つ）
//! ヒット可能高さ外でボタン押下しても発射されない
//! タイムアウトまたはボール落下でFault

use bevy::prelude::*;

use crate::components::{Ball, InputState, LogicalPosition, Player, TossBall, TossBallBundle, Velocity};
use crate::core::{CourtSide, ShotEvent};
use crate::resource::scoring::{MatchFlowState, ServeState, ServeSubPhase};
use crate::resource::{GameConfig, MatchScore};

/// ServeState リソースの初期化/リセットシステム
/// @spec 30102_serve_spec.md#req-30102-080
/// Serve状態に入った時にServeStateを初期化する
pub fn serve_init_system(
    mut commands: Commands,
    state: Res<State<MatchFlowState>>,
    serve_state: Option<Res<ServeState>>,
) {
    if *state.get() != MatchFlowState::Serve {
        return;
    }

    // ServeStateがない場合のみ初期化（あれば前回のfault_countを維持）
    if serve_state.is_none() {
        commands.insert_resource(ServeState::new());
    }
}

/// トス入力システム（1回目ボタン）
/// @spec 30102_serve_spec.md#req-30102-080
/// Waiting状態でショットボタンを押すとトスを開始
pub fn serve_toss_input_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    mut serve_state: ResMut<ServeState>,
    player_query: Query<(&Player, &LogicalPosition, &InputState)>,
    toss_ball_query: Query<Entity, With<TossBall>>,
    ball_query: Query<Entity, With<Ball>>,
) {
    // @spec 30102_serve_spec.md#req-30102-080: Waiting状態でのみトス可能
    if serve_state.phase != ServeSubPhase::Waiting {
        return;
    }

    // すでにトスボールまたは通常ボールがある場合は何もしない
    if !toss_ball_query.is_empty() || !ball_query.is_empty() {
        return;
    }

    // サーバーを特定
    let Some((player, logical_pos, input_state)) = player_query
        .iter()
        .find(|(p, _, _)| p.court_side == match_score.server)
    else {
        return;
    };

    // ショット入力をチェック
    if !input_state.shot_pressed {
        return;
    }

    // @spec 30102_serve_spec.md#req-30102-080: トスボール生成
    let toss_pos = logical_pos.value + Vec3::new(0.0, config.serve.toss_start_offset_y, 0.0);
    let toss_vel = Vec3::new(0.0, config.serve.toss_velocity_y, 0.0);

    commands.spawn(TossBallBundle::new(toss_pos, toss_vel));

    // ServeState更新
    serve_state.start_toss(logical_pos.value);

    info!(
        "Toss: Ball tossed at {:?} with velocity {:?} by Player{}",
        toss_pos, toss_vel, player.id
    );
}

/// トス物理システム（重力適用）
/// @spec 30102_serve_spec.md#req-30102-081
/// トスボールに重力を適用する
pub fn serve_toss_physics_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut serve_state: ResMut<ServeState>,
    mut toss_ball_query: Query<(&mut LogicalPosition, &mut Velocity), With<TossBall>>,
) {
    // Tossing状態でのみ実行
    if serve_state.phase != ServeSubPhase::Tossing {
        return;
    }

    for (mut pos, mut vel) in toss_ball_query.iter_mut() {
        // 重力適用
        vel.value.y += config.physics.gravity * time.delta_secs();

        // 位置更新
        pos.value += vel.value * time.delta_secs();
    }

    // トス時間更新
    serve_state.update_toss_time(time.delta_secs());
}

/// ヒット入力システム（2回目ボタン）
/// @spec 30102_serve_spec.md#req-30102-082
/// @spec 30102_serve_spec.md#req-30102-083
/// Tossing状態でショットボタンを押すとヒットを試行
pub fn serve_hit_input_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    mut serve_state: ResMut<ServeState>,
    mut next_state: ResMut<NextState<MatchFlowState>>,
    player_query: Query<(&Player, &LogicalPosition, &InputState)>,
    toss_ball_query: Query<(Entity, &LogicalPosition), With<TossBall>>,
    mut shot_event_writer: MessageWriter<ShotEvent>,
) {
    // @spec 30102_serve_spec.md#req-30102-082: Tossing状態でのみヒット可能
    if serve_state.phase != ServeSubPhase::Tossing {
        return;
    }

    // サーバーを特定
    let Some((player, player_pos, input_state)) = player_query
        .iter()
        .find(|(p, _, _)| p.court_side == match_score.server)
    else {
        return;
    };

    // ショット入力をチェック
    if !input_state.shot_pressed {
        return;
    }

    // トスボールを取得
    let Some((toss_entity, toss_pos)) = toss_ball_query.iter().next() else {
        return;
    };

    let ball_height = toss_pos.value.y;

    // @spec 30102_serve_spec.md#req-30102-083: ヒット可能高さ判定
    if ball_height < config.serve.hit_height_min || ball_height > config.serve.hit_height_max {
        // ヒット可能範囲外: 何もしない
        info!(
            "Serve hit ignored: ball height {:.2}m not in range [{:.2}, {:.2}]",
            ball_height, config.serve.hit_height_min, config.serve.hit_height_max
        );
        return;
    }

    // @spec 30102_serve_spec.md#req-30102-082: ヒット成功
    // 打点位置を記録（トスボールの位置を使用）
    let hit_pos = toss_pos.value;

    // トスボールを削除
    commands.entity(toss_entity).despawn();

    // 入力方向を取得
    let raw_direction = input_state.movement;
    let direction = if raw_direction.length() > 0.0 {
        raw_direction.normalize()
    } else {
        Vec2::ZERO
    };

    // ServeState更新
    serve_state.on_hit_success();

    // @spec 30102_serve_spec.md#req-30102-082: Rally状態に遷移
    next_state.set(MatchFlowState::Rally);

    // ShotEvent発行（is_serve = true）
    // @spec 30602_shot_direction_spec.md#req-30602-031
    // ボール生成と弾道計算は shot_direction_system で実行
    shot_event_writer.write(ShotEvent {
        player_id: player.id,
        court_side: match_score.server,
        direction,
        jump_height: player_pos.value.y,
        is_serve: true,
        hit_position: Some(hit_pos),
    });

    info!(
        "Serve hit success: ShotEvent sent with hit_pos {:?} by Player{}",
        hit_pos, player.id
    );
}

/// トスタイムアウト/落下判定システム
/// @spec 30102_serve_spec.md#req-30102-084
/// タイムアウトまたはボールが落下しすぎた場合Faultとする
pub fn serve_toss_timeout_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut serve_state: ResMut<ServeState>,
    toss_ball_query: Query<(Entity, &LogicalPosition, &Velocity), With<TossBall>>,
) {
    // Tossing状態でのみ実行
    if serve_state.phase != ServeSubPhase::Tossing {
        return;
    }

    // トスボールを取得
    let Some((toss_entity, toss_pos, velocity)) = toss_ball_query.iter().next() else {
        return;
    };

    let ball_height = toss_pos.value.y;
    let is_timeout = serve_state.toss_time >= config.serve.toss_timeout;
    // ボールが落下中（velocity.y < 0）の場合のみ「低すぎる」判定
    let is_falling = velocity.value.y < 0.0;
    let is_too_low = is_falling && ball_height < config.serve.hit_height_min;

    if is_timeout || is_too_low {
        // @spec 30102_serve_spec.md#req-30102-084: Fault
        commands.entity(toss_entity).despawn();
        serve_state.record_fault();

        let reason = if is_timeout { "timeout" } else { "ball too low" };
        info!(
            "Serve fault: {} (fault_count: {})",
            reason, serve_state.fault_count
        );
    }
}

/// ダブルフォルト処理システム
/// @spec 30102_serve_spec.md#req-30102-089
/// fault_countが2に達したら相手にポイントを与える
pub fn serve_double_fault_system(
    mut serve_state: ResMut<ServeState>,
    mut match_score: ResMut<MatchScore>,
    mut next_state: ResMut<NextState<MatchFlowState>>,
) {
    // @spec 30102_serve_spec.md#req-30102-089: ダブルフォルト判定
    if !serve_state.is_double_fault() {
        return;
    }

    // 相手にポイント
    let receiver = match_score.server.opponent();
    match_score.add_point(receiver);

    info!(
        "Double fault! Point to {:?}. Score: P1={}, P2={}",
        receiver,
        match_score.get_point_index(CourtSide::Left),
        match_score.get_point_index(CourtSide::Right)
    );

    // ServeStateリセット
    serve_state.reset_for_new_point();

    // PointEnd状態に遷移
    next_state.set(MatchFlowState::PointEnd);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30104-080: トス開始テスト
    /// @spec 30102_serve_spec.md#req-30102-080
    #[test]
    fn test_req_30102_080_toss_start() {
        let mut serve_state = ServeState::new();
        assert_eq!(serve_state.phase, ServeSubPhase::Waiting);

        let origin = Vec3::new(0.0, 0.0, -5.0);
        serve_state.start_toss(origin);

        assert_eq!(serve_state.phase, ServeSubPhase::Tossing);
        assert_eq!(serve_state.toss_origin, Some(origin));
        assert_eq!(serve_state.toss_time, 0.0);
    }

    /// TST-30104-084: トスタイムアウトテスト
    /// @spec 30102_serve_spec.md#req-30102-084
    #[test]
    fn test_req_30102_084_toss_timeout() {
        let mut serve_state = ServeState::new();
        serve_state.start_toss(Vec3::ZERO);

        // タイムアウト前
        serve_state.update_toss_time(2.9);
        assert!(!serve_state.is_double_fault());

        // fault記録
        serve_state.record_fault();
        assert_eq!(serve_state.fault_count, 1);
        assert_eq!(serve_state.phase, ServeSubPhase::Waiting);
    }

    /// TST-30104-089: ダブルフォルトテスト
    /// @spec 30102_serve_spec.md#req-30102-089
    #[test]
    fn test_req_30102_089_double_fault() {
        let mut serve_state = ServeState::new();

        // 1回目のfault
        serve_state.record_fault();
        assert!(!serve_state.is_double_fault());
        assert_eq!(serve_state.fault_count, 1);

        // 2回目のfault
        serve_state.record_fault();
        assert!(serve_state.is_double_fault());
        assert_eq!(serve_state.fault_count, 2);
    }

    /// TST-30104-083: ヒット可能高さテスト
    /// @spec 30102_serve_spec.md#req-30102-083
    #[test]
    fn test_req_30102_083_hit_height_range() {
        let hit_min = 1.8;
        let hit_max = 2.7;

        // 範囲内
        let ball_height = 2.2;
        let can_hit = ball_height >= hit_min && ball_height <= hit_max;
        assert!(can_hit);

        // 範囲外（低すぎ）
        let ball_height = 1.5;
        let can_hit = ball_height >= hit_min && ball_height <= hit_max;
        assert!(!can_hit);

        // 範囲外（高すぎ）
        let ball_height = 3.0;
        let can_hit = ball_height >= hit_min && ball_height <= hit_max;
        assert!(!can_hit);
    }
}
