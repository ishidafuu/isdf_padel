//! Presentation層: Bevy Sprite、Transform、UI
//! @spec 20001_layers.md#layer-5-presentation

mod debug_ui;
mod visual_feedback;

pub use debug_ui::DebugUiPlugin;
pub use visual_feedback::{
    ball_spin_color_system, player_hold_visual_system, save_player_original_color_system,
};

use bevy::prelude::*;

use crate::components::{Ball, HasShadow, LogicalPosition, Player, Shadow};
use crate::resource::config::GameConfig;

/// ワールド座標→ピクセル座標変換用スケール
/// テニスコート（12x16）が1280x720ウィンドウに収まるよう調整
/// 16 * 50 = 800px, 12 * 50 = 600px
pub const WORLD_SCALE: f32 = 50.0;

/// 論理座標を表示用Transformに同期するシステム
/// 論理座標系: X=横移動, Y=高さ（ジャンプ）, Z=奥行き（コート前後＝打ち合い方向）
/// 表示座標系: X=打ち合い方向（左右）, Y=横移動+高さ（上下）, Z=レイヤー深度
/// @spec REQ-30801-005
pub fn sync_transform_system(mut query: Query<(&LogicalPosition, &mut Transform)>) {
    for (logical_pos, mut transform) in query.iter_mut() {
        // 論理座標を表示座標に変換（90度回転：左右の打ち合い）
        // X: 論理Z（奥行き）→ 画面左右（打ち合い方向）
        // Y: 論理X（横移動）+ 論理Y（高さ）→ 画面上下
        // Z: レイヤー深度
        let display_x = logical_pos.value.z * WORLD_SCALE;
        let display_y = logical_pos.value.x * WORLD_SCALE + logical_pos.value.y * WORLD_SCALE;
        let display_z = 1.0 - logical_pos.value.x * 0.01; // 奥行きでレイヤー調整

        transform.translation = Vec3::new(display_x, display_y, display_z);
    }
}

/// 影の位置を更新するシステム
/// 影は所有者の足元（地面）に表示される
/// @spec REQ-30801-003
pub fn sync_shadow_system(
    config: Res<GameConfig>,
    owner_query: Query<&LogicalPosition>,
    player_query: Query<Entity, With<Player>>,
    mut shadow_query: Query<(&Shadow, &mut Transform)>,
) {
    for (shadow, mut transform) in shadow_query.iter_mut() {
        if let Ok(owner_pos) = owner_query.get(shadow.owner) {
            // 影は地面（Y=0）に表示（90度回転版）
            let display_x = owner_pos.value.z * WORLD_SCALE;
            // プレイヤーの影は足元にオフセット、ボールの影はオフセットなし
            let y_offset = if player_query.get(shadow.owner).is_ok() {
                config.shadow.player_y_offset
            } else {
                config.shadow.ball_y_offset
            };
            let display_y = owner_pos.value.x * WORLD_SCALE - y_offset;
            // 影は背面に表示
            let display_z = config.shadow.z_layer;

            transform.translation = Vec3::new(display_x, display_y, display_z);
        }
    }
}

/// ボール生成時に影をスポーンするシステム
/// @spec REQ-30801-002
pub fn spawn_ball_shadow_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    ball_query: Query<Entity, Added<Ball>>,
    shadow_query: Query<&Shadow>,
) {
    for ball_entity in ball_query.iter() {
        // すでに影があるかチェック
        let has_shadow = shadow_query.iter().any(|s| s.owner == ball_entity);
        if has_shadow {
            continue;
        }

        // 影をスポーン
        let (width, height) = config.shadow.ball_size;
        commands.spawn((
            Shadow { owner: ball_entity },
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, config.shadow.ball_alpha),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, config.shadow.z_layer),
        ));
    }
}

/// プレイヤーに影をスポーンするシステム
/// HasShadowを持たないプレイヤーに対して影を生成する
/// @spec REQ-30801-001
pub fn spawn_player_shadow_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    player_query: Query<Entity, (With<Player>, Without<HasShadow>)>,
) {
    for player_entity in player_query.iter() {
        // 影をスポーン（プレイヤーはボールより大きい）
        let (width, height) = config.shadow.player_size;
        commands.spawn((
            Shadow { owner: player_entity },
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, config.shadow.player_alpha),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, config.shadow.z_layer),
        ));
        // HasShadowマーカーを追加して重複スポーンを防ぐ
        commands.entity(player_entity).insert(HasShadow);
    }
}

/// ボール消滅時にボールの影を削除するシステム
/// プレイヤーの影は削除しない
/// @spec REQ-30801-004
pub fn despawn_ball_shadow_system(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    player_query: Query<Entity, With<Player>>,
    shadow_query: Query<(Entity, &Shadow)>,
) {
    for (shadow_entity, shadow) in shadow_query.iter() {
        // プレイヤーの影はスキップ
        if player_query.get(shadow.owner).is_ok() {
            continue;
        }
        // ボールの影で、所有者が存在しなければ削除
        if ball_query.get(shadow.owner).is_err() {
            commands.entity(shadow_entity).despawn();
        }
    }
}
