//! パーツ分離キャラクターのBundle定義
//! @spec 31001_parts_spec.md

use bevy::prelude::*;

use super::animation::CharacterAnimationState;
use super::components::{
    ArticulatedCharacter, CharacterFacing, PartDefinition, PartKind, PartState,
};
use crate::components::{
    GroundedState, InputState, KnockbackState, LogicalPosition, Player, ShotState, Velocity,
};
use crate::core::CourtSide;

/// パーツ分離キャラクターのパーツ設定
/// @spec 31001_parts_spec.md
#[derive(Debug, Clone)]
pub struct PartConfig {
    pub kind: PartKind,
    pub base_offset: Vec3,
    pub size: Vec2,
    pub color: Color,
}

/// デフォルトのパーツ設定を取得
/// @spec 31001_parts_spec.md
pub fn default_part_configs(base_color: Color) -> Vec<PartConfig> {
    vec![
        // 頭部（ボディより手前に表示: z=2）
        PartConfig {
            kind: PartKind::Head,
            base_offset: Vec3::new(0.0, 24.0, 2.0),
            size: Vec2::new(16.0, 16.0),
            color: base_color,
        },
        // 胴体（少し暗めの色で区別: z=1）
        PartConfig {
            kind: PartKind::Torso,
            base_offset: Vec3::new(0.0, 12.0, 1.0),
            size: Vec2::new(12.0, 16.0),
            color: Color::srgb(0.55, 0.15, 0.55),
        },
        // 左手（向きによってZ優先度が動的に調整される: z=2）
        // @spec 31001_parts_spec.md#req-31001-007
        PartConfig {
            kind: PartKind::LeftHand,
            base_offset: Vec3::new(-12.0, 16.0, 2.0),
            size: Vec2::new(8.0, 8.0),
            color: base_color,
        },
        // 右手
        PartConfig {
            kind: PartKind::RightHand,
            base_offset: Vec3::new(12.0, 16.0, 2.0),
            size: Vec2::new(8.0, 8.0),
            color: base_color,
        },
        // 左膝
        PartConfig {
            kind: PartKind::LeftKnee,
            base_offset: Vec3::new(-4.0, 4.0, 2.0),
            size: Vec2::new(6.0, 6.0),
            color: base_color,
        },
        // 右膝
        PartConfig {
            kind: PartKind::RightKnee,
            base_offset: Vec3::new(4.0, 4.0, 2.0),
            size: Vec2::new(6.0, 6.0),
            color: base_color,
        },
        // 左足
        PartConfig {
            kind: PartKind::LeftFoot,
            base_offset: Vec3::new(-6.0, -4.0, 2.0),
            size: Vec2::new(8.0, 8.0),
            color: base_color,
        },
        // 右足
        PartConfig {
            kind: PartKind::RightFoot,
            base_offset: Vec3::new(6.0, -4.0, 2.0),
            size: Vec2::new(8.0, 8.0),
            color: base_color,
        },
        // ラケット（右手に追従: z=3で最前面）
        // @spec 31001_parts_spec.md#req-31001-008
        PartConfig {
            kind: PartKind::Racket,
            base_offset: Vec3::new(12.0, 24.0, 3.0),
            size: Vec2::new(6.0, 16.0),
            color: Color::srgb(0.9, 0.9, 0.3),
        },
    ]
}

/// パーツ分離キャラクタープレイヤーをスポーン
/// @spec 31001_parts_spec.md#req-31001-002
pub fn spawn_articulated_player(
    commands: &mut Commands,
    player_id: u8,
    position: Vec3,
    base_color: Color,
) -> Entity {
    let court_side = if player_id == 1 {
        CourtSide::Left
    } else {
        CourtSide::Right
    };

    // 親エンティティ（キャラクター本体）をスポーン
    // @spec 31001_parts_spec.md#req-31001-002
    // @spec 31002_animation_spec.md#req-31002-002
    let parent_entity = commands
        .spawn((
            ArticulatedCharacter,
            Player {
                id: player_id,
                court_side,
            },
            LogicalPosition { value: position },
            Velocity::default(),
            KnockbackState::default(),
            GroundedState::default(),
            ShotState::default(),
            InputState::default(),
            CharacterFacing::default(),
            CharacterAnimationState::default(), // アニメーション状態
            // 親には表示なし（子パーツで表示）
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    // パーツをスポーン
    let part_configs = default_part_configs(base_color);
    for config in part_configs {
        let part_entity = commands
            .spawn((
                PartDefinition {
                    kind: config.kind,
                    base_offset: config.base_offset,
                    size: config.size,
                },
                PartState::default(),
                Sprite {
                    color: config.color,
                    custom_size: Some(config.size),
                    ..default()
                },
                Transform::from_translation(config.base_offset),
            ))
            .id();

        // 親子関係を設定
        commands.entity(parent_entity).add_child(part_entity);
    }

    parent_entity
}
