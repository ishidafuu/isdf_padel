//! ゲーム設定
//! @data 80101_game_constants.md

mod ai_config;
mod ball_config;
mod character_config;
mod collision_config;
mod court_config;
mod input_config;
mod physics_config;
mod player_config;
mod scoring_config;
mod serve_config;
mod shot_config;
mod visual_config;

// Re-exports
pub use ai_config::AiConfig;
pub use ball_config::BallConfig;
pub use character_config::CharacterConfig;
pub use collision_config::{CollisionConfig, KnockbackConfig};
pub use court_config::CourtConfig;
pub use input_config::{GamepadButtonsConfig, InputConfig, InputKeysConfig};
pub use physics_config::{PhysicsConfig, SpinPhysicsConfig};
pub use player_config::{PlayerConfig, PlayerVisualConfig};
pub use scoring_config::ScoringConfig;
pub use serve_config::{ServeConfig, ServeSide};
pub use shot_config::{
    ApproachCurvePoint, DistanceCurvePoint, HeightCurvePoint, ShotAttributesConfig, ShotConfig,
    SpinCurvePoint, TimingCurvePoint, TrajectoryConfig,
};
pub use visual_config::{ShadowConfig, VisualFeedbackConfig};

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use serde::Deserialize;

/// ゲーム全体の設定
/// @data 80101_game_constants.md#gameconfig-構造
#[derive(Asset, TypePath, Resource, Deserialize, Clone, Debug)]
pub struct GameConfig {
    pub physics: PhysicsConfig,
    pub court: CourtConfig,
    pub player: PlayerConfig,
    pub ball: BallConfig,
    pub collision: CollisionConfig,
    pub knockback: KnockbackConfig,
    pub shot: ShotConfig,
    pub scoring: ScoringConfig,
    /// TODO: v0.2で入力バッファリング機能として使用予定
    #[allow(dead_code)]
    pub input: InputConfig,
    /// 入力キーバインド設定
    #[serde(default)]
    pub input_keys: InputKeysConfig,
    /// ゲームパッドボタン設定
    /// @spec 20006_input_system.md#req-20006-053
    #[serde(default)]
    pub gamepad_buttons: GamepadButtonsConfig,
    #[serde(default)]
    pub shadow: ShadowConfig,
    #[serde(default)]
    pub shot_attributes: ShotAttributesConfig,
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub visual_feedback: VisualFeedbackConfig,
    /// プレイヤービジュアル設定（色、サイズ）
    #[serde(default)]
    pub player_visual: PlayerVisualConfig,
    /// サーブ設定
    #[serde(default)]
    pub serve: ServeConfig,
    /// スピン物理パラメータ
    /// @data 80101_game_constants.md#spin-physics-config
    #[serde(default)]
    pub spin_physics: SpinPhysicsConfig,
    /// パーツ分離キャラクター設定
    #[serde(default)]
    pub character: CharacterConfig,
    /// 弾道計算パラメータ
    /// @spec 30605_trajectory_calculation_spec.md
    #[serde(default)]
    pub trajectory: TrajectoryConfig,
}

/// RONファイルからGameConfigをロード
pub fn load_game_config(path: &str) -> Result<GameConfig, String> {
    let config_str =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;
    ron::from_str(&config_str).map_err(|e| format!("Failed to parse config: {}", e))
}

// ============================================================================
// ホットリロード対応
// @spec 30026: GameConfig ホットリロード対応
// ============================================================================

/// GameConfig のハンドルを保持するリソース
#[derive(Resource)]
pub struct GameConfigHandle(pub Handle<GameConfig>);

/// GameConfig の RON ファイルをロードするカスタム AssetLoader
#[derive(Default)]
pub struct GameConfigLoader;

/// GameConfigLoader のエラー型
#[derive(Debug, thiserror::Error)]
pub enum GameConfigLoaderError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

impl AssetLoader for GameConfigLoader {
    type Asset = GameConfig;
    type Settings = ();
    type Error = GameConfigLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let config: GameConfig = ron::de::from_bytes(&bytes)?;
        Ok(config)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
