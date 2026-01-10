//! 入力関連システム
//! @spec 20001_layers.md#layer-4-systems

mod gamepad;
mod human;
mod shot;

pub use gamepad::gamepad_input_system;
pub use human::human_input_system;
pub use shot::*;
