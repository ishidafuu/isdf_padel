//! パーツ分離キャラクターのSystem実装
//! @spec 31001_parts_spec.md
//! @spec 31002_animation_spec.md

use bevy::prelude::*;

use super::animation::{
    AnimationId, CharacterAnimationState, CharacterAnimations, CharacterAnimationsRon,
};
use super::components::{ArticulatedCharacter, CharacterFacing, PartDefinition, PartState};
use crate::components::{GroundedState, LogicalPosition, Player, Velocity};
use crate::core::events::ShotEvent;
use crate::resource::FixedDeltaTime;
use crate::resource::GameConfig;

/// 起動時にアニメーションデータをロードするシステム
/// @data config.character.animation_file_path
pub fn load_character_animations_system(
    config: Res<GameConfig>,
    mut animations: ResMut<CharacterAnimations>,
) {
    let file_path = &config.character.animation_file_path;
    // ファイルから直接読み込み
    match std::fs::read_to_string(file_path) {
        Ok(content) => match ron::from_str::<CharacterAnimationsRon>(&content) {
            Ok(ron_data) => {
                *animations = CharacterAnimations::from_ron(&ron_data);
                info!(
                    "Loaded character animations: {} animations",
                    animations.animations.len()
                );
            }
            Err(e) => {
                warn!("Failed to parse character animations: {}", e);
            }
        },
        Err(e) => {
            warn!(
                "Failed to load character animations from {}: {}",
                file_path, e
            );
        }
    }
}

/// パーツのTransformを同期するシステム
/// @spec 31001_parts_spec.md#req-31001-006
/// @spec 31001_parts_spec.md#req-31001-007
/// @data config.character.z_priority_offset
pub fn sync_part_transforms_system(
    config: Res<GameConfig>,
    parent_query: Query<
        (&LogicalPosition, &CharacterFacing, &Children),
        With<ArticulatedCharacter>,
    >,
    mut part_query: Query<(&PartDefinition, &PartState, &mut Transform)>,
) {
    let z_priority_offset = config.character.z_priority_offset;

    for (_logical_pos, facing, children) in parent_query.iter() {
        let mirror = facing.mirror();

        for child in children.iter() {
            if let Ok((definition, state, mut transform)) = part_query.get_mut(child) {
                // パーツのローカル座標を計算
                // @spec 31001_parts_spec.md#req-31001-006
                let local_x = (definition.base_offset.x + state.local_position.x) * mirror;
                let local_y = definition.base_offset.y + state.local_position.y;

                // Z優先度計算（向きに応じて左右パーツの前後関係を調整）
                // @spec 31001_parts_spec.md#req-31001-007
                let z_priority = if definition.kind.is_symmetric() {
                    if definition.kind.is_right() {
                        mirror * z_priority_offset // 右パーツ: 右向きで前、左向きで後
                    } else {
                        -mirror * z_priority_offset // 左パーツ: 右向きで後、左向きで前
                    }
                } else {
                    0.0 // 中央パーツ（Head, Torso）は変更なし
                };
                let local_z = (definition.base_offset.z + state.local_position.z + z_priority) * 0.01;

                // Transformを更新（親の座標系内でのローカル位置）
                transform.translation.x = local_x;
                transform.translation.y = local_y;
                transform.translation.z = local_z;

                // 回転（ミラーリング考慮）
                transform.rotation =
                    Quat::from_rotation_z((state.local_rotation * mirror).to_radians());
            }
        }
    }
}

/// キャラクター向きを更新するシステム（移動方向から自動判定）
/// @spec 31001_parts_spec.md#req-31001-005
pub fn update_character_facing_system(
    mut query: Query<(&Velocity, &mut CharacterFacing), With<ArticulatedCharacter>>,
) {
    for (velocity, mut facing) in query.iter_mut() {
        // X方向の速度で向きを判定
        if velocity.value.x > 0.1 {
            *facing = CharacterFacing::Right;
        } else if velocity.value.x < -0.1 {
            *facing = CharacterFacing::Left;
        }
        // 速度が小さい場合は向きを維持
    }
}

