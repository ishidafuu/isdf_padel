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

// サブディレクトリモジュール
mod ai;
mod ball;
mod input;
mod match_control;
mod player;
mod point_judgment;
mod shot;
mod trajectory_calculator;

// ルートレベルモジュール
mod boundary;
mod court_factory;
mod debug_marker;

// re-export
pub use ai::*;
pub use ball::*;
pub use boundary::*;
pub use debug_marker::*;
pub use input::*;
pub use match_control::*;
pub use player::*;
pub use point_judgment::*;
pub use shot::*;
