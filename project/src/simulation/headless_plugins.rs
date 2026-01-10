//! Headless Plugins
//! @spec 77100_headless_sim.md
//!
//! ヘッドレス実行用のプラグインセット。
//! 描画系を除外し、ゲームロジックのみを含む。

use bevy::prelude::*;

use crate::character::CharacterPlugin;
use crate::components::AiController;
use crate::core::{
    BallHitEvent, PlayerJumpEvent, PlayerKnockbackEvent, PlayerLandEvent, PlayerMoveEvent,
    ShotEvent, ShotExecutedEvent,
};
use crate::resource::config::{GameConfig, GameConfigLoader};
use crate::resource::debug::LastShotDebugInfo;
use crate::resource::MatchFlowState;
use crate::systems::{
    ai_movement_system, ai_shot_system, ceiling_collision_system, gravity_system,
    jump_system, knockback_movement_system, knockback_start_system, knockback_timer_system,
    landing_system, movement_system, shot_cooldown_system, shot_direction_system,
    shot_input_system, vertical_movement_system, AiServePlugin, BallCollisionPlugin,
    BallTrajectoryPlugin, BoundaryPlugin, FaultJudgmentPlugin, GameSystemSet, MatchFlowPlugin,
    PointJudgmentPlugin, ScoringPlugin,
};

use super::AnomalyDetectorPlugin;

/// ヘッドレス実行用プラグインセット
pub struct HeadlessPlugins;

impl Plugin for HeadlessPlugins {
    fn build(&self, app: &mut App) {
        // GameConfig の Asset 登録
        app.init_asset::<GameConfig>()
            .init_asset_loader::<GameConfigLoader>();

        // ゲームロジックプラグイン
        app.add_plugins(BoundaryPlugin)
            .add_plugins(BallTrajectoryPlugin)
            .add_plugins(BallCollisionPlugin)
            .add_plugins(ScoringPlugin)
            .add_plugins(PointJudgmentPlugin)
            .add_plugins(FaultJudgmentPlugin)
            .add_plugins(MatchFlowPlugin)
            .add_plugins(AiServePlugin)
            .add_plugins(CharacterPlugin)
            .add_plugins(AnomalyDetectorPlugin);

        // リソース初期化
        app.init_resource::<LastShotDebugInfo>();

        // イベント登録
        app.add_message::<PlayerMoveEvent>()
            .add_message::<PlayerJumpEvent>()
            .add_message::<PlayerLandEvent>()
            .add_message::<BallHitEvent>()
            .add_message::<PlayerKnockbackEvent>()
            .add_message::<ShotEvent>()
            .add_message::<ShotExecutedEvent>();

        // SystemSet の順序を設定
        app.configure_sets(Update, GameSystemSet::Input.before(GameSystemSet::GameLogic));

        // ゲームロジックシステム（入力は AI が担当するため human_input_system は不要）
        app.add_systems(
            Update,
            (
                // ふっとばし開始
                knockback_start_system,
                // ジャンプ・重力
                (jump_system, gravity_system, vertical_movement_system).chain(),
                // 水平移動
                movement_system,
                // AI移動
                ai_movement_system,
                // ショット入力（AI担当）
                shot_input_system.run_if(in_state(MatchFlowState::Rally)),
                // AIショット
                ai_shot_system.run_if(in_state(MatchFlowState::Rally)),
                // 方向計算・クールダウン
                (shot_direction_system, shot_cooldown_system),
                // ふっとばし移動・タイマー
                (knockback_movement_system, knockback_timer_system),
                // 境界チェック
                (ceiling_collision_system, landing_system),
            )
                .chain()
                .in_set(GameSystemSet::GameLogic),
        );
    }
}

/// ヘッドレスセットアップ（プレイヤー・コートのスポーンなし版）
/// シミュレーション実行時に SimulationRunner から呼び出す
/// TODO: Bevy App 実装時に使用
#[allow(dead_code)]
pub fn headless_setup(commands: &mut Commands, config: &GameConfig) {
    // Player 1 (AI)
    let player1_pos = Vec3::new(config.player.x_min + 1.0, 0.0, 0.0);
    let (r, g, b) = config.player_visual.player1_color;
    let player1_color = Color::srgb(r, g, b);
    let player1_entity =
        crate::character::spawn_articulated_player(commands, 1, player1_pos, player1_color);
    // Player 1 も AI として動作させる
    commands.entity(player1_entity).insert(AiController {
        home_position: player1_pos,
        target_position: player1_pos,
        ..Default::default()
    });

    // Player 2 (AI)
    let player2_pos = Vec3::new(config.player.x_max - 1.0, 0.0, 0.0);
    let (r, g, b) = config.player_visual.player2_color;
    let player2_color = Color::srgb(r, g, b);
    let player2_entity =
        crate::character::spawn_articulated_player(commands, 2, player2_pos, player2_color);
    commands.entity(player2_entity).insert(AiController {
        home_position: player2_pos,
        target_position: player2_pos,
        ..Default::default()
    });
}
