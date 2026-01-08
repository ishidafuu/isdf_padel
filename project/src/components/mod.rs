//! Components層: Entityのデータ構造定義
//! @spec 20001_layers.md#layer-3-components
//! @spec 30401_trajectory_spec.md

use bevy::prelude::*;

use crate::core::CourtSide;

/// 論理座標コンポーネント（ゲームロジック用）
/// X: 左右, Y: 高さ（ジャンプ）, Z: 奥行き（コート前後）
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct LogicalPosition {
    pub value: Vec3,
}

impl LogicalPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }
}

/// 影マーカーコンポーネント
/// 親エンティティの影を表示する
#[derive(Component, Debug, Clone, Copy)]
pub struct Shadow {
    /// 影の所有者エンティティ
    pub owner: Entity,
}

/// 影がスポーンされたことを示すマーカー
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct HasShadow;

/// プレイヤーマーカーコンポーネント
/// @spec 30200_player_overview.md
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    /// プレイヤーID（1 or 2）
    pub id: u8,
    /// プレイヤーがどちら側のコートにいるか
    pub court_side: CourtSide,
}

/// AIコントローラーマーカーコンポーネント
/// @spec 30301_ai_movement_spec.md
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct AiController {
    /// ホームポジション（待機位置）
    /// @spec 30301_ai_movement_spec.md#req-30301-005
    pub home_position: Vec3,
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

/// 最後にショットを打ったプレイヤー追跡コンポーネント
/// @spec 30103_point_end_spec.md#req-30103-003
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct LastShooter {
    /// 最後にショットを打ったプレイヤー側
    pub side: Option<CourtSide>,
}

impl LastShooter {
    /// ショット元を記録
    pub fn record(&mut self, shooter: CourtSide) {
        self.side = Some(shooter);
    }

    /// リセット
    pub fn reset(&mut self) {
        self.side = None;
    }
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

/// ショット状態コンポーネント
/// @spec 30601_shot_input_spec.md
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ShotState {
    /// クールダウン残り時間（秒）
    /// @spec 30601_shot_input_spec.md#req-30601-004
    pub cooldown_timer: f32,
}

impl ShotState {
    /// クールダウン中かどうか
    /// @spec 30601_shot_input_spec.md#req-30601-004
    #[inline]
    pub fn is_on_cooldown(&self) -> bool {
        self.cooldown_timer > 0.0
    }

    /// クールダウンを開始
    /// @spec 30601_shot_input_spec.md#req-30601-004
    pub fn start_cooldown(&mut self, duration: f32) {
        self.cooldown_timer = duration;
    }

    /// クールダウンタイマーを更新
    /// @spec 30601_shot_input_spec.md#req-30601-004
    pub fn update_cooldown(&mut self, delta: f32) {
        self.cooldown_timer = (self.cooldown_timer - delta).max(0.0);
    }
}

/// 入力方式（プッシュ/ホールド）
/// @spec 30604_shot_attributes_spec.md#req-30604-050
/// @spec 30604_shot_attributes_spec.md#req-30604-051
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum InputMode {
    /// プッシュ: ボタン押下時にボールがヒット範囲内
    #[default]
    Push,
    /// ホールド: ボタン押し続けでボールがヒット範囲に入る
    Hold,
}

/// ショット実行時のコンテキスト（5要素の入力値）
/// @spec 30604_shot_attributes_spec.md
#[derive(Debug, Clone, Copy, Default)]
pub struct ShotContext {
    /// 入力方式（プッシュ/ホールド）
    /// @spec 30604_shot_attributes_spec.md#req-30604-050
    pub input_mode: InputMode,
    /// プッシュ精度（ボタン押下時刻 - ボールがヒット範囲に入った時刻）、ミリ秒
    /// @spec 30604_shot_attributes_spec.md#req-30604-053
    pub push_timing_diff: f32,
    /// ホールド継続時間、ミリ秒
    /// @spec 30604_shot_attributes_spec.md#req-30604-052
    pub hold_duration: f32,
    /// 打点の高さ（Y座標）、メートル
    /// @spec 30604_shot_attributes_spec.md#req-30604-054
    pub hit_height: f32,
    /// バウンド経過時間（秒）、None = ボレー（ノーバウンド）
    /// @spec 30604_shot_attributes_spec.md#req-30604-056
    pub bounce_elapsed: Option<f32>,
    /// 移動ベクトルとボール方向の内積（-1.0〜+1.0）
    /// @spec 30604_shot_attributes_spec.md#req-30604-059
    pub approach_dot: f32,
    /// ボールとの距離（XZ平面）、メートル
    /// @spec 30604_shot_attributes_spec.md#req-30604-061
    pub ball_distance: f32,
}

/// 計算されたショット属性（5つの出力属性）
/// @spec 30604_shot_attributes_spec.md
#[derive(Debug, Clone, Copy)]
pub struct ShotAttributes {
    /// 威力（ボール初速度）、m/s
    /// @spec 30604_shot_attributes_spec.md#req-30604-063
    pub power: f32,
    /// 安定性（ミス確率に影響）、0.0〜2.0
    /// @spec 30604_shot_attributes_spec.md#req-30604-064
    pub stability: f32,
    /// 発射角度、度
    /// @spec 30604_shot_attributes_spec.md#req-30604-065
    pub angle: f32,
    /// スピン量（正: トップスピン、負: スライス）、-1.0〜+1.0
    /// @spec 30604_shot_attributes_spec.md#req-30604-066
    pub spin: f32,
    /// 精度（コースブレに影響）、0.0〜2.0
    /// @spec 30604_shot_attributes_spec.md#req-30604-067
    pub accuracy: f32,
}

impl Default for ShotAttributes {
    fn default() -> Self {
        Self {
            power: 10.0,
            stability: 1.0,
            angle: 15.0,
            spin: 0.0,
            accuracy: 1.0,
        }
    }
}

/// ボールのバウンス状態追跡コンポーネント
/// @spec 30604_shot_attributes_spec.md#req-30604-056
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BounceState {
    /// 最後にバウンドしてからの経過時間（秒）
    /// None = まだバウンドしていない（ボレー対象）
    pub time_since_bounce: Option<f32>,
}

/// ボールのスピン状態コンポーネント
/// @spec 30802_visual_feedback_spec.md#req-30802-004
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BallSpin {
    /// スピン値（-1.0〜+1.0）
    /// 正: トップスピン、負: スライス、0: ニュートラル
    pub value: f32,
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
/// @spec 30601_shot_input_spec.md
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub logical_position: LogicalPosition,
    pub velocity: Velocity,
    pub knockback: KnockbackState,
    pub grounded: GroundedState,
    pub shot_state: ShotState,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl PlayerBundle {
    pub fn new(id: u8, position: Vec3) -> Self {
        let court_side = if id == 1 {
            CourtSide::Player1
        } else {
            CourtSide::Player2
        };
        // Player1: 青、Player2: 赤
        let color = if id == 1 {
            Color::srgb(0.2, 0.4, 0.8)
        } else {
            Color::srgb(0.8, 0.2, 0.2)
        };
        Self {
            player: Player { id, court_side },
            logical_position: LogicalPosition { value: position },
            velocity: Velocity::default(),
            knockback: KnockbackState::default(),
            grounded: GroundedState::default(),
            shot_state: ShotState::default(),
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(40.0, 60.0)),
                ..default()
            },
            transform: Transform::default(),
        }
    }
}

/// ボールバンドル（ボール生成時に使用）
/// @spec 30401_trajectory_spec.md
/// @spec 30604_shot_attributes_spec.md
/// @spec 30802_visual_feedback_spec.md
#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub logical_position: LogicalPosition,
    pub velocity: Velocity,
    pub bounce_count: BounceCount,
    pub bounce_state: BounceState,
    pub last_shooter: LastShooter,
    pub ball_spin: BallSpin,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl BallBundle {
    /// 通常ショット用ボールを生成
    /// @spec 30401_trajectory_spec.md#req-30401-002
    pub fn new(position: Vec3, velocity: Vec3) -> Self {
        Self {
            ball: Ball,
            logical_position: LogicalPosition { value: position },
            velocity: Velocity { value: velocity },
            bounce_count: BounceCount::default(),
            bounce_state: BounceState::default(),
            last_shooter: LastShooter::default(),
            ball_spin: BallSpin::default(),
            sprite: Sprite {
                color: Color::srgb(0.9, 0.9, 0.2), // 黄色
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::default(),
        }
    }

    /// ショット元を指定してボールを生成
    /// @spec 30103_point_end_spec.md#req-30103-003
    pub fn with_shooter(position: Vec3, velocity: Vec3, shooter: CourtSide) -> Self {
        Self {
            ball: Ball,
            logical_position: LogicalPosition { value: position },
            velocity: Velocity { value: velocity },
            bounce_count: BounceCount::default(),
            bounce_state: BounceState::default(),
            last_shooter: LastShooter { side: Some(shooter) },
            ball_spin: BallSpin::default(),
            sprite: Sprite {
                color: Color::srgb(0.9, 0.9, 0.2), // 黄色
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::default(),
        }
    }
}
