//! Debug Logger
//! @spec 77100_headless_sim.md
//!
//! カテゴリ別のデバッグログ出力を管理するリソース。

use bevy::prelude::*;
use std::fs::File;
use std::io::Write;

use super::config::DebugConfig;

/// カテゴリ別デバッグログ出力リソース
#[derive(Resource)]
pub struct DebugLogger {
    config: DebugConfig,
    file_handle: Option<File>,
    last_log_time: f32,
}

impl DebugLogger {
    /// DebugConfigからDebugLoggerを作成
    pub fn new(config: DebugConfig) -> Self {
        let file_handle = config.log_file.as_ref().and_then(|path| {
            File::create(path)
                .map_err(|e| eprintln!("[DebugLogger] Failed to create log file '{}': {}", path, e))
                .ok()
        });

        Self {
            config,
            file_handle,
            last_log_time: 0.0,
        }
    }

    /// AIログを出力
    pub fn log_ai(&mut self, message: &str) {
        if self.config.log_ai {
            self.write_log("AI", message);
        }
    }

    /// 物理イベントログを出力
    pub fn log_physics(&mut self, message: &str) {
        if self.config.log_physics {
            self.write_log("PHYSICS", message);
        }
    }

    /// 得点イベントログを出力
    pub fn log_scoring(&mut self, message: &str) {
        if self.config.log_scoring {
            self.write_log("SCORING", message);
        }
    }

    /// 状態遷移ログを出力
    pub fn log_state(&mut self, message: &str) {
        if self.config.log_state {
            self.write_log("STATE", message);
        }
    }

    /// 定期ログを出力すべきタイミングかチェックし、出力
    /// 返り値: ログ出力した場合 true
    pub fn log_periodic(&mut self, current_time: f32, message: &str) -> bool {
        if self.config.log_interval_secs <= 0.0 {
            return false;
        }

        if current_time - self.last_log_time >= self.config.log_interval_secs {
            self.last_log_time = current_time;
            self.write_log("PERIODIC", message);
            true
        } else {
            false
        }
    }

    /// AI ログが有効か
    pub fn is_ai_enabled(&self) -> bool {
        self.config.log_ai
    }

    /// 物理ログが有効か
    pub fn is_physics_enabled(&self) -> bool {
        self.config.log_physics
    }

    /// 得点ログが有効か
    pub fn is_scoring_enabled(&self) -> bool {
        self.config.log_scoring
    }

    /// 状態遷移ログが有効か
    pub fn is_state_enabled(&self) -> bool {
        self.config.log_state
    }

    /// 定期ログ間隔を取得
    pub fn log_interval_secs(&self) -> f32 {
        self.config.log_interval_secs
    }

    fn write_log(&mut self, category: &str, message: &str) {
        let formatted = format!("[{}] {}", category, message);

        // 標準エラー出力
        eprintln!("{}", formatted);

        // ファイル出力（設定されている場合）
        if let Some(ref mut file) = self.file_handle {
            let _ = writeln!(file, "{}", formatted);
        }
    }
}

impl Default for DebugLogger {
    fn default() -> Self {
        Self::new(DebugConfig::default())
    }
}
