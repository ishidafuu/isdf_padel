//! Simulation Runner
//! @spec 77100_headless_sim.md
//!
//! シミュレーション実行の制御を担当。
//! AI vs AI の対戦をセットアップし、指定回数の試合を実行する。

use bevy::prelude::*;

use crate::resource::config::GameConfig;

use super::{AnomalyDetector, MatchResult, SimulationReport, SimulationReporter};

/// シミュレーション設定
#[derive(Clone, Debug)]
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

/// シミュレーション実行器
pub struct SimulationRunner {
    config: SimulationConfig,
    reporter: SimulationReporter,
    /// TODO: Bevy App 実装時に使用
    #[allow(dead_code)]
    anomaly_detector: AnomalyDetector,
}

impl SimulationRunner {
    /// 新規作成
    pub fn new(config: SimulationConfig) -> Self {
        Self {
            config,
            reporter: SimulationReporter::new(),
            anomaly_detector: AnomalyDetector::new(),
        }
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
    fn run_single_match(&mut self, _game_config: &GameConfig, match_index: u32) -> MatchResult {
        // TODO: Bevy App を構築して実行
        // 現在はスタブ実装
        MatchResult {
            match_index,
            winner: Some(1),
            duration_secs: 0.0,
            rally_count: 0,
            anomalies: vec![],
            completed: true,
        }
    }
}

/// セットアップシステム（Startup で実行）
/// TODO: Bevy App 実装時に使用
#[allow(dead_code)]
pub fn simulation_setup_system(mut commands: Commands, config: Res<GameConfig>) {
    super::headless_plugins::headless_setup(&mut commands, &config);
}
