//! Trace Narrator Data Types
//! @spec 77201_narrative_spec.md REQ-77201-002
//!
//! JSONテレメトリログのパース用データ構造。
//! simulation/event_tracer.rs の構造と互換性を持つがserde対応。

use serde::Deserialize;

/// エンティティ種別
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum EntityType {
    Player1,
    Player2,
    Ball,
}

/// コートサイド
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum CourtSide {
    Left,
    Right,
}

/// 3次元ベクトル（f32配列からデシリアライズ）
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<'de> Deserialize<'de> for Vec3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr: [f32; 3] = Deserialize::deserialize(deserializer)?;
        Ok(Vec3 {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        })
    }
}

/// 異常レベル
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Severity {
    Warning,
    Error,
}

/// ゲームイベント（タグ付きEnum）
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum GameEvent {
    /// ショット実行
    BallHit {
        player: u8,
        shot_type: String,
    },
    /// 地面バウンス
    Bounce {
        position: Vec3,
        court_side: CourtSide,
    },
    /// 壁反射
    WallReflect {
        position: Vec3,
        wall_type: String,
    },
    /// ポイント獲得
    Point {
        winner: u8,
        reason: String,
    },
    /// フォールト
    Fault {
        fault_type: String,
    },
    /// 状態遷移
    StateChange {
        from: String,
        to: String,
    },
    /// ショット属性計算詳細
    ShotAttributesCalculated {
        player_id: u8,
        input_mode: String,
        hit_height: f32,
        bounce_elapsed: Option<f32>,
        approach_dot: f32,
        ball_distance: f32,
        height_factors: [f32; 3],
        timing_factors: [f32; 3],
        approach_factors: [f32; 2],
        distance_factors: [f32; 3],
        final_power: f32,
        final_stability: f32,
        final_angle: f32,
        final_spin: f32,
        final_accuracy: f32,
    },
    /// AI移動決定詳細
    AiMovementDecision {
        player_id: u8,
        movement_state: String,
        ball_coming_to_me: bool,
        reaction_timer: f32,
        landing_time: Option<f32>,
        landing_position: Option<Vec3>,
        trajectory_line_z: f32,
        arrival_distance: f32,
        target_position: Vec3,
    },
    /// 物理異常マーカー
    PhysicsAnomaly {
        anomaly_type: String,
        position: Vec3,
        velocity: Vec3,
        expected_value: f32,
        actual_value: f32,
        severity: String,
    },
}

impl GameEvent {
    /// イベント種別名を取得
    pub fn type_name(&self) -> &'static str {
        match self {
            GameEvent::BallHit { .. } => "BallHit",
            GameEvent::Bounce { .. } => "Bounce",
            GameEvent::WallReflect { .. } => "WallReflect",
            GameEvent::Point { .. } => "Point",
            GameEvent::Fault { .. } => "Fault",
            GameEvent::StateChange { .. } => "StateChange",
            GameEvent::ShotAttributesCalculated { .. } => "ShotAttributesCalculated",
            GameEvent::AiMovementDecision { .. } => "AiMovementDecision",
            GameEvent::PhysicsAnomaly { .. } => "PhysicsAnomaly",
        }
    }

    /// Pointイベントかどうか
    pub fn is_point(&self) -> bool {
        matches!(self, GameEvent::Point { .. })
    }

    /// 異常イベントかどうか
    pub fn is_anomaly(&self) -> bool {
        matches!(self, GameEvent::PhysicsAnomaly { .. })
    }
}

/// エンティティのトレースデータ
#[derive(Debug, Clone, Deserialize)]
pub struct EntityTrace {
    /// エンティティ種別
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    /// 位置
    pub position: Vec3,
    /// 速度
    pub velocity: Vec3,
}

/// 1フレーム分のトレースデータ
#[derive(Debug, Clone, Deserialize)]
pub struct FrameTrace {
    /// フレーム番号
    pub frame: u64,
    /// 経過時間（秒）
    pub timestamp: f32,
    /// エンティティのトレース（位置・速度）
    #[serde(default)]
    pub entities: Vec<EntityTrace>,
    /// このフレームで発生したイベント
    #[serde(default)]
    pub events: Vec<GameEvent>,
}

/// トレースファイル全体（JSON配列形式用）
#[derive(Debug, Clone, Deserialize)]
pub struct TraceFile {
    pub frames: Vec<FrameTrace>,
}
