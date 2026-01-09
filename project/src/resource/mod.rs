//! Resource層: ゲーム設定、マスタデータ
//! @spec 20001_layers.md#layer-2-resource

pub mod config;
pub mod debug;
pub mod scoring;

pub use config::*;
pub use debug::*;
pub use scoring::*;
