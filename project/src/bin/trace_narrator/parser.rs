//! Trace File Parser
//! @spec 77201_narrative_spec.md REQ-77201-001, REQ-77201-002
//!
//! JSONL/JSON形式のテレメトリログを読み込み、FrameTraceに変換する。

use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::types::{FrameTrace, TraceFile};

/// パースエラー
#[derive(Debug)]
pub enum ParseError {
    /// ファイル読み込みエラー
    IoError(std::io::Error),
    /// JSON解析エラー
    JsonError(serde_json::Error),
    /// 空ファイル
    EmptyFile,
    /// フォーマット不明
    UnknownFormat,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::IoError(e) => write!(f, "IO error: {}", e),
            ParseError::JsonError(e) => write!(f, "JSON parse error: {}", e),
            ParseError::EmptyFile => write!(f, "Empty file"),
            ParseError::UnknownFormat => write!(f, "Unknown file format"),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> Self {
        ParseError::IoError(e)
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(e: serde_json::Error) -> Self {
        ParseError::JsonError(e)
    }
}

/// パース結果
pub struct ParseResult {
    /// パースされたフレーム
    pub frames: Vec<FrameTrace>,
    /// スキップされた行数（不正な行）
    pub skipped_lines: usize,
    /// パースエラー情報（行番号 -> エラー）
    pub errors: Vec<(usize, String)>,
}

/// トレースファイルをパース
///
/// @spec REQ-77201-001: JSONL読み込み
/// - 1行ずつJSONをパース
/// - 不正な行はスキップしてログ出力
/// - 大容量ファイルでもメモリ効率的に処理
///
/// @spec REQ-77201-002: フレームトレース構造解析
/// - frame, timestamp でソート
pub fn parse_trace_file<P: AsRef<Path>>(path: P) -> Result<ParseResult, ParseError> {
    let mut file = File::open(&path)?;

    // ファイル全体を読み込んで形式を判定
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if content.trim().is_empty() {
        return Err(ParseError::EmptyFile);
    }

    // まずJSON配列形式（TraceFile）としてパースを試みる
    if let Ok(trace_file) = serde_json::from_str::<TraceFile>(&content) {
        let mut frames = trace_file.frames;
        sort_frames(&mut frames);
        return Ok(ParseResult {
            frames,
            skipped_lines: 0,
            errors: Vec::new(),
        });
    }

    // JSON配列形式でなければJSONL形式として処理
    parse_jsonl_content(&content)
}

/// JSONL形式のコンテンツをパース
///
/// @spec REQ-77201-001: 不正な行はスキップしてログ出力
fn parse_jsonl_content(content: &str) -> Result<ParseResult, ParseError> {
    let mut frames = Vec::new();
    let mut skipped_lines = 0;
    let mut errors = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let actual_line_num = line_num + 1; // 1-indexed
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue; // 空行はスキップ
        }

        match serde_json::from_str::<FrameTrace>(trimmed) {
            Ok(frame) => frames.push(frame),
            Err(e) => {
                skipped_lines += 1;
                errors.push((actual_line_num, e.to_string()));
            }
        }
    }

    if frames.is_empty() {
        return Err(ParseError::EmptyFile);
    }

    // @spec REQ-77201-002: frame, timestampでソート
    sort_frames(&mut frames);

    Ok(ParseResult {
        frames,
        skipped_lines,
        errors,
    })
}

/// フレームをframe番号とtimestampでソート
/// @spec REQ-77201-002: イベントを時系列で整理
fn sort_frames(frames: &mut [FrameTrace]) {
    frames.sort_by(|a, b| {
        a.frame.cmp(&b.frame).then_with(|| {
            a.timestamp
                .partial_cmp(&b.timestamp)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_json_array_format() {
        let content = r#"{
            "frames": [
                {"frame": 0, "timestamp": 0.0, "entities": [], "events": []},
                {"frame": 1, "timestamp": 0.016, "entities": [], "events": []}
            ]
        }"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_trace_file(file.path()).unwrap();
        assert_eq!(result.frames.len(), 2);
        assert_eq!(result.skipped_lines, 0);
    }

    #[test]
    fn test_parse_jsonl_format() {
        let content = r#"{"frame": 0, "timestamp": 0.0, "entities": [], "events": []}
{"frame": 1, "timestamp": 0.016, "entities": [], "events": []}"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_trace_file(file.path()).unwrap();
        assert_eq!(result.frames.len(), 2);
        assert_eq!(result.skipped_lines, 0);
    }

    #[test]
    fn test_frames_are_sorted() {
        let content = r#"{"frame": 2, "timestamp": 0.032, "entities": [], "events": []}
{"frame": 0, "timestamp": 0.0, "entities": [], "events": []}
{"frame": 1, "timestamp": 0.016, "entities": [], "events": []}"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_trace_file(file.path()).unwrap();
        assert_eq!(result.frames[0].frame, 0);
        assert_eq!(result.frames[1].frame, 1);
        assert_eq!(result.frames[2].frame, 2);
    }

    #[test]
    fn test_skip_invalid_lines() {
        let content = r#"{"frame": 0, "timestamp": 0.0, "entities": [], "events": []}
invalid json line
{"frame": 1, "timestamp": 0.016, "entities": [], "events": []}"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let result = parse_trace_file(file.path()).unwrap();
        assert_eq!(result.frames.len(), 2);
        assert_eq!(result.skipped_lines, 1);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].0, 2); // 2行目がエラー
    }
}
