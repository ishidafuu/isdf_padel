//! EventTracer - フレーム単位の詳細データ記録
//! @spec 77100_headless_sim.md
//!
//! シミュレーション中の位置・速度・イベントをフレーム単位で記録する。
//! TraceConfig の設定に基づいて記録内容を制御する。

use bevy::prelude::*;

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

    /// 位置を記録すべきフレームかどうか（後方互換性のため残存）
    #[deprecated(note = "Use should_record_frame() instead")]
    pub fn should_record_positions(&self) -> bool {
        self.should_record_frame()
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
}
