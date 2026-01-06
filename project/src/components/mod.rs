//! Components層: Entityのデータ構造定義
//! @spec 20001_layers.md#layer-3-components
//! @spec 30401_trajectory_spec.md

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
    /// @spec 30203_knockback_spec.md#req-30203-001
    pub is_active: bool,
    /// 残りふっとばし時間（秒）- 操作不能時間
    /// @spec 30203_knockback_spec.md#req-30203-004
    pub remaining_time: f32,
    /// 残り無敵時間（秒）
    /// @spec 30203_knockback_spec.md#req-30203-005
    pub invincibility_time: f32,
    /// ふっとばし速度ベクトル
    /// @spec 30203_knockback_spec.md#req-30203-002
    pub velocity: Vec3,
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

impl KnockbackState {
    /// ふっとばし中かどうか（操作不能状態）
    /// @spec 30203_knockback_spec.md#req-30203-006
    #[inline]
    pub fn is_knockback_active(&self) -> bool {
        self.is_active && self.remaining_time > 0.0
    }

    /// 無敵状態かどうか
    /// @spec 30203_knockback_spec.md#req-30203-005
    #[inline]
    pub fn is_invincible(&self) -> bool {
        self.invincibility_time > 0.0
    }

    /// ふっとばしを開始
    /// @spec 30203_knockback_spec.md#req-30203-001
    pub fn start(&mut self, velocity: Vec3, duration: f32, invincibility_time: f32) {
        self.is_active = true;
        self.velocity = velocity;
        self.remaining_time = duration;
        self.invincibility_time = invincibility_time;
    }

    /// ふっとばしを終了
    /// @spec 30203_knockback_spec.md#req-30203-004
    pub fn end(&mut self) {
        self.is_active = false;
        self.velocity = Vec3::ZERO;
        self.remaining_time = 0.0;
    }
}

/// プレイヤーバンドル（プレイヤー生成時に使用）
/// @spec 30200_player_overview.md
/// @spec 30202_jump_spec.md
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub velocity: Velocity,
    pub knockback: KnockbackState,
    pub grounded: GroundedState,
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
            grounded: GroundedState::default(),
            transform: Transform::from_translation(position),
        }
    }
}

/// ボールバンドル（ボール生成時に使用）
/// @spec 30401_trajectory_spec.md
#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub velocity: Velocity,
    pub bounce_count: BounceCount,
    pub transform: Transform,
}

impl BallBundle {
    /// 通常ショット用ボールを生成
    /// @spec 30401_trajectory_spec.md#req-30401-002
    pub fn new(position: Vec3, velocity: Vec3) -> Self {
        Self {
            ball: Ball,
            velocity: Velocity { value: velocity },
            bounce_count: BounceCount::default(),
            transform: Transform::from_translation(position),
        }
    }
}
