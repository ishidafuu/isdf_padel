//! 型定義 - EntityType, EntityTrace, FrameTrace
//! @spec 77100_headless_sim.md

use bevy::prelude::*;

use super::events::GameEvent;

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
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            EntityType::Player1 => "Player1",
            EntityType::Player2 => "Player2",
            EntityType::Ball => "Ball",
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
