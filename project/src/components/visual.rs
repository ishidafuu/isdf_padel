//! 視覚関連コンポーネント
//! @spec 20001_layers.md#layer-3-components

use bevy::prelude::*;

/// 影マーカーコンポーネント
/// 親エンティティの影を表示する
#[derive(Component, Debug, Clone, Copy)]
pub struct Shadow {
    /// 影の所有者エンティティ
    pub owner: Entity,
}

/// 影がスポーンされたことを示すマーカー
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct HasShadow;
