//! Headless Simulation CLI
//! @spec 77100_headless_sim.md
//!
//! AI vs AI の自動対戦を描画なしで高速実行し、異常を検出する。
//!
//! # Usage
//!
//! ```bash
//! cargo run --bin headless_sim -- [OPTIONS]
//!
//! Options:
//!   -n, --matches <COUNT>    試合数 [default: 10]
//!   -t, --timeout <SECONDS>  1試合の最大時間 [default: 300]
//!   -o, --output <FILE>      JSON出力パス
//!   -s, --seed <SEED>        乱数シード（再現性用）
//!   -v, --verbose            詳細ログ
//! ```

use clap::Parser;

use padel_game::resource::config::load_game_config;
use padel_game::simulation::{load_simulation_config, SimulationConfig, SimulationRunner};

/// Headless Simulation for Padel Game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of matches to simulate
    #[arg(short = 'n', long, default_value_t = 10)]
    matches: u32,

    /// Timeout per match in seconds
    #[arg(short, long, default_value_t = 300)]
    timeout: u32,

    /// Output JSON file path
    #[arg(short, long)]
    output: Option<String>,

    /// Random seed for reproducibility
    #[arg(short, long)]
    seed: Option<u64>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    println!("=== Padel Game Headless Simulator ===\n");

    // GameConfig をロード
    let config = match load_game_config("assets/config/game_config.ron") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load game config: {}", e);
            std::process::exit(1);
        }
    };

    // SimulationFileConfig をロード（異常検出閾値等）
    let sim_file_config = match load_simulation_config("assets/config/simulation_config.ron") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Warning: Failed to load simulation config: {}", e);
            eprintln!("Using default values.");
            padel_game::simulation::SimulationFileConfig::default()
        }
    };

    if args.verbose {
        println!("Anomaly thresholds:");
        println!("  bounds_margin: {}", sim_file_config.anomaly_thresholds.bounds_margin);
        println!("  height_limit: {}", sim_file_config.anomaly_thresholds.height_limit);
        println!("  state_stuck_secs: {}", sim_file_config.anomaly_thresholds.state_stuck_secs);
        println!("  infinite_rally_secs: {}", sim_file_config.anomaly_thresholds.infinite_rally_secs);
        println!("  max_velocity: {}", sim_file_config.anomaly_thresholds.max_velocity);
        println!();
    }

    // シミュレーション設定（CLI引数）
    let sim_config = SimulationConfig {
        match_count: args.matches,
        timeout_secs: args.timeout,
        seed: args.seed,
        verbose: args.verbose,
        output_path: args.output,
    };

    // シミュレーション実行
    let mut runner = SimulationRunner::new(sim_config);
    let report = runner.run(&config);

    // サマリー出力
    padel_game::simulation::SimulationReporter::new().print_summary(&report);

    // 異常があった場合は終了コード1
    if report.total_anomalies > 0 {
        eprintln!("\nSimulation completed with {} anomalies.", report.total_anomalies);
        std::process::exit(1);
    }

    println!("\nSimulation completed successfully.");
}
