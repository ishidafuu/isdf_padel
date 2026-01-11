//! Simulation Module
//! @spec 77100_headless_sim.md
//!
//! ヘッドレスシミュレーション用のモジュール群。
//! 描画なしで高速にゲームロジックを実行し、異常を検出する。

mod anomaly_detector;
mod config;
mod debug_logger;
mod event_tracer;
mod headless_plugins;
mod result_reporter;
mod simulation_runner;
mod trace_system;

pub use anomaly_detector::{AnomalyDetector, AnomalyDetectorPlugin, AnomalyDetectorResource, AnomalyReport, AnomalyThresholdsResource, AnomalyType};
pub use config::{
    load_simulation_config, AnomalyThresholds, DebugConfig, ExecutionConfig, OutputConfig,
    SimulationFileConfig, TraceConfig,
};
pub use debug_logger::DebugLogger;
pub use event_tracer::{EntityTrace, EntityType, EventTracer, FrameTrace, GameEvent};
pub use headless_plugins::HeadlessPlugins;
pub use result_reporter::{MatchResult, SimulationReport, SimulationReporter};
pub use simulation_runner::{SimulationConfig, SimulationRunner, SimulationStateResource};
pub use trace_system::TraceSystemPlugin;
