//! Simulation Config
//! @spec 77100_headless_sim.md
//!
//! シミュレーター専用の外部設定。
//! ゲーム本体の GameConfig とは独立して管理。

use serde::Deserialize;
use std::fs;
use std::path::Path;

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

/// シミュレーター設定ファイル構造
#[derive(Clone, Debug, Deserialize, Default)]
pub struct SimulationFileConfig {
    /// 異常検出の閾値
    pub anomaly_thresholds: AnomalyThresholds,
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
