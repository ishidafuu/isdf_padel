//! Padel Game - MVP v0.1
//! @spec 20000_overview.md

mod components;
mod core;
mod presentation;
mod resource;
mod systems;

use bevy::prelude::*;
use components::PlayerBundle;
use core::{BallHitEvent, PlayerJumpEvent, PlayerKnockbackEvent, PlayerLandEvent, PlayerMoveEvent};
use resource::config::{load_game_config, GameConfig};
use systems::{
    ceiling_collision_system, gravity_system, jump_system, knockback_movement_system,
    knockback_start_system, knockback_timer_system, landing_system, movement_system,
    read_input_system, read_jump_input_system, vertical_movement_system, BallTrajectoryPlugin,
    BoundaryPlugin, JumpInput, MovementInput,
};

fn main() {
    // GameConfig をロード
    let config = load_game_config("assets/config/game_config.ron")
        .expect("Failed to load game config");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Padel Game - MVP v0.1".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BoundaryPlugin)
        .add_plugins(BallTrajectoryPlugin)
        .insert_resource(config)
        .init_resource::<MovementInput>()
        .init_resource::<JumpInput>()
        .add_message::<PlayerMoveEvent>()
        .add_message::<PlayerJumpEvent>()
        .add_message::<PlayerLandEvent>()
        .add_message::<BallHitEvent>()
        .add_message::<PlayerKnockbackEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                // 入力読み取り
                (read_input_system, read_jump_input_system),
                // ふっとばし開始（BallHitEvent を処理）
                knockback_start_system,
                // ジャンプ・重力
                (jump_system, gravity_system, vertical_movement_system).chain(),
                // 水平移動（ふっとばし中はスキップ）
                movement_system,
                // ふっとばし移動・タイマー
                (knockback_movement_system, knockback_timer_system),
                // 境界チェック
                (ceiling_collision_system, landing_system),
            )
                .chain(),
        )
        .run();
}

/// 初期セットアップ
/// @spec 30200_player_overview.md
fn setup(mut commands: Commands, config: Res<GameConfig>) {
    // Camera2d をスポーン
    commands.spawn(Camera2d);

    // GameConfig のロード確認
    info!("GameConfig loaded successfully!");
    info!("Court size: {}x{}", config.court.width, config.court.depth);
    info!("Player move speed: X={}, Z={}", config.player.move_speed, config.player.move_speed_z);

    // Player 1 をスポーン（1Pコート側）
    let player1_pos = Vec3::new(0.0, 0.0, config.player.z_min + 1.0);
    commands.spawn(PlayerBundle::new(1, player1_pos));
    info!("Player 1 spawned at {:?}", player1_pos);

    // Player 2 をスポーン（2Pコート側）
    let player2_pos = Vec3::new(0.0, 0.0, config.player.z_max - 1.0);
    commands.spawn(PlayerBundle::new(2, player2_pos));
    info!("Player 2 spawned at {:?}", player2_pos);
}
