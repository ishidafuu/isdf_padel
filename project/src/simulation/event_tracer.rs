//! EventTracer - フレーム単位の詳細データ記録
//! @spec 77100_headless_sim.md
//!
//! シミュレーション中の位置・速度・イベントをフレーム単位で記録する。
//! TraceConfig の設定に基づいて記録内容を制御する。
//!
//! Note: 将来のヘッドレスシミュレーション統合に向けて実装済み

#![allow(dead_code)]

use bevy::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Result as IoResult, Write};
use std::path::Path;

use crate::core::CourtSide;

use super::TraceConfig;

/// JSON配列の要素を書き出すヘルパー
fn write_json_array<W: Write>(
    writer: &mut W,
    items: &[String],
    indent: &str,
) -> IoResult<()> {
    for (i, item) in items.iter().enumerate() {
        let comma = if i < items.len() - 1 { "," } else { "" };
        writeln!(writer, "{}{}{}", indent, item, comma)?;
    }
    Ok(())
}

/// エンティティ種別
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    /// プレイヤー1（Left側）
    Player1,
    /// プレイヤー2（Right側）
    Player2,
    /// ボール
    Ball,
}

impl EntityType {
    /// 文字列表現を取得
    fn as_str(&self) -> &'static str {
        match self {
            EntityType::Player1 => "Player1",
            EntityType::Player2 => "Player2",
            EntityType::Ball => "Ball",
        }
    }
}

