//! 壁反射ロジック
//! @spec 30503_boundary_behavior.md
//!
//! ## 座標系
//! - X軸: 打ち合い方向（前後壁 = バックライン）
//! - Y軸: 高さ方向（天井）
//! - Z軸: コート幅方向（左右壁 = サイドウォール）
//!
//! ## 反射の物理
//! - 入射角 = 反射角（法線に対して）
//! - BounceFactor による速度減衰
//!
//! ## 優先順位 (BEH-30503-007)
//! 左右壁（Z） → 前後壁（X） → 天井（Y）

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
/// @spec 30503_boundary_behavior.md#beh-30503-004
pub struct WallReflection;

impl WallReflection {
    /// 左右壁の反射（速度Z成分反転、コート幅方向）
    /// @spec 30503_boundary_behavior.md#beh-30503-004
    ///
    /// # Arguments
    /// * `velocity` - 入射速度
    /// * `bounce_factor` - バウンド係数（0.0〜1.0）
    ///
    /// # Returns
    /// 反射後の速度（Z成分のみ反転・減衰、他成分は維持）
    #[inline]
    pub fn reflect_side_wall(velocity: Vec3, bounce_factor: f32) -> Vec3 {
        Vec3::new(
            velocity.x, // 維持
            velocity.y, // 維持
            -velocity.z * bounce_factor,
        )
    }

    /// 前後壁の反射（速度X成分反転、打ち合い方向）
    /// @spec 30503_boundary_behavior.md#beh-30503-004
    ///
    /// # Arguments
    /// * `velocity` - 入射速度
    /// * `bounce_factor` - バウンド係数（0.0〜1.0）
    ///
    /// # Returns
    /// 反射後の速度（X成分のみ反転・減衰、他成分は維持）
    #[inline]
    pub fn reflect_back_wall(velocity: Vec3, bounce_factor: f32) -> Vec3 {
        Vec3::new(
            -velocity.x * bounce_factor,
            velocity.y, // 維持
            velocity.z, // 維持
        )
    }

