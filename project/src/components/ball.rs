//! ボール関連コンポーネント
//! @spec 30400_overview.md
//! @spec 30401_trajectory_spec.md

use bevy::prelude::*;

use crate::core::CourtSide;

use super::physics::{LogicalPosition, Velocity};

/// ボールマーカーコンポーネント
/// @spec 30400_overview.md
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Ball;

/// トスボールマーカーコンポーネント
/// @spec 30102_serve_spec.md#req-30102-080
/// サーブ前のトス中ボールを識別する
/// ヒット成功時に削除され、通常Ballが生成される
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TossBall;

/// ポイント終了済みマーカーコンポーネント
/// ボールに対して RallyEndEvent が発行済みであることを示す
/// 複数のポイント判定システムによる重複発行を防止
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PointEnded;

/// バウンス回数追跡コンポーネント（ツーバウンド判定用）
/// @spec 30503_boundary_behavior.md#beh-30503-006
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BounceCount {
    /// 現在のコート側でのバウンス回数
    pub count: u32,
    /// 最後にバウンドしたコート側
    pub last_court_side: Option<CourtSide>,
    /// RallyEndEvent 発行済みフラグ（重複発行防止）
    pub event_sent: bool,
}

impl BounceCount {
    /// バウンスを記録
    pub fn record_bounce(&mut self, court_side: CourtSide) {
        if self.last_court_side == Some(court_side) {
            self.count += 1;
        } else {
            // コート側が変わったらカウントとフラグをリセット
            self.last_court_side = Some(court_side);
            self.count = 1;
            self.event_sent = false;
        }
    }

    /// リセット（ショット後など）
    pub fn reset(&mut self) {
        self.count = 0;
        self.last_court_side = None;
        self.event_sent = false;
    }
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
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.side = None;
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

/// `Option<&BallSpin>` からスピン値を取得するための拡張トレイト
pub trait BallSpinExt {
    /// スピン値を取得（Noneまたはデフォルトの場合は0.0を返す）
    fn value_or_default(&self) -> f32;
}

impl BallSpinExt for Option<&BallSpin> {
    #[inline]
    fn value_or_default(&self) -> f32 {
        self.map_or(0.0, |s| s.value)
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
    #[allow(dead_code)]
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

/// トスボールバンドル（トスボール生成時に使用）
/// @spec 30102_serve_spec.md#req-30102-080
#[derive(Bundle)]
pub struct TossBallBundle {
    pub toss_ball: TossBall,
    pub logical_position: LogicalPosition,
    pub velocity: Velocity,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl TossBallBundle {
    /// トスボールを生成
    /// @spec 30102_serve_spec.md#req-30102-080
    pub fn new(position: Vec3, velocity: Vec3) -> Self {
        Self {
            toss_ball: TossBall,
            logical_position: LogicalPosition { value: position },
            velocity: Velocity { value: velocity },
            sprite: Sprite {
                color: Color::srgb(0.9, 0.9, 0.2), // 黄色
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::default(),
        }
    }
}
