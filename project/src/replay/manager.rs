#![allow(dead_code)]
//! Replay Manager
//! @spec 77103_replay_spec.md
//!
//! リプレイファイルの保存、読み込み、管理を行う。
//! バイナリ形式（.replay）で保存。

use bevy::prelude::*;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use super::data::{load_replay_config, BinaryFrameInput, ReplayConfig, ReplayData, ReplayMetadata};
use super::loader;

/// リプレイファイルのマジックナンバー
const REPLAY_MAGIC: &[u8; 4] = b"RPLY";
/// リプレイファイルのバージョン
const REPLAY_VERSION: u16 = 1;
/// リプレイファイルの拡張子
const REPLAY_EXTENSION: &str = "replay";

/// リプレイ設定ファイルのデフォルトパス
const REPLAY_CONFIG_PATH: &str = "assets/config/replay_config.ron";

/// リプレイマネージャーリソース
/// @spec REQ-77103-003, REQ-77103-004, REQ-77103-005
#[derive(Resource)]
pub struct ReplayManager {
    config: ReplayConfig,
}

impl Default for ReplayManager {
    fn default() -> Self {
        // 設定ファイルを読み込む（失敗時はデフォルト値を使用）
        let config = match load_replay_config(REPLAY_CONFIG_PATH) {
            Ok(c) => {
                info!("Loaded replay config from {}", REPLAY_CONFIG_PATH);
                c
            }
            Err(e) => {
                warn!("Failed to load replay config: {}. Using defaults.", e);
                ReplayConfig::default()
            }
        };
        Self { config }
    }
}

impl ReplayManager {
    /// 新しいマネージャーを作成
    pub fn new(config: ReplayConfig) -> Self {
        Self { config }
    }

    /// 設定を取得
    pub fn config(&self) -> &ReplayConfig {
        &self.config
    }

    /// 保存ディレクトリのパスを取得
    pub fn save_directory(&self) -> &Path {
        Path::new(&self.config.file_management.save_directory)
    }

    /// リプレイを保存（バイナリ形式）
    /// @spec REQ-77103-003
    pub fn save_replay(&self, data: &ReplayData) -> Result<PathBuf, String> {
        // ディレクトリが存在しなければ作成
        let save_dir = self.save_directory();
        if !save_dir.exists() {
            fs::create_dir_all(save_dir)
                .map_err(|e| format!("Failed to create replay directory: {}", e))?;
        }

        // ファイル名を生成（timestamp ベース）
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!(
            "{}{}.{}",
            self.config.file_management.file_prefix, timestamp, REPLAY_EXTENSION
        );
        let filepath = save_dir.join(&filename);

        // バイナリ形式で書き込み
        self.write_binary_replay(&filepath, data)?;

        info!("Replay saved: {:?} ({} frames, {} bytes)",
            filepath,
            data.frames.len(),
            fs::metadata(&filepath).map(|m| m.len()).unwrap_or(0)
        );
        Ok(filepath)
    }

    /// バイナリ形式でリプレイを書き込み
    fn write_binary_replay(&self, path: &Path, data: &ReplayData) -> Result<(), String> {
        let file = File::create(path)
            .map_err(|e| format!("Failed to create file: {}", e))?;
        let mut writer = BufWriter::new(file);

        // メタデータをbincodeでシリアライズ
        let metadata_bytes = bincode::serialize(&data.metadata)
            .map_err(|e| format!("Failed to serialize metadata: {}", e))?;

        // ヘッダー書き込み（16バイト）
        // Magic (4 bytes)
        writer.write_all(REPLAY_MAGIC)
            .map_err(|e| format!("Failed to write magic: {}", e))?;
        // Version (2 bytes, little endian)
        writer.write_all(&REPLAY_VERSION.to_le_bytes())
            .map_err(|e| format!("Failed to write version: {}", e))?;
        // Reserved (2 bytes)
        writer.write_all(&[0u8; 2])
            .map_err(|e| format!("Failed to write reserved: {}", e))?;
        // Frame count (4 bytes, little endian)
        let frame_count = data.frames.len() as u32;
        writer.write_all(&frame_count.to_le_bytes())
            .map_err(|e| format!("Failed to write frame count: {}", e))?;
        // Metadata size (4 bytes, little endian)
        let metadata_size = metadata_bytes.len() as u32;
        writer.write_all(&metadata_size.to_le_bytes())
            .map_err(|e| format!("Failed to write metadata size: {}", e))?;

        // メタデータ書き込み
        writer.write_all(&metadata_bytes)
            .map_err(|e| format!("Failed to write metadata: {}", e))?;

        // フレームデータ書き込み（各6バイト）
        let mut frame_buf = [0u8; 6];
        for frame in &data.frames {
            let binary_frame = BinaryFrameInput::from_frame_input(frame);
            binary_frame.write_to(&mut frame_buf);
            writer.write_all(&frame_buf)
                .map_err(|e| format!("Failed to write frame: {}", e))?;
        }

        writer.flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;

        Ok(())
    }

    /// リプレイを読み込み
    /// @spec REQ-77103-006
    pub fn load_replay<P: AsRef<Path>>(&self, path: P) -> Result<ReplayData, String> {
        // loader モジュールに委譲（バージョンチェック含む）
        loader::load_replay(path)
    }

