//! Result Reporter
//! @spec 77100_headless_sim.md
//!
//! シミュレーション結果の出力を担当。
//! - コンソール出力（サマリー）
//! - JSONファイル出力（詳細）

use serde::Serialize;
use std::fs::File;
use std::io::Write;

use super::AnomalyReport;

/// 試合結果
#[derive(Clone, Debug, Serialize)]
pub struct MatchResult {
    /// 試合インデックス
    pub match_index: u32,
    /// 勝者（1 or 2, None = 未決着）
    pub winner: Option<u8>,
    /// 試合時間（秒）
    pub duration_secs: f32,
    /// ラリー数
    pub rally_count: u32,
    /// 検出された異常
    #[serde(skip)]
    pub anomalies: Vec<AnomalyReport>,
    /// 正常終了したか
    pub completed: bool,
}

/// シミュレーションレポート
#[derive(Clone, Debug, Serialize)]
pub struct SimulationReport {
    /// 実行日時
    pub timestamp: String,
    /// 総試合数
    pub total_matches: u32,
    /// 完了試合数
    pub completed_matches: u32,
    /// Player 1 勝利数
    pub player1_wins: u32,
    /// Player 2 勝利数
    pub player2_wins: u32,
    /// 異常検出数
    pub total_anomalies: u32,
    /// 平均試合時間（秒）
    pub avg_duration_secs: f32,
    /// 平均ラリー数
    pub avg_rally_count: f32,
    /// 各試合の結果
    pub matches: Vec<MatchResult>,
}

/// シミュレーション結果レポーター
pub struct SimulationReporter {
    results: Vec<MatchResult>,
}

impl SimulationReporter {
    /// 新規作成
    pub fn new() -> Self {
        Self { results: vec![] }
    }

    /// 試合結果を追加
    pub fn add_result(&mut self, result: MatchResult) {
        self.results.push(result);
    }

    /// レポートを生成
    pub fn generate_report(&self) -> SimulationReport {
        let total_matches = self.results.len() as u32;
        let completed_matches = self.results.iter().filter(|r| r.completed).count() as u32;
        let player1_wins = self.results.iter().filter(|r| r.winner == Some(1)).count() as u32;
        let player2_wins = self.results.iter().filter(|r| r.winner == Some(2)).count() as u32;
        let total_anomalies: u32 = self.results.iter().map(|r| r.anomalies.len() as u32).sum();

        let total_duration: f32 = self.results.iter().map(|r| r.duration_secs).sum();
        let total_rallies: u32 = self.results.iter().map(|r| r.rally_count).sum();

        let avg_duration_secs = if total_matches > 0 {
            total_duration / total_matches as f32
        } else {
            0.0
        };

        let avg_rally_count = if total_matches > 0 {
            total_rallies as f32 / total_matches as f32
        } else {
            0.0
        };

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        SimulationReport {
            timestamp,
            total_matches,
            completed_matches,
            player1_wins,
            player2_wins,
            total_anomalies,
            avg_duration_secs,
            avg_rally_count,
            matches: self.results.clone(),
        }
    }

    /// コンソールにサマリーを出力
    pub fn print_summary(&self, report: &SimulationReport) {
        println!("\n========================================");
        println!("       SIMULATION REPORT");
        println!("========================================");
        println!("Timestamp:         {}", report.timestamp);
        println!("Total Matches:     {}", report.total_matches);
        println!("Completed:         {}", report.completed_matches);
        println!("Player 1 Wins:     {}", report.player1_wins);
        println!("Player 2 Wins:     {}", report.player2_wins);
        println!("Total Anomalies:   {}", report.total_anomalies);
        println!("Avg Duration:      {:.2}s", report.avg_duration_secs);
        println!("Avg Rally Count:   {:.1}", report.avg_rally_count);
        println!("========================================\n");

        if report.total_anomalies > 0 {
            println!("ANOMALIES DETECTED:");
            for (i, result) in report.matches.iter().enumerate() {
                if !result.anomalies.is_empty() {
                    println!("  Match {}: {} anomalies", i + 1, result.anomalies.len());
                    for anomaly in &result.anomalies {
                        println!(
                            "    - Frame {}: {:?}",
                            anomaly.frame, anomaly.anomaly_type
                        );
                    }
                }
            }
        }
    }

    /// JSONファイルに出力
    pub fn save_to_file(&self, path: &str, report: &SimulationReport) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(report)
            .map_err(std::io::Error::other)?;

        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;

        println!("Report saved to: {}", path);
        Ok(())
    }
}

impl Default for SimulationReporter {
    fn default() -> Self {
        Self::new()
    }
}
