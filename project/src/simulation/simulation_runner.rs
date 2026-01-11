//! Simulation Runner
//! @spec 77100_headless_sim.md
//!
//! シミュレーション実行の制御を担当。
//! AI vs AI の対戦をセットアップし、指定回数の試合を実行する。

use bevy::app::ScheduleRunnerPlugin;
use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

use crate::components::AiController;
use crate::core::CourtSide;
use crate::resource::config::GameConfig;
use crate::resource::scoring::{GameState, MatchScore};
use crate::resource::{FixedDeltaTime, GameRng};
use crate::resource::MatchFlowState;

use super::{
    AnomalyDetectorResource, AnomalyThresholdsResource, DebugLogger, EventTracer, HeadlessPlugins,
    MatchResult, SimulationFileConfig, SimulationReport, SimulationReporter, TraceSystemPlugin,
};

/// シミュレーション設定
#[derive(Clone, Debug, Resource)]
pub struct SimulationConfig {
    /// 実行する試合数
    pub match_count: u32,
    /// 1試合の最大秒数
    pub timeout_secs: u32,
    /// 乱数シード（再現性用）
    pub seed: Option<u64>,
    /// 詳細ログ出力
    pub verbose: bool,
    /// JSON出力パス
    pub output_path: Option<String>,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            match_count: 10,
            timeout_secs: 300,
            seed: None,
            verbose: false,
            output_path: None,
        }
    }
}

/// シミュレーション状態リソース（App内で使用）
#[derive(Resource)]
pub struct SimulationStateResource {
    /// 試合が終了したか
    pub match_finished: bool,
    /// タイムアウトしたか
    pub timed_out: bool,
    /// 勝者（1 or 2, None = 未決着）
    pub winner: Option<u8>,
    /// 経過時間（秒）
    pub elapsed_secs: f32,
    /// タイムアウト閾値（秒）
    pub timeout_secs: f32,
    /// ラリー数
    pub rally_count: u32,
}

impl SimulationStateResource {
    pub fn new(timeout_secs: u32) -> Self {
        Self {
            match_finished: false,
            timed_out: false,
            winner: None,
            elapsed_secs: 0.0,
            timeout_secs: timeout_secs as f32,
            rally_count: 0,
        }
    }
}

/// シミュレーション実行器
pub struct SimulationRunner {
    config: SimulationConfig,
    reporter: SimulationReporter,
    /// シミュレーション設定ファイル（オプション）
    file_config: Option<SimulationFileConfig>,
}

impl SimulationRunner {
    /// 新規作成
    pub fn new(config: SimulationConfig) -> Self {
        Self {
            config,
            reporter: SimulationReporter::new(),
            file_config: None,
        }
    }

    /// シミュレーション設定ファイルを設定
    pub fn with_file_config(mut self, file_config: SimulationFileConfig) -> Self {
        self.file_config = Some(file_config);
        self
    }

    /// シミュレーション実行
    pub fn run(&mut self, game_config: &GameConfig) -> SimulationReport {
        println!(
            "Starting simulation: {} matches, timeout {}s",
            self.config.match_count, self.config.timeout_secs
        );

        if let Some(seed) = self.config.seed {
            println!("Using seed: {}", seed);
        }

        for i in 0..self.config.match_count {
            println!("Match {}/{}", i + 1, self.config.match_count);
            let result = self.run_single_match(game_config, i);
            self.reporter.add_result(result);
        }

        let report = self.reporter.generate_report();

        // 結果出力
        if let Some(ref path) = self.config.output_path {
            if let Err(e) = self.reporter.save_to_file(path, &report) {
                eprintln!("Failed to save report: {}", e);
            }
        }

        report
    }

