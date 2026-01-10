//! Anomaly Detector
//! @spec 77100_headless_sim.md
//!
//! シミュレーション中の異常を検出する。
//! - NaN座標
//! - コート外消失
//! - 状態遷移スタック
//! - 無限ラリー
//! - 物理異常（速度異常等）

use bevy::prelude::*;

use crate::components::{Ball, LogicalPosition, Player, Velocity};
use crate::resource::config::GameConfig;
use crate::resource::FixedDeltaTime;
use crate::resource::MatchFlowState;

use super::config::AnomalyThresholds;

/// 異常の種類
#[derive(Clone, Debug, PartialEq)]
pub enum AnomalyType {
    /// NaN座標検出
    NanPosition { entity_type: String },
    /// NaN速度検出
    NanVelocity { entity_type: String },
    /// コート外消失
    OutOfBounds { entity_type: String, position: Vec3 },
    /// 状態遷移スタック（一定時間同じ状態が継続）
    StateStuck { state: String, duration_secs: f32 },
    /// 無限ラリー（ラリー時間が上限を超過）
    InfiniteRally { duration_secs: f32 },
    /// 物理異常（速度が異常に大きい等）
    PhysicsAnomaly { description: String },
}

/// 異常レポート
#[derive(Clone, Debug)]
pub struct AnomalyReport {
    /// 異常の種類
    pub anomaly_type: AnomalyType,
    /// 発生フレーム
    pub frame: u64,
    /// 発生時刻（シミュレーション開始からの経過秒数）
    pub timestamp_secs: f32,
}

/// 異常検出器
#[derive(Default)]
pub struct AnomalyDetector {
    /// 検出された異常のリスト
    anomalies: Vec<AnomalyReport>,
    /// 現在のフレーム数
    frame_count: u64,
    /// シミュレーション経過時間
    elapsed_secs: f32,
    /// 最後の状態遷移からの経過時間
    state_duration: f32,
    /// 前回の状態
    last_state: Option<MatchFlowState>,
    /// ラリー開始からの経過時間
    rally_duration: f32,
}

impl AnomalyDetector {
    /// 新規作成
    pub fn new() -> Self {
        Self::default()
    }

    /// 検出された異常を取得
    pub fn anomalies(&self) -> &[AnomalyReport] {
        &self.anomalies
    }

    /// 異常をクリア
    pub fn clear(&mut self) {
        self.anomalies.clear();
        self.frame_count = 0;
        self.elapsed_secs = 0.0;
        self.state_duration = 0.0;
        self.last_state = None;
        self.rally_duration = 0.0;
    }

    /// 異常を記録
    fn record_anomaly(&mut self, anomaly_type: AnomalyType) {
        let report = AnomalyReport {
            anomaly_type: anomaly_type.clone(),
            frame: self.frame_count,
            timestamp_secs: self.elapsed_secs,
        };
        eprintln!(
            "[ANOMALY] Frame {}: {:?}",
            self.frame_count, anomaly_type
        );
        self.anomalies.push(report);
    }
}

/// 異常検出リソース
#[derive(Resource, Default)]
pub struct AnomalyDetectorResource {
    pub detector: AnomalyDetector,
}

/// 異常検出閾値リソース（simulation_config.ron から読み込み）
#[derive(Resource, Default)]
pub struct AnomalyThresholdsResource(pub AnomalyThresholds);

/// 異常検出プラグイン
pub struct AnomalyDetectorPlugin;

impl Plugin for AnomalyDetectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnomalyDetectorResource>()
            .init_resource::<AnomalyThresholdsResource>()
            .add_systems(Update, (
                detect_nan_positions,
                detect_out_of_bounds,
                detect_state_stuck,
                detect_infinite_rally,
                detect_physics_anomaly,
                update_detector_state,
            ).chain());
    }
}

/// NaN座標検出システム
fn detect_nan_positions(
    mut detector: ResMut<AnomalyDetectorResource>,
    players: Query<&LogicalPosition, With<Player>>,
    balls: Query<&LogicalPosition, With<Ball>>,
) {
    for pos in players.iter() {
        if pos.value.x.is_nan() || pos.value.y.is_nan() || pos.value.z.is_nan() {
            detector.detector.record_anomaly(AnomalyType::NanPosition {
                entity_type: "Player".to_string(),
            });
        }
    }

    for pos in balls.iter() {
        if pos.value.x.is_nan() || pos.value.y.is_nan() || pos.value.z.is_nan() {
            detector.detector.record_anomaly(AnomalyType::NanPosition {
                entity_type: "Ball".to_string(),
            });
        }
    }
}

