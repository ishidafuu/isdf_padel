//! 物理関連コンポーネント
//! @spec 20001_layers.md#layer-3-components

use bevy::prelude::*;

/// 論理座標コンポーネント（ゲームロジック用）
/// X: 左右, Y: 高さ（ジャンプ）, Z: 奥行き（コート前後）
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct LogicalPosition {
    pub value: Vec3,
}

impl LogicalPosition {
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }
}

/// 速度コンポーネント
/// @spec 30201_movement_spec.md
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Velocity {
    /// 速度ベクトル
    pub value: Vec3,
}

impl Velocity {
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn is_moving(&self) -> bool {
        self.value.length_squared() > f32::EPSILON
    }
}

/// 接地状態コンポーネント
/// @spec 30202_jump_spec.md
#[derive(Component, Debug, Clone, Copy)]
pub struct GroundedState {
    /// 接地しているかどうか
    pub is_grounded: bool,
}

impl Default for GroundedState {
    fn default() -> Self {
        Self { is_grounded: true }
    }
}
