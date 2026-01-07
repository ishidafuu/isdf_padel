//! Padel Game - MVP v0.1
//! @spec 20000_overview.md

mod components;
mod core;
mod presentation;
mod resource;
mod systems;

use bevy::prelude::*;
use components::PlayerBundle;
use core::{
    BallHitEvent, PlayerJumpEvent, PlayerKnockbackEvent, PlayerLandEvent, PlayerMoveEvent,
    ShotEvent, ShotExecutedEvent,
};
use presentation::{
    despawn_ball_shadow_system, spawn_ball_shadow_system, spawn_player_shadow_system,
    sync_shadow_system, sync_transform_system, DebugUiPlugin, WORLD_SCALE,
};
use resource::config::{load_game_config, GameConfig};
use resource::MatchFlowState;
use systems::{
    ceiling_collision_system, gravity_system, jump_system, knockback_movement_system,
    knockback_start_system, knockback_timer_system, landing_system, movement_system,
    read_input_system, read_jump_input_system, read_shot_input_system, shot_cooldown_system,
    shot_direction_system, shot_input_system, vertical_movement_system, BallCollisionPlugin,
    BallTrajectoryPlugin, BoundaryPlugin, FaultJudgmentPlugin, JumpInput, MatchFlowPlugin,
    MovementInput, PointJudgmentPlugin, ScoringPlugin, ShotInput,
};

fn main() {
    // GameConfig をロード
    let config = load_game_config("assets/config/game_config.ron")
        .expect("Failed to load game config");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Padel Game - MVP v0.1".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BoundaryPlugin)
        .add_plugins(BallTrajectoryPlugin)
        .add_plugins(BallCollisionPlugin)
        .add_plugins(ScoringPlugin)
        .add_plugins(PointJudgmentPlugin)
        .add_plugins(FaultJudgmentPlugin)
        .add_plugins(MatchFlowPlugin)
        .add_plugins(DebugUiPlugin)
        .insert_resource(config)
        .init_resource::<MovementInput>()
        .init_resource::<JumpInput>()
        .init_resource::<ShotInput>()
        .add_message::<PlayerMoveEvent>()
        .add_message::<PlayerJumpEvent>()
        .add_message::<PlayerLandEvent>()
        .add_message::<BallHitEvent>()
        .add_message::<PlayerKnockbackEvent>()
        .add_message::<ShotEvent>()
        .add_message::<ShotExecutedEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                // 入力読み取り
                (read_input_system, read_jump_input_system, read_shot_input_system),
                // ふっとばし開始（BallHitEvent を処理）
                knockback_start_system,
                // ジャンプ・重力
                (jump_system, gravity_system, vertical_movement_system).chain(),
                // 水平移動（ふっとばし中はスキップ）
                movement_system,
                // ショット入力処理（Rally状態でのみ動作 - サーブ中は shot_input_system を動かさない）
                shot_input_system.run_if(in_state(MatchFlowState::Rally)),
                // 方向計算・クールダウン（常に動作 - サーブの ShotEvent も処理する）
                (shot_direction_system, shot_cooldown_system),
                // ふっとばし移動・タイマー
                (knockback_movement_system, knockback_timer_system),
                // 境界チェック
                (ceiling_collision_system, landing_system),
                // 座標変換（論理座標→表示座標）
                sync_transform_system,
                // 影システム
                (spawn_ball_shadow_system, spawn_player_shadow_system, sync_shadow_system, despawn_ball_shadow_system),
            )
                .chain(),
        )
        .run();
}

/// 初期セットアップ
/// @spec 30200_player_overview.md
fn setup(mut commands: Commands, config: Res<GameConfig>) {
    // Camera2d をスポーン
    commands.spawn(Camera2d);

    // GameConfig のロード確認
    info!("GameConfig loaded successfully!");
    info!("Court size: {}x{}", config.court.width, config.court.depth);
    info!("Player move speed: X={}, Z={}", config.player.move_speed, config.player.move_speed_z);

    // コート境界を描画
    spawn_court(&mut commands, &config);

    // Player 1 をスポーン（1Pコート側: 画面下部）
    // 論理座標系: X=左右, Y=高さ, Z=奥行き
    let player1_pos = Vec3::new(
        0.0,
        0.0, // 地面
        config.player.z_min + 1.0,
    );
    commands.spawn(PlayerBundle::new(1, player1_pos));
    info!("Player 1 spawned at {:?}", player1_pos);

    // Player 2 をスポーン（2Pコート側: 画面上部）
    let player2_pos = Vec3::new(
        0.0,
        0.0, // 地面
        config.player.z_max - 1.0,
    );
    commands.spawn(PlayerBundle::new(2, player2_pos));
    info!("Player 2 spawned at {:?}", player2_pos);
}

/// コートの境界線とネットを描画（横向き：左右の打ち合い）
fn spawn_court(commands: &mut Commands, config: &GameConfig) {
    // 90度回転：論理Z（depth）→ 画面X、論理X（width）→ 画面Y
    let court_display_width = config.court.depth * WORLD_SCALE;  // 画面X方向
    let court_display_height = config.court.width * WORLD_SCALE; // 画面Y方向
    let half_display_width = court_display_width / 2.0;
    let half_display_height = court_display_height / 2.0;

    // コート背景（緑）
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.5, 0.2),
            custom_size: Some(Vec2::new(court_display_width, court_display_height)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    // ネット（白い線、中央の縦線）
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(4.0, court_display_height)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 左境界線（1Pコート側）
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(4.0, court_display_height)),
            ..default()
        },
        Transform::from_xyz(-half_display_width, 0.0, 0.0),
    ));

    // 右境界線（2Pコート側）
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(4.0, court_display_height)),
            ..default()
        },
        Transform::from_xyz(half_display_width, 0.0, 0.0),
    ));

    // 上境界線
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(court_display_width, 4.0)),
            ..default()
        },
        Transform::from_xyz(0.0, half_display_height, 0.0),
    ));

    // 下境界線
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(court_display_width, 4.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -half_display_height, 0.0),
    ));

    // サービスライン描画
    // @spec 30501_court_spec.md#req-30501-008
    let service_line_x = config.court.service_box_depth * WORLD_SCALE;

    // 1P側サービスライン（縦線）
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(4.0, court_display_height)),
            ..default()
        },
        Transform::from_xyz(-service_line_x, 0.0, 0.0),
    ));

    // 2P側サービスライン（縦線）
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(4.0, court_display_height)),
            ..default()
        },
        Transform::from_xyz(service_line_x, 0.0, 0.0),
    ));

    // センターサービスライン（ネットからサービスラインまでの中央線）
    // 1P側センターライン
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(service_line_x, 4.0)),
            ..default()
        },
        Transform::from_xyz(-service_line_x / 2.0, 0.0, 0.0),
    ));

    // 2P側センターライン
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(service_line_x, 4.0)),
            ..default()
        },
        Transform::from_xyz(service_line_x / 2.0, 0.0, 0.0),
    ));

    info!("Court spawned: {}x{} pixels (横向き)", court_display_width, court_display_height);
}
