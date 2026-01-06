//! イベント定義
//! @spec 20005_event_system.md
//! @spec 30502_wall_design.md
//! @spec 30201_movement_spec.md
//! @spec 30202_jump_spec.md
//! @spec 30401_trajectory_spec.md

use bevy::prelude::*;

/// プレイヤー移動イベント
/// @spec 30201_movement_spec.md#req-30201-006
#[derive(Event, Message, Debug, Clone)]
pub struct PlayerMoveEvent {
    /// プレイヤーID
    pub player_id: u8,
    /// 新しい位置
    pub position: Vec3,
    /// 移動速度ベクトル
    pub velocity: Vec3,
}

/// プレイヤージャンプイベント
/// @spec 30202_jump_spec.md#req-30202-007
#[derive(Event, Message, Debug, Clone)]
pub struct PlayerJumpEvent {
    /// プレイヤーID
    pub player_id: u8,
    /// ジャンプ初速度
    pub jump_velocity: f32,
}

/// プレイヤー着地イベント
/// @spec 30202_jump_spec.md#req-30202-008
#[derive(Event, Message, Debug, Clone)]
pub struct PlayerLandEvent {
    /// プレイヤーID
    pub player_id: u8,
    /// 着地位置
    pub land_position: Vec3,
}

/// ボールヒットイベント（プレイヤーにボールが当たった）
/// @spec 30203_knockback_spec.md#req-30203-001
#[derive(Event, Message, Debug, Clone)]
pub struct BallHitEvent {
    /// ボールのEntity
    pub ball_id: Entity,
    /// 被弾したプレイヤーのEntity
    pub target_id: Entity,
    /// ボールの速度
    pub ball_velocity: Vec3,
    /// 衝突位置
    pub hit_point: Vec3,
}

/// プレイヤーふっとばしイベント
/// @spec 30203_knockback_spec.md#req-30203-007
#[derive(Event, Message, Debug, Clone)]
pub struct PlayerKnockbackEvent {
    /// プレイヤーID
    pub player_id: u8,
    /// ふっとばし速度ベクトル
    pub knockback_velocity: Vec3,
    /// ふっとばし時間（秒）
    pub duration: f32,
    /// 無敵時間（秒）
    pub invincibility_time: f32,
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

/// ボールアウトオブバウンズイベント
/// @spec 30401_trajectory_spec.md#req-30401-006
#[derive(Event, Message, Debug, Clone)]
pub struct BallOutOfBoundsEvent {
    /// アウトになったボールのEntity
    pub ball: Entity,
    /// 最終位置
    pub final_position: Vec3,
}
