//! デバッグUI表示システム
//! ゲーム状態を画面に表示

use bevy::prelude::*;

use crate::components::{Ball, BounceCount, KnockbackState, Player, ShotState};
use crate::core::CourtSide;
use crate::resource::{GameConfig, MatchScore, RallyPhase, RallyState};

/// デバッグUIプラグイン
pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_debug_ui)
            .add_systems(Update, update_debug_ui);
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

    let point_values = &config.scoring.point_values;

    // スコア表示
    let p1_point = match_score.get_point_display(CourtSide::Player1, point_values);
    let p2_point = match_score.get_point_display(CourtSide::Player2, point_values);

    let score_text = format!(
        "Score: {} - {} (G: {}-{}, S: {}-{})",
        p1_point,
        p2_point,
        match_score.player1_score.games,
        match_score.player2_score.games,
        match_score.player1_score.sets,
        match_score.player2_score.sets,
    );

    // ラリー状態表示
    let phase_text = match rally_state.phase {
        RallyPhase::WaitingServe => "WAITING SERVE",
        RallyPhase::Serving => "SERVING",
        RallyPhase::Rally => "RALLY",
        RallyPhase::PointEnded => "POINT END",
    };

    let server_text = match rally_state.server {
        CourtSide::Player1 => "P1",
        CourtSide::Player2 => "P2",
    };

    // ボールバウンス状態
    let bounce_text = if let Some(bounce) = ball_query.iter().next() {
        let side = match bounce.last_court_side {
            Some(CourtSide::Player1) => "P1",
            Some(CourtSide::Player2) => "P2",
            None => "-",
        };
        format!("Bounce: {} ({})", bounce.count, side)
    } else {
        "Ball: None".to_string()
    };

    // プレイヤー状態
    let mut player_states = Vec::new();
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

        player_states.push(format!(
            "P{}: {} | Shot: {}",
            player.id, knockback_icon, shot_icon
        ));
    }

    // テキスト組み立て
    let full_text = format!(
        "{}\nPhase: {} | Server: {} | Fault: {}\n{}\n{}",
        score_text,
        phase_text,
        server_text,
        rally_state.fault_count,
        bounce_text,
        player_states.join("\n")
    );

    **text = full_text;
}
