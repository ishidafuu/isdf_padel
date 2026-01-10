//! Systems層: ゲームロジックの実装
//! @spec 20001_layers.md#layer-4-systems

use bevy::prelude::*;

/// システム実行順序を制御するための SystemSet
/// 入力読み取り → ゲームロジック の順序を保証
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSystemSet {
    /// 入力読み取り（human_input, gamepad_input）
    Input,
    /// ゲームロジック（サーブ、ショット、移動など）
    GameLogic,
}

mod ai_movement;
mod ai_serve;
mod ai_shot;
mod ball_collision;
mod ball_trajectory;
mod boundary;
mod court_factory;
mod debug_marker;
mod fault_judgment;
mod gamepad_input;
mod human_input;
mod jump;
mod knockback;
mod match_flow;
mod movement;
mod point_judgment;
mod scoring;
mod serve;
mod shot_attributes;
mod shot_direction;
mod shot_input;
mod trajectory_calculator;

pub use ai_movement::*;
pub use ai_serve::*;
pub use ai_shot::*;
pub use ball_collision::*;
pub use ball_trajectory::*;
pub use boundary::*;
pub use debug_marker::*;
pub use fault_judgment::*;
pub use gamepad_input::gamepad_input_system;
pub use human_input::human_input_system;
pub use jump::*;
pub use knockback::*;
pub use match_flow::*;
pub use movement::*;
pub use point_judgment::*;
pub use scoring::*;
pub use serve::*;
pub use shot_direction::*;
pub use shot_input::*;
