//! EventTracer - フレーム単位の詳細データ記録
//! @spec 77100_headless_sim.md
//!
//! シミュレーション中の位置・速度・イベントをフレーム単位で記録する。
//! TraceConfig の設定に基づいて記録内容を制御する。

use bevy::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::core::CourtSide;

use super::TraceConfig;

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
#[derive(Resource)]
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

impl Default for EventTracer {
    fn default() -> Self {
        Self {
            enabled: false,
            frames: Vec::new(),
            config: TraceConfig::default(),
            current_frame: 0,
            last_position_frame: 0,
            pending_events: Vec::new(),
        }
    }
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
                let entity_name = match entity.entity_type {
                    EntityType::Player1 => "Player1",
                    EntityType::Player2 => "Player2",
                    EntityType::Ball => "Ball",
                };
                writeln!(
                    writer,
                    "{},{:.3},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},,",
                    frame.frame,
                    frame.timestamp,
                    entity_name,
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
                let (event_type, event_detail) = match event {
                    GameEvent::BallHit { player, shot_type } => {
                        ("BallHit".to_string(), format!("player={},type={}", player, shot_type))
                    }
                    GameEvent::Bounce { position, court_side } => (
                        "Bounce".to_string(),
                        format!(
                            "pos=({:.2},{:.2},{:.2}),side={:?}",
                            position.x, position.y, position.z, court_side
                        ),
                    ),
                    GameEvent::WallReflect { position, wall_type } => (
                        "WallReflect".to_string(),
                        format!(
                            "pos=({:.2},{:.2},{:.2}),type={}",
                            position.x, position.y, position.z, wall_type
                        ),
                    ),
                    GameEvent::Point { winner, reason } => {
                        ("Point".to_string(), format!("winner={},reason={}", winner, reason))
                    }
                    GameEvent::Fault { fault_type } => {
                        ("Fault".to_string(), format!("type={}", fault_type))
                    }
                    GameEvent::StateChange { from, to } => {
                        ("StateChange".to_string(), format!("from={},to={}", from, to))
                    }
                };
                writeln!(
                    writer,
                    "{},{:.3},,,,,,,,{},{}",
                    frame.frame, frame.timestamp, event_type, event_detail
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
            writeln!(writer, "      \"entities\": [")?;
            for (j, entity) in frame.entities.iter().enumerate() {
                let entity_name = match entity.entity_type {
                    EntityType::Player1 => "Player1",
                    EntityType::Player2 => "Player2",
                    EntityType::Ball => "Ball",
                };
                let comma = if j < frame.entities.len() - 1 { "," } else { "" };
                writeln!(
                    writer,
                    "        {{\"type\": \"{}\", \"position\": [{:.2}, {:.2}, {:.2}], \"velocity\": [{:.2}, {:.2}, {:.2}]}}{}",
                    entity_name,
                    entity.position.x, entity.position.y, entity.position.z,
                    entity.velocity.x, entity.velocity.y, entity.velocity.z,
                    comma
                )?;
            }
            writeln!(writer, "      ],")?;

            // events
            writeln!(writer, "      \"events\": [")?;
            for (j, event) in frame.events.iter().enumerate() {
                let event_json = match event {
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
                };
                let comma = if j < frame.events.len() - 1 { "," } else { "" };
                writeln!(writer, "        {}{}", event_json, comma)?;
            }
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