/// ゲームイベント種別
#[derive(Debug, Clone)]
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
        /// 中間係数 (power, stability, angle)
        height_factors: (f32, f32, f32),
        /// 中間係数 (power, stability, angle)
        timing_factors: (f32, f32, f32),
        /// 中間係数 (power, angle)
        approach_factors: (f32, f32),
        /// 中間係数 (power, stability, accuracy)
        distance_factors: (f32, f32, f32),
        /// 最終結果
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
    fn type_name(&self) -> &'static str {
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

    /// CSV形式の詳細文字列を取得
    fn to_csv_detail(&self) -> String {
        match self {
            GameEvent::BallHit { player, shot_type } => {
                format!("player={},type={}", player, shot_type)
            }
            GameEvent::Bounce { position, court_side } => {
                format!(
                    "pos=({:.2},{:.2},{:.2}),side={:?}",
                    position.x, position.y, position.z, court_side
                )
            }
            GameEvent::WallReflect { position, wall_type } => {
                format!(
                    "pos=({:.2},{:.2},{:.2}),type={}",
                    position.x, position.y, position.z, wall_type
                )
            }
            GameEvent::Point { winner, reason } => {
                format!("winner={},reason={}", winner, reason)
            }
            GameEvent::Fault { fault_type } => {
                format!("type={}", fault_type)
            }
            GameEvent::StateChange { from, to } => {
                format!("from={},to={}", from, to)
            }
            GameEvent::ShotAttributesCalculated {
                player_id,
                input_mode,
                hit_height,
                bounce_elapsed,
                approach_dot,
                ball_distance,
                height_factors,
                timing_factors,
                approach_factors,
                distance_factors,
                final_power,
                final_stability,
                final_angle,
                final_spin,
                final_accuracy,
            } => {
                let bounce_str = bounce_elapsed
                    .map(|v| format!("{:.3}", v))
                    .unwrap_or_else(|| "none".to_string());
                format!(
                    "player={},mode={},height={:.2},bounce={},approach={:.2},dist={:.2},\
                     hf=({:.2},{:.2},{:.2}),tf=({:.2},{:.2},{:.2}),af=({:.2},{:.2}),df=({:.2},{:.2},{:.2}),\
                     power={:.2},stability={:.2},angle={:.2},spin={:.2},accuracy={:.2}",
                    player_id,
                    input_mode,
                    hit_height,
                    bounce_str,
                    approach_dot,
                    ball_distance,
                    height_factors.0,
                    height_factors.1,
                    height_factors.2,
                    timing_factors.0,
                    timing_factors.1,
                    timing_factors.2,
                    approach_factors.0,
                    approach_factors.1,
                    distance_factors.0,
                    distance_factors.1,
                    distance_factors.2,
                    final_power,
                    final_stability,
                    final_angle,
                    final_spin,
                    final_accuracy
                )
            }
            GameEvent::AiMovementDecision {
                player_id,
                movement_state,
                ball_coming_to_me,
                reaction_timer,
                landing_time,
                landing_position,
                trajectory_line_z,
                arrival_distance,
                target_position,
            } => {
                let land_time_str = landing_time
                    .map(|v| format!("{:.3}", v))
                    .unwrap_or_else(|| "none".to_string());
                let land_pos_str = landing_position
                    .map(|p| format!("({:.2},{:.2},{:.2})", p.x, p.y, p.z))
                    .unwrap_or_else(|| "none".to_string());
                format!(
                    "player={},state={},coming={},react={:.3},land_t={},land_p={},\
                     traj_z={:.2},arr_dist={:.2},target=({:.2},{:.2},{:.2})",
                    player_id,
                    movement_state,
                    ball_coming_to_me,
                    reaction_timer,
                    land_time_str,
                    land_pos_str,
                    trajectory_line_z,
                    arrival_distance,
                    target_position.x,
                    target_position.y,
                    target_position.z
                )
            }
            GameEvent::PhysicsAnomaly {
                anomaly_type,
                position,
                velocity,
                expected_value,
                actual_value,
                severity,
            } => {
                format!(
                    "type={},pos=({:.2},{:.2},{:.2}),vel=({:.2},{:.2},{:.2}),\
                     expected={:.2},actual={:.2},severity={}",
                    anomaly_type,
                    position.x,
                    position.y,
                    position.z,
                    velocity.x,
                    velocity.y,
                    velocity.z,
                    expected_value,
                    actual_value,
                    severity
                )
            }
        }
    }

    /// JSON形式の文字列を取得
    fn to_json(&self) -> String {
        match self {
            GameEvent::BallHit { player, shot_type } => {
                format!(
                    "{{\"type\": \"BallHit\", \"player\": {}, \"shot_type\": \"{}\"}}",
                    player, shot_type
                )
            }
            GameEvent::Bounce { position, court_side } => {
                format!(
                    "{{\"type\": \"Bounce\", \"position\": [{:.2}, {:.2}, {:.2}], \"court_side\": \"{:?}\"}}",
                    position.x, position.y, position.z, court_side
                )
            }
            GameEvent::WallReflect { position, wall_type } => {
                format!(
                    "{{\"type\": \"WallReflect\", \"position\": [{:.2}, {:.2}, {:.2}], \"wall_type\": \"{}\"}}",
                    position.x, position.y, position.z, wall_type
                )
            }
            GameEvent::Point { winner, reason } => {
                format!(
                    "{{\"type\": \"Point\", \"winner\": {}, \"reason\": \"{}\"}}",
                    winner, reason
                )
            }
            GameEvent::Fault { fault_type } => {
                format!("{{\"type\": \"Fault\", \"fault_type\": \"{}\"}}", fault_type)
            }
            GameEvent::StateChange { from, to } => {
                format!(
                    "{{\"type\": \"StateChange\", \"from\": \"{}\", \"to\": \"{}\"}}",
                    from, to
                )
            }
            GameEvent::ShotAttributesCalculated {
                player_id,
                input_mode,
                hit_height,
                bounce_elapsed,
                approach_dot,
                ball_distance,
                height_factors,
                timing_factors,
                approach_factors,
                distance_factors,
                final_power,
                final_stability,
                final_angle,
                final_spin,
                final_accuracy,
            } => {
                let bounce_json = bounce_elapsed
                    .map(|v| format!("{:.3}", v))
                    .unwrap_or_else(|| "null".to_string());
                format!(
                    "{{\"type\": \"ShotAttributesCalculated\", \"player_id\": {}, \"input_mode\": \"{}\", \
                     \"hit_height\": {:.2}, \"bounce_elapsed\": {}, \"approach_dot\": {:.2}, \"ball_distance\": {:.2}, \
                     \"height_factors\": [{:.2}, {:.2}, {:.2}], \"timing_factors\": [{:.2}, {:.2}, {:.2}], \
                     \"approach_factors\": [{:.2}, {:.2}], \"distance_factors\": [{:.2}, {:.2}, {:.2}], \
                     \"final_power\": {:.2}, \"final_stability\": {:.2}, \"final_angle\": {:.2}, \
                     \"final_spin\": {:.2}, \"final_accuracy\": {:.2}}}",
                    player_id,
                    input_mode,
                    hit_height,
                    bounce_json,
                    approach_dot,
                    ball_distance,
                    height_factors.0, height_factors.1, height_factors.2,
                    timing_factors.0, timing_factors.1, timing_factors.2,
                    approach_factors.0, approach_factors.1,
                    distance_factors.0, distance_factors.1, distance_factors.2,
                    final_power,
                    final_stability,
                    final_angle,
                    final_spin,
                    final_accuracy
                )
            }
            GameEvent::AiMovementDecision {
                player_id,
                movement_state,
                ball_coming_to_me,
                reaction_timer,
                landing_time,
                landing_position,
                trajectory_line_z,
                arrival_distance,
                target_position,
            } => {
                let land_time_json = landing_time
                    .map(|v| format!("{:.3}", v))
                    .unwrap_or_else(|| "null".to_string());
                let land_pos_json = landing_position
                    .map(|p| format!("[{:.2}, {:.2}, {:.2}]", p.x, p.y, p.z))
                    .unwrap_or_else(|| "null".to_string());
                format!(
                    "{{\"type\": \"AiMovementDecision\", \"player_id\": {}, \"movement_state\": \"{}\", \
                     \"ball_coming_to_me\": {}, \"reaction_timer\": {:.3}, \"landing_time\": {}, \
                     \"landing_position\": {}, \"trajectory_line_z\": {:.2}, \"arrival_distance\": {:.2}, \
                     \"target_position\": [{:.2}, {:.2}, {:.2}]}}",
                    player_id,
                    movement_state,
                    ball_coming_to_me,
                    reaction_timer,
                    land_time_json,
                    land_pos_json,
                    trajectory_line_z,
                    arrival_distance,
                    target_position.x, target_position.y, target_position.z
                )
            }
            GameEvent::PhysicsAnomaly {
                anomaly_type,
                position,
                velocity,
                expected_value,
                actual_value,
                severity,
            } => {
                format!(
                    "{{\"type\": \"PhysicsAnomaly\", \"anomaly_type\": \"{}\", \
                     \"position\": [{:.2}, {:.2}, {:.2}], \"velocity\": [{:.2}, {:.2}, {:.2}], \
                     \"expected_value\": {:.2}, \"actual_value\": {:.2}, \"severity\": \"{}\"}}",
                    anomaly_type,
                    position.x, position.y, position.z,
                    velocity.x, velocity.y, velocity.z,
                    expected_value,
                    actual_value,
                    severity
                )
            }
        }
    }
}

