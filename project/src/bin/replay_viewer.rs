//! Replay Viewer CLI
//! @spec 77103_replay_spec.md REQ-77103-050
//!
//! リプレイファイルを同一シードで再シミュレーションし、視覚的に表示する。
//! 注意: これは入力再生ではなく、同一シードによる再シミュレーションです。
//!
//! # Usage
//!
//! ```bash
//! # リプレイを表示付きで再生
//! cargo run --bin replay_viewer -- assets/replays/replay_20260110_123456.replay
//!
//! # 詳細出力付き
//! cargo run --bin replay_viewer -- -v assets/replays/replay_20260110_123456.replay
//! ```

use bevy::prelude::*;
use clap::Parser;

use padel_game::character::{self, CharacterPlugin};
use padel_game::components::AiController;
use padel_game::replay::data::{ControlType, ReplayData};
use padel_game::core::{
    BallHitEvent, PlayerJumpEvent, PlayerKnockbackEvent, PlayerLandEvent, PlayerMoveEvent,
    ShotEvent, ShotExecutedEvent,
};
use padel_game::presentation::{
    ball_spin_color_system, despawn_ball_shadow_system, player_hold_visual_system,
    save_player_original_color_system, spawn_ball_shadow_system, spawn_player_shadow_system,
    sync_shadow_system, sync_transform_system, DebugUiPlugin, WORLD_SCALE,
};
use padel_game::replay::loader::load_replay;
use padel_game::replay::player::{replay_input_system, ReplayPlayer};
use padel_game::resource::config::{load_game_config, GameConfig};
use padel_game::resource::debug::LastShotDebugInfo;
use padel_game::resource::{FixedDeltaTime, GameRng, MatchFlowState};
use padel_game::simulation::AnomalyDetectorPlugin;
use padel_game::systems::{
    ai_movement_system, ai_shot_system, ceiling_collision_system, gravity_system, jump_system,
    knockback_movement_system, knockback_start_system, knockback_timer_system, landing_system,
    movement_system, shot_cooldown_system, shot_direction_system, shot_input_system,
    vertical_movement_system, AiServePlugin, BallCollisionPlugin, BallTrajectoryPlugin,
    BoundaryPlugin, FaultJudgmentPlugin, GameSystemSet, MatchFlowPlugin, PointJudgmentPlugin,
    ScoringPlugin,
};

/// Replay Viewer for Padel Game
#[derive(Parser, Debug)]
#[command(author, version, about = "Replay viewer with visual playback for Padel Game")]
struct Args {
    /// Path to the replay file
    replay_file: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Skip version check
    #[arg(long)]
    skip_version_check: bool,
}

fn main() {
    let args = Args::parse();

    println!("=== Padel Game Replay Viewer ===\n");
    println!("Replay file: {}", args.replay_file);

    // リプレイをロード
    let replay_data = if args.skip_version_check {
        padel_game::replay::loader::load_replay_unchecked(&args.replay_file)
    } else {
        load_replay(&args.replay_file)
    };

    let replay_data = match replay_data {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to load replay: {}", e);
            std::process::exit(1);
        }
    };

    // メタデータを表示
    println!("\nReplay metadata:");
    println!("  Version: {}", replay_data.metadata.game_version);
    println!("  Recorded at: {}", replay_data.metadata.recorded_at);
    println!("  Seed: {}", replay_data.metadata.seed);
    println!(
        "  Initial serve side: {:?}",
        replay_data.metadata.initial_serve_side
    );
    println!("  Total frames: {}", replay_data.frames.len());
    println!();

    // GameConfig をロード
    let game_config = match load_game_config("assets/config/game_config.ron") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load game config: {}", e);
            std::process::exit(1);
        }
    };

    // リプレイのシード値でGameRngを初期化（AI動作の再現性確保）
    // 同一シードにより、元の試合と同じ結果を再シミュレーション
    let game_rng = GameRng::from_seed(replay_data.metadata.seed);

    // Bevy アプリを構築して実行
    let mut app = App::new();

    // DefaultPlugins（ウィンドウ表示付き）
    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Padel Game - Replay Viewer".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }),
    );

    // StatesPlugin（State管理用、DefaultPluginsに含まれている可能性があるがredundantでも問題なし）
    // 念のため追加しない（DefaultPluginsに含まれている）

    // ゲームロジックプラグイン
    app.add_plugins(ReplayViewerPlugins);

    // リソース挿入
    app.insert_resource(game_config)
        .insert_resource(game_rng)
        .init_resource::<FixedDeltaTime>()
        .insert_resource(ReplayViewerConfig {
            verbose: args.verbose,
        })
        .insert_resource(replay_data)
        .init_resource::<ReplayPlayer>();

    // 完了検出用の状態とエスケープキー
    app.add_systems(Update, (check_replay_finished, escape_to_exit));

    println!("Starting replay playback with visual display...\n");
    println!("Press ESC to exit.\n");

    // 実行
    app.run();
}

