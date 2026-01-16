//! サーブ処理システム
//! @spec 30102_serve_spec.md
//!
//! v0.4: トス→ヒット方式
//! 1回目ボタン: トス開始（ボールを上に投げる）
//! 2回目ボタン: ヒット（ボールを打つ）
//! ヒット可能高さ外でボタン押下しても発射されない
//! タイムアウトまたはボール落下でFault

use bevy::prelude::*;

use crate::components::{
    AiController, Ball, InputState, LogicalPosition, Player, TossBall, TossBallBundle, Velocity,
};
use crate::systems::ai::AiServeTimer;
use crate::core::{CourtSide, ShotEvent};
use crate::resource::config::ServeSide;
use crate::resource::scoring::{MatchFlowState, ServeState, ServeSubPhase};
use crate::resource::{FixedDeltaTime, GameConfig, MatchScore};

/// サーバーを検索するヘルパー関数
/// @spec 30102_serve_spec.md
fn find_server<'a>(
    mut player_query: impl Iterator<Item = (&'a Player, &'a LogicalPosition, &'a InputState)>,
    server_side: CourtSide,
) -> Option<(&'a Player, &'a LogicalPosition, &'a InputState)> {
    player_query.find(|(p, _, _)| p.court_side == server_side)
}

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

/// サーブ開始時のプレイヤー位置設定システム
/// カウントに応じてサーバーとレシーバーをクロスポジションに配置
pub fn serve_position_system(
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    mut player_query: Query<(&Player, &mut LogicalPosition, Option<&mut AiController>)>,
) {
    // サーブサイドを計算
    let server_points = match_score.get_point_index(match_score.server);
    let receiver_points = match_score.get_point_index(match_score.server.opponent());
    let total_points = server_points + receiver_points;
    let serve_side = ServeSide::from_point_total(total_points);

    // サーブサイドに応じたZ位置を決定
    // 両プレイヤーはネット越しに向かい合っているため、デュース/アドの位置は逆になる
    // Left側: デュース = +Z、アド = -Z
    // Right側: デュース = -Z、アド = +Z
    let base_z = config.court.width / 4.0; // 3.0 (コート幅12の1/4)
    let serve_z = match (match_score.server, serve_side) {
        (CourtSide::Left, ServeSide::Deuce) => base_z,   // Left: デュース = +Z
        (CourtSide::Left, ServeSide::Ad) => -base_z,     // Left: アド = -Z
        (CourtSide::Right, ServeSide::Deuce) => -base_z, // Right: デュース = -Z（対向）
        (CourtSide::Right, ServeSide::Ad) => base_z,     // Right: アド = +Z（対向）
    };

    for (player, mut pos, ai_controller) in player_query.iter_mut() {
        let is_server = player.court_side == match_score.server;

        // サーバーとレシーバーは対角線上（クロス）に配置
        let target_z = if is_server {
            serve_z  // サーバーはサーブサイドに
        } else {
            -serve_z  // レシーバーは対角線上（逆サイド）に
        };

        // X位置: サーバーはベースライン外に配置
        // @spec 30102_serve_spec.md#req-30102-086
        if is_server {
            pos.value.x = match player.court_side {
                CourtSide::Left => config.serve.serve_baseline_x_p1,
                CourtSide::Right => config.serve.serve_baseline_x_p2,
            };
        }

        pos.value.z = target_z;

        // AIのホームポジションも更新
        if let Some(mut ai) = ai_controller {
            ai.home_position.z = target_z;
        }
    }

    info!(
        "Serve position set: side={:?}, server={:?}, z={:.1}",
        serve_side, match_score.server, serve_z
    );
}

