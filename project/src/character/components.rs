//! パーツ分離キャラクターのComponent定義
//! @spec 31001_parts_spec.md

use bevy::prelude::*;

/// パーツ種別列挙型
/// @spec 31001_parts_spec.md#req-31001-001
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartKind {
    Head,
    Torso,
    LeftHand,
    RightHand,
    LeftKnee,
    RightKnee,
    LeftFoot,
    RightFoot,
    /// @spec 31001_parts_spec.md#req-31001-008
    Racket,
}

impl PartKind {
    /// 全パーツ種別を取得（将来の拡張用）
    #[allow(dead_code)]
    pub fn all() -> &'static [PartKind] {
        &[
            PartKind::Head,
            PartKind::Torso,
            PartKind::LeftHand,
            PartKind::RightHand,
            PartKind::LeftKnee,
            PartKind::RightKnee,
            PartKind::LeftFoot,
            PartKind::RightFoot,
            PartKind::Racket,
        ]
    }

    /// 左右対称パーツかどうか
    /// Racketは右手側のみなので対称パーツには含めない
    pub fn is_symmetric(&self) -> bool {
        matches!(
            self,
            PartKind::LeftHand
                | PartKind::RightHand
                | PartKind::LeftKnee
                | PartKind::RightKnee
                | PartKind::LeftFoot
                | PartKind::RightFoot
                | PartKind::Racket
        )
    }

    /// 右側パーツかどうか
    /// @spec 31001_parts_spec.md#req-31001-007
    pub fn is_right(&self) -> bool {
        matches!(
            self,
            PartKind::RightHand | PartKind::RightKnee | PartKind::RightFoot | PartKind::Racket
        )
    }
}

/// パーツ定義Component
/// @spec 31001_parts_spec.md#req-31001-003
#[derive(Component, Debug, Clone)]
pub struct PartDefinition {
    /// パーツ種別
    pub kind: PartKind,
    /// ニュートラル状態での基準位置（親からの相対）
    pub base_offset: Vec3,
    /// スプライトサイズ（将来の動的サイズ変更用）
    #[allow(dead_code)]
    pub size: Vec2,
}

/// パーツ状態Component（動的状態）
/// @spec 31001_parts_spec.md#req-31001-004
#[derive(Component, Debug, Clone, Default)]
pub struct PartState {
    /// 現在のローカル位置（base_offsetからの相対）
    pub local_position: Vec3,
    /// 現在の回転角度（度）
    pub local_rotation: f32,
}

/// パーツ分離キャラクターマーカーComponent
/// @spec 31001_parts_spec.md#req-31001-002
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ArticulatedCharacter;

/// キャラクター向きComponent
/// @spec 31001_parts_spec.md#req-31001-005
#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CharacterFacing {
    /// 右向き（デフォルト）
    #[default]
    Right,
    /// 左向き
    Left,
}

impl CharacterFacing {
    /// ミラー係数を取得（右=1.0, 左=-1.0）
    #[inline]
    pub fn mirror(&self) -> f32 {
        match self {
            CharacterFacing::Right => 1.0,
            CharacterFacing::Left => -1.0,
        }
    }
}
