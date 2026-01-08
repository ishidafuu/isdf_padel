//! 壁反射ロジック
//! @spec 30502_wall_design.md
//!
//! ## 反射の物理
//! - 入射角 = 反射角（法線に対して）
//! - BounceFactor による速度減衰
//!
//! ## 優先順位 (BEH-30502-004)
//! 左右壁 → 前後壁 → 天井

use bevy::prelude::*;

use super::CourtBounds;
use crate::core::events::WallType;

/// 壁反射計算結果
#[derive(Debug, Clone)]
pub struct WallReflectionResult {
    /// 反射した壁の種類
    pub wall_type: WallType,
    /// 接触点
    pub contact_point: Vec3,
    /// 反射後の速度
    pub reflected_velocity: Vec3,
}

/// 壁反射システム
/// @spec 30502_wall_design.md
pub struct WallReflection;

impl WallReflection {
    /// 左右壁の反射（速度X成分反転）
    /// @spec 30502_wall_design.md#beh-30502-001
    ///
    /// # Arguments
    /// * `velocity` - 入射速度
    /// * `bounce_factor` - バウンド係数（0.0〜1.0）
    ///
    /// # Returns
    /// 反射後の速度（X成分のみ反転・減衰、他成分は維持）
    #[inline]
    pub fn reflect_side_wall(velocity: Vec3, bounce_factor: f32) -> Vec3 {
        Vec3::new(
            -velocity.x * bounce_factor,
            velocity.y,  // 維持
            velocity.z,  // 維持
        )
    }

    /// 前後壁の反射（速度Z成分反転）
    /// @spec 30502_wall_design.md#beh-30502-002
    ///
    /// # Arguments
    /// * `velocity` - 入射速度
    /// * `bounce_factor` - バウンド係数（0.0〜1.0）
    ///
    /// # Returns
    /// 反射後の速度（Z成分のみ反転・減衰、他成分は維持）
    #[inline]
    pub fn reflect_back_wall(velocity: Vec3, bounce_factor: f32) -> Vec3 {
        Vec3::new(
            velocity.x,  // 維持
            velocity.y,  // 維持
            -velocity.z * bounce_factor,
        )
    }

    /// 天井の反射（速度Y成分反転）
    /// @spec 30502_wall_design.md#beh-30502-003
    ///
    /// # Arguments
    /// * `velocity` - 入射速度
    /// * `bounce_factor` - バウンド係数（0.0〜1.0）
    ///
    /// # Returns
    /// 反射後の速度（Y成分のみ反転・減衰、他成分は維持）
    #[inline]
    pub fn reflect_ceiling(velocity: Vec3, bounce_factor: f32) -> Vec3 {
        Vec3::new(
            velocity.x,  // 維持
            -velocity.y * bounce_factor,
            velocity.z,  // 維持
        )
    }

    /// 壁種別に応じた反射計算
    /// @spec 30502_wall_design.md#beh-30502-001
    /// @spec 30502_wall_design.md#beh-30502-002
    /// @spec 30502_wall_design.md#beh-30502-003
    ///
    /// # Arguments
    /// * `wall_type` - 壁の種類
    /// * `velocity` - 入射速度
    /// * `bounce_factor` - バウンド係数（0.0〜1.0）
    ///
    /// # Returns
    /// 反射後の速度
    pub fn reflect(wall_type: WallType, velocity: Vec3, bounce_factor: f32) -> Vec3 {
        match wall_type {
            WallType::LeftWall | WallType::RightWall => {
                Self::reflect_side_wall(velocity, bounce_factor)
            }
            WallType::BackWall1P | WallType::BackWall2P => {
                Self::reflect_back_wall(velocity, bounce_factor)
            }
            WallType::Ceiling => Self::reflect_ceiling(velocity, bounce_factor),
        }
    }

