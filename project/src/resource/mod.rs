//! Resource層: ゲーム設定、マスタデータ
//! @spec 20001_layers.md#layer-2-resource

pub mod config;
pub mod debug;
pub mod debug_control;
pub mod fixed_delta;
pub mod game_rng;
pub mod scoring;

pub use config::*;
pub use debug_control::*;
pub use fixed_delta::*;
pub use game_rng::*;
pub use scoring::*;
