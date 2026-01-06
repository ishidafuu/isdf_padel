//! イベント定義
//! @spec 20005_event_system.md

use bevy::prelude::*;

/// ボールヒットイベント
#[derive(Event)]
pub struct BallHitEvent {
    pub ball_id: Entity,
    pub target_id: Entity,
    pub hit_point: Vec3,
}

/// 壁ヒットイベント
#[derive(Event)]
pub struct WallHitEvent {
    pub entity_id: Entity,
    pub wall_type: WallType,
}

/// 壁の種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallType {
    Side,      // 左右の壁
    FrontBack, // 前後の壁
    Ceiling,   // 天井
}
