//! Markdown Formatter
//! @spec 77201_narrative_spec.md REQ-77201-007, REQ-77201-008
//!
//! ラリー解析結果をマークダウン形式で出力する。

use super::analyzer::{AnalysisResult, Anomaly, Rally};
use super::types::FrameTrace;

/// 詳細度レベル
/// @spec REQ-77201-008
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DetailLevel {
    /// ラリー要約のみ
    Summary,
    /// ラリー要約 + 主要イベント
    #[default]
    Normal,
    /// 全イベント + AI決定詳細
    Full,
}

impl DetailLevel {
    /// 文字列からパース
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "summary" => DetailLevel::Summary,
            "full" => DetailLevel::Full,
            _ => DetailLevel::Normal,
        }
    }
}

/// フォーマットオプション
/// @spec REQ-77201-007, REQ-77201-008
#[derive(Debug, Clone)]
pub struct FormatOptions {
    /// 詳細度レベル
    pub detail_level: DetailLevel,
    /// 物理詳細を含める
    pub include_physics: bool,
    /// ラリー要約のみ
    pub rally_only: bool,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            detail_level: DetailLevel::Normal,
            include_physics: false,
            rally_only: false,
        }
    }
}

/// マークダウン形式で出力
/// @spec REQ-77201-007
pub fn format_markdown(
    frames: &[FrameTrace],
    analysis: &AnalysisResult,
    options: &FormatOptions,
) -> String {
    let mut output = String::new();

    // ヘッダー
    output.push_str(&format_header(frames, analysis));

    // サマリー（rally_onlyでない場合のみ詳細を含む）
    if !options.rally_only {
        output.push_str(&format_summary(analysis));
    }

    // ラリー詳細
    output.push_str("\n## Rallies\n\n");
    for rally in &analysis.rallies {
        output.push_str(&format_rally(rally, options));
    }

    // 統計的異常（full詳細度のみ）
    if options.detail_level == DetailLevel::Full && !analysis.statistical_anomalies.is_empty() {
        output.push_str(&format_statistical_anomalies(
            &analysis.statistical_anomalies,
        ));
    }

    output
}

/// ヘッダーを生成
fn format_header(frames: &[FrameTrace], analysis: &AnalysisResult) -> String {
    let mut output = String::from("# Match Report\n\n");

    if let (Some(first), Some(last)) = (frames.first(), frames.last()) {
        let duration_secs = last.timestamp - first.timestamp;
        let minutes = (duration_secs / 60.0) as u32;
        let secs = (duration_secs % 60.0) as u32;

        output.push_str(&format!("- **Duration**: {}m {}s\n", minutes, secs));
        output.push_str(&format!(
            "- **Frame Range**: {} - {}\n",
            first.frame, last.frame
        ));
    }

    output.push_str(&format!(
        "- **Total Rallies**: {}\n",
        analysis.rallies.len()
    ));

    // ポイント集計
    let p1_points = analysis.rallies.iter().filter(|r| r.winner == 1).count();
    let p2_points = analysis.rallies.iter().filter(|r| r.winner == 2).count();
    output.push_str(&format!(
        "- **Points**: P1: {}, P2: {}\n",
        p1_points, p2_points
    ));

    output.push('\n');
    output
}