    /// 天井の反射（速度Y成分反転）
    /// @spec 30503_boundary_behavior.md#beh-30503-004
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
            velocity.x, // 維持
            -velocity.y * bounce_factor,
            velocity.z, // 維持
        )
    }

    /// 壁種別に応じた反射計算
    /// @spec 30503_boundary_behavior.md#beh-30503-004
    ///
    /// # Arguments
    /// * `wall_type` - 壁の種類
    /// * `velocity` - 入射速度
    /// * `bounce_factor` - バウンド係数（0.0〜1.0）
    ///
    /// # Returns
    /// 反射後の速度
    #[allow(dead_code)] // テストで使用、将来の壁反射統合で使用予定
    pub fn reflect(wall_type: WallType, velocity: Vec3, bounce_factor: f32) -> Vec3 {
        match wall_type {
            WallType::LeftWall | WallType::RightWall => {
                Self::reflect_side_wall(velocity, bounce_factor)
            }
            WallType::BackWallLeft | WallType::BackWallRight => {
                Self::reflect_back_wall(velocity, bounce_factor)
            }
            WallType::Ceiling => Self::reflect_ceiling(velocity, bounce_factor),
        }
    }

    /// 位置が壁に接触しているかチェックし、接触していれば反射結果を返す
    /// @spec 30503_boundary_behavior.md#beh-30503-007
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
    /// 左右壁（Z） → 前後壁（X） → 天井（Y）
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

        // 優先順位1: 左右壁（Z方向、コート幅）
        if position.z <= bounds.left && velocity.z < 0.0 {
            return Some(WallReflectionResult {
                wall_type: WallType::LeftWall,
                contact_point: Vec3::new(position.x, position.y, bounds.left),
                reflected_velocity: Self::reflect_side_wall(velocity, bounce_factor),
            });
        }
        if position.z >= bounds.right && velocity.z > 0.0 {
            return Some(WallReflectionResult {
                wall_type: WallType::RightWall,
                contact_point: Vec3::new(position.x, position.y, bounds.right),
                reflected_velocity: Self::reflect_side_wall(velocity, bounce_factor),
            });
        }

        // 優先順位2: 前後壁（X方向、打ち合い方向）
        if position.x <= bounds.back_left && velocity.x < 0.0 {
            return Some(WallReflectionResult {
                wall_type: WallType::BackWallLeft,
                contact_point: Vec3::new(bounds.back_left, position.y, position.z),
                reflected_velocity: Self::reflect_back_wall(velocity, bounce_factor),
            });
        }
        if position.x >= bounds.back_right && velocity.x > 0.0 {
            return Some(WallReflectionResult {
                wall_type: WallType::BackWallRight,
                contact_point: Vec3::new(bounds.back_right, position.y, position.z),
                reflected_velocity: Self::reflect_back_wall(velocity, bounce_factor),
            });
        }

        // 優先順位3: 天井（Y方向）
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

    /// テスト用の境界を生成
    /// - Z方向（コート幅）: left=-5, right=5
    /// - X方向（打ち合い方向）: back_left=-3, back_right=3
    /// - Y方向: ground=0, ceiling=5
    fn test_bounds() -> CourtBounds {
        CourtBounds::new(-5.0, 5.0, -3.0, 3.0, 0.0, 5.0)
    }

    /// TST-30504-007: 左右壁の反射（Z成分反転）
    #[test]
    fn test_beh_30503_004_side_wall_reflection() {
        let velocity = Vec3::new(10.0, 5.0, 3.0);
        let bounce_factor = 0.8;

        let reflected = WallReflection::reflect_side_wall(velocity, bounce_factor);

        // Z成分のみ反転・減衰、他成分は維持
        assert_eq!(reflected.x, 10.0); // 維持
        assert_eq!(reflected.y, 5.0); // 維持
        assert_eq!(reflected.z, -3.0 * 0.8);
    }

    /// TST-30504-008: 前後壁の反射（X成分反転）
    #[test]
    fn test_beh_30503_004_back_wall_reflection() {
        let velocity = Vec3::new(10.0, 5.0, 3.0);
        let bounce_factor = 0.8;

        let reflected = WallReflection::reflect_back_wall(velocity, bounce_factor);

        // X成分のみ反転・減衰、他成分は維持
        assert_eq!(reflected.x, -10.0 * 0.8);
        assert_eq!(reflected.y, 5.0); // 維持
        assert_eq!(reflected.z, 3.0); // 維持
    }

    /// TST-30504-009: 天井の反射
    #[test]
    fn test_beh_30503_004_ceiling_reflection() {
        let velocity = Vec3::new(10.0, 5.0, 3.0);
        let bounce_factor = 0.8;

        let reflected = WallReflection::reflect_ceiling(velocity, bounce_factor);

        // Y成分のみ反転・減衰、他成分は維持
        assert_eq!(reflected.x, 10.0); // 維持
        assert_eq!(reflected.y, -5.0 * 0.8);
        assert_eq!(reflected.z, 3.0); // 維持
    }

    /// 壁種別ごとの反射
    #[test]
    fn test_reflect_by_wall_type() {
        let velocity = Vec3::new(10.0, 5.0, 3.0);
        let bounce_factor = 0.8;

        // 左壁（Z成分のみ反転・減衰、コート幅方向）
        let left = WallReflection::reflect(WallType::LeftWall, velocity, bounce_factor);
        assert_eq!(left.x, 10.0); // 維持
        assert_eq!(left.y, 5.0); // 維持
        assert_eq!(left.z, -2.4);

        // 右壁（Z成分のみ反転・減衰）
        let right = WallReflection::reflect(WallType::RightWall, velocity, bounce_factor);
        assert_eq!(right.z, -2.4);

        // 後壁（Left側）（X成分のみ反転・減衰、打ち合い方向）
        let back_left = WallReflection::reflect(WallType::BackWallLeft, velocity, bounce_factor);
        assert_eq!(back_left.x, -8.0);
        assert_eq!(back_left.z, 3.0); // 維持

        // 後壁（Right側）（X成分のみ反転・減衰）
        let back_right = WallReflection::reflect(WallType::BackWallRight, velocity, bounce_factor);
        assert_eq!(back_right.x, -8.0);

        // 天井（Y成分のみ反転・減衰）
        let ceiling = WallReflection::reflect(WallType::Ceiling, velocity, bounce_factor);
        assert_eq!(ceiling.y, -4.0);
        assert_eq!(ceiling.z, 3.0); // 維持
    }

    /// BEH-30503-007: 壁反射の優先順位
    #[test]
    fn test_beh_30503_007_reflection_priority() {
        let bounds = test_bounds();
        let bounce_factor = 0.8;

        // 左壁に接触（優先順位1、Z方向負側）
        let pos_left = Vec3::new(0.0, 2.5, -5.0);
        let vel_left = Vec3::new(0.0, 0.0, -10.0);
        let result = WallReflection::check_and_reflect(pos_left, vel_left, &bounds, bounce_factor);
        assert!(result.is_some());
        assert_eq!(result.unwrap().wall_type, WallType::LeftWall);

        // 右壁に接触（Z方向正側）
        let pos_right = Vec3::new(0.0, 2.5, 5.0);
        let vel_right = Vec3::new(0.0, 0.0, 10.0);
        let result =
            WallReflection::check_and_reflect(pos_right, vel_right, &bounds, bounce_factor);
        assert!(result.is_some());
        assert_eq!(result.unwrap().wall_type, WallType::RightWall);

        // 後壁Left側に接触（優先順位2、X方向負側）
        let pos_back_left = Vec3::new(-3.0, 2.5, 0.0);
        let vel_back_left = Vec3::new(-10.0, 0.0, 0.0);
        let result =
            WallReflection::check_and_reflect(pos_back_left, vel_back_left, &bounds, bounce_factor);
        assert!(result.is_some());
        assert_eq!(result.unwrap().wall_type, WallType::BackWallLeft);

        // 後壁Right側に接触（X方向正側）
        let pos_back_right = Vec3::new(3.0, 2.5, 0.0);
        let vel_back_right = Vec3::new(10.0, 0.0, 0.0);
        let result = WallReflection::check_and_reflect(
            pos_back_right,
            vel_back_right,
            &bounds,
            bounce_factor,
        );
        assert!(result.is_some());
        assert_eq!(result.unwrap().wall_type, WallType::BackWallRight);

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

        // 左壁位置にいるが、右に移動中（Z正方向）
        let pos = Vec3::new(0.0, 2.5, -5.0);
        let vel = Vec3::new(0.0, 0.0, 10.0); // 右向き（Z正方向）
        let result = WallReflection::check_and_reflect(pos, vel, &bounds, bounce_factor);
        assert!(result.is_none());
    }

    /// 静止している場合は反射しない（事前条件）
    #[test]
    fn test_no_reflection_when_stationary() {
        let bounds = test_bounds();
        let bounce_factor = 0.8;

        let pos = Vec3::new(0.0, 2.5, -5.0);
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
        assert!(!WallType::BackWallLeft.is_side_wall());

        assert!(WallType::BackWallLeft.is_back_wall());
        assert!(WallType::BackWallRight.is_back_wall());
        assert!(!WallType::Ceiling.is_back_wall());
    }

    /// WallType の法線ベクトル
    /// 新座標系: LeftWall/RightWall = Z方向, BackWall = X方向
    #[test]
    fn test_wall_type_normal() {
        // 左壁: +Z方向の法線（Z負側の壁）
        assert_eq!(WallType::LeftWall.normal(), Vec3::Z);
        // 右壁: -Z方向の法線（Z正側の壁）
        assert_eq!(WallType::RightWall.normal(), Vec3::NEG_Z);
        // 後壁Left側: +X方向の法線（X負側の壁）
        assert_eq!(WallType::BackWallLeft.normal(), Vec3::X);
        // 後壁Right側: -X方向の法線（X正側の壁）
        assert_eq!(WallType::BackWallRight.normal(), Vec3::NEG_X);
        // 天井: -Y方向の法線
        assert_eq!(WallType::Ceiling.normal(), Vec3::NEG_Y);
    }
}
