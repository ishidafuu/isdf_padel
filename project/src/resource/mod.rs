//! Resource層: ゲーム設定、マスタデータ
//! @spec 20001_layers.md#layer-2-resource

pub mod config;
pub mod debug;
pub mod fixed_delta;
pub mod scoring;

pub use config::*;
pub use fixed_delta::*;
pub use scoring::*;