/// サマリーを生成
fn format_summary(analysis: &AnalysisResult) -> String {
    let mut output = String::from("## Summary\n\n");

    if analysis.rallies.is_empty() {
        output.push_str("_No rallies recorded._\n\n");
        return output;
    }

    // プレイヤー別の集計
    let (p1_power, p1_accuracy, p1_shots) =
        analysis
            .rallies
            .iter()
            .fold((0.0f32, 0.0f32, 0u32), |acc, r| {
                (
                    acc.0 + r.stats.p1_avg_power * r.stats.p1_shot_count as f32,
                    acc.1 + r.stats.p1_avg_accuracy * r.stats.p1_shot_count as f32,
                    acc.2 + r.stats.p1_shot_count,
                )
            });

    let (p2_power, p2_accuracy, p2_shots) =
        analysis
            .rallies
            .iter()
            .fold((0.0f32, 0.0f32, 0u32), |acc, r| {
                (
                    acc.0 + r.stats.p2_avg_power * r.stats.p2_shot_count as f32,
                    acc.1 + r.stats.p2_avg_accuracy * r.stats.p2_shot_count as f32,
                    acc.2 + r.stats.p2_shot_count,
                )
            });

    let p1_avg_power = if p1_shots > 0 {
        p1_power / p1_shots as f32
    } else {
        0.0
    };
    let p1_avg_accuracy = if p1_shots > 0 {
        p1_accuracy / p1_shots as f32
    } else {
        0.0
    };
    let p2_avg_power = if p2_shots > 0 {
        p2_power / p2_shots as f32
    } else {
        0.0
    };
    let p2_avg_accuracy = if p2_shots > 0 {
        p2_accuracy / p2_shots as f32
    } else {
        0.0
    };

    // テーブル出力
    output.push_str("| Metric | P1 | P2 |\n");
    output.push_str("|--------|----|----|");
    output.push_str(&format!("\n| Total Shots | {} | {} |", p1_shots, p2_shots));
    output.push_str(&format!(
        "\n| Avg Power | {:.2} | {:.2} |",
        p1_avg_power, p2_avg_power
    ));
    output.push_str(&format!(
        "\n| Avg Accuracy | {:.2} | {:.2} |",
        p1_avg_accuracy, p2_avg_accuracy
    ));

    output.push_str("\n\n");
    output
}

/// ラリーを生成
/// @spec REQ-77201-008: 詳細度に応じた出力
fn format_rally(rally: &Rally, options: &FormatOptions) -> String {
    let mut output = format!(
        "### Rally {} (Frame {}-{})\n\n",
        rally.number, rally.start_frame, rally.end_frame
    );

    // 基本情報
    output.push_str(&format!(
        "**Result**: P{} wins ({})\n",
        rally.winner, rally.end_reason
    ));
    output.push_str(&format!("**Duration**: {:.2}s\n", rally.duration_secs));
    output.push_str(&format!("**Shots**: {}\n\n", rally.stats.shot_count));

    // @spec REQ-77201-008: 詳細度に応じて play-by-play を出力
    match options.detail_level {
        DetailLevel::Summary => {
            // サマリーのみ - play-by-play なし
        }
        DetailLevel::Normal => {
            // 主要ショット情報のみ（stats から出力）
            if rally.stats.shot_count > 0 {
                output.push_str("#### Shot Summary\n\n");
                output.push_str("| Player | Power | Accuracy | Spin |\n");
                output.push_str("|--------|-------|----------|------|\n");

                if rally.stats.p1_shot_count > 0 {
                    output.push_str(&format!(
                        "| P1 (x{}) | {:.2} | {:.2} | {:.2} |\n",
                        rally.stats.p1_shot_count,
                        rally.stats.p1_avg_power,
                        rally.stats.p1_avg_accuracy,
                        rally.stats.p1_avg_spin
                    ));
                }
                if rally.stats.p2_shot_count > 0 {
                    output.push_str(&format!(
                        "| P2 (x{}) | {:.2} | {:.2} | {:.2} |\n",
                        rally.stats.p2_shot_count,
                        rally.stats.p2_avg_power,
                        rally.stats.p2_avg_accuracy,
                        rally.stats.p2_avg_spin
                    ));
                }
                output.push('\n');
            }
        }
        DetailLevel::Full => {
            // 全ショット詳細
            if !rally.shots.is_empty() {
                output.push_str("#### Play-by-Play\n\n");
                output.push_str("| Frame | Player | Power | Stability | Accuracy | Spin |\n");
                output.push_str("|-------|--------|-------|-----------|----------|------|\n");

                for shot in &rally.shots {
                    output.push_str(&format!(
                        "| {} | P{} | {:.2} | {:.2} | {:.2} | {:.2} |\n",
                        shot.frame,
                        shot.player,
                        shot.power,
                        shot.stability,
                        shot.accuracy,
                        shot.spin
                    ));
                }
                output.push('\n');
            }
        }
    }

    // バウンス・壁反射情報
    if options.detail_level != DetailLevel::Summary {
        output.push_str(&format!(
            "- Bounces: {}, Wall reflects: {}\n\n",
            rally.bounce_count, rally.wall_reflect_count
        ));
    }

    // @spec REQ-77201-005: 異常ハイライト
    if !rally.anomalies.is_empty() {
        output.push_str("#### Anomalies\n\n");
        for anomaly in &rally.anomalies {
            output.push_str(&format!(
                "- {} **Frame {}**: {}\n",
                anomaly.severity.emoji(),
                anomaly.frame,
                anomaly.description
            ));
        }
        output.push('\n');
    }

    output
}

