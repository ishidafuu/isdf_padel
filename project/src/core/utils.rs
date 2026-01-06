//! ユーティリティ関数
//! @spec 20001_layers.md#layer-1-core

use bevy::prelude::*;

/// 反射ベクトルを計算
pub fn reflect(velocity: Vec3, normal: Vec3) -> Vec3 {
    velocity - 2.0 * velocity.dot(normal) * normal
}

/// 2D距離（XY平面）
pub fn distance_2d(a: Vec3, b: Vec3) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

/// XZ平面での距離
pub fn distance_xz(a: Vec3, b: Vec3) -> f32 {
    ((a.x - b.x).powi(2) + (a.z - b.z).powi(2)).sqrt()
}
