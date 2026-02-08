//! Trace Narrator CLI
//! @spec 77201_narrative_spec.md
//!
//! JSONLテレメトリログを人間/LLMが読みやすいマークダウン形式に変換するCLIツール。
//!
//! # Usage
//!
//! ```bash
//! # 基本変換（標準出力）
//! cargo run --bin trace_narrator -- input.jsonl
//!
//! # ファイル出力
//! cargo run --bin trace_narrator -- input.jsonl -o output.md
//!
//! # 詳細度指定
//! cargo run --bin trace_narrator -- input.jsonl -d full -o output.md
//! ```

// モジュール名の衝突を避けるためにpathアトリビュートを使用
#[path = "trace_narrator/analyzer.rs"]
mod analyzer;
#[path = "trace_narrator/formatter.rs"]
mod formatter;
#[path = "trace_narrator/parser.rs"]
mod parser;
#[path = "trace_narrator/types.rs"]
mod types;

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::Parser;

use analyzer::analyze_rallies;
use formatter::{format_markdown, DetailLevel, FormatOptions};
use parser::parse_trace_file;

/// Trace Narrator - テレメトリログ変換ツール
/// @spec REQ-77201-007, REQ-77201-008
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Convert telemetry logs to narrative markdown format"
)]
struct Args {
    /// 入力JSONLファイル
    input: PathBuf,

    /// 出力ファイル（省略時はstdout）
    /// @spec REQ-77201-007
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// 詳細度 (summary/normal/full)
    /// @spec REQ-77201-008
    #[arg(short, long, default_value = "normal")]
    detail_level: String,

    /// 異常検出の閾値倍率
    /// @spec REQ-77201-006
    #[arg(short, long, default_value = "1.5")]
    anomaly_threshold: f32,

    /// 物理詳細を含める
    #[arg(long)]
    include_physics: bool,

    /// ラリー要約のみ出力
    #[arg(long)]
    rally_only: bool,

    /// 統計情報を標準エラー出力に表示
    #[arg(long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    // ファイル読み込み
    if args.verbose {
        eprintln!("Loading trace file: {:?}", args.input);
    }

    let result = match parse_trace_file(&args.input) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // パース結果サマリー（verbose モードのみ）
    if args.verbose {
        eprintln!("\n=== Parse Result ===");
        eprintln!("Total frames: {}", result.frames.len());
        eprintln!("Skipped lines: {}", result.skipped_lines);

        if !result.errors.is_empty() {
            eprintln!("\nParse errors:");
            for (line, error) in result.errors.iter().take(5) {
                eprintln!("  Line {}: {}", line, error);
            }
            if result.errors.len() > 5 {
                eprintln!("  ... and {} more errors", result.errors.len() - 5);
            }
        }
    }

    // ラリー解析
    // @spec REQ-77201-003, REQ-77201-004, REQ-77201-005, REQ-77201-006
    let analysis = analyze_rallies(&result.frames, args.anomaly_threshold);

    if args.verbose {
        eprintln!("\n=== Rally Analysis ===");
        eprintln!("Total rallies: {}", analysis.rallies.len());
        eprintln!("Physics anomalies: {}", analysis.all_anomalies.len());
        eprintln!(
            "Statistical anomalies: {}",
            analysis.statistical_anomalies.len()
        );
    }

    // フォーマットオプション構築
    // @spec REQ-77201-008
    let options = FormatOptions {
        detail_level: DetailLevel::from_str(&args.detail_level),
        include_physics: args.include_physics,
        rally_only: args.rally_only,
    };

    // @spec REQ-77201-007: マークダウン出力
    let markdown = format_markdown(&result.frames, &analysis, &options);

    // 出力先に応じて書き込み
    if let Some(ref output_path) = args.output {
        match fs::write(output_path, &markdown) {
            Ok(_) => {
                if args.verbose {
                    eprintln!("\nOutput written to: {:?}", output_path);
                }
            }
            Err(e) => {
                eprintln!("Error writing output file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // stdout に出力
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        if let Err(e) = handle.write_all(markdown.as_bytes()) {
            eprintln!("Error writing to stdout: {}", e);
            std::process::exit(1);
        }
    }
}
