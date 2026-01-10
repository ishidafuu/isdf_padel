#![allow(dead_code)]
//! Replay Data Structures
//! @spec 77103_replay_spec.md
//!
//! リプレイ機能で使用するデータ構造を定義。

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::CourtSide;

/// リプレイデータ全体
/// @spec REQ-77103-001, REQ-77103-002
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayData {
    /// メタデータ
    pub metadata: ReplayMetadata,
    /// フレームごとの入力データ
    pub frames: Vec<FrameInput>,
}

impl ReplayData {
    /// 新しいリプレイデータを作成
    pub fn new(metadata: ReplayMetadata) -> Self {
        Self {
            metadata,
            frames: Vec::new(),
        }
    }

    /// 入力フレームを追加
    pub fn push_frame(&mut self, frame: FrameInput) {
        self.frames.push(frame);
    }
}

/// リプレイメタデータ
/// @spec REQ-77103-002
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayMetadata {
    /// ゲームバージョン（不一致時は削除対象）
    pub game_version: String,
    /// 記録開始時刻（ISO 8601形式）
    pub recorded_at: String,
    /// 乱数シード
    pub seed: u64,
    /// 最初のサーブ側
    pub initial_serve_side: CourtSide,
}

impl ReplayMetadata {
    /// 現在のバージョンで新しいメタデータを作成
    pub fn new(seed: u64, initial_serve_side: CourtSide) -> Self {
        Self {
            game_version: env!("CARGO_PKG_VERSION").to_string(),
            recorded_at: chrono::Utc::now().to_rfc3339(),
            seed,
            initial_serve_side,
        }
    }

    /// バージョンが一致するか確認
    pub fn is_version_compatible(&self) -> bool {
        self.game_version == env!("CARGO_PKG_VERSION")
    }
}

/// 1フレームの入力データ
/// @spec REQ-77103-001
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameInput {
    /// フレーム番号
    pub frame: u32,
    /// Player 1 の入力
    pub p1: InputSnapshot,
    /// Player 2 の入力
    pub p2: InputSnapshot,
}

impl FrameInput {
    /// 新しいフレーム入力を作成
    pub fn new(frame: u32, p1: InputSnapshot, p2: InputSnapshot) -> Self {
        Self { frame, p1, p2 }
    }
}

/// 入力状態のスナップショット
/// InputState コンポーネントのシリアライズ可能なコピー
/// @spec REQ-77103-001
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct InputSnapshot {
    /// 移動入力（-1.0 〜 1.0）
    pub movement: Vec2,
    /// ジャンプボタンが押されたか
    pub jump_pressed: bool,
    /// ショットボタンが押されたか
    pub shot_pressed: bool,
    /// ショットボタンを保持中か
    pub holding: bool,
    /// ホールド継続時間（秒）
    pub hold_time: f32,
}

impl InputSnapshot {
    /// InputState から変換
    pub fn from_input_state(input: &crate::components::InputState) -> Self {
        Self {
            movement: input.movement,
            jump_pressed: input.jump_pressed,
            shot_pressed: input.shot_pressed,
            holding: input.holding,
            hold_time: input.hold_time,
        }
    }
}

/// リプレイ設定
/// @data 87103_replay_config.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayConfig {
    /// ファイル管理設定
    pub file_management: FileManagementConfig,
    /// クリーンアップポリシー
    pub cleanup_policy: CleanupPolicyConfig,
}

impl Default for ReplayConfig {
    fn default() -> Self {
        Self {
            file_management: FileManagementConfig::default(),
            cleanup_policy: CleanupPolicyConfig::default(),
        }
    }
}

/// ファイル管理設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileManagementConfig {
    /// リプレイ保存先ディレクトリ
    pub save_directory: String,
    /// ファイル名プレフィックス
    pub file_prefix: String,
    /// 保存上限件数
    pub max_replay_count: u32,
}

impl Default for FileManagementConfig {
    fn default() -> Self {
        Self {
            save_directory: "assets/replays".to_string(),
            file_prefix: "replay_".to_string(),
            max_replay_count: 100,
        }
    }
}

/// クリーンアップポリシー
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupPolicyConfig {
    /// バージョン不一致時に削除
    pub delete_on_version_mismatch: bool,
    /// 上限超過時に古いものを削除
    pub delete_oldest_on_limit: bool,
}

impl Default for CleanupPolicyConfig {
    fn default() -> Self {
        Self {
            delete_on_version_mismatch: true,
            delete_oldest_on_limit: true,
        }
    }
}

/// 設定ファイルを読み込む
/// @data 87103_replay_config.md
pub fn load_replay_config<P: AsRef<std::path::Path>>(path: P) -> Result<ReplayConfig, String> {
    let path = path.as_ref();

    if !path.exists() {
        return Ok(ReplayConfig::default());
    }

    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    ron::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))
}