/// アニメーション時間を進めるシステム
/// @spec 31002_animation_spec.md#req-31002-006
pub fn advance_animation_system(
    fixed_dt: Res<FixedDeltaTime>,
    animations: Res<CharacterAnimations>,
    mut query: Query<&mut CharacterAnimationState, With<ArticulatedCharacter>>,
) {
    for mut anim_state in query.iter_mut() {
        // 現在のアニメーションデータを取得
        let Some(anim_data) = animations.get(anim_state.current_animation) else {
            continue;
        };

        // 経過時間を更新
        // @spec 31002_animation_spec.md#req-31002-006
        anim_state.elapsed += fixed_dt.delta_secs() * anim_state.speed;

        // ループ処理
        if anim_state.looping && anim_state.elapsed >= anim_data.duration {
            anim_state.elapsed %= anim_data.duration;
        } else if !anim_state.looping && anim_state.elapsed >= anim_data.duration {
            anim_state.elapsed = anim_data.duration;
        }
    }
}

/// キーフレーム補間でPartStateを更新するシステム
/// @spec 31002_animation_spec.md#req-31002-007
pub fn update_part_states_system(
    animations: Res<CharacterAnimations>,
    parent_query: Query<(&CharacterAnimationState, &Children), With<ArticulatedCharacter>>,
    mut part_query: Query<(&PartDefinition, &mut PartState)>,
) {
    for (anim_state, children) in parent_query.iter() {
        // 現在のアニメーションデータを取得
        let Some(anim_data) = animations.get(anim_state.current_animation) else {
            continue;
        };

        for child in children.iter() {
            if let Ok((definition, mut part_state)) = part_query.get_mut(child) {
                // キーフレーム補間でPartStateを計算
                // @spec 31002_animation_spec.md#req-31002-005
                let sampled = anim_data.sample(definition.kind, anim_state.elapsed);
                *part_state = sampled;
            }
        }
    }
}

/// ShotEventを受け取ってショットアニメーションをトリガーするシステム
/// @spec 31002_animation_spec.md#req-31002-052
pub fn trigger_shot_animation_system(
    animations: Res<CharacterAnimations>,
    mut shot_events: MessageReader<ShotEvent>,
    mut query: Query<(&Player, &mut CharacterAnimationState), With<ArticulatedCharacter>>,
) {
    for event in shot_events.read() {
        for (player, mut anim_state) in query.iter_mut() {
            if player.id == event.player_id {
                // ショットアニメーションを開始（ループなし）
                let looping = animations
                    .get(AnimationId::Shot)
                    .is_some_and(|data| data.looping);
                anim_state.play(AnimationId::Shot, looping);
            }
        }
    }
}

/// キャラクター状態に基づいてアニメーションを自動切り替えするシステム
/// @spec 31002_animation_spec.md#req-31002-051
/// @data config.character.walk_velocity_threshold
/// @data config.character.jump_velocity_threshold
pub fn update_animation_state_system(
    config: Res<GameConfig>,
    animations: Res<CharacterAnimations>,
    mut query: Query<
        (
            &mut CharacterAnimationState,
            Option<&Velocity>,
            Option<&GroundedState>,
        ),
        With<ArticulatedCharacter>,
    >,
) {
    let walk_threshold = config.character.walk_velocity_threshold;
    let jump_threshold = config.character.jump_velocity_threshold;

    for (mut anim_state, velocity, grounded) in query.iter_mut() {
        // ショットアニメーション再生中は遷移しない
        // @spec 31002_animation_spec.md#req-31002-052
        if anim_state.current_animation == AnimationId::Shot {
            if let Some(shot_data) = animations.get(AnimationId::Shot) {
                if !anim_state.is_finished(shot_data.duration) {
                    continue;
                }
            }
        }

        // VelocityやGroundedStateがない場合はIdleを維持
        let Some(velocity) = velocity else {
            continue;
        };

        let is_grounded = grounded.is_some_and(|g| g.is_grounded);
        let vx = velocity.value.x.abs();
        let vy = velocity.value.y;

        // 次のアニメーションを決定
        let next_animation = if !is_grounded {
            // 空中
            if vy > jump_threshold {
                AnimationId::Jump
            } else {
                AnimationId::Fall
            }
        } else {
            // 地上
            if vx > walk_threshold {
                AnimationId::Walk
            } else {
                AnimationId::Idle
            }
        };

        // アニメーション変更が必要な場合
        if anim_state.current_animation != next_animation {
            // 新しいアニメーションのデータを取得してループ設定を決定
            let looping = animations
                .get(next_animation)
                .is_some_and(|data| data.looping);
            anim_state.play(next_animation, looping);
        }
    }
}
