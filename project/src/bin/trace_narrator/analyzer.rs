//! Rally Analyzer
//! @spec 77201_narrative_spec.md REQ-77201-003, REQ-77201-004, REQ-77201-005, REQ-77201-006
//!
//! ラリー境界検出、統計計算、異常検出を行う。

use super::types::{FrameTrace, GameEvent};

/// 異常の重大度
/// @spec REQ-77201-005
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnomalySeverity {
    Warning,
    Error,
}

impl AnomalySeverity {
    /// 絵文字表現を取得
    /// @spec REQ-77201-005: severity: "Error" → 🔴, "Warning" → ⚠️
    pub fn emoji(&self) -> &'static str {
        match self {
            AnomalySeverity::Warning => "⚠️",
            AnomalySeverity::Error => "🔴",
        }
    }
}

/// 異常情報
/// @spec REQ-77201-005, REQ-77201-006
#[derive(Debug, Clone)]
pub struct Anomaly {
    /// 発生フレーム
    pub frame: u64,
    /// 重大度
    pub severity: AnomalySeverity,
    /// 説明
    pub description: String,
    /// 期待値
    pub expected: Option<f32>,
    /// 実際の値
    pub actual: Option<f32>,
}

/// ショット情報（統計計算用）
#[derive(Debug, Clone)]
pub struct ShotInfo {
    pub frame: u64,
    pub player: u8,
    pub power: f32,
    pub stability: f32,
    pub accuracy: f32,
    pub spin: f32,
}

/// ラリー統計
/// @spec REQ-77201-004
#[derive(Debug, Clone, Default)]
pub struct RallyStats {
    /// 総ショット数
    pub shot_count: u32,
    /// P1ショット数
    pub p1_shot_count: u32,
    /// P2ショット数
    pub p2_shot_count: u32,
    /// P1平均パワー
    pub p1_avg_power: f32,
    /// P2平均パワー
    pub p2_avg_power: f32,
    /// P1平均精度
    pub p1_avg_accuracy: f32,
    /// P2平均精度
    pub p2_avg_accuracy: f32,
    /// P1平均スピン
    pub p1_avg_spin: f32,
    /// P2平均スピン
    pub p2_avg_spin: f32,
}

/// ラリー情報
/// @spec REQ-77201-003, REQ-77201-004
#[derive(Debug, Clone)]
pub struct Rally {
    /// ラリー番号（1始まり）
    pub number: u32,
    /// 開始フレーム
    pub start_frame: u64,
    /// 終了フレーム
    pub end_frame: u64,
    /// 持続時間（秒）
    pub duration_secs: f32,
    /// 勝者（1 or 2）
    pub winner: u8,
    /// 終了理由
    pub end_reason: String,
    /// ショット情報一覧
    pub shots: Vec<ShotInfo>,
    /// バウンス数
    pub bounce_count: u32,
    /// 壁反射数
    pub wall_reflect_count: u32,
    /// 異常一覧
    pub anomalies: Vec<Anomaly>,
    /// 統計
    pub stats: RallyStats,
}

/// 解析結果
#[derive(Debug)]
pub struct AnalysisResult {
    /// ラリー一覧
    pub rallies: Vec<Rally>,
    /// 全体の異常一覧
    pub all_anomalies: Vec<Anomaly>,
    /// 統計的外れ値（ラリーをまたいだ検出）
    pub statistical_anomalies: Vec<Anomaly>,
}

/// 現在のラリー状態（一時データ保持用）
#[derive(Default)]
struct CurrentRallyState {
    start_frame: u64,
    start_time: f32,
    shots: Vec<ShotInfo>,
    bounces: u32,
    wall_reflects: u32,
    anomalies: Vec<Anomaly>,
    rally_number: u32,
}

impl CurrentRallyState {
    /// ショット属性イベントを処理
    fn handle_shot(&mut self, shot: ShotInfo) -> ShotInfo {
        self.shots.push(shot.clone());
        shot
    }

    /// バウンスイベントを処理
    fn handle_bounce(&mut self) {
        self.bounces += 1;
    }

    /// 壁反射イベントを処理
    fn handle_wall_reflect(&mut self) {
        self.wall_reflects += 1;
    }

    /// 物理異常イベントを処理
    fn handle_anomaly(&mut self, anomaly: Anomaly) -> Anomaly {
        self.anomalies.push(anomaly.clone());
        anomaly
    }

