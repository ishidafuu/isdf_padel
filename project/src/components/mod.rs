//! Components層: Entityのデータ構造定義
//! @spec 20001_layers.md#layer-3-components

use bevy::prelude::*;

use crate::core::CourtSide;

/// プレイヤーマーカーコンポーネント
/// @spec 30201_player_spec.md
#[derive(Component, Debug, Clone, Copy)]
pub struct Player {
    /// プレイヤーがどちら側のコートにいるか
    pub court_side: CourtSide,
}

/// ボールマーカーコンポーネント
/// @spec 30401_ball_spec.md
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Ball;

/// 速度コンポーネント
/// @spec 30503_boundary_behavior.md
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }

    #[inline]
    pub fn is_moving(&self) -> bool {
        self.0.length_squared() > f32::EPSILON
    }
}

/// バウンス回数追跡コンポーネント（ツーバウンド判定用）
/// @spec 30503_boundary_behavior.md#beh-30503-006
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BounceCount {
    /// 現在のコート側でのバウンス回数
    pub count: u32,
    /// 最後にバウンドしたコート側
    pub last_court_side: Option<CourtSide>,
}

impl BounceCount {
    /// バウンスを記録
    pub fn record_bounce(&mut self, court_side: CourtSide) {
        if self.last_court_side == Some(court_side) {
            self.count += 1;
        } else {
            self.last_court_side = Some(court_side);
            self.count = 1;
        }
    }

    /// リセット（ショット後など）
    pub fn reset(&mut self) {
        self.count = 0;
        self.last_court_side = None;
    }
}
