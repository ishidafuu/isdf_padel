//! サーブパラメータ
//! @spec 30102_serve_spec.md#req-30102-060
//! @spec 30102_serve_spec.md#req-30102-080

use serde::Deserialize;

/// サーブパラメータ
/// @spec 30102_serve_spec.md#req-30102-060
/// @spec 30102_serve_spec.md#req-30102-080
#[derive(Deserialize, Clone, Debug)]
pub struct ServeConfig {
    /// オーバーハンドサーブの打点高さオフセット（m）
    /// @spec 30102_serve_spec.md#req-30102-060
    #[serde(default = "default_ball_spawn_offset_y")]
    pub ball_spawn_offset_y: f32,
    /// サーブ速度（m/s）
    /// @spec 30102_serve_spec.md#req-30102-060
    #[serde(default = "default_serve_speed")]
    pub serve_speed: f32,
    /// サーブ角度（度）
    /// @spec 30102_serve_spec.md#req-30102-060
    #[serde(default = "default_serve_angle")]
    pub serve_angle: f32,
    /// Left側のデフォルトサーブ方向（X軸：打ち合い方向）
    #[serde(default = "default_p1_default_direction_x")]
    pub p1_default_direction_x: f32,
    /// Right側のデフォルトサーブ方向（X軸：打ち合い方向）
    #[serde(default = "default_p2_default_direction_x")]
    pub p2_default_direction_x: f32,
    /// トスボール生成高さ（手元位置）
    /// @spec 30102_serve_spec.md#req-30102-080
    #[serde(default = "default_toss_start_offset_y")]
    pub toss_start_offset_y: f32,
    /// トス上向き初速度（m/s）
    /// @spec 30102_serve_spec.md#req-30102-080
    #[serde(default = "default_toss_velocity_y")]
    pub toss_velocity_y: f32,
    /// トス失敗までの時間（秒）
    /// @spec 30102_serve_spec.md#req-30102-084
    #[serde(default = "default_toss_timeout")]
    pub toss_timeout: f32,
    /// ヒット可能最低高さ（m）
    /// @spec 30102_serve_spec.md#req-30102-083
    #[serde(default = "default_hit_height_min")]
    pub hit_height_min: f32,
    /// ヒット可能最高高さ（m）
    /// @spec 30102_serve_spec.md#req-30102-083
    #[serde(default = "default_hit_height_max")]
    pub hit_height_max: f32,
    /// AI用ヒット最適高さ（m）
    /// @spec 30102_serve_spec.md#req-30102-088
    #[serde(default = "default_hit_height_optimal")]
    pub hit_height_optimal: f32,
    /// AI用ヒット許容範囲（m）
    /// @spec 30102_serve_spec.md#req-30102-088
    #[serde(default = "default_ai_hit_tolerance")]
    pub ai_hit_tolerance: f32,
    /// Left側のベースライン位置
    /// @spec 30102_serve_spec.md#req-30102-086
    #[serde(default = "default_serve_baseline_x_p1")]
    pub serve_baseline_x_p1: f32,
    /// Right側のベースライン位置
    /// @spec 30102_serve_spec.md#req-30102-086
    #[serde(default = "default_serve_baseline_x_p2")]
    pub serve_baseline_x_p2: f32,
}

impl Default for ServeConfig {
    fn default() -> Self {
        Self {
            ball_spawn_offset_y: default_ball_spawn_offset_y(),
            serve_speed: default_serve_speed(),
            serve_angle: default_serve_angle(),
            p1_default_direction_x: default_p1_default_direction_x(),
            p2_default_direction_x: default_p2_default_direction_x(),
            toss_start_offset_y: default_toss_start_offset_y(),
            toss_velocity_y: default_toss_velocity_y(),
            toss_timeout: default_toss_timeout(),
            hit_height_min: default_hit_height_min(),
            hit_height_max: default_hit_height_max(),
            hit_height_optimal: default_hit_height_optimal(),
            ai_hit_tolerance: default_ai_hit_tolerance(),
            serve_baseline_x_p1: default_serve_baseline_x_p1(),
            serve_baseline_x_p2: default_serve_baseline_x_p2(),
        }
    }
}

fn default_ball_spawn_offset_y() -> f32 {
    2.0 // オーバーハンドサーブの打点高さ
}
fn default_serve_speed() -> f32 {
    10.0 // m/s
}
fn default_serve_angle() -> f32 {
    -15.0 // 度（負の値=下向き発射）
}
fn default_p1_default_direction_x() -> f32 {
    1.0 // +X方向（2Pコートへ）
}
fn default_p2_default_direction_x() -> f32 {
    -1.0 // -X方向（1Pコートへ）
}
fn default_toss_start_offset_y() -> f32 {
    1.0 // 手元高さ
}
fn default_toss_velocity_y() -> f32 {
    3.5 // m/s（上向き）
}
fn default_toss_timeout() -> f32 {
    3.0 // 秒
}
fn default_hit_height_min() -> f32 {
    1.8 // m
}
fn default_hit_height_max() -> f32 {
    2.7 // m
}
fn default_hit_height_optimal() -> f32 {
    2.2 // m（AI用）
}
fn default_ai_hit_tolerance() -> f32 {
    0.1 // m（± 許容範囲）
}
fn default_serve_baseline_x_p1() -> f32 {
    -7.0 // Left側のベースライン
}
fn default_serve_baseline_x_p2() -> f32 {
    7.0 // Right側のベースライン
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
        if total % 2 == 0 {
            ServeSide::Deuce
        } else {
            ServeSide::Ad
        }
    }
}
