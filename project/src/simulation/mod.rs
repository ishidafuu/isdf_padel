//! Simulation Module
//! @spec 77100_headless_sim.md
//!
//! ヘッドレスシミュレーション用のモジュール群。
//! 描画なしで高速にゲームロジックを実行し、異常を検出する。
//!
//! Note: このモジュールはヘッドレスシミュレーション機能として設計されているが、
//! 現在は一部の機能のみが統合されている。未使用のエクスポートは将来の拡張用に保持。

mod anomaly_detector;
mod config;
mod debug_logger;
mod event_tracer;
mod headless_plugins;
mod result_reporter;
mod simulation_runner;
mod trace_system;

// シミュレーションモジュールの公開インターフェース
// 一部は将来の統合に向けて公開されているが、現在は未使用
#[allow(unused_imports)]
pub use anomaly_detector::{AnomalyDetector, AnomalyDetectorPlugin, AnomalyDetectorResource, AnomalyReport, AnomalyThresholdsResource, AnomalyType};
#[allow(unused_imports)]
pub use config::{
    load_simulation_config, AnomalyThresholds, DebugConfig, ExecutionConfig, OutputConfig,
    SimulationFileConfig, TraceConfig,
};
pub use debug_logger::DebugLogger;
#[allow(unused_imports)]
pub use event_tracer::{EntityTrace, EntityType, EventTracer, FrameTrace, GameEvent};
pub use headless_plugins::HeadlessPlugins;
pub use result_reporter::{MatchResult, SimulationReport, SimulationReporter};
#[allow(unused_imports)]
pub use simulation_runner::{SimulationConfig, SimulationRunner, SimulationStateResource};
pub use trace_system::TraceSystemPlugin;
