//! Padel Game - MVP v0.1
//! @spec 20000_overview.md

mod character;
mod components;
mod core;
mod presentation;
mod replay;
mod resource;
mod simulation;
mod systems;

use bevy::{asset::AssetPlugin, prelude::*};
use character::CharacterPlugin;
use components::HumanControlled;
use core::{
    BallHitEvent, FaultEvent, GroundBounceEvent, PlayerJumpEvent, PlayerKnockbackEvent,
    PlayerLandEvent, PlayerMoveEvent, PointScoredEvent, RallyEndEvent, ShotAttributesCalculatedEvent,
    ShotEvent, ShotExecutedEvent, WallReflectionEvent,
};
use presentation::{
    ball_spin_color_system, despawn_ball_shadow_system, player_hold_visual_system,
    save_player_original_color_system, spawn_ball_shadow_system, spawn_player_shadow_system,
    sync_shadow_system, sync_transform_system, DebugUiPlugin, WORLD_SCALE,
};
use resource::config::{load_game_config, GameConfig, GameConfigHandle, GameConfigLoader};
use resource::debug::LastShotDebugInfo;
use resource::{FixedDeltaTime, GameRng};
use resource::MatchFlowState;
use systems::{
    ai_movement_system, ai_shot_system, ceiling_collision_system, debug_marker_system,
    gamepad_input_system, gravity_system, human_input_system, jump_system,
    knockback_movement_system, knockback_start_system, knockback_timer_system, landing_system,
    movement_system, shot_cooldown_system, shot_direction_system, shot_input_system,
    vertical_movement_system, AiServePlugin, BallCollisionPlugin, BallTrajectoryPlugin,
    BoundaryPlugin, FaultJudgmentPlugin, GameSystemSet, MatchFlowPlugin, PointJudgmentPlugin,
    ScoringPlugin,
};

fn main() {
    // GameConfig をロード（初回起動用、アセットシステム起動前に同期的に読み込み）
    let config = load_game_config("assets/config/game_config.ron")
        .expect("Failed to load game config");

    let mut app = App::new();

    // Bevy基本プラグイン・アセット登録
    add_default_plugins(&mut app);

    // ゲームプラグイン追加
    add_game_plugins(&mut app);

    // リソース・メッセージ初期化
    add_resources(&mut app, config);
    add_messages(&mut app);

    // システム登録
    app.add_systems(Startup, (setup, load_config_asset));
    app.configure_sets(Update, GameSystemSet::Input.before(GameSystemSet::GameLogic));
    app.add_systems(Update, escape_to_exit);
    add_input_systems(&mut app);
    add_game_logic_systems(&mut app);

    app.run();
}

/// DefaultPlugins とアセット登録
fn add_default_plugins(app: &mut App) {
    app.add_plugins(
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
    );

    // GameConfig の Asset 登録
    app.init_asset::<GameConfig>()
        .init_asset_loader::<GameConfigLoader>();
}

/// リソースを初期化
fn add_resources(app: &mut App, config: GameConfig) {
    app.insert_resource(config)
        .init_resource::<FixedDeltaTime>()
        .init_resource::<GameRng>()
        .init_resource::<LastShotDebugInfo>();
}

/// ゲームプラグインを追加
fn add_game_plugins(app: &mut App) {
    app.add_plugins(BoundaryPlugin)
        .add_plugins(BallTrajectoryPlugin)
        .add_plugins(BallCollisionPlugin)
        .add_plugins(ScoringPlugin)
        .add_plugins(PointJudgmentPlugin)
        .add_plugins(FaultJudgmentPlugin)
        .add_plugins(MatchFlowPlugin)
        // @spec 30102_serve_spec.md#req-30102-070: AI自動サーブ
        .add_plugins(AiServePlugin)
        .add_plugins(DebugUiPlugin)
        .add_plugins(CharacterPlugin)
        // @spec 77103_replay_spec.md: リプレイ記録機能
        .add_plugins(replay::ReplayRecordPlugin);
}

/// メッセージ（イベント）を登録
fn add_messages(app: &mut App) {
    app.add_message::<PlayerMoveEvent>()
        .add_message::<PlayerJumpEvent>()
        .add_message::<PlayerLandEvent>()
        .add_message::<BallHitEvent>()
        .add_message::<PlayerKnockbackEvent>()
        .add_message::<ShotEvent>()
        .add_message::<ShotExecutedEvent>()
        // トレース用イベント
        .add_message::<ShotAttributesCalculatedEvent>()
        .add_message::<GroundBounceEvent>()
        .add_message::<WallReflectionEvent>()
        .add_message::<RallyEndEvent>()
        .add_message::<PointScoredEvent>()
        .add_message::<FaultEvent>();
}

/// 入力システムを追加（Input セット）
fn add_input_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            // 人間入力読み取り（HumanControlled を持つプレイヤーの InputState を更新）
            // @spec 20006_input_system.md
            human_input_system,
            // ゲームパッド入力読み取り（device_id=1 の HumanControlled）
            // @spec 20006_input_system.md#req-20006-050
            gamepad_input_system,
        )
            .chain()
            .in_set(GameSystemSet::Input),
    );
}

