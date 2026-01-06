//! イベント定義
//! @spec 20005_event_system.md
//! @spec 30502_wall_design.md

use bevy::prelude::*;

/// ボールヒットイベント
#[derive(Message)]
pub struct BallHitEvent {
    pub ball_id: Entity,
    pub target_id: Entity,
    pub hit_point: Vec3,
}

/// 壁の種類
/// @spec 30502_wall_design.md#des-30502-002
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WallType {
    /// 左壁（X = -Court.Width/2）
    LeftWall,
    /// 右壁（X = +Court.Width/2）
    RightWall,
    /// 後壁（1Pコート側、Z = -Court.Depth/2）
    BackWall1P,
    /// 後壁（2Pコート側、Z = +Court.Depth/2）
    BackWall2P,
    /// 天井（Y = Court.CeilingHeight）
    Ceiling,
}

impl WallType {
    /// 壁の法線ベクトルを返す
    /// @spec 30502_wall_design.md#des-30502-001
    #[inline]
    pub fn normal(&self) -> Vec3 {
        match self {
            WallType::LeftWall => Vec3::X,      // 右向き（+X）
            WallType::RightWall => Vec3::NEG_X, // 左向き（-X）
            WallType::BackWall1P => Vec3::Z,    // 前向き（+Z）
            WallType::BackWall2P => Vec3::NEG_Z, // 後ろ向き（-Z）
            WallType::Ceiling => Vec3::NEG_Y,   // 下向き（-Y）
        }
    }

    /// 左右壁かどうか
    #[inline]
    pub fn is_side_wall(&self) -> bool {
        matches!(self, WallType::LeftWall | WallType::RightWall)
    }

    /// 前後壁かどうか
    #[inline]
    pub fn is_back_wall(&self) -> bool {
        matches!(self, WallType::BackWall1P | WallType::BackWall2P)
    }
}

/// 壁反射イベント
/// @spec 30502_wall_design.md#des-30502-003
/// @spec 30502_wall_design.md#beh-30502-005
#[derive(Message, Debug, Clone)]
pub struct WallReflectionEvent {
    /// 反射したボールのEntity
    pub ball: Entity,
    /// 反射した壁の種類
    pub wall_type: WallType,
    /// 接触点の座標
    pub contact_point: Vec3,
    /// 反射前の速度
    pub incident_velocity: Vec3,
    /// 反射後の速度
    pub reflected_velocity: Vec3,
}

/// ネット接触イベント
/// @spec 30503_boundary_behavior.md#beh-30503-005
#[derive(Message, Debug, Clone)]
pub struct NetHitEvent {
    /// ネットに当たったボールのEntity
    pub ball: Entity,
    /// 接触点の座標
    pub contact_point: Vec3,
}

/// 地面バウンドイベント
/// @spec 30503_boundary_behavior.md#beh-30503-006
#[derive(Message, Debug, Clone)]
pub struct GroundBounceEvent {
    /// バウンドしたボールのEntity
    pub ball: Entity,
    /// バウンド位置
    pub bounce_point: Vec3,
    /// バウンドしたコート側
    pub court_side: super::court::CourtSide,
}
