//! パーツ分離キャラクターシステム
//! @spec 31000_overview.md
//!
//! ジョイメカファイト風のパーツ分離キャラクターを実装。
//! 頭・胴体・手・膝・足が分離したロボットキャラクターを表現する。

// モジュール外部公開用（クレート内では直接使用されないがAPIとして公開）
#![allow(unused_imports)]

pub mod animation;
pub mod bundle;
pub mod components;
pub mod systems;

pub use animation::{
    AnimationId, CharacterAnimationState, CharacterAnimations, CharacterAnimationsLoader,
    CharacterAnimationsRon,
};
pub use bundle::{default_part_configs, spawn_articulated_player, PartConfig};
pub use components::{
    ArticulatedCharacter, CharacterFacing, PartDefinition, PartKind, PartState,
};
pub use systems::{
    advance_animation_system, load_character_animations_system, sync_part_transforms_system,
    trigger_shot_animation_system, update_animation_state_system, update_character_facing_system,
    update_part_states_system,
};

use bevy::prelude::*;

/// パーツ分離キャラクターPlugin
/// @spec 31000_overview.md
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        // アニメーションアセットローダーを登録
        app.init_asset::<CharacterAnimationsRon>()
            .init_asset_loader::<CharacterAnimationsLoader>()
            .init_resource::<CharacterAnimations>();

        // 起動時にアニメーションデータをロード
        app.add_systems(Startup, load_character_animations_system);

        app.add_systems(
            Update,
            (
                // ShotEventでショットアニメーションをトリガー
                // @spec 31002_animation_spec.md#req-31002-052
                trigger_shot_animation_system,
                // 状態に基づいてアニメーションを切り替え
                // @spec 31002_animation_spec.md#req-31002-051
                update_animation_state_system,
                // 移動方向から向きを自動判定
                // @spec 31001_parts_spec.md#req-31001-005
                update_character_facing_system,
                // アニメーション時間を進める
                // @spec 31002_animation_spec.md#req-31002-006
                advance_animation_system,
                // キーフレーム補間でPartStateを更新
                // @spec 31002_animation_spec.md#req-31002-007
                update_part_states_system,
                // パーツTransform同期は最後に実行
                // @spec 31001_parts_spec.md#req-31001-006
                sync_part_transforms_system,
            )
                .chain(),
        );
    }
}