/// ゲームロジックシステムを追加（GameLogic セット）
fn add_game_logic_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            // 設定ホットリロード
            update_config_on_change,
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
            // デバッグマーカー（Xボタンで弾道情報をログ出力）
            debug_marker_system,
        )
            .chain()
            .in_set(GameSystemSet::GameLogic),
    );
}

/// 初期セットアップ
/// @spec 30200_overview.md
fn setup(mut commands: Commands, config: Res<GameConfig>) {
    // Camera2d をスポーン
    commands.spawn(Camera2d);

    // GameConfig のロード確認
    info!("GameConfig loaded successfully!");
    info!("Court size: {}x{}", config.court.width, config.court.depth);
    info!("Player move speed: X={}, Z={}", config.player.move_speed, config.player.move_speed_z);

    // コート境界を描画
    spawn_court(&mut commands, &config);

    // Player 1 をスポーン（1Pコート側: 画面左側）- 人間操作
    // 論理座標系: X=打ち合い方向, Y=高さ, Z=コート幅
    // @spec 20006_input_system.md
    // @spec 31001_parts_spec.md - パーツ分離キャラクター
    let player1_pos = Vec3::new(
        config.player.x_min + 1.0, // 1Pコート側（-X方向）
        0.0,                       // 地面
        0.0,                       // コート中央
    );
    let (r, g, b) = config.player_visual.player1_color;
    let player1_color = Color::srgb(r, g, b);
    let player1_entity = character::spawn_articulated_player(&mut commands, 1, player1_pos, player1_color);
    commands.entity(player1_entity).insert(HumanControlled::default());
    info!("Player 1 (Human/Articulated) spawned at {:?}", player1_pos);

    // Player 2 をスポーン（2Pコート側: 画面右側）- AI操作
    // @spec 30301_ai_movement_spec.md
    // @spec 31001_parts_spec.md - パーツ分離キャラクター
    let player2_pos = Vec3::new(
        config.player.x_max - 1.0, // 2Pコート側（+X方向）
        0.0,                       // 地面
        0.0,                       // コート中央
    );
    let (r, g, b) = config.player_visual.player2_color;
    let player2_color = Color::srgb(r, g, b);
    let player2_entity = character::spawn_articulated_player(&mut commands, 2, player2_pos, player2_color);
    commands.entity(player2_entity).insert(components::AiController {
        home_position: player2_pos,
        target_position: player2_pos,
        ..Default::default()
    });
    info!("Player 2 (AI) spawned at {:?}", player2_pos);

}


/// スプライト（矩形）を生成するヘルパー
fn spawn_rect(commands: &mut Commands, width: f32, height: f32, x: f32, y: f32, z: f32, color: Color) {
    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        Transform::from_xyz(x, y, z),
    ));
}

/// コートの境界線とネットを描画（横向き：左右の打ち合い）
fn spawn_court(commands: &mut Commands, config: &GameConfig) {
    // 直接マッピング：論理X（depth/打ち合い方向）→ 画面X、論理Z（width/コート幅）→ 画面Y
    let court_display_width = config.court.depth * WORLD_SCALE;  // 画面X方向（打ち合い方向）
    let court_display_height = config.court.width * WORLD_SCALE; // 画面Y方向（コート幅）
    let half_w = court_display_width / 2.0;
    let half_h = court_display_height / 2.0;
    // @spec 30501_court_spec.md#req-30501-008
    let service_x = config.court.service_box_depth * WORLD_SCALE;

    let white = Color::srgb(1.0, 1.0, 1.0);
    let line = 4.0;

    // コート背景（緑）
    spawn_rect(commands, court_display_width, court_display_height, 0.0, 0.0, -1.0, Color::srgb(0.2, 0.5, 0.2));
    // ネット（白い線、中央の縦線）
    spawn_rect(commands, line, court_display_height, 0.0, 0.0, 0.0, white);
    // 境界線（左右上下）
    spawn_rect(commands, line, court_display_height, -half_w, 0.0, 0.0, white); // 左（1P側）
    spawn_rect(commands, line, court_display_height, half_w, 0.0, 0.0, white);  // 右（2P側）
    spawn_rect(commands, court_display_width, line, 0.0, half_h, 0.0, white);   // 上
    spawn_rect(commands, court_display_width, line, 0.0, -half_h, 0.0, white);  // 下
    // サービスライン（縦線）
    spawn_rect(commands, line, court_display_height, -service_x, 0.0, 0.0, white); // 1P側
    spawn_rect(commands, line, court_display_height, service_x, 0.0, 0.0, white);  // 2P側
    // センターサービスライン（ネットからサービスラインまでの中央線）
    spawn_rect(commands, service_x, line, -service_x / 2.0, 0.0, 0.0, white); // 1P側
    spawn_rect(commands, service_x, line, service_x / 2.0, 0.0, 0.0, white);  // 2P側

    info!("Court spawned: {}x{} pixels (直接マッピング)", court_display_width, court_display_height);
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

/// エスケープキーでウィンドウを閉じる
fn escape_to_exit(keyboard: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
