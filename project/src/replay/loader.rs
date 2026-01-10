#![allow(dead_code)]
//! Replay Loader
//! @spec 77103_replay_spec.md
//!
//! リプレイファイルの読み込み機能。

use std::fs;
use std::path::Path;

use super::data::ReplayData;

/// リプレイファイルを読み込む
/// @spec REQ-77103-006
pub fn load_replay<P: AsRef<Path>>(path: P) -> Result<ReplayData, String> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(format!("Replay file not found: {:?}", path));
    }

    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read replay file: {}", e))?;

    let data: ReplayData =
        ron::from_str(&content).map_err(|e| format!("Failed to parse replay: {}", e))?;

    // バージョンチェック
    let current_version = env!("CARGO_PKG_VERSION");
    if data.metadata.game_version != current_version {
        return Err(format!(
            "Version mismatch: replay={}, current={}",
            data.metadata.game_version, current_version
        ));
    }

    Ok(data)
}

/// リプレイファイルを読み込む（バージョンチェックなし）
/// デバッグ用
pub fn load_replay_unchecked<P: AsRef<Path>>(path: P) -> Result<ReplayData, String> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(format!("Replay file not found: {:?}", path));
    }

    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read replay file: {}", e))?;

    ron::from_str(&content).map_err(|e| format!("Failed to parse replay: {}", e))
}
