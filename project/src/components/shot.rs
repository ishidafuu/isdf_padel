//! ショット関連コンポーネント
//! @spec 30601_shot_input_spec.md
//! @spec 30604_shot_attributes_spec.md

use bevy::prelude::*;

/// ラケットスイング状態
/// @spec 30606_racket_contact_spec.md
#[derive(Debug, Clone, Copy)]
pub struct RacketSwingState {
    /// スイング進行中かどうか
    pub is_active: bool,
    /// 接触済みかどうか
    pub contact_done: bool,
    /// 経過時間（秒）
    pub elapsed_seconds: f32,
    /// スイング全体時間（秒）
    pub duration_seconds: f32,
    /// 接触目標時刻（秒）
    pub contact_time_seconds: f32,
    /// 入力方向（ショット方向制御）
    pub input_direction: Vec2,
    /// ホールド時間（ミリ秒）
    pub hold_time_ms: f32,
    /// 予測接触点（論理座標）
    pub planned_hit_position: Vec3,
    /// スイング軌道制御点（論理座標）
    pub start_position: Vec3,
    pub pre_contact_position: Vec3,
    pub post_contact_position: Vec3,
    pub end_position: Vec3,
    pub follow_through_control_position: Vec3,
    /// ラケット中心位置（論理座標）
    pub previous_racket_position: Vec3,
    pub current_racket_position: Vec3,
}

impl Default for RacketSwingState {
    fn default() -> Self {
        Self {
            is_active: false,
            contact_done: false,
            elapsed_seconds: 0.0,
            duration_seconds: 0.0,
            contact_time_seconds: 0.0,
            input_direction: Vec2::ZERO,
            hold_time_ms: 0.0,
            planned_hit_position: Vec3::ZERO,
            start_position: Vec3::ZERO,
            pre_contact_position: Vec3::ZERO,
            post_contact_position: Vec3::ZERO,
            end_position: Vec3::ZERO,
            follow_through_control_position: Vec3::ZERO,
            previous_racket_position: Vec3::ZERO,
            current_racket_position: Vec3::ZERO,
        }
    }
}

impl RacketSwingState {
    /// スイング状態をリセット
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// ショット状態コンポーネント
/// @spec 30601_shot_input_spec.md
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ShotState {
    /// クールダウン残り時間（秒）
    /// @spec 30601_shot_input_spec.md#req-30601-004
    pub cooldown_timer: f32,
    /// ラケット接触駆動スイング状態
    /// @spec 30606_racket_contact_spec.md
    pub racket_swing: RacketSwingState,
}

impl ShotState {
    /// クールダウン中かどうか
    /// @spec 30601_shot_input_spec.md#req-30601-004
    #[inline]
    pub fn is_on_cooldown(&self) -> bool {
        self.cooldown_timer > 0.0
    }

    /// スイング進行中かどうか
    #[inline]
    pub fn is_swing_active(&self) -> bool {
        self.racket_swing.is_active
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
    /// NOTE: v0.4以降は trajectory_calculator が角度を算出するため、
    ///       このフィールドは後方互換性のために残している
    #[allow(dead_code)]
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
