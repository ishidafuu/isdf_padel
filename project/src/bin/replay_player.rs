//! Replay Player CLI
//! @spec 77103_replay_spec.md
//!
//! リプレイファイルをヘッドレスで再生し、トレース出力を行う。
//!
//! # Usage
//!
//! ```bash
//! # リプレイを再生
//! cargo run --bin replay_player -- assets/replays/replay_20260110_123456.ron
//!
//! # 詳細出力付き
//! cargo run --bin replay_player -- -v assets/replays/replay_20260110_123456.ron
//! ```

use bevy::prelude::*;
use clap::Parser;

use padel_game::character::CharacterPlugin;
use padel_game::components::AiController;
use padel_game::core::{
    BallHitEvent, CourtSide, PlayerJumpEvent, PlayerKnockbackEvent, PlayerLandEvent,
    PlayerMoveEvent, ShotEvent, ShotExecutedEvent,
};
use padel_game::replay::{load_replay, player::replay_input_system, ReplayPlayer};
use padel_game::resource::config::load_game_config;
use padel_game::resource::debug::LastShotDebugInfo;
use padel_game::resource::{GameConfig, MatchFlowState};
use padel_game::simulation::AnomalyDetectorPlugin;
use padel_game::systems::{
    ceiling_collision_system, gravity_system, jump_system, knockback_movement_system,
    knockback_start_system, knockback_timer_system, landing_system, movement_system,
    shot_cooldown_system, shot_direction_system, shot_input_system, vertical_movement_system,
    BallCollisionPlugin, BallTrajectoryPlugin, BoundaryPlugin, FaultJudgmentPlugin, GameSystemSet,
    MatchFlowPlugin, PointJudgmentPlugin, ScoringPlugin,
};

/// Replay Player for Padel Game
#[derive(Parser, Debug)]
#[command(author, version, about = "Replay player for Padel Game")]
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

    println!("=== Padel Game Replay Player ===\n");
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

    // ReplayPlayer リソースを初期化
    let mut replay_player = ReplayPlayer::new();
    replay_player.start_playback(replay_data.clone());

    // TODO: シード値による乱数初期化
    // 現在のゲームは rand::rng() を直接使用しており、シード可能な乱数リソースになっていない。
    // 完全な再現のためには、ゲーム全体の乱数システムをシード可能なリソースにリファクタリングする必要がある。
    // 現時点では入力の再現のみ対応。AIの乱数動作の再現は将来の拡張として保留。
    println!("Note: Seed {} is recorded but not used for RNG initialization.", replay_data.metadata.seed);
    println!("      AI behavior may differ from original recording.\n");

    // Bevy アプリを構築して実行
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(ReplayPlaybackPlugins)
        .insert_resource(game_config)
        .insert_resource(replay_player)
        .insert_resource(ReplayConfig { verbose: args.verbose });

    // 完了検出用の状態
    app.add_systems(Update, check_replay_finished);

    println!("Starting replay playback...\n");

    // 実行
    app.run();
}

/// リプレイ再生設定
#[derive(Resource)]
struct ReplayConfig {
    verbose: bool,
}

/// リプレイ再生用プラグインセット
struct ReplayPlaybackPlugins;

impl Plugin for ReplayPlaybackPlugins {
    fn build(&self, app: &mut App) {
        // ゲームロジックプラグイン
        app.add_plugins(BoundaryPlugin)
            .add_plugins(BallTrajectoryPlugin)
            .add_plugins(BallCollisionPlugin)
            .add_plugins(ScoringPlugin)
            .add_plugins(PointJudgmentPlugin)
            .add_plugins(FaultJudgmentPlugin)
            .add_plugins(MatchFlowPlugin)
            .add_plugins(CharacterPlugin)
            .add_plugins(AnomalyDetectorPlugin);

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

        // 入力システム（リプレイから入力を注入）
        app.add_systems(Update, replay_input_system.in_set(GameSystemSet::Input));

        // ゲームロジックシステム
        app.add_systems(
            Update,
            (
                // ふっとばし開始
                knockback_start_system,
                // ジャンプ・重力
                (jump_system, gravity_system, vertical_movement_system).chain(),
                // 水平移動
                movement_system,
                // ショット入力処理（Rally状態でのみ）
                shot_input_system.run_if(in_state(MatchFlowState::Rally)),
                // 方向計算・クールダウン
                (shot_direction_system, shot_cooldown_system),
                // ふっとばし移動・タイマー
                (knockback_movement_system, knockback_timer_system),
                // 境界チェック
                (ceiling_collision_system, landing_system),
            )
                .chain()
                .in_set(GameSystemSet::GameLogic),
        );
    }
}

/// リプレイ再生完了チェック
fn check_replay_finished(
    replay_player: Res<ReplayPlayer>,
    match_state: Res<State<MatchFlowState>>,
    config: Res<ReplayConfig>,
    mut app_exit: MessageWriter<AppExit>,
    mut frame_count: Local<u32>,
) {
    *frame_count += 1;

    // 詳細出力
    if config.verbose && *frame_count % 60 == 0 {
        println!(
            "Frame {}: MatchState={:?}, ReplayFrame={}/{}",
            *frame_count,
            match_state.get(),
            replay_player.current_frame(),
            replay_player.total_frames()
        );
    }

    // 試合終了またはリプレイ再生完了で終了
    if *match_state.get() == MatchFlowState::MatchEnd || replay_player.is_finished() {
        println!("\n=== Replay Playback Complete ===");
        println!("Final MatchState: {:?}", match_state.get());
        println!("Frames played: {}", replay_player.current_frame());
        println!("Total app frames: {}", *frame_count);
        app_exit.write(AppExit::Success);
    }
}
