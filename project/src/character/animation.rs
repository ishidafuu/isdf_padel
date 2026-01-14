//! パーツ分離キャラクターのアニメーションシステム
//! @spec 31002_animation_spec.md

use std::collections::HashMap;

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use serde::Deserialize;

use super::components::{PartKind, PartState};

/// アニメーション識別子
/// @spec 31002_animation_spec.md#req-31002-001
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Deserialize)]
pub enum AnimationId {
    /// 待機アニメーション（浮遊感のある微動）
    #[default]
    Idle,
    /// 歩行アニメーション
    Walk,
    /// ジャンプアニメーション
    Jump,
    /// 落下アニメーション
    Fall,
    /// ショットアニメーション
    Shot,
}

/// アニメーション状態Component
/// @spec 31002_animation_spec.md#req-31002-002
#[derive(Component, Debug, Clone)]
pub struct CharacterAnimationState {
    /// 再生中のアニメーション
    pub current_animation: AnimationId,
    /// 経過時間（秒）
    pub elapsed: f32,
    /// ループ再生フラグ
    pub looping: bool,
    /// 再生速度（1.0 = 通常速度）
    pub speed: f32,
}

impl Default for CharacterAnimationState {
    fn default() -> Self {
        Self {
            current_animation: AnimationId::Idle,
            elapsed: 0.0,
            looping: true,
            speed: 1.0,
        }
    }
}

impl CharacterAnimationState {
    /// 新しいアニメーションを開始
    pub fn play(&mut self, animation: AnimationId, looping: bool) {
        self.current_animation = animation;
        self.elapsed = 0.0;
        self.looping = looping;
    }

    /// アニメーションが終了したかどうか
    pub fn is_finished(&self, duration: f32) -> bool {
        !self.looping && self.elapsed >= duration
    }
}

/// キーフレームデータ
/// @spec 31002_animation_spec.md#req-31002-003
#[derive(Debug, Clone, Deserialize)]
pub struct Keyframe {
    /// キーフレーム時刻（秒）
    pub time: f32,
    /// ローカル位置（base_offsetからの相対）
    #[serde(default)]
    pub position: Vec3,
    /// 回転角度（度）
    #[serde(default)]
    pub rotation: f32,
}

/// パーツごとのキーフレーム配列（RONデシリアライズ用）
#[derive(Debug, Clone, Deserialize)]
pub struct PartKeyframes {
    pub head: Option<Vec<Keyframe>>,
    pub torso: Option<Vec<Keyframe>>,
    pub left_hand: Option<Vec<Keyframe>>,
    pub right_hand: Option<Vec<Keyframe>>,
    pub left_knee: Option<Vec<Keyframe>>,
    pub right_knee: Option<Vec<Keyframe>>,
    pub left_foot: Option<Vec<Keyframe>>,
    pub right_foot: Option<Vec<Keyframe>>,
    /// @spec 31001_parts_spec.md#req-31001-008
    pub racket: Option<Vec<Keyframe>>,
}

impl PartKeyframes {
    /// HashMapに変換
    pub fn to_hashmap(&self) -> HashMap<PartKind, Vec<Keyframe>> {
        let mut map = HashMap::new();
        if let Some(kf) = &self.head {
            map.insert(PartKind::Head, kf.clone());
        }
        if let Some(kf) = &self.torso {
            map.insert(PartKind::Torso, kf.clone());
        }
        if let Some(kf) = &self.left_hand {
            map.insert(PartKind::LeftHand, kf.clone());
        }
        if let Some(kf) = &self.right_hand {
            map.insert(PartKind::RightHand, kf.clone());
        }
        if let Some(kf) = &self.left_knee {
            map.insert(PartKind::LeftKnee, kf.clone());
        }
        if let Some(kf) = &self.right_knee {
            map.insert(PartKind::RightKnee, kf.clone());
        }
        if let Some(kf) = &self.left_foot {
            map.insert(PartKind::LeftFoot, kf.clone());
        }
        if let Some(kf) = &self.right_foot {
            map.insert(PartKind::RightFoot, kf.clone());
        }
        if let Some(kf) = &self.racket {
            map.insert(PartKind::Racket, kf.clone());
        }
        map
    }
}

/// アニメーションデータ（RONデシリアライズ用）
/// @spec 31002_animation_spec.md#req-31002-004
#[derive(Debug, Clone, Deserialize)]
pub struct AnimationDataRon {
    /// アニメーション全体の長さ（秒）
    pub duration: f32,
    /// デフォルトでループ再生するか
    #[serde(default = "default_looping")]
    pub looping: bool,
    /// パーツごとのキーフレーム
    pub keyframes: PartKeyframes,
}

fn default_looping() -> bool {
    true
}

