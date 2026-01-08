//! デバッグUI表示システム
//! ゲーム状態を画面に表示

use bevy::prelude::*;

use crate::components::{Ball, BounceCount, KnockbackState, LogicalPosition, Player, ShotState};
use crate::core::CourtSide;
use crate::presentation::WORLD_SCALE;
use crate::resource::{GameConfig, MatchScore, RallyPhase, RallyState};

/// デバッグUIプラグイン
pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_debug_ui, setup_shot_range_visuals))
            .add_systems(Update, (update_debug_ui, update_shot_range_visuals));
    }
}

/// ショット判定枠マーカー
#[derive(Component)]
pub struct ShotRangeVisual {
    /// 追従するプレイヤーEntity
    pub owner: Entity,
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

    let p1_score = match_score.get_score(CourtSide::Player1);
    let p2_score = match_score.get_score(CourtSide::Player2);

    let score_text = format!(
        "Score: {} - {} (G: {}-{}, S: {}-{})",
        p1_point,
        p2_point,
        p1_score.games,
        p2_score.games,
        p1_score.sets,
        p2_score.sets,
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

/// ショット判定枠のセットアップ（プレイヤー生成時に呼ばれる）
fn setup_shot_range_visuals(
    mut commands: Commands,
    config: Res<GameConfig>,
    player_query: Query<Entity, With<Player>>,
) {
    let range_size = config.shot.max_distance * 2.0 * WORLD_SCALE;

    for player_entity in player_query.iter() {
        // ショット判定枠（半透明の矩形）
        commands.spawn((
            ShotRangeVisual {
                owner: player_entity,
            },
            Sprite {
                color: Color::srgba(1.0, 1.0, 0.0, 0.2), // 半透明の黄色
                custom_size: Some(Vec2::new(range_size, range_size)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.1), // プレイヤーより奥
        ));
    }
}

/// ショット判定枠の位置を更新
fn update_shot_range_visuals(
    config: Res<GameConfig>,
    player_query: Query<(Entity, &LogicalPosition), With<Player>>,
    mut visual_query: Query<(&ShotRangeVisual, &mut Transform, &mut Sprite)>,
    mut commands: Commands,
) {
    let range_size = config.shot.max_distance * 2.0 * WORLD_SCALE;

    // 既存のビジュアルを更新
    for (visual, mut transform, mut sprite) in visual_query.iter_mut() {
        if let Ok((_, logical_pos)) = player_query.get(visual.owner) {
            // プレイヤーの地面位置に追従（sync_transform_systemと同じ座標変換）
            let display_x = logical_pos.value.x * WORLD_SCALE;
            let display_y = logical_pos.value.z * WORLD_SCALE + logical_pos.value.y * WORLD_SCALE;

            transform.translation.x = display_x;
            transform.translation.y = display_y;

            // サイズ更新（設定変更に対応）
            sprite.custom_size = Some(Vec2::new(range_size, range_size));
        }
    }

    // 新規プレイヤー用のビジュアルを生成
    for (player_entity, _) in player_query.iter() {
        let has_visual = visual_query.iter().any(|(v, _, _)| v.owner == player_entity);
        if !has_visual {
            commands.spawn((
                ShotRangeVisual {
                    owner: player_entity,
                },
                Sprite {
                    color: Color::srgba(1.0, 1.0, 0.0, 0.2),
                    custom_size: Some(Vec2::new(range_size, range_size)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.1),
            ));
        }
    }
}
