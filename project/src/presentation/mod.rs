//! Presentation層: Bevy Sprite、Transform、UI
//! @spec 20001_layers.md#layer-5-presentation

mod debug_ui;

pub use debug_ui::DebugUiPlugin;

use bevy::prelude::*;

use crate::components::{Ball, LogicalPosition, Shadow};

/// ワールド座標→ピクセル座標変換用スケール
/// 1ワールドユニット = 100ピクセル
pub const WORLD_SCALE: f32 = 100.0;

/// 論理座標を表示用Transformに同期するシステム
/// 論理座標系: X=横移動, Y=高さ（ジャンプ）, Z=奥行き（コート前後＝打ち合い方向）
/// 表示座標系: X=打ち合い方向（左右）, Y=横移動+高さ（上下）, Z=レイヤー深度
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
/// 影は所有者の真下（地面）に表示される
pub fn sync_shadow_system(
    owner_query: Query<&LogicalPosition>,
    mut shadow_query: Query<(&Shadow, &mut Transform)>,
) {
    for (shadow, mut transform) in shadow_query.iter_mut() {
        if let Ok(owner_pos) = owner_query.get(shadow.owner) {
            // 影は地面（Y=0）に表示（90度回転版）
            let display_x = owner_pos.value.z * WORLD_SCALE;
            let display_y = owner_pos.value.x * WORLD_SCALE; // Y=0なので高さ成分なし
            let display_z = -1.0; // 影は最背面

            transform.translation = Vec3::new(display_x, display_y, display_z);
        }
    }
}

/// ボール生成時に影をスポーンするシステム
pub fn spawn_ball_shadow_system(
    mut commands: Commands,
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
        commands.spawn((
            Shadow { owner: ball_entity },
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.5), // 半透明の黒
                custom_size: Some(Vec2::new(25.0, 10.0)), // 楕円形の影
                ..default()
            },
            Transform::default(),
        ));
    }
}

/// ボール消滅時に影を削除するシステム
pub fn despawn_ball_shadow_system(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    shadow_query: Query<(Entity, &Shadow)>,
) {
    for (shadow_entity, shadow) in shadow_query.iter() {
        // 所有者が存在しなければ影を削除
        if ball_query.get(shadow.owner).is_err() {
            commands.entity(shadow_entity).despawn();
        }
    }
}
