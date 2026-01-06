//! Padel Game - MVP v0.1
//! @spec 20000_overview.md

mod components;
mod core;
mod presentation;
mod resource;
mod systems;

use bevy::prelude::*;
use resource::config::{load_game_config, GameConfig};

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
        .insert_resource(config)
        .add_systems(Startup, setup)
        .run();
}

/// 初期セットアップ
fn setup(mut commands: Commands, config: Res<GameConfig>) {
    // Camera2d をスポーン
    commands.spawn(Camera2d);

    // GameConfig のロード確認
    info!("GameConfig loaded successfully!");
    info!("Court size: {}x{}", config.court.width, config.court.depth);
    info!("Player move speed: {}", config.player.move_speed);
}