/// エンティティ単体のトレースデータ
#[derive(Debug, Clone)]
pub struct EntityTrace {
    /// エンティティ種別
    pub entity_type: EntityType,
    /// 位置
    pub position: Vec3,
    /// 速度
    pub velocity: Vec3,
}

impl EntityTrace {
    /// JSON形式の文字列を取得
    fn to_json(&self) -> String {
        format!(
            "{{\"type\": \"{}\", \"position\": [{:.2}, {:.2}, {:.2}], \"velocity\": [{:.2}, {:.2}, {:.2}]}}",
            self.entity_type.as_str(),
            self.position.x, self.position.y, self.position.z,
            self.velocity.x, self.velocity.y, self.velocity.z
        )
    }
}

/// 1フレーム分のトレースデータ
#[derive(Debug, Clone)]
pub struct FrameTrace {
    /// フレーム番号
    pub frame: u64,
    /// 経過時間（秒）
    pub timestamp: f32,
    /// エンティティのトレース（位置・速度）
    pub entities: Vec<EntityTrace>,
    /// このフレームで発生したイベント
    pub events: Vec<GameEvent>,
}

impl FrameTrace {
    /// 新規フレームトレースを作成
    pub fn new(frame: u64, timestamp: f32) -> Self {
        Self {
            frame,
            timestamp,
            entities: Vec::new(),
            events: Vec::new(),
        }
    }
}

/// EventTracer リソース
/// シミュレーション中のデータを記録する
#[derive(Resource, Default)]
pub struct EventTracer {
    /// トレース有効フラグ
    pub enabled: bool,
    /// 記録されたフレームデータ
    pub frames: Vec<FrameTrace>,
    /// トレース設定
    pub config: TraceConfig,
    /// 現在のフレーム番号
    current_frame: u64,
    /// 最後に位置を記録したフレーム
    last_position_frame: u64,
    /// 現在フレームのイベントバッファ
    pending_events: Vec<GameEvent>,
}

impl EventTracer {
    /// TraceConfig から EventTracer を作成
    pub fn from_config(config: TraceConfig) -> Self {
        Self {
            enabled: config.enabled,
            config,
            ..Default::default()
        }
    }

    /// フレームを進める
    pub fn advance_frame(&mut self) {
        self.current_frame += 1;
    }

