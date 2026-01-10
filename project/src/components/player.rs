//! プレイヤー関連コンポーネント
//! @spec 30200_overview.md

use bevy::prelude::*;

use crate::core::CourtSide;
use crate::resource::config::PlayerVisualConfig;

use super::input::InputState;
use super::physics::{GroundedState, LogicalPosition, Velocity};
use super::shot::ShotState;

/// プレイヤーマーカーコンポーネント
/// @spec 30200_overview.md
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    /// プレイヤーID（1 or 2）
    pub id: u8,
    /// プレイヤーがどちら側のコートにいるか
    pub court_side: CourtSide,
}

/// 人間操作プレイヤーマーカーコンポーネント
/// @spec 20006_input_system.md
#[derive(Component, Debug, Clone, Copy)]
pub struct HumanControlled {
    /// 入力デバイスID（キーボード=0, ゲームパッド=1,2,...）
    pub device_id: usize,
}

impl Default for HumanControlled {
    fn default() -> Self {
        Self { device_id: 0 }
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
/// 互換性維持のため残存。新規はcharacter::spawn_articulated_playerを使用
/// @spec 30200_overview.md
/// @spec 30202_jump_spec.md
/// @spec 30601_shot_input_spec.md
/// @spec 20006_input_system.md
#[allow(dead_code)]
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub logical_position: LogicalPosition,
    pub velocity: Velocity,
    pub knockback: KnockbackState,
    pub grounded: GroundedState,
    pub shot_state: ShotState,
    pub input_state: InputState,
    pub sprite: Sprite,
    pub transform: Transform,
}

#[allow(dead_code)]
impl PlayerBundle {
    pub fn new(id: u8, position: Vec3, visual_config: &PlayerVisualConfig) -> Self {
        let court_side = if id == 1 {
            CourtSide::Left
        } else {
            CourtSide::Right
        };
        // @data 80101_game_constants.md#player-visual-config
        let (r, g, b) = if id == 1 {
            visual_config.player1_color
        } else {
            visual_config.player2_color
        };
        let color = Color::srgb(r, g, b);
        let (width, height) = visual_config.size;
        Self {
            player: Player { id, court_side },
            logical_position: LogicalPosition { value: position },
            velocity: Velocity::default(),
            knockback: KnockbackState::default(),
            grounded: GroundedState::default(),
            shot_state: ShotState::default(),
            input_state: InputState::default(),
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::default(),
        }
    }
}
