//! Headless Simulation CLI
//! @spec 77100_headless_sim.md
//!
//! AI vs AI の自動対戦を描画なしで高速実行し、異常を検出する。
//!
//! # Usage
//!
//! ```bash
//! # デフォルト設定で実行（simulation_config.ron を使用）
//! cargo run --bin headless_sim
//!
//! # 設定名を指定
//! cargo run --bin headless_sim -- -c debug
//! # → assets/config/simulation_debug.ron を読み込み
//! ```

use clap::Parser;

use padel_game::resource::config::load_game_config;
use padel_game::simulation::{load_simulation_config, SimulationConfig, SimulationRunner};

/// Headless Simulation for Padel Game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Configuration name (e.g., "debug" -> simulation_debug.ron)
    /// If omitted, uses simulation_config.ron
    #[arg(short, long)]
    config: Option<String>,
}

/// 設定ファイルパスを解決
/// - None -> "assets/config/simulation_config.ron"
/// - Some("debug") -> "assets/config/simulation_debug.ron"
fn resolve_config_path(config_name: Option<&str>) -> String {
    match config_name {
        Some(name) => format!("assets/config/simulation_{}.ron", name),
        None => "assets/config/simulation_config.ron".to_string(),
    }
}

fn main() {
    let args = Args::parse();

    println!("=== Padel Game Headless Simulator ===\n");

    // GameConfig をロード
    let game_config = match load_game_config("assets/config/game_config.ron") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load game config: {}", e);
            std::process::exit(1);
        }
    };

    // SimulationFileConfig をロード
    let config_path = resolve_config_path(args.config.as_deref());
    println!("Loading config: {}", config_path);

    let sim_file_config = match load_simulation_config(&config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Warning: Failed to load simulation config: {}", e);
            eprintln!("Using default values.");
            padel_game::simulation::SimulationFileConfig::default()
        }
    };

    if sim_file_config.execution.verbose {
        println!("\nExecution settings:");
        println!("  match_count: {}", sim_file_config.execution.match_count);
        println!("  timeout_secs: {}", sim_file_config.execution.timeout_secs);
        println!("  seed: {:?}", sim_file_config.execution.seed);
        println!("  verbose: {}", sim_file_config.execution.verbose);

        println!("\nOutput settings:");
        println!("  result_file: {:?}", sim_file_config.output.result_file);
        println!("  trace_file: {:?}", sim_file_config.output.trace_file);

        println!("\nTrace settings:");
        println!("  enabled: {}", sim_file_config.trace.enabled);
        println!("  position: {}", sim_file_config.trace.position);
        println!("  velocity: {}", sim_file_config.trace.velocity);
        println!("  events: {}", sim_file_config.trace.events);
        println!(
            "  interval_frames: {}",
            sim_file_config.trace.interval_frames
        );

        println!("\nAnomaly thresholds:");
        println!(
            "  bounds_margin: {}",
            sim_file_config.anomaly_thresholds.bounds_margin
        );
        println!(
            "  height_limit: {}",
            sim_file_config.anomaly_thresholds.height_limit
        );
        println!(
            "  state_stuck_secs: {}",
            sim_file_config.anomaly_thresholds.state_stuck_secs
        );
        println!(
            "  infinite_rally_secs: {}",
            sim_file_config.anomaly_thresholds.infinite_rally_secs
        );
        println!(
            "  max_velocity: {}",
            sim_file_config.anomaly_thresholds.max_velocity
        );
        println!();
    }

    // SimulationConfig を設定ファイルから構築
    let sim_config = SimulationConfig {
        match_count: sim_file_config.execution.match_count,
        timeout_secs: sim_file_config.execution.timeout_secs,
        seed: sim_file_config.execution.seed,
        verbose: sim_file_config.execution.verbose,
        output_path: sim_file_config.output.result_file.clone(),
    };

    // シミュレーション実行
    let mut runner = SimulationRunner::new(sim_config).with_file_config(sim_file_config);
    let report = runner.run(&game_config);

    // サマリー出力
    padel_game::simulation::SimulationReporter::new().print_summary(&report);

    // 異常があった場合は終了コード1
    if report.total_anomalies > 0 {
        eprintln!(
            "\nSimulation completed with {} anomalies.",
            report.total_anomalies
        );
        std::process::exit(1);
    }

    println!("\nSimulation completed successfully.");
}