    /// 位置が壁に接触しているかチェックし、接触していれば反射結果を返す
    /// @spec 30502_wall_design.md#beh-30502-004
    ///
    /// # Arguments
    /// * `position` - 現在位置
    /// * `velocity` - 入射速度
    /// * `bounds` - コート境界
    /// * `bounce_factor` - バウンド係数
    ///
    /// # Returns
    /// 接触していれば `Some(WallReflectionResult)`、なければ `None`
    ///
    /// # 優先順位
    /// 左右壁 → 前後壁 → 天井
    pub fn check_and_reflect(
        position: Vec3,
        velocity: Vec3,
        bounds: &CourtBounds,
        bounce_factor: f32,
    ) -> Option<WallReflectionResult> {
        // 静止している場合は反射しない（事前条件）
        if velocity.length_squared() < f32::EPSILON {
            return None;
        }

        // 優先順位1: 左右壁
        if position.x <= bounds.left && velocity.x < 0.0 {
            return Some(WallReflectionResult {
                wall_type: WallType::LeftWall,
                contact_point: Vec3::new(bounds.left, position.y, position.z),
                reflected_velocity: Self::reflect_side_wall(velocity, bounce_factor),
            });
        }
        if position.x >= bounds.right && velocity.x > 0.0 {
            return Some(WallReflectionResult {
                wall_type: WallType::RightWall,
                contact_point: Vec3::new(bounds.right, position.y, position.z),
                reflected_velocity: Self::reflect_side_wall(velocity, bounce_factor),
            });
        }

        // 優先順位2: 前後壁
        if position.z <= bounds.back_1p && velocity.z < 0.0 {
            return Some(WallReflectionResult {
                wall_type: WallType::BackWall1P,
                contact_point: Vec3::new(position.x, position.y, bounds.back_1p),
                reflected_velocity: Self::reflect_back_wall(velocity, bounce_factor),
            });
        }
        if position.z >= bounds.back_2p && velocity.z > 0.0 {
            return Some(WallReflectionResult {
                wall_type: WallType::BackWall2P,
                contact_point: Vec3::new(position.x, position.y, bounds.back_2p),
                reflected_velocity: Self::reflect_back_wall(velocity, bounce_factor),
            });
        }

        // 優先順位3: 天井
        if position.y >= bounds.ceiling && velocity.y > 0.0 {
            return Some(WallReflectionResult {
                wall_type: WallType::Ceiling,
                contact_point: Vec3::new(position.x, bounds.ceiling, position.z),
                reflected_velocity: Self::reflect_ceiling(velocity, bounce_factor),
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト用の境界を生成（width=10, depth=6, ceiling=5）
    fn test_bounds() -> CourtBounds {
        // left=-5, right=5, back_1p=-3, back_2p=3, ground=0, ceiling=5
        CourtBounds::new(-5.0, 5.0, -3.0, 3.0, 0.0, 5.0)
    }

    /// TST-30504-007: 左右壁の反射
    #[test]
    fn test_beh_30502_001_side_wall_reflection() {
        let velocity = Vec3::new(10.0, 5.0, 3.0);
        let bounce_factor = 0.8;

        let reflected = WallReflection::reflect_side_wall(velocity, bounce_factor);

        // X成分のみ反転・減衰、他成分は維持
        assert_eq!(reflected.x, -10.0 * 0.8);
        assert_eq!(reflected.y, 5.0);  // 維持
        assert_eq!(reflected.z, 3.0);  // 維持
    }

    /// TST-30504-008: 前後壁の反射
    #[test]
    fn test_beh_30502_002_back_wall_reflection() {
        let velocity = Vec3::new(10.0, 5.0, 3.0);
        let bounce_factor = 0.8;

        let reflected = WallReflection::reflect_back_wall(velocity, bounce_factor);

        // Z成分のみ反転・減衰、他成分は維持
        assert_eq!(reflected.x, 10.0);  // 維持
        assert_eq!(reflected.y, 5.0);   // 維持
        assert_eq!(reflected.z, -3.0 * 0.8);
    }

    /// TST-30504-009: 天井の反射
    #[test]
    fn test_beh_30502_003_ceiling_reflection() {
        let velocity = Vec3::new(10.0, 5.0, 3.0);
        let bounce_factor = 0.8;

        let reflected = WallReflection::reflect_ceiling(velocity, bounce_factor);

        // Y成分のみ反転・減衰、他成分は維持
        assert_eq!(reflected.x, 10.0);  // 維持
        assert_eq!(reflected.y, -5.0 * 0.8);
        assert_eq!(reflected.z, 3.0);   // 維持
    }

    /// 壁種別ごとの反射
    #[test]
    fn test_reflect_by_wall_type() {
        let velocity = Vec3::new(10.0, 5.0, 3.0);
        let bounce_factor = 0.8;

        // 左壁（X成分のみ反転・減衰）
        let left = WallReflection::reflect(WallType::LeftWall, velocity, bounce_factor);
        assert_eq!(left.x, -8.0);
        assert_eq!(left.y, 5.0);  // 維持
        assert_eq!(left.z, 3.0);  // 維持

        // 右壁（X成分のみ反転・減衰）
        let right = WallReflection::reflect(WallType::RightWall, velocity, bounce_factor);
        assert_eq!(right.x, -8.0);

        // 後壁（1P側）（Z成分のみ反転・減衰）
        let back_1p = WallReflection::reflect(WallType::BackWall1P, velocity, bounce_factor);
        assert_eq!(back_1p.x, 10.0);  // 維持
        assert_eq!(back_1p.z, -2.4);

        // 後壁（2P側）（Z成分のみ反転・減衰）
        let back_2p = WallReflection::reflect(WallType::BackWall2P, velocity, bounce_factor);
        assert_eq!(back_2p.z, -2.4);

        // 天井（Y成分のみ反転・減衰）
        let ceiling = WallReflection::reflect(WallType::Ceiling, velocity, bounce_factor);
        assert_eq!(ceiling.y, -4.0);
        assert_eq!(ceiling.z, 3.0);  // 維持
    }

    /// BEH-30502-004: 壁反射の優先順位
    #[test]
    fn test_beh_30502_004_reflection_priority() {
        let bounds = test_bounds();
        let bounce_factor = 0.8;

        // 左壁に接触（優先順位1）
        let pos_left = Vec3::new(-5.0, 2.5, 0.0);
        let vel_left = Vec3::new(-10.0, 0.0, 0.0);
        let result = WallReflection::check_and_reflect(pos_left, vel_left, &bounds, bounce_factor);
        assert!(result.is_some());
        assert_eq!(result.unwrap().wall_type, WallType::LeftWall);

        // 右壁に接触
        let pos_right = Vec3::new(5.0, 2.5, 0.0);
        let vel_right = Vec3::new(10.0, 0.0, 0.0);
        let result = WallReflection::check_and_reflect(pos_right, vel_right, &bounds, bounce_factor);
        assert!(result.is_some());
        assert_eq!(result.unwrap().wall_type, WallType::RightWall);

        // 後壁1P側に接触（優先順位2）
        let pos_back1p = Vec3::new(0.0, 2.5, -3.0);
        let vel_back1p = Vec3::new(0.0, 0.0, -10.0);
        let result =
            WallReflection::check_and_reflect(pos_back1p, vel_back1p, &bounds, bounce_factor);
        assert!(result.is_some());
        assert_eq!(result.unwrap().wall_type, WallType::BackWall1P);

        // 後壁2P側に接触
        let pos_back2p = Vec3::new(0.0, 2.5, 3.0);
        let vel_back2p = Vec3::new(0.0, 0.0, 10.0);
        let result =
            WallReflection::check_and_reflect(pos_back2p, vel_back2p, &bounds, bounce_factor);
        assert!(result.is_some());
        assert_eq!(result.unwrap().wall_type, WallType::BackWall2P);

        // 天井に接触（優先順位3）
        let pos_ceiling = Vec3::new(0.0, 5.0, 0.0);
        let vel_ceiling = Vec3::new(0.0, 10.0, 0.0);
        let result =
            WallReflection::check_and_reflect(pos_ceiling, vel_ceiling, &bounds, bounce_factor);
        assert!(result.is_some());
        assert_eq!(result.unwrap().wall_type, WallType::Ceiling);
    }

    /// 壁に向かっていない場合は反射しない
    #[test]
    fn test_no_reflection_when_moving_away() {
        let bounds = test_bounds();
        let bounce_factor = 0.8;

        // 左壁位置にいるが、右に移動中
        let pos = Vec3::new(-5.0, 2.5, 0.0);
        let vel = Vec3::new(10.0, 0.0, 0.0); // 右向き
        let result = WallReflection::check_and_reflect(pos, vel, &bounds, bounce_factor);
        assert!(result.is_none());
    }

    /// 静止している場合は反射しない（事前条件）
    #[test]
    fn test_no_reflection_when_stationary() {
        let bounds = test_bounds();
        let bounce_factor = 0.8;

        let pos = Vec3::new(-5.0, 2.5, 0.0);
        let vel = Vec3::ZERO;
        let result = WallReflection::check_and_reflect(pos, vel, &bounds, bounce_factor);
        assert!(result.is_none());
    }

    /// コート内にいる場合は反射しない
    #[test]
    fn test_no_reflection_inside_court() {
        let bounds = test_bounds();
        let bounce_factor = 0.8;

        let pos = Vec3::new(0.0, 2.5, 0.0);
        let vel = Vec3::new(-10.0, 5.0, -3.0);
        let result = WallReflection::check_and_reflect(pos, vel, &bounds, bounce_factor);
        assert!(result.is_none());
    }

    /// WallType のヘルパーメソッド
    #[test]
    fn test_wall_type_helpers() {
        assert!(WallType::LeftWall.is_side_wall());
        assert!(WallType::RightWall.is_side_wall());
        assert!(!WallType::BackWall1P.is_side_wall());

        assert!(WallType::BackWall1P.is_back_wall());
        assert!(WallType::BackWall2P.is_back_wall());
        assert!(!WallType::Ceiling.is_back_wall());
    }

    /// WallType の法線ベクトル
    #[test]
    fn test_wall_type_normal() {
        assert_eq!(WallType::LeftWall.normal(), Vec3::X);
        assert_eq!(WallType::RightWall.normal(), Vec3::NEG_X);
        assert_eq!(WallType::BackWall1P.normal(), Vec3::Z);
        assert_eq!(WallType::BackWall2P.normal(), Vec3::NEG_Z);
        assert_eq!(WallType::Ceiling.normal(), Vec3::NEG_Y);
    }
}
