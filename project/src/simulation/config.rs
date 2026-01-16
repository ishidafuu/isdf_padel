//! Simulation Config
//! @spec 77100_headless_sim.md
//!
//! シミュレーター専用の外部設定。
//! ゲーム本体の GameConfig とは独立して管理。
//!
//! Note: 将来のヘッドレスシミュレーション統合に向けて実装済み

#![allow(dead_code)]

use serde::Deserialize;
use std::fs;
use std::path::Path;

/// 実行設定
#[derive(Clone, Debug, Deserialize)]
pub struct ExecutionConfig {
    /// 実行試合数
    pub match_count: u32,
    /// 1試合タイムアウト（秒）
    pub timeout_secs: u32,
    /// 乱数シード（None=ランダム）
    pub seed: Option<u64>,
    /// 詳細ログ
    pub verbose: bool,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            match_count: 10,
            timeout_secs: 300,
            seed: None,
            verbose: false,
        }
    }
}

/// 出力設定
#[derive(Clone, Debug, Deserialize, Default)]
pub struct OutputConfig {
    /// JSON結果出力先（None=出力なし）
    pub result_file: Option<String>,
    /// トレース出力先
    pub trace_file: Option<String>,
}

/// トレース設定
#[derive(Clone, Debug, Deserialize)]
pub struct TraceConfig {
    /// トレース有効化
    pub enabled: bool,
    /// 座標記録
    pub position: bool,
    /// 速度記録
    pub velocity: bool,
    /// イベント記録
    pub events: bool,
    /// 記録間隔（フレーム）
    pub interval_frames: u32,
}

impl Default for TraceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            position: true,
            velocity: true,
            events: false,
            interval_frames: 10,
        }
    }
}

/// 異常検出の閾値設定
#[derive(Clone, Debug, Deserialize)]
pub struct AnomalyThresholds {
    /// コート外判定のマージン（メートル）
    pub bounds_margin: f32,
    /// 高さ上限（メートル）
    pub height_limit: f32,
    /// 高さ下限（メートル）
    pub height_floor: f32,
    /// 状態遷移スタック判定時間（秒）
    pub state_stuck_secs: f32,
    /// 無限ラリー判定時間（秒）
    pub infinite_rally_secs: f32,
    /// 物理異常とみなす速度閾値
    pub max_velocity: f32,
}

impl Default for AnomalyThresholds {
    fn default() -> Self {
        Self {
            bounds_margin: 50.0,
            height_limit: 100.0,
            height_floor: -10.0,
            state_stuck_secs: 60.0,
            infinite_rally_secs: 300.0,
            max_velocity: 1000.0,
        }
    }
}

/// カテゴリ別デバッグログ設定
#[derive(Clone, Debug, Deserialize)]
pub struct DebugConfig {
    /// AI行動ログ
    pub log_ai: bool,
    /// 物理イベントログ
    pub log_physics: bool,
    /// 得点イベントログ
    pub log_scoring: bool,
    /// 状態遷移ログ
    pub log_state: bool,
    /// 定期ログ間隔（秒）（0で無効）
    pub log_interval_secs: f32,
    /// ログファイル出力パス
    pub log_file: Option<String>,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            log_ai: false,
            log_physics: false,
            log_scoring: false,
            log_state: false,
            log_interval_secs: 0.0,
            log_file: None,
        }
    }
}

/// シミュレーター設定ファイル構造
#[derive(Clone, Debug, Deserialize, Default)]
pub struct SimulationFileConfig {
    /// 実行設定
    #[serde(default)]
    pub execution: ExecutionConfig,
    /// 出力設定
    #[serde(default)]
    pub output: OutputConfig,
    /// トレース設定
    #[serde(default)]
    pub trace: TraceConfig,
    /// 異常検出の閾値
    #[serde(default)]
    pub anomaly_thresholds: AnomalyThresholds,
    /// デバッグログ設定
    #[serde(default)]
    pub debug: DebugConfig,
}

/// シミュレーター設定をファイルから読み込む
pub fn load_simulation_config<P: AsRef<Path>>(path: P) -> Result<SimulationFileConfig, String> {
    let path = path.as_ref();

    if !path.exists() {
        // ファイルが存在しない場合はデフォルト値を使用
        return Ok(SimulationFileConfig::default());
    }

    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read simulation config: {}", e))?;

    ron::from_str(&content)
        .map_err(|e| format!("Failed to parse simulation config: {}", e))
}
