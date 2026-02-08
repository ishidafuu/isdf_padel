//! サーブパラメータ
//! @spec 30102_serve_spec.md#req-30102-060
//! @spec 30102_serve_spec.md#req-30102-080

use serde::Deserialize;

/// サーブパラメータ
/// @spec 30102_serve_spec.md#req-30102-060
/// @spec 30102_serve_spec.md#req-30102-080
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct ServeConfig {
    /// オーバーハンドサーブの打点高さオフセット（m）
    /// @spec 30102_serve_spec.md#req-30102-060
    pub ball_spawn_offset_y: f32,
    /// サーブ速度（m/s）
    /// @spec 30102_serve_spec.md#req-30102-060
    pub serve_speed: f32,
    /// サーブ角度（度）
    /// @spec 30102_serve_spec.md#req-30102-060
    pub serve_angle: f32,
    /// Left側のデフォルトサーブ方向（X軸：打ち合い方向）
    pub p1_default_direction_x: f32,
    /// Right側のデフォルトサーブ方向（X軸：打ち合い方向）
    pub p2_default_direction_x: f32,
    /// トスボール生成高さ（手元位置）
    /// @spec 30102_serve_spec.md#req-30102-080
    pub toss_start_offset_y: f32,
    /// トス上向き初速度（m/s）
    /// @spec 30102_serve_spec.md#req-30102-080
    pub toss_velocity_y: f32,
    /// 長押し最小時のトス上向き初速度（m/s）
    pub toss_velocity_min_y: f32,
    /// 長押し最大時のトス上向き初速度（m/s）
    pub toss_velocity_max_y: f32,
    /// 長押し最大として扱う時間（秒）
    pub toss_hold_max_secs: f32,
    /// トス高さによる着地点深さシフト量（m）
    pub toss_depth_shift: f32,
    /// トス高さによる発射角ボーナス上限（度）
    pub toss_launch_angle_bonus_deg: f32,
    /// サーブ練習モード（true のとき着地後に常に再サーブ）
    pub practice_infinite_mode: bool,
    /// トス失敗までの時間（秒）
    /// @spec 30102_serve_spec.md#req-30102-084
    pub toss_timeout: f32,
    /// ヒット可能最低高さ（m）
    /// @spec 30102_serve_spec.md#req-30102-083
    pub hit_height_min: f32,
    /// ヒット可能最高高さ（m）
    /// @spec 30102_serve_spec.md#req-30102-083
    pub hit_height_max: f32,
    /// AI用ヒット最適高さ（m）
    /// @spec 30102_serve_spec.md#req-30102-088
    pub hit_height_optimal: f32,
    /// AI用ヒット許容範囲（m）
    /// @spec 30102_serve_spec.md#req-30102-088
    pub ai_hit_tolerance: f32,
    /// Left側のベースライン位置
    /// @spec 30102_serve_spec.md#req-30102-086
    pub serve_baseline_x_p1: f32,
    /// Right側のベースライン位置
    /// @spec 30102_serve_spec.md#req-30102-086
    pub serve_baseline_x_p2: f32,
}

impl Default for ServeConfig {
    fn default() -> Self {
        Self {
            ball_spawn_offset_y: 2.0,
            serve_speed: 10.0,
            serve_angle: -15.0,
            p1_default_direction_x: 1.0,
            p2_default_direction_x: -1.0,
            toss_start_offset_y: 1.0,
            toss_velocity_y: 3.5,
            toss_velocity_min_y: 2.8,
            toss_velocity_max_y: 4.8,
            toss_hold_max_secs: 0.3,
            toss_depth_shift: 0.45,
            toss_launch_angle_bonus_deg: 6.0,
            practice_infinite_mode: false,
            toss_timeout: 3.0,
            hit_height_min: 1.8,
            hit_height_max: 2.7,
            hit_height_optimal: 2.2,
            ai_hit_tolerance: 0.1,
            serve_baseline_x_p1: -8.5,
            serve_baseline_x_p2: 8.5,
        }
    }
}

/// サーブサイド
/// @spec 30903_serve_authority_spec.md#req-30903-003
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Deserialize)]
pub enum ServeSide {
    /// デュースサイド（右側）- ポイント合計が偶数
    #[default]
    Deuce,
    /// アドバンテージサイド（左側）- ポイント合計が奇数
    Ad,
}

impl ServeSide {
    /// ポイント合計からサーブサイドを判定
    /// @spec 30903_serve_authority_spec.md#req-30903-003
    #[inline]
    pub fn from_point_total(total: usize) -> Self {
        if total.is_multiple_of(2) {
            ServeSide::Deuce
        } else {
            ServeSide::Ad
        }
    }
}
