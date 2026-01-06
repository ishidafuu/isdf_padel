//! Components層: Entityのデータ構造定義
//! @spec 20001_layers.md#layer-3-components

use bevy::prelude::*;

use crate::core::CourtSide;

/// プレイヤーマーカーコンポーネント
/// @spec 30200_player_overview.md
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    /// プレイヤーID（1 or 2）
    pub id: u8,
    /// プレイヤーがどちら側のコートにいるか
    pub court_side: CourtSide,
}

/// ボールマーカーコンポーネント
/// @spec 30401_ball_spec.md
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Ball;

/// 速度コンポーネント
/// @spec 30201_movement_spec.md
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Velocity {
    /// 速度ベクトル
    pub value: Vec3,
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }

    #[inline]
    pub fn is_moving(&self) -> bool {
        self.value.length_squared() > f32::EPSILON
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

/// ふっとばし状態コンポーネント
/// @spec 30203_knockback_spec.md
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct KnockbackState {
    /// ふっとばし中かどうか
    pub is_active: bool,
    /// 残り時間（秒）
    pub remaining_time: f32,
    /// ふっとばし方向
    pub direction: Vec3,
}

impl KnockbackState {
    /// ふっとばし中かどうか
    /// @spec 30201_movement_spec.md#req-30201-005
    #[inline]
    pub fn is_knockback_active(&self) -> bool {
        self.is_active && self.remaining_time > 0.0
    }
}

/// プレイヤーバンドル（プレイヤー生成時に使用）
/// @spec 30200_player_overview.md
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub velocity: Velocity,
    pub knockback: KnockbackState,
    pub transform: Transform,
}

impl PlayerBundle {
    pub fn new(id: u8, position: Vec3) -> Self {
        let court_side = if id == 1 {
            CourtSide::Player1
        } else {
            CourtSide::Player2
        };
        Self {
            player: Player { id, court_side },
            velocity: Velocity::default(),
            knockback: KnockbackState::default(),
            transform: Transform::from_translation(position),
        }
    }
}