/// アニメーションデータ（内部使用）
#[derive(Debug, Clone)]
pub struct CharacterAnimationData {
    /// アニメーション全体の長さ（秒）
    pub duration: f32,
    /// デフォルトでループ再生するか
    pub looping: bool,
    /// パーツごとのキーフレーム配列
    pub keyframes: HashMap<PartKind, Vec<Keyframe>>,
}

impl CharacterAnimationData {
    /// キーフレーム補間でPartStateを計算
    /// @spec 31002_animation_spec.md#req-31002-005
    pub fn sample(&self, part_kind: PartKind, time: f32) -> PartState {
        let Some(keyframes): Option<&Vec<Keyframe>> = self.keyframes.get(&part_kind) else {
            return PartState::default();
        };

        if keyframes.is_empty() {
            return PartState::default();
        }

        // 時刻が最初のキーフレームより前
        if time <= keyframes[0].time {
            return PartState {
                local_position: keyframes[0].position,
                local_rotation: keyframes[0].rotation,
            };
        }

        // 時刻が最後のキーフレームより後
        let last = &keyframes[keyframes.len() - 1];
        if time >= last.time {
            return PartState {
                local_position: last.position,
                local_rotation: last.rotation,
            };
        }

        // 補間するキーフレームを探す
        for i in 0..keyframes.len() - 1 {
            let kf_prev = &keyframes[i];
            let kf_next = &keyframes[i + 1];

            if time >= kf_prev.time && time < kf_next.time {
                // 線形補間
                let t = (time - kf_prev.time) / (kf_next.time - kf_prev.time);
                return PartState {
                    local_position: kf_prev.position.lerp(kf_next.position, t),
                    local_rotation: kf_prev.rotation + (kf_next.rotation - kf_prev.rotation) * t,
                };
            }
        }

        PartState::default()
    }
}

/// 全アニメーションデータ（RONファイル構造）
#[derive(Debug, Clone, Deserialize, Asset, TypePath)]
pub struct CharacterAnimationsRon {
    pub idle: Option<AnimationDataRon>,
    pub walk: Option<AnimationDataRon>,
    pub jump: Option<AnimationDataRon>,
    pub fall: Option<AnimationDataRon>,
    pub shot: Option<AnimationDataRon>,
}

/// 全アニメーションを保持するResource
/// @spec 31002_animation_spec.md#req-31002-004
#[derive(Resource, Debug, Clone, Default)]
pub struct CharacterAnimations {
    pub animations: HashMap<AnimationId, CharacterAnimationData>,
}

impl CharacterAnimations {
    /// RONデータから構築
    pub fn from_ron(ron: &CharacterAnimationsRon) -> Self {
        let mut animations = HashMap::new();

        if let Some(data) = &ron.idle {
            animations.insert(
                AnimationId::Idle,
                CharacterAnimationData {
                    duration: data.duration,
                    looping: data.looping,
                    keyframes: data.keyframes.to_hashmap(),
                },
            );
        }

        if let Some(data) = &ron.walk {
            animations.insert(
                AnimationId::Walk,
                CharacterAnimationData {
                    duration: data.duration,
                    looping: data.looping,
                    keyframes: data.keyframes.to_hashmap(),
                },
            );
        }

        if let Some(data) = &ron.jump {
            animations.insert(
                AnimationId::Jump,
                CharacterAnimationData {
                    duration: data.duration,
                    looping: data.looping,
                    keyframes: data.keyframes.to_hashmap(),
                },
            );
        }

        if let Some(data) = &ron.fall {
            animations.insert(
                AnimationId::Fall,
                CharacterAnimationData {
                    duration: data.duration,
                    looping: data.looping,
                    keyframes: data.keyframes.to_hashmap(),
                },
            );
        }

        if let Some(data) = &ron.shot {
            animations.insert(
                AnimationId::Shot,
                CharacterAnimationData {
                    duration: data.duration,
                    looping: data.looping,
                    keyframes: data.keyframes.to_hashmap(),
                },
            );
        }

        Self { animations }
    }

    /// アニメーションデータを取得
    pub fn get(&self, id: AnimationId) -> Option<&CharacterAnimationData> {
        self.animations.get(&id)
    }
}

/// CharacterAnimationsRon の RON ファイルをロードするカスタム AssetLoader
#[derive(Default)]
pub struct CharacterAnimationsLoader;

/// CharacterAnimationsLoader のエラー型
#[derive(Debug, thiserror::Error)]
pub enum CharacterAnimationsLoaderError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

impl AssetLoader for CharacterAnimationsLoader {
    type Asset = CharacterAnimationsRon;
    type Settings = ();
    type Error = CharacterAnimationsLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let data: CharacterAnimationsRon = ron::de::from_bytes(&bytes)?;
        Ok(data)
    }

    fn extensions(&self) -> &[&str] {
        &["anim.ron"]
    }
}