/// 統計的異常を生成
fn format_statistical_anomalies(anomalies: &[Anomaly]) -> String {
    let mut output = String::from("## Statistical Anomalies\n\n");

    for anomaly in anomalies {
        output.push_str(&format!(
            "- {} **Frame {}**: {}\n",
            anomaly.severity.emoji(),
            anomaly.frame,
            anomaly.description
        ));
    }

    output.push('\n');
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::{AnomalySeverity, RallyStats};

    fn make_rally(number: u32, winner: u8, shots: u32) -> Rally {
        Rally {
            number,
            start_frame: 0,
            end_frame: 100,
            duration_secs: 5.0,
            winner,
            end_reason: "DoubleBounce".to_string(),
            shots: Vec::new(),
            bounce_count: 2,
            wall_reflect_count: 0,
            anomalies: Vec::new(),
            stats: RallyStats {
                shot_count: shots,
                p1_shot_count: shots / 2,
                p2_shot_count: shots - shots / 2,
                p1_avg_power: 0.7,
                p2_avg_power: 0.65,
                p1_avg_accuracy: 0.8,
                p2_avg_accuracy: 0.75,
                p1_avg_spin: 0.1,
                p2_avg_spin: -0.1,
            },
        }
    }

    #[test]
    fn test_detail_level_from_str() {
        assert_eq!(DetailLevel::from_str("summary"), DetailLevel::Summary);
        assert_eq!(DetailLevel::from_str("FULL"), DetailLevel::Full);
        assert_eq!(DetailLevel::from_str("normal"), DetailLevel::Normal);
        assert_eq!(DetailLevel::from_str("unknown"), DetailLevel::Normal);
    }

    #[test]
    fn test_format_rally_summary_mode() {
        let rally = make_rally(1, 1, 4);
        let options = FormatOptions {
            detail_level: DetailLevel::Summary,
            ..Default::default()
        };

        let output = format_rally(&rally, &options);

        assert!(output.contains("Rally 1"));
        assert!(output.contains("P1 wins"));
        // Summary モードでは Play-by-Play や Shot Summary は含まれない
        assert!(!output.contains("Play-by-Play"));
        assert!(!output.contains("Shot Summary"));
    }

    #[test]
    fn test_format_rally_normal_mode() {
        let rally = make_rally(1, 2, 6);
        let options = FormatOptions {
            detail_level: DetailLevel::Normal,
            ..Default::default()
        };

        let output = format_rally(&rally, &options);

        assert!(output.contains("Rally 1"));
        assert!(output.contains("Shot Summary"));
        assert!(!output.contains("Play-by-Play"));
    }

    #[test]
    fn test_format_rally_with_anomalies() {
        let mut rally = make_rally(1, 1, 2);
        rally.anomalies.push(Anomaly {
            frame: 50,
            severity: AnomalySeverity::Warning,
            description: "Test anomaly".to_string(),
            expected: Some(1.0),
            actual: Some(2.0),
        });

        let options = FormatOptions::default();
        let output = format_rally(&rally, &options);

        assert!(output.contains("Anomalies"));
        assert!(output.contains("⚠️"));
        assert!(output.contains("Frame 50"));
    }
}
