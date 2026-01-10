//! Padel Game Library
//! @spec 77100_headless_sim.md
//!
//! ゲームロジックを共有ライブラリとして公開し、
//! 別バイナリ（headless_sim等）から利用可能にする。

pub mod character;
pub mod components;
pub mod core;
pub mod presentation;
pub mod replay;
pub mod resource;
pub mod simulation;
pub mod systems;

// リプレイ機能を再公開
pub use replay::{ReplayManager, ReplayRecordPlugin};

// 主要な型を再公開
pub use character::CharacterPlugin;
pub use components::{AiController, HumanControlled};
pub use core::{
    BallHitEvent, PlayerJumpEvent, PlayerKnockbackEvent, PlayerLandEvent, PlayerMoveEvent,
    ShotEvent, ShotExecutedEvent,
};
pub use resource::config::{load_game_config, GameConfig, GameConfigHandle, GameConfigLoader};
pub use resource::MatchFlowState;
pub use systems::{
    AiServePlugin, BallCollisionPlugin, BallTrajectoryPlugin, BoundaryPlugin, FaultJudgmentPlugin,
    GameSystemSet, MatchFlowPlugin, PointJudgmentPlugin, ScoringPlugin,
};