/// リプレイビューア設定
#[derive(Resource)]
struct ReplayViewerConfig {
    verbose: bool,
}

/// リプレイビューア用プラグインセット
struct ReplayViewerPlugins;

impl Plugin for ReplayViewerPlugins {
    fn build(&self, app: &mut App) {
        // ゲームロジックプラグイン
        app.add_plugins(BoundaryPlugin)
            .add_plugins(BallTrajectoryPlugin)
            .add_plugins(BallCollisionPlugin)
            .add_plugins(ScoringPlugin)
            .add_plugins(PointJudgmentPlugin)
            .add_plugins(FaultJudgmentPlugin)
            .add_plugins(MatchFlowPlugin)
            .add_plugins(AiServePlugin)
            .add_plugins(CharacterPlugin)
            .add_plugins(AnomalyDetectorPlugin)
            .add_plugins(DebugUiPlugin);

        // リソース初期化
        app.init_resource::<LastShotDebugInfo>();

        // イベント登録
        app.add_message::<PlayerMoveEvent>()
            .add_message::<PlayerJumpEvent>()
            .add_message::<PlayerLandEvent>()
            .add_message::<BallHitEvent>()
            .add_message::<PlayerKnockbackEvent>()
            .add_message::<ShotEvent>()
            .add_message::<ShotExecutedEvent>();

        // SystemSet の順序を設定
        app.configure_sets(Update, GameSystemSet::Input.before(GameSystemSet::GameLogic));

        // セットアップシステム
        app.add_systems(Startup, setup_replay_viewer);

        // 入力システム（Human プレイヤーの入力注入）
        app.add_systems(
            Update,
            replay_input_system.in_set(GameSystemSet::Input),
        );

        // ゲームロジックシステム（AI による再シミュレーション）
        app.add_systems(
            Update,
            (
                // ふっとばし開始
                knockback_start_system,
                // ジャンプ・重力
                (jump_system, gravity_system, vertical_movement_system).chain(),
                // 水平移動
                movement_system,
                // AI移動（AiController を持つプレイヤー）
                ai_movement_system,
                // ショット入力処理（Rally状態でのみ）
                shot_input_system.run_if(in_state(MatchFlowState::Rally)),
                // AIショット処理（Rally状態でのみ）
                ai_shot_system.run_if(in_state(MatchFlowState::Rally)),
                // 方向計算・クールダウン
                (shot_direction_system, shot_cooldown_system),
                // ふっとばし移動・タイマー
                (knockback_movement_system, knockback_timer_system),
                // 境界チェック
                (ceiling_collision_system, landing_system),
                // 座標変換（論理座標→表示座標）
                sync_transform_system,
                // 影システム
                (
                    spawn_ball_shadow_system,
                    spawn_player_shadow_system,
                    sync_shadow_system,
                    despawn_ball_shadow_system,
                ),
                // 視覚フィードバック（色変化）
                (
                    save_player_original_color_system,
                    player_hold_visual_system,
                    ball_spin_color_system,
                ),
            )
                .chain()
                .in_set(GameSystemSet::GameLogic),
        );
    }
}

