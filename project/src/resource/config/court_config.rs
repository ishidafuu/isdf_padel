//! コートサイズ・範囲
//! @data 80101_game_constants.md#court-config

use serde::Deserialize;

/// コートサイズ・範囲
/// @data 80101_game_constants.md#court-config
#[derive(Deserialize, Clone, Debug)]
pub struct CourtConfig {
    #[serde(default = "default_court_width")]
    pub width: f32,
    #[serde(default = "default_court_depth")]
    pub depth: f32,
    #[serde(default = "default_ceiling_height")]
    pub ceiling_height: f32,
    /// TODO: v0.2でジャンプ高さ制限として使用予定
    #[allow(dead_code)]
    #[serde(default = "default_max_jump_height")]
    pub max_jump_height: f32,
    #[serde(default = "default_net_height")]
    pub net_height: f32,
    #[serde(default = "default_net_x")]
    pub net_x: f32,
    /// サービスボックスの奥行き（ネットからの距離）
    /// @spec 30902_fault_spec.md#req-30902-001
    #[serde(default = "default_service_box_depth")]
    pub service_box_depth: f32,
    /// 外壁位置（コート幅方向、Z軸）
    /// @spec 30503_boundary_behavior.md#beh-30503-001
    #[serde(default = "default_outer_wall_z")]
    pub outer_wall_z: f32,
    /// 外壁位置（打ち合い方向、X軸）
    /// @spec 30503_boundary_behavior.md#beh-30503-002
    #[serde(default = "default_outer_wall_x")]
    pub outer_wall_x: f32,
}

fn default_court_width() -> f32 {
    10.0
}
fn default_court_depth() -> f32 {
    6.0
}
fn default_ceiling_height() -> f32 {
    5.0
}
fn default_max_jump_height() -> f32 {
    5.0
}
fn default_net_height() -> f32 {
    1.0
}
fn default_net_x() -> f32 {
    0.0
}
fn default_service_box_depth() -> f32 {
    1.5
}
fn default_outer_wall_z() -> f32 {
    8.0 // コートライン（width/2=5.0）より外側
}
fn default_outer_wall_x() -> f32 {
    10.0 // コートライン（depth/2=3.0）より外側
}
