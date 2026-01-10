//! Simulation Module
//! @spec 77100_headless_sim.md
//!
//! ヘッドレスシミュレーション用のモジュール群。
//! 描画なしで高速にゲームロジックを実行し、異常を検出する。

mod anomaly_detector;
mod config;
mod headless_plugins;
mod result_reporter;
mod simulation_runner;

pub use anomaly_detector::{AnomalyDetector, AnomalyDetectorPlugin, AnomalyDetectorResource, AnomalyReport, AnomalyThresholdsResource, AnomalyType};
pub use config::{
    load_simulation_config, AnomalyThresholds, ExecutionConfig, OutputConfig,
    SimulationFileConfig, TraceConfig,
};
pub use headless_plugins::HeadlessPlugins;
pub use result_reporter::{MatchResult, SimulationReport, SimulationReporter};
pub use simulation_runner::{SimulationConfig, SimulationRunner, SimulationStateResource};
