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
//! ```

// モジュール名の衝突を避けるためにpathアトリビュートを使用
#[path = "trace_narrator/types.rs"]
mod types;
#[path = "trace_narrator/parser.rs"]
mod parser;
#[path = "trace_narrator/analyzer.rs"]
mod analyzer;

use std::path::PathBuf;

use clap::Parser;

use analyzer::analyze_rallies;
use parser::parse_trace_file;
use types::GameEvent;

/// Trace Narrator - テレメトリログ変換ツール
#[derive(Parser, Debug)]
#[command(author, version, about = "Convert telemetry logs to narrative format")]
struct Args {
    /// 入力JSONLファイル
    input: PathBuf,

    /// 出力ファイル（省略時はstdout）
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// 詳細度 (summary/normal/full)
    #[arg(short, long, default_value = "normal")]
    detail_level: String,

    /// 異常検出の閾値倍率
    #[arg(short, long, default_value = "1.5")]
    anomaly_threshold: f32,

    /// 物理詳細を含める
    #[arg(long)]
    include_physics: bool,

    /// ラリー要約のみ出力
    #[arg(long)]
    rally_only: bool,
}

fn main() {
    let args = Args::parse();

    // ファイル読み込み
    println!("Loading trace file: {:?}", args.input);

    let result = match parse_trace_file(&args.input) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // パース結果サマリー
    println!("\n=== Parse Result ===");
    println!("Total frames: {}", result.frames.len());
    println!("Skipped lines: {}", result.skipped_lines);

    if !result.errors.is_empty() {
        println!("\nParse errors:");
        for (line, error) in result.errors.iter().take(5) {
            println!("  Line {}: {}", line, error);
        }
        if result.errors.len() > 5 {
            println!("  ... and {} more errors", result.errors.len() - 5);
        }
    }

    // 基本統計
    let total_events: usize = result.frames.iter().map(|f| f.events.len()).sum();
    let point_events = result
        .frames
        .iter()
        .flat_map(|f| &f.events)
        .filter(|e: &&GameEvent| e.is_point())
        .count();
    let anomaly_events = result
        .frames
        .iter()
        .flat_map(|f| &f.events)
        .filter(|e: &&GameEvent| e.is_anomaly())
        .count();

    println!("\n=== Trace Statistics ===");
    if let (Some(first), Some(last)) = (result.frames.first(), result.frames.last()) {
        println!(
            "Frame range: {} - {} ({} frames)",
            first.frame,
            last.frame,
            result.frames.len()
        );
        println!(
            "Time range: {:.3}s - {:.3}s ({:.3}s duration)",
            first.timestamp,
            last.timestamp,
            last.timestamp - first.timestamp
        );
    }
    println!("Total events: {}", total_events);
    println!("Point events: {}", point_events);
    println!("Anomaly events: {}", anomaly_events);

    // イベント種別の内訳
    let mut event_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for frame in &result.frames {
        for event in &frame.events {
            *event_counts.entry(event.type_name()).or_insert(0) += 1;
        }
    }

    if !event_counts.is_empty() {
        println!("\nEvent breakdown:");
        let mut sorted: Vec<_> = event_counts.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (event_type, count) in sorted {
            println!("  {}: {}", event_type, count);
        }
    }

    // 出力オプションの表示（将来の実装用）
    println!("\n=== Options ===");
    println!("Detail level: {}", args.detail_level);
    println!("Anomaly threshold: {}", args.anomaly_threshold);
    println!("Include physics: {}", args.include_physics);
    println!("Rally only: {}", args.rally_only);
    if let Some(ref output) = args.output {
        println!("Output file: {:?}", output);
    } else {
        println!("Output: stdout");
    }

    // @spec REQ-77201-003, REQ-77201-004, REQ-77201-005, REQ-77201-006: ラリー解析
    println!("\n=== Rally Analysis ===");
    let analysis = analyze_rallies(&result.frames, args.anomaly_threshold);

    println!("Total rallies: {}", analysis.rallies.len());
    println!(
        "Physics anomalies: {}",
        analysis.all_anomalies.len()
    );
    println!(
        "Statistical anomalies: {}",
        analysis.statistical_anomalies.len()
    );

    // ラリー詳細を表示
    if !analysis.rallies.is_empty() {
        println!("\n--- Rally Details ---");
        for rally in &analysis.rallies {
            println!(
                "\nRally {} (Frame {}-{}): P{} wins ({}) - {:.2}s",
                rally.number,
                rally.start_frame,
                rally.end_frame,
                rally.winner,
                rally.end_reason,
                rally.duration_secs
            );
            println!(
                "  Shots: {} (P1: {}, P2: {})",
                rally.stats.shot_count, rally.stats.p1_shot_count, rally.stats.p2_shot_count
            );
            println!("  Bounces: {}, Wall reflects: {}", rally.bounce_count, rally.wall_reflect_count);

            if rally.stats.p1_shot_count > 0 || rally.stats.p2_shot_count > 0 {
                println!(
                    "  P1 avg: power={:.2}, accuracy={:.2}, spin={:.2}",
                    rally.stats.p1_avg_power, rally.stats.p1_avg_accuracy, rally.stats.p1_avg_spin
                );
                println!(
                    "  P2 avg: power={:.2}, accuracy={:.2}, spin={:.2}",
                    rally.stats.p2_avg_power, rally.stats.p2_avg_accuracy, rally.stats.p2_avg_spin
                );
            }

            // ラリー内の異常を表示
            for anomaly in &rally.anomalies {
                println!(
                    "  {} Frame {}: {}",
                    anomaly.severity.emoji(),
                    anomaly.frame,
                    anomaly.description
                );
            }
        }
    }

    // 統計的外れ値を表示
    if !analysis.statistical_anomalies.is_empty() {
        println!("\n--- Statistical Anomalies (threshold: {}σ) ---", args.anomaly_threshold);
        for anomaly in analysis.statistical_anomalies.iter().take(10) {
            println!(
                "  {} Frame {}: {}",
                anomaly.severity.emoji(),
                anomaly.frame,
                anomaly.description
            );
        }
        if analysis.statistical_anomalies.len() > 10 {
            println!("  ... and {} more", analysis.statistical_anomalies.len() - 10);
        }
    }

    println!("\n[Note] Markdown output will be implemented in task 30059");
}
