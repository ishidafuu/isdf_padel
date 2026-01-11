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

use bevy::{app::ScheduleRunnerPlugin, asset::AssetPlugin, prelude::*, state::app::StatesPlugin};
use clap::Parser;

use padel_game::character::CharacterPlugin;
use padel_game::core::{
    BallHitEvent, PlayerJumpEvent, PlayerKnockbackEvent, PlayerLandEvent,
    PlayerMoveEvent, ShotEvent, ShotExecutedEvent,
};
use padel_game::replay::loader::load_replay;
use padel_game::replay::player::{replay_input_system, ReplayPlayer};
use padel_game::resource::config::load_game_config;
use padel_game::resource::debug::LastShotDebugInfo;
use padel_game::resource::{FixedDeltaTime, GameRng, MatchFlowState};
use padel_game::simulation::AnomalyDetectorPlugin;
use padel_game::systems::{
    ceiling_collision_system, gravity_system, jump_system, knockback_movement_system,
    knockback_start_system, knockback_timer_system, landing_system, movement_system,
    shot_cooldown_system, shot_direction_system, shot_input_system, vertical_movement_system,
    AiServePlugin, BallCollisionPlugin, BallTrajectoryPlugin, BoundaryPlugin, FaultJudgmentPlugin,
    GameSystemSet, MatchFlowPlugin, PointJudgmentPlugin, ScoringPlugin,
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

    /// Verification mode: validate replay determinism
    /// @spec REQ-77103-007
    #[arg(long)]
    verify: bool,
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

    // リプレイのシード値でGameRngを初期化（AI動作の再現性確保）
    let game_rng = GameRng::from_seed(replay_data.metadata.seed);

    // Bevy アプリを構築して実行
    let mut app = App::new();

    // MinimalPlugins（60FPS固定タイムステップ）
    app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
        std::time::Duration::from_secs_f64(1.0 / 60.0),
    )));

    // StatesPlugin（State管理用、MinimalPluginsに含まれない）
    app.add_plugins(StatesPlugin);

    // AssetPlugin（CharacterPluginがアセットローダーを使用）
    app.add_plugins(AssetPlugin::default());

    // ゲームロジックプラグイン
    app.add_plugins(ReplayPlaybackPlugins);

    // リソース挿入
    app.insert_resource(game_config)
        .insert_resource(game_rng)
        .insert_resource(replay_player)
        .init_resource::<FixedDeltaTime>() // 物理計算用
        .insert_resource(ReplayPlaybackConfig {
            verbose: args.verbose,
            verify: args.verify,
        });

    // 完了検出用の状態
    app.add_systems(Update, check_replay_finished);

    println!("Starting replay playback...\n");

    // 実行
    app.run();
}

/// リプレイ再生設定
#[derive(Resource)]
struct ReplayPlaybackConfig {
    verbose: bool,
    /// 検証モード: RNGの決定性を検証
    verify: bool,
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
            .add_plugins(AiServePlugin) // AiServeTimerリソースを提供
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
    match_score: Res<padel_game::resource::MatchScore>,
    config: Res<ReplayPlaybackConfig>,
    mut app_exit: MessageWriter<AppExit>,
    mut frame_count: Local<u32>,
) {
    *frame_count += 1;

    // 詳細出力
    if config.verbose && (*frame_count).is_multiple_of(60) {
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

        // 検証モード: スコアと最終状態を出力
        // @spec REQ-77103-007
        if config.verify {
            println!("\n=== Verification Results ===");
            println!(
                "Points: P1[{}] vs P2[{}]",
                match_score.points[0].index, match_score.points[1].index
            );
            println!(
                "Games: P1 {} : {} P2",
                match_score.scores[0].games, match_score.scores[1].games
            );
            println!(
                "Sets: P1 {} : {} P2",
                match_score.scores[0].sets, match_score.scores[1].sets
            );
            println!("Server: {:?}", match_score.server);
            println!("Game State: {:?}", match_score.game_state);
            println!(
                "Determinism check: Run this replay multiple times to verify identical results."
            );
            println!("(Future: EventTracer will enable frame-by-frame comparison)");
        }

        app_exit.write(AppExit::Success);
    }
}