    /// 現在のフレーム番号を取得
    pub fn current_frame(&self) -> u64 {
        self.current_frame
    }

    /// フレームを記録すべきかどうか
    /// position または events のどちらかが有効であれば interval_frames ごとに記録
    pub fn should_record_frame(&self) -> bool {
        if !self.enabled {
            return false;
        }
        // position も events も無効なら記録不要
        if !self.config.position && !self.config.events {
            return false;
        }
        let interval = self.config.interval_frames.max(1) as u64;
        self.current_frame >= self.last_position_frame + interval
    }

    /// 位置・速度データを記録
    pub fn record_positions(&mut self, timestamp: f32, entities: Vec<EntityTrace>) {
        if !self.enabled {
            return;
        }

        // イベントがあれば含める
        let events = std::mem::take(&mut self.pending_events);

        let frame_trace = FrameTrace {
            frame: self.current_frame,
            timestamp,
            entities,
            events,
        };
        self.frames.push(frame_trace);
        self.last_position_frame = self.current_frame;
    }

    /// イベントを記録
    pub fn record_event(&mut self, event: GameEvent) {
        if !self.enabled || !self.config.events {
            return;
        }
        self.pending_events.push(event);
    }

    /// トレースデータをクリア
    pub fn clear(&mut self) {
        self.frames.clear();
        self.pending_events.clear();
        self.current_frame = 0;
        self.last_position_frame = 0;
    }

    /// 記録されたフレーム数を取得
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// 記録されたイベント総数を取得
    pub fn event_count(&self) -> usize {
        self.frames.iter().map(|f| f.events.len()).sum()
    }

    /// CSV形式でファイルに出力
    pub fn write_csv<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // ヘッダー行
        writeln!(
            writer,
            "frame,timestamp,entity,pos_x,pos_y,pos_z,vel_x,vel_y,vel_z,event_type,event_detail"
        )?;

        for frame in &self.frames {
            // エンティティ行
            for entity in &frame.entities {
                writeln!(
                    writer,
                    "{},{:.3},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},,",
                    frame.frame,
                    frame.timestamp,
                    entity.entity_type.as_str(),
                    entity.position.x,
                    entity.position.y,
                    entity.position.z,
                    entity.velocity.x,
                    entity.velocity.y,
                    entity.velocity.z,
                )?;
            }

            // イベント行
            for event in &frame.events {
                writeln!(
                    writer,
                    "{},{:.3},,,,,,,,{},{}",
                    frame.frame,
                    frame.timestamp,
                    event.type_name(),
                    event.to_csv_detail()
                )?;
            }
        }

        writer.flush()?;
        Ok(())
    }

    /// JSON形式でファイルに出力
    pub fn write_json<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "{{")?;
        writeln!(writer, "  \"frames\": [")?;

        for (i, frame) in self.frames.iter().enumerate() {
            writeln!(writer, "    {{")?;
            writeln!(writer, "      \"frame\": {},", frame.frame)?;
            writeln!(writer, "      \"timestamp\": {:.3},", frame.timestamp)?;

            // entities
            let entities_json: Vec<String> =
                frame.entities.iter().map(|e| e.to_json()).collect();
            writeln!(writer, "      \"entities\": [")?;
            write_json_array(&mut writer, &entities_json, "        ")?;
            writeln!(writer, "      ],")?;

            // events
            let events_json: Vec<String> = frame.events.iter().map(|e| e.to_json()).collect();
            writeln!(writer, "      \"events\": [")?;
            write_json_array(&mut writer, &events_json, "        ")?;
            writeln!(writer, "      ]")?;

            let comma = if i < self.frames.len() - 1 { "," } else { "" };
            writeln!(writer, "    }}{}", comma)?;
        }

        writeln!(writer, "  ]")?;
        writeln!(writer, "}}")?;

        writer.flush()?;
        Ok(())
    }

    /// 指定されたパスに基づいて適切な形式で出力
    /// - .csv 拡張子: CSV形式のみ
    /// - .json 拡張子: JSON形式のみ
    /// - それ以外: 両方出力（.csv と .json を付加）
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let path = path.as_ref();
        let extension = path.extension().and_then(|e| e.to_str());

        match extension {
            Some("csv") => self.write_csv(path),
            Some("json") => self.write_json(path),
            _ => {
                // 拡張子なし: 両方出力
                let csv_path = path.with_extension("csv");
                let json_path = path.with_extension("json");
                self.write_csv(&csv_path)?;
                self.write_json(&json_path)?;
                Ok(())
            }
        }
    }
}
