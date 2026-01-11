//! ゲーム乱数リソース
//! @spec R30000-027

use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

/// シード可能なゲーム乱数リソース
/// リプレイ再現とシミュレーション再現性のために使用
#[derive(Resource)]
pub struct GameRng {
    rng: StdRng,
    seed: u64,
}

impl GameRng {
    /// 指定シードから乱数生成器を作成
    pub fn from_seed(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed,
        }
    }

    /// システム時刻からシードを生成して乱数生成器を作成
    pub fn from_entropy() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        Self::from_seed(seed)
    }

    /// 現在のシード値を取得
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// 指定範囲内のランダムな値を生成
    pub fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: rand::distr::uniform::SampleUniform,
        R: rand::distr::uniform::SampleRange<T>,
    {
        #[allow(deprecated)]
        self.rng.gen_range(range)
    }
}

impl Default for GameRng {
    fn default() -> Self {
        Self::from_entropy()
    }
}