    /// 単一試合を実行
    fn run_single_match(&mut self, game_config: &GameConfig, match_index: u32) -> MatchResult {
        // Bevy App を構築
        let mut app = App::new();

        // MinimalPlugins（時間とタスク処理のみ）
        // ScheduleRunnerPlugin で固定タイムステップを使用
        // 60FPS固定タイムステップ
        app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        )));

        // StatesPlugin（State管理用、MinimalPluginsに含まれない）
        app.add_plugins(StatesPlugin);

        // AssetPlugin（CharacterPluginがアセットローダーを使用）
        app.add_plugins(AssetPlugin::default());

        // ゲームロジックプラグイン
        app.add_plugins(HeadlessPlugins);

        // GameConfig リソースを挿入
        app.insert_resource(game_config.clone());

        // FixedDeltaTime リソースを挿入（物理計算用）
        app.init_resource::<FixedDeltaTime>();

        // GameRng リソースを挿入（シード制御）
        let game_rng = if let Some(seed) = self.config.seed {
            GameRng::from_seed(seed)
        } else {
            GameRng::from_entropy()
        };
        app.insert_resource(game_rng);

        // シミュレーション状態リソースを挿入
        app.insert_resource(SimulationStateResource::new(self.config.timeout_secs));

        // SimulationConfig をリソースとして挿入（デバッグシステム用）
        app.insert_resource(self.config.clone());

        // 異常検出閾値を設定
        if let Some(ref file_config) = self.file_config {
            app.insert_resource(AnomalyThresholdsResource(
                file_config.anomaly_thresholds.clone(),
            ));
        }

        // EventTracer を設定（TraceConfig が有効な場合）
        if let Some(ref file_config) = self.file_config {
            let tracer = EventTracer::from_config(file_config.trace.clone());
            if tracer.enabled {
                app.insert_resource(tracer);
                app.add_plugins(TraceSystemPlugin);
            } else {
                app.insert_resource(EventTracer::default());
            }
        } else {
            app.insert_resource(EventTracer::default());
        }

        // DebugLogger を設定
        let debug_config = self
            .file_config
            .as_ref()
            .map(|c| c.debug.clone())
            .unwrap_or_default();
        app.insert_resource(DebugLogger::new(debug_config));

        // セットアップシステム（プレイヤーのスポーン）
        app.add_systems(Startup, simulation_setup_system);

        // シミュレーション終了検出システム
        app.add_systems(
            Update,
            (
                check_match_end_system,
                check_timeout_system,
                debug_simulation_state,
                debug_state_transitions,
            )
                .chain(),
        );

        // Appの初期化
        app.finish();
        app.cleanup();

        // 試合ループ
        loop {
            app.update();

            // シミュレーション状態を確認
            let sim_state = app.world().resource::<SimulationStateResource>();

            if sim_state.match_finished || sim_state.timed_out {
                break;
            }
        }

        // 結果を取得
        let sim_state = app.world().resource::<SimulationStateResource>();
        let anomaly_detector = app.world().resource::<AnomalyDetectorResource>();
        let event_tracer = app.world().resource::<EventTracer>();

        let completed = sim_state.match_finished && !sim_state.timed_out;
        let winner = sim_state.winner;
        let duration_secs = sim_state.elapsed_secs;
        let rally_count = sim_state.rally_count;
        let anomalies = anomaly_detector.detector.anomalies().to_vec();

        // トレース統計を出力（有効時のみ）
        if event_tracer.enabled {
            println!(
                "  [Trace] frames={}, events={}",
                event_tracer.frame_count(),
                event_tracer.event_count()
            );
        }

        if self.config.verbose {
            println!(
                "  Match {} result: winner={:?}, duration={:.2}s, rallies={}, anomalies={}",
                match_index + 1,
                winner,
                duration_secs,
                rally_count,
                anomalies.len()
            );
        }

        MatchResult {
            match_index,
            winner,
            duration_secs,
            rally_count,
            anomalies,
            completed,
        }
    }
}

/// セットアップシステム（Startup で実行）
fn simulation_setup_system(mut commands: Commands, config: Res<GameConfig>) {
    // Player 1 (AI) - Left側
    let player1_pos = Vec3::new(config.player.x_min + 1.0, 0.0, 0.0);
    let (r, g, b) = config.player_visual.player1_color;
    let player1_color = Color::srgb(r, g, b);
    let player1_entity =
        crate::character::spawn_articulated_player(&mut commands, 1, player1_pos, player1_color);
    // Player 1 も AI として動作させる
    commands.entity(player1_entity).insert(AiController {
        home_position: player1_pos,
        target_position: player1_pos,
        ..Default::default()
    });

    // Player 2 (AI) - Right側
    let player2_pos = Vec3::new(config.player.x_max - 1.0, 0.0, 0.0);
    let (r, g, b) = config.player_visual.player2_color;
    let player2_color = Color::srgb(r, g, b);
    let player2_entity =
        crate::character::spawn_articulated_player(&mut commands, 2, player2_pos, player2_color);
    commands.entity(player2_entity).insert(AiController {
        home_position: player2_pos,
        target_position: player2_pos,
        ..Default::default()
    });
}