    /// ラリー終了時の処理（Rallyを生成し、状態をリセット）
    fn finalize_rally(&mut self, end_frame: u64, end_time: f32, winner: u8, reason: String) -> Rally {
        self.rally_number += 1;
        let stats = calculate_rally_stats(&self.shots);

        let rally = Rally {
            number: self.rally_number,
            start_frame: self.start_frame,
            end_frame,
            duration_secs: end_time - self.start_time,
            winner,
            end_reason: reason,
            shots: std::mem::take(&mut self.shots),
            bounce_count: self.bounces,
            wall_reflect_count: self.wall_reflects,
            anomalies: std::mem::take(&mut self.anomalies),
            stats,
        };

        // 次のラリーの開始点をリセット
        self.start_frame = end_frame;
        self.start_time = end_time;
        self.bounces = 0;
        self.wall_reflects = 0;

        rally
    }
}

/// GameEventからShotInfoを生成
fn create_shot_info(frame: u64, event: &GameEvent) -> Option<ShotInfo> {
    if let GameEvent::ShotAttributesCalculated {
        player_id,
        final_power,
        final_stability,
        final_accuracy,
        final_spin,
        ..
    } = event
    {
        Some(ShotInfo {
            frame,
            player: *player_id,
            power: *final_power,
            stability: *final_stability,
            accuracy: *final_accuracy,
            spin: *final_spin,
        })
    } else {
        None
    }
}

/// GameEventからAnomalyを生成
fn create_anomaly_from_event(frame: u64, event: &GameEvent) -> Option<Anomaly> {
    if let GameEvent::PhysicsAnomaly {
        anomaly_type,
        expected_value,
        actual_value,
        severity,
        ..
    } = event
    {
        let sev = if severity == "Error" {
            AnomalySeverity::Error
        } else {
            AnomalySeverity::Warning
        };
        Some(Anomaly {
            frame,
            severity: sev,
            description: anomaly_type.clone(),
            expected: Some(*expected_value),
            actual: Some(*actual_value),
        })
    } else {
        None
    }
}

/// ラリー解析を行う
/// @spec REQ-77201-003, REQ-77201-004, REQ-77201-005, REQ-77201-006
pub fn analyze_rallies(frames: &[FrameTrace], anomaly_threshold: f32) -> AnalysisResult {
    let mut rallies = Vec::new();
    let mut all_anomalies = Vec::new();
    let mut all_shots = Vec::new();
    let mut state = CurrentRallyState::default();

    for frame in frames {
        for event in &frame.events {
            match event {
                GameEvent::ShotAttributesCalculated { .. } => {
                    if let Some(shot) = create_shot_info(frame.frame, event) {
                        all_shots.push(state.handle_shot(shot));
                    }
                }
                GameEvent::Bounce { .. } => state.handle_bounce(),
                GameEvent::WallReflect { .. } => state.handle_wall_reflect(),
                GameEvent::PhysicsAnomaly { .. } => {
                    if let Some(anomaly) = create_anomaly_from_event(frame.frame, event) {
                        all_anomalies.push(state.handle_anomaly(anomaly));
                    }
                }
                GameEvent::Point { winner, reason } => {
                    rallies.push(state.finalize_rally(frame.frame, frame.timestamp, *winner, reason.clone()));
                }
                _ => {}
            }
        }
    }

    let statistical_anomalies = detect_statistical_anomalies(&all_shots, anomaly_threshold);

    AnalysisResult {
        rallies,
        all_anomalies,
        statistical_anomalies,
    }
}

/// ラリー統計を計算
/// @spec REQ-77201-004
fn calculate_rally_stats(shots: &[ShotInfo]) -> RallyStats {
    if shots.is_empty() {
        return RallyStats::default();
    }

    let p1_shots: Vec<_> = shots.iter().filter(|s| s.player == 1).collect();
    let p2_shots: Vec<_> = shots.iter().filter(|s| s.player == 2).collect();

    RallyStats {
        shot_count: shots.len() as u32,
        p1_shot_count: p1_shots.len() as u32,
        p2_shot_count: p2_shots.len() as u32,
        p1_avg_power: calculate_average(p1_shots.iter().map(|s| s.power)),
        p2_avg_power: calculate_average(p2_shots.iter().map(|s| s.power)),
        p1_avg_accuracy: calculate_average(p1_shots.iter().map(|s| s.accuracy)),
        p2_avg_accuracy: calculate_average(p2_shots.iter().map(|s| s.accuracy)),
        p1_avg_spin: calculate_average(p1_shots.iter().map(|s| s.spin)),
        p2_avg_spin: calculate_average(p2_shots.iter().map(|s| s.spin)),
    }
}