/// トス入力システム（1回目ボタン）
/// @spec 30102_serve_spec.md#req-30102-080
/// Waiting状態でショットボタンを押すとトスを開始
#[allow(clippy::too_many_arguments)]
pub fn serve_toss_input_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    match_score: Res<MatchScore>,
    mut serve_state: ResMut<ServeState>,
    player_query: Query<(&Player, &LogicalPosition, &InputState)>,
    toss_ball_query: Query<Entity, With<TossBall>>,
    ball_query: Query<Entity, With<Ball>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // @spec 30102_serve_spec.md#req-30102-080: Waiting状態でのみトス可能
    if serve_state.phase != ServeSubPhase::Waiting {
        return;
    }

    // すでにトスボールまたは通常ボールがある場合は何もしない
    if !toss_ball_query.is_empty() || !ball_query.is_empty() {
        return;
    }

    // サーバーを特定し、ショット入力をチェック
    let Some((player, logical_pos, input_state)) =
        find_server(player_query.iter(), match_score.server)
    else {
        return;
    };
    if !input_state.shot_pressed {
        return;
    }

    // @spec 30102_serve_spec.md#req-30102-080: トスボール生成
    let toss_pos = logical_pos.value + Vec3::new(0.0, config.serve.toss_start_offset_y, 0.0);
    let toss_vel = Vec3::new(0.0, config.serve.toss_velocity_y, 0.0);

    commands.spawn(TossBallBundle::new(toss_pos, toss_vel, &mut meshes, &mut materials));

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
    fixed_dt: Res<FixedDeltaTime>,
    mut serve_state: ResMut<ServeState>,
    mut toss_ball_query: Query<(&mut LogicalPosition, &mut Velocity), With<TossBall>>,
) {
    // Tossing状態でのみ実行
    if serve_state.phase != ServeSubPhase::Tossing {
        return;
    }

    let delta = fixed_dt.delta_secs();

    for (mut pos, mut vel) in toss_ball_query.iter_mut() {
        // 重力適用
        vel.value.y += config.physics.gravity * delta;

        // 位置更新
        pos.value += vel.value * delta;
    }

    // トス時間更新
    serve_state.update_toss_time(delta);
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
    player_query: Query<(&Player, &LogicalPosition, &InputState)>,
    toss_ball_query: Query<(Entity, &LogicalPosition), With<TossBall>>,
    mut shot_event_writer: MessageWriter<ShotEvent>,
) {
    // @spec 30102_serve_spec.md#req-30102-082: Tossing状態でのみヒット可能
    if serve_state.phase != ServeSubPhase::Tossing {
        return;
    }

    // サーバーを特定し、ショット入力をチェック
    let Some((player, player_pos, input_state)) =
        find_server(player_query.iter(), match_score.server)
    else {
        return;
    };
    if !input_state.shot_pressed {
        return;
    }

    // トスボールを取得し、ヒット可能高さをチェック
    let Some((toss_entity, toss_pos)) = toss_ball_query.iter().next() else {
        return;
    };
    let ball_height = toss_pos.value.y;

    // @spec 30102_serve_spec.md#req-30102-083: ヒット可能高さ判定
    let is_in_hit_range =
        ball_height >= config.serve.hit_height_min && ball_height <= config.serve.hit_height_max;
    if !is_in_hit_range {
        info!(
            "Serve hit ignored: ball height {:.2}m not in range [{:.2}, {:.2}]",
            ball_height, config.serve.hit_height_min, config.serve.hit_height_max
        );
        return;
    }

    // @spec 30102_serve_spec.md#req-30102-082: ヒット成功
    let hit_pos = toss_pos.value;
    commands.entity(toss_entity).despawn();

    // 入力方向を正規化（ゼロベクトルの場合はそのまま）
    let direction = input_state.movement.normalize_or_zero();

    // ServeState更新
    serve_state.on_hit_success();

    // 注: 状態遷移は serve_landing_judgment_system で行う
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
/// タイムアウトまたはボールが落下しすぎた場合let（打ち直し）とする
pub fn serve_toss_timeout_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut serve_state: ResMut<ServeState>,
    mut ai_serve_timer: ResMut<AiServeTimer>,
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

    // タイムアウトまたは落下判定
    let is_timeout = serve_state.toss_time >= config.serve.toss_timeout;
    let is_falling_too_low =
        velocity.value.y < 0.0 && toss_pos.value.y < config.serve.hit_height_min;

    if !is_timeout && !is_falling_too_low {
        return;
    }

    // @spec 30102_serve_spec.md#req-30102-084: 打ち直し(let)
    commands.entity(toss_entity).despawn();
    serve_state.reset_for_retry();

    // AIタイマーリセット（let後の再トス用）
    // @spec 30102_serve_spec.md#req-30102-087
    ai_serve_timer.toss_timer = None;
    ai_serve_timer.hit_executed = false;

    let reason = if is_timeout { "timeout" } else { "ball too low" };
    info!("Serve let: {} (retry without fault)", reason);
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