/// 試合終了検出システム
fn check_match_end_system(
    mut sim_state: ResMut<SimulationStateResource>,
    match_flow_state: Res<State<MatchFlowState>>,
    match_score: Option<Res<MatchScore>>,
) {
    // MatchFlowState::MatchEnd で試合終了
    if *match_flow_state.get() == MatchFlowState::MatchEnd {
        sim_state.match_finished = true;

        // 勝者を取得
        if let Some(score) = match_score {
            if let GameState::MatchWon(winner_side) = score.game_state {
                // CourtSide から Player番号に変換
                // Left (Player 1), Right (Player 2)
                sim_state.winner = Some(match winner_side {
                    CourtSide::Left => 1,
                    CourtSide::Right => 2,
                });
            }
        }
    }
}

/// タイムアウト検出システム
fn check_timeout_system(mut sim_state: ResMut<SimulationStateResource>, fixed_dt: Res<FixedDeltaTime>) {
    sim_state.elapsed_secs += fixed_dt.delta_secs();

    if sim_state.elapsed_secs >= sim_state.timeout_secs {
        sim_state.timed_out = true;
        eprintln!(
            "[TIMEOUT] Match exceeded {}s limit at {:.2}s",
            sim_state.timeout_secs, sim_state.elapsed_secs
        );
    }
}

/// デバッグシステム: ボールとスコアの状態を定期的に出力（verbose時のみ）
fn debug_simulation_state(
    sim_config: Res<SimulationConfig>,
    time: Res<Time>,
    balls: Query<
        (
            &crate::components::LogicalPosition,
            &crate::components::Velocity,
        ),
        With<crate::components::Ball>,
    >,
    players: Query<&crate::components::LogicalPosition, With<crate::components::Player>>,
    match_score: Option<Res<MatchScore>>,
    mut last_log_time: Local<f32>,
) {
    if !sim_config.verbose {
        return;
    }

    // 5秒ごとに出力
    if time.elapsed_secs() - *last_log_time > 5.0 {
        *last_log_time = time.elapsed_secs();

        let ball_count = balls.iter().count();
        let player_count = players.iter().count();

        eprintln!(
            "[DEBUG] t={:.1}s: balls={}, players={}",
            time.elapsed_secs(),
            ball_count,
            player_count
        );

        for (pos, vel) in balls.iter() {
            eprintln!(
                "[DEBUG] Ball pos=({:.2},{:.2},{:.2}), vel=({:.2},{:.2},{:.2})",
                pos.value.x, pos.value.y, pos.value.z, vel.value.x, vel.value.y, vel.value.z
            );
        }

        if let Some(score) = match_score {
            eprintln!(
                "[DEBUG] Score: {:?}, Server: {:?}, State: {:?}",
                score.scores, score.server, score.game_state
            );
        }
    }
}

/// デバッグシステム: 状態遷移を確認（verbose時のみ）
fn debug_state_transitions(
    sim_config: Res<SimulationConfig>,
    match_flow_state: Res<State<MatchFlowState>>,
    rally_state: Option<Res<crate::resource::RallyState>>,
    mut last_flow_state: Local<Option<MatchFlowState>>,
    mut last_phase: Local<Option<crate::resource::RallyPhase>>,
) {
    if !sim_config.verbose {
        return;
    }

    // MatchFlowState 変化追跡
    let current_flow = *match_flow_state.get();
    if *last_flow_state != Some(current_flow) {
        eprintln!("[DEBUG] MatchFlowState: {:?}", current_flow);
        *last_flow_state = Some(current_flow);
    }

    // RallyPhase 変化追跡
    if let Some(state) = rally_state {
        if *last_phase != Some(state.phase) {
            eprintln!("[DEBUG] RallyPhase: {:?}", state.phase);
            *last_phase = Some(state.phase);
        }
    }
}