/// リプレイビューア初期セットアップ
/// コントロールタイプに応じてプレイヤーをスポーン
/// - Human: AiController を付与しない → replay_input_system で入力注入
/// - AI: AiController を付与 → AIによる再シミュレーション
fn setup_replay_viewer(
    mut commands: Commands,
    config: Res<GameConfig>,
    replay_data: Res<ReplayData>,
    mut replay_player: ResMut<ReplayPlayer>,
) {
    // Camera2d をスポーン
    commands.spawn(Camera2d);

    let left_control = replay_data.metadata.left_control;
    let right_control = replay_data.metadata.right_control;

    info!(
        "Replay Viewer: Setting up with left={:?}, right={:?}",
        left_control, right_control
    );

    // Human プレイヤーがいる場合はリプレイ再生を開始
    let has_human = left_control == ControlType::Human || right_control == ControlType::Human;
    if has_human {
        // ReplayData を clone してリプレイ再生を開始
        replay_player.start_playback(replay_data.clone());
        info!("Replay playback started for Human player input injection");
    }

    // コート境界を描画
    spawn_court(&mut commands, &config);

    // Player 1 (Left側) をスポーン（1Pコート側: 画面左側）
    let player1_pos = Vec3::new(
        config.player.x_min + 1.0,
        0.0,
        0.0,
    );
    let (r, g, b) = config.player_visual.player1_color;
    let player1_color = Color::srgb(r, g, b);
    let player1_entity = character::spawn_articulated_player(&mut commands, 1, player1_pos, player1_color);

    // Left側のコントロールタイプに応じて AiController を付与
    if left_control == ControlType::Ai {
        commands.entity(player1_entity).insert(AiController {
            home_position: player1_pos,
            target_position: player1_pos,
            ..Default::default()
        });
        info!("Player 1 (AI) spawned at {:?}", player1_pos);
    } else {
        info!("Player 1 (Human - replay input) spawned at {:?}", player1_pos);
    }

    // Player 2 (Right側) をスポーン（2Pコート側: 画面右側）
    let player2_pos = Vec3::new(
        config.player.x_max - 1.0,
        0.0,
        0.0,
    );
    let (r, g, b) = config.player_visual.player2_color;
    let player2_color = Color::srgb(r, g, b);
    let player2_entity = character::spawn_articulated_player(&mut commands, 2, player2_pos, player2_color);

    // Right側のコントロールタイプに応じて AiController を付与
    if right_control == ControlType::Ai {
        commands.entity(player2_entity).insert(AiController {
            home_position: player2_pos,
            target_position: player2_pos,
            ..Default::default()
        });
        info!("Player 2 (AI) spawned at {:?}", player2_pos);
    } else {
        info!("Player 2 (Human - replay input) spawned at {:?}", player2_pos);
    }
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
    let court_display_width = config.court.depth * WORLD_SCALE;
    let court_display_height = config.court.width * WORLD_SCALE;
    let half_w = court_display_width / 2.0;
    let half_h = court_display_height / 2.0;
    let service_x = config.court.service_box_depth * WORLD_SCALE;

    let white = Color::srgb(1.0, 1.0, 1.0);
    let line = 4.0;

    // コート背景（緑）
    spawn_rect(commands, court_display_width, court_display_height, 0.0, 0.0, -1.0, Color::srgb(0.2, 0.5, 0.2));
    // ネット（白い線、中央の縦線）
    spawn_rect(commands, line, court_display_height, 0.0, 0.0, 0.0, white);
    // 境界線（左右上下）
    spawn_rect(commands, line, court_display_height, -half_w, 0.0, 0.0, white);
    spawn_rect(commands, line, court_display_height, half_w, 0.0, 0.0, white);
    spawn_rect(commands, court_display_width, line, 0.0, half_h, 0.0, white);
    spawn_rect(commands, court_display_width, line, 0.0, -half_h, 0.0, white);
    // サービスライン
    spawn_rect(commands, line, court_display_height, -service_x, 0.0, 0.0, white);
    spawn_rect(commands, line, court_display_height, service_x, 0.0, 0.0, white);
    // センターサービスライン
    spawn_rect(commands, service_x, line, -service_x / 2.0, 0.0, 0.0, white);
    spawn_rect(commands, service_x, line, service_x / 2.0, 0.0, 0.0, white);

    info!("Court spawned for replay viewer");
}

/// 試合終了チェック（再シミュレーション完了検出）
fn check_replay_finished(
    match_state: Res<State<MatchFlowState>>,
    match_score: Res<padel_game::resource::MatchScore>,
    config: Res<ReplayViewerConfig>,
    mut app_exit: MessageWriter<AppExit>,
    mut frame_count: Local<u32>,
) {
    *frame_count += 1;

    // 詳細出力
    if config.verbose && (*frame_count).is_multiple_of(60) {
        println!(
            "Frame {}: MatchState={:?}",
            *frame_count,
            match_state.get(),
        );
    }

    // 試合終了で終了
    if *match_state.get() == MatchFlowState::MatchEnd {
        println!("\n=== Re-simulation Complete ===");
        println!("Final MatchState: {:?}", match_state.get());
        println!("Total frames: {}", *frame_count);
        println!(
            "Final Score: P1 {} : {} P2 (games)",
            match_score.scores[0].games, match_score.scores[1].games
        );

        app_exit.write(AppExit::Success);
    }
}

/// エスケープキーでウィンドウを閉じる
fn escape_to_exit(keyboard: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
