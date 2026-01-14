#![allow(dead_code)]
//! Replay Data Structures
//! @spec 77103_replay_spec.md
//!
//! リプレイ機能で使用するデータ構造を定義。

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::CourtSide;


/// プレイヤーのコントロールタイプ
/// @spec REQ-77103-002
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlType {
    /// 人間操作
    Human,
    /// AI操作
    Ai,
}

impl Default for ControlType {
    fn default() -> Self {
        ControlType::Ai // 後方互換性：デフォルトはAI
    }
}

/// リプレイデータ全体
/// @spec REQ-77103-001, REQ-77103-002
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
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
    /// Left側プレイヤーのコントロールタイプ
    #[serde(default)]
    pub left_control: ControlType,
    /// Right側プレイヤーのコントロールタイプ
    #[serde(default)]
    pub right_control: ControlType,
}

impl ReplayMetadata {
    /// 現在のバージョンで新しいメタデータを作成
    pub fn new(
        seed: u64,
        initial_serve_side: CourtSide,
        left_control: ControlType,
        right_control: ControlType,
    ) -> Self {
        Self {
            game_version: env!("CARGO_PKG_VERSION").to_string(),
            recorded_at: chrono::Utc::now().to_rfc3339(),
            seed,
            initial_serve_side,
            left_control,
            right_control,
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
}

impl InputSnapshot {
    /// InputState から変換（hold_timeは再生時に再計算するため保存しない）
    pub fn from_input_state(input: &crate::components::InputState) -> Self {
        Self {
            movement: input.movement,
            jump_pressed: input.jump_pressed,
            shot_pressed: input.shot_pressed,
            holding: input.holding,
        }
    }
}

// ============================================================================
// バイナリシリアライズ用構造体
// ============================================================================

/// バイナリ形式のフレーム入力（6バイト）
/// @spec REQ-77103-001
#[derive(Debug, Clone, Copy)]
pub struct BinaryFrameInput {
    /// Player 1 の入力（3バイト）
    pub p1: BinaryInputSnapshot,
    /// Player 2 の入力（3バイト）
    pub p2: BinaryInputSnapshot,
}

/// バイナリ形式の入力スナップショット（3バイト）
/// @spec REQ-77103-001
#[derive(Debug, Clone, Copy)]
pub struct BinaryInputSnapshot {
    /// 移動X（-127〜127 → -1.0〜1.0）
    pub movement_x: i8,
    /// 移動Y（-127〜127 → -1.0〜1.0）
    pub movement_y: i8,
    /// フラグ（bit0: jump, bit1: shot, bit2: holding）
    pub flags: u8,
}

impl BinaryInputSnapshot {
    /// フラグビット定義
    const FLAG_JUMP: u8 = 0b001;
    const FLAG_SHOT: u8 = 0b010;
    const FLAG_HOLDING: u8 = 0b100;

    /// InputSnapshot からバイナリ形式に変換
    pub fn from_snapshot(snapshot: &InputSnapshot) -> Self {
        let movement_x = (snapshot.movement.x * 127.0).round() as i8;
        let movement_y = (snapshot.movement.y * 127.0).round() as i8;

        let mut flags = 0u8;
        if snapshot.jump_pressed {
            flags |= Self::FLAG_JUMP;
        }
        if snapshot.shot_pressed {
            flags |= Self::FLAG_SHOT;
        }
        if snapshot.holding {
            flags |= Self::FLAG_HOLDING;
        }

        Self {
            movement_x,
            movement_y,
            flags,
        }
    }

    /// バイナリ形式から InputSnapshot に変換
    pub fn to_snapshot(&self) -> InputSnapshot {
        InputSnapshot {
            movement: Vec2::new(
                self.movement_x as f32 / 127.0,
                self.movement_y as f32 / 127.0,
            ),
            jump_pressed: (self.flags & Self::FLAG_JUMP) != 0,
            shot_pressed: (self.flags & Self::FLAG_SHOT) != 0,
            holding: (self.flags & Self::FLAG_HOLDING) != 0,
        }
    }

    /// バイト配列に書き込み
    pub fn write_to(&self, buf: &mut [u8; 3]) {
        buf[0] = self.movement_x as u8;
        buf[1] = self.movement_y as u8;
        buf[2] = self.flags;
    }

    /// バイト配列から読み込み
    pub fn read_from(buf: &[u8; 3]) -> Self {
        Self {
            movement_x: buf[0] as i8,
            movement_y: buf[1] as i8,
            flags: buf[2],
        }
    }
}

impl BinaryFrameInput {
    /// FrameInput からバイナリ形式に変換
    pub fn from_frame_input(frame: &FrameInput) -> Self {
        Self {
            p1: BinaryInputSnapshot::from_snapshot(&frame.p1),
            p2: BinaryInputSnapshot::from_snapshot(&frame.p2),
        }
    }

    /// バイナリ形式から FrameInput に変換
    pub fn to_frame_input(&self, frame_number: u32) -> FrameInput {
        FrameInput {
            frame: frame_number,
            p1: self.p1.to_snapshot(),
            p2: self.p2.to_snapshot(),
        }
    }

    /// バイト配列に書き込み（6バイト）
    pub fn write_to(&self, buf: &mut [u8; 6]) {
        let mut p1_buf = [0u8; 3];
        let mut p2_buf = [0u8; 3];
        self.p1.write_to(&mut p1_buf);
        self.p2.write_to(&mut p2_buf);
        buf[0..3].copy_from_slice(&p1_buf);
        buf[3..6].copy_from_slice(&p2_buf);
    }

    /// バイト配列から読み込み（6バイト）
    pub fn read_from(buf: &[u8; 6]) -> Self {
        let p1_buf: [u8; 3] = [buf[0], buf[1], buf[2]];
        let p2_buf: [u8; 3] = [buf[3], buf[4], buf[5]];
        Self {
            p1: BinaryInputSnapshot::read_from(&p1_buf),
            p2: BinaryInputSnapshot::read_from(&p2_buf),
        }
    }
}

/// リプレイ設定
/// @data 87103_replay_config.md
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplayConfig {
    /// ファイル管理設定
    pub file_management: FileManagementConfig,
    /// クリーンアップポリシー
    pub cleanup_policy: CleanupPolicyConfig,
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