    /// 保存ディレクトリ内のリプレイファイル一覧を取得
    pub fn list_replays(&self) -> Result<Vec<ReplayFileInfo>, String> {
        let save_dir = self.save_directory();

        if !save_dir.exists() {
            return Ok(Vec::new());
        }

        let mut replays = Vec::new();

        let entries = fs::read_dir(save_dir)
            .map_err(|e| format!("Failed to read replay directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();

            // .replay または .ron ファイル
            let ext = path.extension().and_then(|e| e.to_str());
            if ext == Some(REPLAY_EXTENSION) || ext == Some("ron") {
                if let Some(info) = self.get_replay_info(&path) {
                    replays.push(info);
                }
            }
        }

        // recorded_at で降順ソート（新しい順）
        replays.sort_by(|a, b| b.recorded_at.cmp(&a.recorded_at));

        Ok(replays)
    }

    /// リプレイファイルの情報を取得
    fn get_replay_info(&self, path: &Path) -> Option<ReplayFileInfo> {
        let ext = path.extension().and_then(|e| e.to_str())?;

        if ext == REPLAY_EXTENSION {
            // バイナリ形式
            self.get_binary_replay_info(path)
        } else if ext == "ron" {
            // 旧RON形式（互換性のため）
            self.get_ron_replay_info(path)
        } else {
            None
        }
    }

    /// バイナリ形式のリプレイ情報を取得
    fn get_binary_replay_info(&self, path: &Path) -> Option<ReplayFileInfo> {
        let file = File::open(path).ok()?;
        let mut reader = BufReader::new(file);

        // ヘッダー読み込み
        let mut header = [0u8; 16];
        reader.read_exact(&mut header).ok()?;

        // マジックナンバー確認
        if &header[0..4] != REPLAY_MAGIC {
            return None;
        }

        // フレーム数
        let frame_count = u32::from_le_bytes([header[8], header[9], header[10], header[11]]) as usize;
        // メタデータサイズ
        let metadata_size = u32::from_le_bytes([header[12], header[13], header[14], header[15]]) as usize;

        // メタデータ読み込み
        let mut metadata_bytes = vec![0u8; metadata_size];
        reader.read_exact(&mut metadata_bytes).ok()?;
        let metadata: ReplayMetadata = bincode::deserialize(&metadata_bytes).ok()?;

        Some(ReplayFileInfo {
            path: path.to_path_buf(),
            game_version: metadata.game_version,
            recorded_at: metadata.recorded_at,
            frame_count,
        })
    }

    /// RON形式のリプレイ情報を取得（旧形式互換）
    fn get_ron_replay_info(&self, path: &Path) -> Option<ReplayFileInfo> {
        let content = fs::read_to_string(path).ok()?;
        let data: ReplayData = ron::from_str(&content).ok()?;

        Some(ReplayFileInfo {
            path: path.to_path_buf(),
            game_version: data.metadata.game_version,
            recorded_at: data.metadata.recorded_at,
            frame_count: data.frames.len(),
        })
    }

    /// バージョン不一致のリプレイを削除
    /// @spec REQ-77103-004
    pub fn cleanup_version_mismatch(&self) -> Result<usize, String> {
        if !self.config.cleanup_policy.delete_on_version_mismatch {
            return Ok(0);
        }

        let replays = self.list_replays()?;
        let current_version = env!("CARGO_PKG_VERSION");
        let mut deleted_count = 0;

        for replay in replays {
            if replay.game_version != current_version {
                if let Err(e) = fs::remove_file(&replay.path) {
                    warn!("Failed to delete replay {:?}: {}", replay.path, e);
                } else {
                    info!("Deleted version-mismatched replay: {:?}", replay.path);
                    deleted_count += 1;
                }
            }
        }

        Ok(deleted_count)
    }

    /// 保存数上限を超えた古いリプレイを削除
    /// @spec REQ-77103-005
    pub fn cleanup_excess_replays(&self) -> Result<usize, String> {
        if !self.config.cleanup_policy.delete_oldest_on_limit {
            return Ok(0);
        }

        let replays = self.list_replays()?;
        let max_count = self.config.file_management.max_replay_count as usize;

        if replays.len() <= max_count {
            return Ok(0);
        }

        let mut deleted_count = 0;
        // replays は新しい順にソートされているので、末尾から削除
        for replay in replays.iter().skip(max_count) {
            if let Err(e) = fs::remove_file(&replay.path) {
                warn!("Failed to delete excess replay {:?}: {}", replay.path, e);
            } else {
                info!("Deleted excess replay: {:?}", replay.path);
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
    }

    /// 起動時のクリーンアップを実行
    pub fn startup_cleanup(&self) -> Result<(), String> {
        info!("Running replay cleanup...");

        let version_deleted = self.cleanup_version_mismatch()?;
        if version_deleted > 0 {
            info!("Deleted {} version-mismatched replays", version_deleted);
        }

        let excess_deleted = self.cleanup_excess_replays()?;
        if excess_deleted > 0 {
            info!("Deleted {} excess replays", excess_deleted);
        }

        Ok(())
    }
}

/// リプレイファイル情報
#[derive(Debug, Clone)]
pub struct ReplayFileInfo {
    /// ファイルパス
    pub path: PathBuf,
    /// ゲームバージョン
    pub game_version: String,
    /// 記録日時
    pub recorded_at: String,
    /// フレーム数
    pub frame_count: usize,
}

/// 起動時クリーンアップシステム
/// @spec REQ-77103-004
pub fn startup_cleanup_system(manager: Res<ReplayManager>) {
    if let Err(e) = manager.startup_cleanup() {
        error!("Replay cleanup failed: {}", e);
    }
}