/// コート外消失検出システム
fn detect_out_of_bounds(
    mut detector: ResMut<AnomalyDetectorResource>,
    thresholds: Res<AnomalyThresholdsResource>,
    config: Res<GameConfig>,
    players: Query<&LogicalPosition, With<Player>>,
    balls: Query<&LogicalPosition, With<Ball>>,
) {
    let x_limit = config.court.depth / 2.0 + thresholds.0.bounds_margin;
    let z_limit = config.court.width / 2.0 + thresholds.0.bounds_margin;

    for pos in players.iter() {
        let p = pos.value;
        if p.x.abs() > x_limit || p.z.abs() > z_limit || p.y > thresholds.0.height_limit || p.y < thresholds.0.height_floor {
            detector.detector.record_anomaly(AnomalyType::OutOfBounds {
                entity_type: "Player".to_string(),
                position: p,
            });
        }
    }

    for pos in balls.iter() {
        let p = pos.value;
        if p.x.abs() > x_limit || p.z.abs() > z_limit || p.y > thresholds.0.height_limit || p.y < thresholds.0.height_floor {
            detector.detector.record_anomaly(AnomalyType::OutOfBounds {
                entity_type: "Ball".to_string(),
                position: p,
            });
        }
    }
}

/// 状態遷移スタック検出システム
fn detect_state_stuck(
    mut detector: ResMut<AnomalyDetectorResource>,
    thresholds: Res<AnomalyThresholdsResource>,
    state: Res<State<MatchFlowState>>,
    fixed_dt: Res<FixedDeltaTime>,
) {
    let current_state = *state.get();
    let dt = fixed_dt.delta_secs();

    if detector.detector.last_state == Some(current_state) {
        detector.detector.state_duration += dt;

        if detector.detector.state_duration > thresholds.0.state_stuck_secs {
            let duration = detector.detector.state_duration;
            detector.detector.record_anomaly(AnomalyType::StateStuck {
                state: format!("{:?}", current_state),
                duration_secs: duration,
            });
            // 一度記録したらリセット（連続記録を防ぐ）
            detector.detector.state_duration = 0.0;
        }
    } else {
        detector.detector.last_state = Some(current_state);
        detector.detector.state_duration = 0.0;
    }
}

/// 無限ラリー検出システム
fn detect_infinite_rally(
    mut detector: ResMut<AnomalyDetectorResource>,
    thresholds: Res<AnomalyThresholdsResource>,
    state: Res<State<MatchFlowState>>,
    fixed_dt: Res<FixedDeltaTime>,
) {
    let current_state = *state.get();
    let dt = fixed_dt.delta_secs();

    if current_state == MatchFlowState::Rally {
        detector.detector.rally_duration += dt;

        if detector.detector.rally_duration > thresholds.0.infinite_rally_secs {
            let duration = detector.detector.rally_duration;
            detector.detector.record_anomaly(AnomalyType::InfiniteRally {
                duration_secs: duration,
            });
            // 一度記録したらリセット
            detector.detector.rally_duration = 0.0;
        }
    } else {
        detector.detector.rally_duration = 0.0;
    }
}

/// 物理異常検出システム
fn detect_physics_anomaly(
    mut detector: ResMut<AnomalyDetectorResource>,
    thresholds: Res<AnomalyThresholdsResource>,
    velocities: Query<&Velocity>,
) {
    for vel in velocities.iter() {
        let v = vel.value;
        let speed = v.length();
        if speed > thresholds.0.max_velocity {
            detector.detector.record_anomaly(AnomalyType::PhysicsAnomaly {
                description: format!("Velocity too high: {}", speed),
            });
        }
        if v.x.is_nan() || v.y.is_nan() || v.z.is_nan() {
            detector.detector.record_anomaly(AnomalyType::NanVelocity {
                entity_type: "Unknown".to_string(),
            });
        }
    }
}

/// 検出器状態更新システム
fn update_detector_state(
    mut detector: ResMut<AnomalyDetectorResource>,
    fixed_dt: Res<FixedDeltaTime>,
) {
    detector.detector.frame_count += 1;
    detector.detector.elapsed_secs += fixed_dt.delta_secs();
}
