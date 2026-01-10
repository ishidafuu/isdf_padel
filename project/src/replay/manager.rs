//! Replay Manager
//! @spec 77103_replay_spec.md
//!
//! リプレイファイルの保存、読み込み、管理を行う。

use bevy::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

use super::data::{ReplayConfig, ReplayData};

/// リプレイマネージャーリソース
/// @spec REQ-77103-003, REQ-77103-004, REQ-77103-005
#[derive(Resource)]
pub struct ReplayManager {
    config: ReplayConfig,
}

impl Default for ReplayManager {
    fn default() -> Self {
        Self {
            config: ReplayConfig::default(),
        }
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

    /// リプレイを保存
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
            "{}{}.ron",
            self.config.file_management.file_prefix, timestamp
        );
        let filepath = save_dir.join(&filename);

        // RON形式でシリアライズ
        let content = ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::default())
            .map_err(|e| format!("Failed to serialize replay: {}", e))?;

        // ファイルに書き込み
        fs::write(&filepath, content)
            .map_err(|e| format!("Failed to write replay file: {}", e))?;

        info!("Replay saved: {:?}", filepath);
        Ok(filepath)
    }

    /// リプレイを読み込み
    /// @spec REQ-77103-006
    pub fn load_replay<P: AsRef<Path>>(&self, path: P) -> Result<ReplayData, String> {
        let path = path.as_ref();

        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read replay file: {}", e))?;

        let data: ReplayData =
            ron::from_str(&content).map_err(|e| format!("Failed to parse replay: {}", e))?;

        // バージョンチェック
        if !data.metadata.is_version_compatible() {
            return Err(format!(
                "Version mismatch: replay={}, current={}",
                data.metadata.game_version,
                env!("CARGO_PKG_VERSION")
            ));
        }

        Ok(data)
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

            // .ron ファイルのみ
            if path.extension().map_or(false, |ext| ext == "ron") {
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
