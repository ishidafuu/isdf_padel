//! コートサイズ・範囲
//! @data 80101_game_constants.md#court-config

use serde::Deserialize;

/// コートサイズ・範囲
/// @data 80101_game_constants.md#court-config
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct CourtConfig {
    pub width: f32,
    pub depth: f32,
    pub ceiling_height: f32,
    /// TODO: v0.2でジャンプ高さ制限として使用予定
    #[allow(dead_code)]
    pub max_jump_height: f32,
    pub net_height: f32,
    pub net_x: f32,
    /// サービスボックスの奥行き（ネットからの距離）
    /// @spec 30902_fault_spec.md#req-30902-001
    pub service_box_depth: f32,
    /// 外壁位置（コート幅方向、Z軸）
    /// @spec 30503_boundary_behavior.md#beh-30503-001
    pub outer_wall_z: f32,
    /// 外壁位置（打ち合い方向、X軸）
    /// @spec 30503_boundary_behavior.md#beh-30503-002
    pub outer_wall_x: f32,
}

impl Default for CourtConfig {
    fn default() -> Self {
        Self {
            width: 10.0,
            depth: 6.0,
            ceiling_height: 5.0,
            max_jump_height: 5.0,
            net_height: 1.0,
            net_x: 0.0,
            service_box_depth: 1.5,
            outer_wall_z: 8.0,
            outer_wall_x: 10.0,
        }
    }
}
