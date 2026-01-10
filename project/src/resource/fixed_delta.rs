//! 固定タイムステップ提供リソース
//! @spec 77100_headless_sim.md
//!
//! ヘッドレスシミュレーションの高速化を可能にするため、
//! 物理計算に固定のdelta_secsを提供する。

use bevy::prelude::*;

/// 固定タイムステップを提供するリソース
/// @spec 77100_headless_sim.md
#[derive(Resource)]
pub struct FixedDeltaTime {
    delta_secs: f32,
}

impl Default for FixedDeltaTime {
    fn default() -> Self {
        // 60FPS固定
        Self {
            delta_secs: 1.0 / 60.0,
        }
    }
}

impl FixedDeltaTime {
    /// 固定のdelta_secsを取得
    #[inline]
    pub fn delta_secs(&self) -> f32 {
        self.delta_secs
    }
}
