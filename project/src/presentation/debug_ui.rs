//! デバッグUI表示システム
//! ゲーム状態を画面に表示

use bevy::prelude::*;

use crate::components::{Ball, BounceCount, KnockbackState, Player, ShotState};
use crate::core::CourtSide;
use crate::presentation::WORLD_SCALE;
use crate::resource::{GameConfig, MatchScore, RallyPhase, RallyState};

/// デバッグUIプラグイン
pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_debug_ui).add_systems(
            Update,
            (
                update_debug_ui,
                draw_hit_range_gizmos,
                draw_racket_swing_gizmos,
            ),
        );
    }
}

/// デバッグUIマーカー
#[derive(Component)]
pub struct DebugUiText;

/// デバッグUIセットアップ
fn setup_debug_ui(mut commands: Commands) {
    // デバッグテキスト（左上）
    commands.spawn((
        DebugUiText,
        Text::new(""),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
    ));
}

/// スコア表示テキストを生成
fn format_score_text(match_score: &MatchScore, point_values: &[u32]) -> String {
    let p1_point = match_score.get_point_display(CourtSide::Left, point_values);
    let p2_point = match_score.get_point_display(CourtSide::Right, point_values);
    let p1_score = match_score.get_score(CourtSide::Left);
    let p2_score = match_score.get_score(CourtSide::Right);
    format!(
        "Score: {} - {} (G: {}-{}, S: {}-{})",
        p1_point, p2_point, p1_score.games, p2_score.games, p1_score.sets, p2_score.sets,
    )
}

/// フェーズ情報テキストを生成
fn format_phase_info(rally_state: &RallyState) -> String {
    let phase = match rally_state.phase {
        RallyPhase::WaitingServe => "WAITING SERVE",
        RallyPhase::Serving => "SERVING",
        RallyPhase::Rally => "RALLY",
        RallyPhase::PointEnded => "POINT END",
    };
    let server = match rally_state.server {
        CourtSide::Left => "P1",
        CourtSide::Right => "P2",
    };
    format!(
        "Phase: {} | Server: {} | Fault: {}",
        phase, server, rally_state.fault_count
    )
}

/// バウンス情報テキストを生成
fn format_bounce_info(ball_query: &Query<&BounceCount, With<Ball>>) -> String {
    if let Some(bounce) = ball_query.iter().next() {
        let side = match bounce.last_court_side {
            Some(CourtSide::Left) => "P1",
            Some(CourtSide::Right) => "P2",
            None => "-",
        };
        format!("Bounce: {} ({})", bounce.count, side)
    } else {
        "Ball: None".to_string()
    }
}

/// プレイヤー状態テキストを生成
fn format_player_states(player_query: &Query<(&Player, &KnockbackState, &ShotState)>) -> String {
    let mut states = Vec::new();
    for (player, knockback, shot_state) in player_query.iter() {
        let knockback_icon = if knockback.is_knockback_active() {
            "KNOCKED"
        } else if knockback.is_invincible() {
            "INVINCIBLE"
        } else {
            "OK"
        };
        let shot_icon = if shot_state.is_on_cooldown() {
            format!("CD:{:.1}", shot_state.cooldown_timer)
        } else {
            "READY".to_string()
        };
        states.push(format!(
            "P{}: {} | Shot: {}",
            player.id, knockback_icon, shot_icon
        ));
    }
    states.join("\n")
}

/// デバッグUI更新システム
fn update_debug_ui(
    match_score: Res<MatchScore>,
    rally_state: Res<RallyState>,
    config: Res<GameConfig>,
    ball_query: Query<&BounceCount, With<Ball>>,
    player_query: Query<(&Player, &KnockbackState, &ShotState)>,
    mut text_query: Query<&mut Text, With<DebugUiText>>,
) {
    let Some(mut text) = text_query.iter_mut().next() else {
        return;
    };

    let score_text = format_score_text(&match_score, &config.scoring.point_values);
    let phase_info = format_phase_info(&rally_state);
    let bounce_info = format_bounce_info(&ball_query);
    let player_states = format_player_states(&player_query);

    **text = format!(
        "{}\n{}\n{}\n{}",
        score_text, phase_info, bounce_info, player_states
    );
}

/// 当たり判定（球体）をGizmosで描画
/// 論理座標系: X=打ち合い方向, Y=高さ, Z=コート幅
/// 画面座標系: X=打ち合い方向, Y=コート幅
fn draw_hit_range_gizmos(
    mut gizmos: Gizmos,
    config: Res<GameConfig>,
    player_query: Query<(&Player, &Transform)>,
    ball_query: Query<&Transform, With<Ball>>,
) {
    let hit_radius = config.shot.max_distance * WORLD_SCALE;

    // 各プレイヤーの当たり判定範囲を描画
    for (player, transform) in player_query.iter() {
        let color = if player.id == 1 {
            Color::srgba(0.0, 1.0, 1.0, 0.5) // シアン（半透明）
        } else {
            Color::srgba(1.0, 0.5, 0.0, 0.5) // オレンジ（半透明）
        };

        // 2D画面上での円を描画（キャラクターの足元位置を中心に）
        gizmos.circle_2d(
            Isometry2d::from_translation(transform.translation.truncate()),
            hit_radius,
            color,
        );
    }

    // ボールの位置にも小さいマーカーを描画
    for ball_transform in ball_query.iter() {
        let ball_pos = ball_transform.translation.truncate();
        gizmos.circle_2d(
            Isometry2d::from_translation(ball_pos),
            config.ball.radius * WORLD_SCALE,
            Color::srgba(1.0, 1.0, 0.0, 0.8), // 黄色
        );
    }
}

/// ラケットスイングの可視化を描画
/// @spec 31003_racket_trajectory_spec.md#req-31003-003
fn draw_racket_swing_gizmos(mut gizmos: Gizmos, player_query: Query<(&Player, &ShotState)>) {
    for (player, shot_state) in player_query.iter() {
        let swing = &shot_state.racket_swing;
        if !swing.is_active {
            continue;
        }

        let color = if player.id == 1 {
            Color::srgba(1.0, 0.2, 0.2, 0.9)
        } else {
            Color::srgba(0.2, 0.8, 1.0, 0.9)
        };

        let prev = logical_to_display_2d(swing.previous_racket_position);
        let curr = logical_to_display_2d(swing.current_racket_position);
        let hit = logical_to_display_2d(swing.planned_hit_position);

        gizmos.line_2d(prev, curr, color);
        gizmos.circle_2d(Isometry2d::from_translation(curr), 6.0, color);
        gizmos.circle_2d(
            Isometry2d::from_translation(hit),
            4.0,
            Color::srgba(1.0, 1.0, 1.0, 0.9),
        );
    }
}

#[inline]
fn logical_to_display_2d(pos: Vec3) -> Vec2 {
    Vec2::new(pos.x * WORLD_SCALE, (pos.z + pos.y) * WORLD_SCALE)
}
