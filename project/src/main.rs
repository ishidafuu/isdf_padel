//! Padel Game - MVP v0.1
//! @spec 20000_overview.md

mod components;
mod core;
mod presentation;
mod resource;
mod systems;

use bevy::{asset::AssetPlugin, prelude::*};
use components::{AiController, HumanControlled, PlayerBundle};
use core::{
    BallHitEvent, PlayerJumpEvent, PlayerKnockbackEvent, PlayerLandEvent, PlayerMoveEvent,
    ShotEvent, ShotExecutedEvent,
};
use presentation::{
    ball_spin_color_system, despawn_ball_shadow_system, player_hold_visual_system,
    save_player_original_color_system, spawn_ball_shadow_system, spawn_player_shadow_system,
    sync_shadow_system, sync_transform_system, DebugUiPlugin, WORLD_SCALE,
};
use resource::config::{load_game_config, GameConfig, GameConfigHandle, GameConfigLoader};
use resource::MatchFlowState;
use systems::{
    ai_movement_system, ai_shot_system, ceiling_collision_system, gravity_system,
    human_input_system, jump_system, knockback_movement_system, knockback_start_system,
    knockback_timer_system, landing_system, movement_system, shot_cooldown_system,
    shot_direction_system, shot_input_system, vertical_movement_system, BallCollisionPlugin,
    BallTrajectoryPlugin, BoundaryPlugin, FaultJudgmentPlugin, MatchFlowPlugin,
    PointJudgmentPlugin, ScoringPlugin,
};

fn main() {
    // GameConfig をロード（初回起動用、アセットシステム起動前に同期的に読み込み）
    let config = load_game_config("assets/config/game_config.ron")
        .expect("Failed to load game config");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Padel Game - MVP v0.1".into(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                })
                // ホットリロード有効化
                // @spec 30026: GameConfig ホットリロード対応
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                }),
        )
        // GameConfig の Asset 登録
        .init_asset::<GameConfig>()
        .init_asset_loader::<GameConfigLoader>()
        .add_plugins(BoundaryPlugin)
        .add_plugins(BallTrajectoryPlugin)
        .add_plugins(BallCollisionPlugin)
        .add_plugins(ScoringPlugin)
        .add_plugins(PointJudgmentPlugin)
        .add_plugins(FaultJudgmentPlugin)
        .add_plugins(MatchFlowPlugin)
        .add_plugins(DebugUiPlugin)
        .insert_resource(config)
        .add_message::<PlayerMoveEvent>()
        .add_message::<PlayerJumpEvent>()
        .add_message::<PlayerLandEvent>()
        .add_message::<BallHitEvent>()
        .add_message::<PlayerKnockbackEvent>()
        .add_message::<ShotEvent>()
        .add_message::<ShotExecutedEvent>()
        .add_systems(Startup, (setup, load_config_asset))
        .add_systems(
            Update,
            (
                // 設定ホットリロード
                update_config_on_change,
                // 人間入力読み取り（HumanControlled を持つプレイヤーの InputState を更新）
                // @spec 20006_input_system.md
                human_input_system,
                // ふっとばし開始（BallHitEvent を処理）
                knockback_start_system,
                // ジャンプ・重力
                (jump_system, gravity_system, vertical_movement_system).chain(),
                // 水平移動（ふっとばし中はスキップ）
                movement_system,
                // AI移動（AiController を持つプレイヤー）
                // @spec 30301_ai_movement_spec.md
                ai_movement_system,
                // ショット入力処理（Rally状態でのみ動作 - サーブ中は shot_input_system を動かさない）
                shot_input_system.run_if(in_state(MatchFlowState::Rally)),
                // AIショット処理（Rally状態でのみ動作）
                // @spec 30302_ai_shot_spec.md
                ai_shot_system.run_if(in_state(MatchFlowState::Rally)),
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
                // 視覚フィードバック（色変化）
                // @spec 30802_visual_feedback_spec.md
                (save_player_original_color_system, player_hold_visual_system, ball_spin_color_system),
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

    // Player 1 をスポーン（1Pコート側: 画面下部）- 人間操作
    // 論理座標系: X=左右, Y=高さ, Z=奥行き
    // @spec 20006_input_system.md
    let player1_pos = Vec3::new(
        0.0,
        0.0, // 地面
        config.player.z_min + 1.0,
    );
    commands.spawn((
        PlayerBundle::new(1, player1_pos),
        HumanControlled::default(),
    ));
    info!("Player 1 (Human) spawned at {:?}", player1_pos);

    // Player 2 をスポーン（2Pコート側: 画面上部）- AI制御
    // @spec 30301_ai_movement_spec.md
    let player2_pos = Vec3::new(
        0.0,
        0.0, // 地面
        config.player.z_max - 1.0,
    );
    let home_position = Vec3::new(
        config.ai.home_position_x,
        0.0,
        config.ai.home_position_z,
    );
    commands.spawn((
        PlayerBundle::new(2, player2_pos),
        AiController { home_position },
    ));
    info!("Player 2 (AI) spawned at {:?}", player2_pos);
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

// ============================================================================
// ホットリロード対応
// @spec 30026: GameConfig ホットリロード対応
// ============================================================================

/// GameConfig をロードしてハンドルを保持
fn load_config_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<GameConfig> = asset_server.load("config/game_config.ron");
    commands.insert_resource(GameConfigHandle(handle));
    info!("GameConfig loading started (hot-reload enabled)");
}

/// 設定ファイル変更時に GameConfig リソースを更新
#[allow(deprecated)] // EventReader は Bevy 0.17 で MessageReader にリネーム予定
fn update_config_on_change(
    mut events: EventReader<AssetEvent<GameConfig>>,
    assets: Res<Assets<GameConfig>>,
    handle: Option<Res<GameConfigHandle>>,
    mut config: ResMut<GameConfig>,
) {
    let Some(handle) = handle else { return };

    for event in events.read() {
        if let AssetEvent::Modified { id } = event {
            if handle.0.id() == *id {
                if let Some(new_config) = assets.get(*id) {
                    *config = new_config.clone();
                    info!("GameConfig hot-reloaded!");
                }
            }
        }
    }
}
