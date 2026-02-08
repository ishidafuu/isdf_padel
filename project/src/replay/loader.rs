#![allow(dead_code)]
//! Replay Loader
//! @spec 77103_replay_spec.md
//!
//! リプレイファイルの読み込み機能。
//! バイナリ形式（.replay）と旧RON形式（.ron）の両方に対応。

use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;

use super::data::{BinaryFrameInput, ReplayData, ReplayMetadata};

/// リプレイファイルのマジックナンバー
const REPLAY_MAGIC: &[u8; 4] = b"RPLY";

/// リプレイファイルを読み込む
/// @spec REQ-77103-006
pub fn load_replay<P: AsRef<Path>>(path: P) -> Result<ReplayData, String> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(format!("Replay file not found: {:?}", path));
    }

    // 拡張子で形式を判別
    let ext = path.extension().and_then(|e| e.to_str());
    let data = match ext {
        Some("replay") => load_binary_replay(path)?,
        Some("ron") => load_ron_replay(path)?,
        _ => return Err(format!("Unknown replay format: {:?}", path)),
    };

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

/// バイナリ形式のリプレイを読み込む
fn load_binary_replay(path: &Path) -> Result<ReplayData, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut reader = BufReader::new(file);

    // ヘッダー読み込み（16バイト）
    let mut header = [0u8; 16];
    reader
        .read_exact(&mut header)
        .map_err(|e| format!("Failed to read header: {}", e))?;

    // マジックナンバー確認
    if &header[0..4] != REPLAY_MAGIC {
        return Err("Invalid replay file: bad magic number".to_string());
    }

    // バージョン確認（将来の拡張用）
    let _version = u16::from_le_bytes([header[4], header[5]]);

    // フレーム数
    let frame_count = u32::from_le_bytes([header[8], header[9], header[10], header[11]]);

    // メタデータサイズ
    let metadata_size =
        u32::from_le_bytes([header[12], header[13], header[14], header[15]]) as usize;

    // メタデータ読み込み
    let mut metadata_bytes = vec![0u8; metadata_size];
    reader
        .read_exact(&mut metadata_bytes)
        .map_err(|e| format!("Failed to read metadata: {}", e))?;
    let metadata: ReplayMetadata = bincode::deserialize(&metadata_bytes)
        .map_err(|e| format!("Failed to deserialize metadata: {}", e))?;

    // フレームデータ読み込み
    let mut frames = Vec::with_capacity(frame_count as usize);
    let mut frame_buf = [0u8; 6];

    for i in 0..frame_count {
        reader
            .read_exact(&mut frame_buf)
            .map_err(|e| format!("Failed to read frame {}: {}", i, e))?;
        let binary_frame = BinaryFrameInput::read_from(&frame_buf);
        frames.push(binary_frame.to_frame_input(i));
    }

    Ok(ReplayData { metadata, frames })
}

/// RON形式のリプレイを読み込む（旧形式互換）
fn load_ron_replay(path: &Path) -> Result<ReplayData, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read replay file: {}", e))?;

    ron::from_str(&content).map_err(|e| format!("Failed to parse replay: {}", e))
}

/// リプレイファイルを読み込む（バージョンチェックなし）
/// デバッグ用
pub fn load_replay_unchecked<P: AsRef<Path>>(path: P) -> Result<ReplayData, String> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(format!("Replay file not found: {:?}", path));
    }

    // 拡張子で形式を判別
    let ext = path.extension().and_then(|e| e.to_str());
    match ext {
        Some("replay") => load_binary_replay(path),
        Some("ron") => load_ron_replay(path),
        _ => Err(format!("Unknown replay format: {:?}", path)),
    }
}