/// 平均を計算
fn calculate_average(values: impl Iterator<Item = f32>) -> f32 {
    let v: Vec<_> = values.collect();
    if v.is_empty() {
        return 0.0;
    }
    v.iter().sum::<f32>() / v.len() as f32
}

/// 標準偏差を計算
fn calculate_std(values: &[f32], mean: f32) -> f32 {
    if values.len() < 2 {
        return 0.0;
    }
    let variance: f32 = values.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / values.len() as f32;
    variance.sqrt()
}

/// 統計的異常を検出
/// @spec REQ-77201-006: 平均から閾値×標準偏差を超える値を異常としてマーキング
fn detect_statistical_anomalies(shots: &[ShotInfo], threshold: f32) -> Vec<Anomaly> {
    if shots.len() < 3 {
        return Vec::new();
    }

    let mut anomalies = Vec::new();
    detect_power_anomalies(shots, threshold, &mut anomalies);
    detect_spin_anomalies(shots, threshold, &mut anomalies);
    anomalies
}

/// パワーの外れ値を検出
fn detect_power_anomalies(shots: &[ShotInfo], threshold: f32, anomalies: &mut Vec<Anomaly>) {
    let powers: Vec<f32> = shots.iter().map(|s| s.power).collect();
    let power_mean = calculate_average(powers.iter().copied());
    let power_std = calculate_std(&powers, power_mean);

    for shot in shots {
        if power_std > 0.0 && (shot.power - power_mean).abs() > threshold * power_std {
            anomalies.push(Anomaly {
                frame: shot.frame,
                severity: AnomalySeverity::Warning,
                description: format!(
                    "Power outlier (P{}): {:.2} (mean={:.2}, std={:.2})",
                    shot.player, shot.power, power_mean, power_std
                ),
                expected: Some(power_mean),
                actual: Some(shot.power),
            });
        }
    }
}

/// スピンの外れ値を検出
fn detect_spin_anomalies(shots: &[ShotInfo], threshold: f32, anomalies: &mut Vec<Anomaly>) {
    let spins: Vec<f32> = shots.iter().map(|s| s.spin).collect();
    let spin_mean = calculate_average(spins.iter().copied());
    let spin_std = calculate_std(&spins, spin_mean);

    for shot in shots {
        if spin_std > 0.0 && (shot.spin - spin_mean).abs() > threshold * spin_std {
            anomalies.push(Anomaly {
                frame: shot.frame,
                severity: AnomalySeverity::Warning,
                description: format!(
                    "Spin outlier (P{}): {:.2} (mean={:.2}, std={:.2})",
                    shot.player, shot.spin, spin_mean, spin_std
                ),
                expected: Some(spin_mean),
                actual: Some(shot.spin),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_shot(frame: u64, player: u8, power: f32, spin: f32) -> ShotInfo {
        ShotInfo {
            frame,
            player,
            power,
            stability: 0.5,
            accuracy: 0.8,
            spin,
        }
    }

    #[test]
    fn test_calculate_rally_stats_empty() {
        let stats = calculate_rally_stats(&[]);
        assert_eq!(stats.shot_count, 0);
        assert_eq!(stats.p1_avg_power, 0.0);
    }

    #[test]
    fn test_calculate_rally_stats() {
        let shots = vec![
            make_shot(1, 1, 0.6, 0.2),
            make_shot(2, 2, 0.8, -0.1),
            make_shot(3, 1, 0.7, 0.3),
        ];
        let stats = calculate_rally_stats(&shots);

        assert_eq!(stats.shot_count, 3);
        assert_eq!(stats.p1_shot_count, 2);
        assert_eq!(stats.p2_shot_count, 1);
        assert!((stats.p1_avg_power - 0.65).abs() < 0.01);
        assert!((stats.p2_avg_power - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_statistical_anomaly_detection() {
        // 正常範囲のショットと外れ値
        let shots = vec![
            make_shot(1, 1, 0.5, 0.0),
            make_shot(2, 1, 0.52, 0.0),
            make_shot(3, 1, 0.48, 0.0),
            make_shot(4, 1, 0.51, 0.0),
            make_shot(5, 1, 0.49, 0.0),
            make_shot(6, 1, 0.95, 0.0), // 外れ値
        ];

        let anomalies = detect_statistical_anomalies(&shots, 1.5);
        assert!(!anomalies.is_empty(), "Should detect power outlier");
        assert_eq!(anomalies[0].frame, 6);
    }

    #[test]
    fn test_anomaly_severity_emoji() {
        assert_eq!(AnomalySeverity::Warning.emoji(), "⚠️");
        assert_eq!(AnomalySeverity::Error.emoji(), "🔴");
    }
}
