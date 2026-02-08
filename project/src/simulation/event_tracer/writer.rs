//! ファイル出力 - CSV/JSON/JSONL形式での出力処理
//! @spec 77100_headless_sim.md

use std::fs::File;
use std::io::{BufWriter, Result as IoResult, Write};
use std::path::Path;

use super::EventTracer;

/// JSON配列の要素を書き出すヘルパー
fn write_json_array<W: Write>(writer: &mut W, items: &[String], indent: &str) -> IoResult<()> {
    for (i, item) in items.iter().enumerate() {
        let comma = if i < items.len() - 1 { "," } else { "" };
        writeln!(writer, "{}{}{}", indent, item, comma)?;
    }
    Ok(())
}

impl EventTracer {
    /// CSV形式でファイルに出力
    pub fn write_csv<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // ヘッダー行
        writeln!(
            writer,
            "frame,timestamp,entity,pos_x,pos_y,pos_z,vel_x,vel_y,vel_z,event_type,event_detail"
        )?;

        for frame in &self.frames {
            // エンティティ行
            for entity in &frame.entities {
                writeln!(
                    writer,
                    "{},{:.3},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},,",
                    frame.frame,
                    frame.timestamp,
                    entity.entity_type.as_str(),
                    entity.position.x,
                    entity.position.y,
                    entity.position.z,
                    entity.velocity.x,
                    entity.velocity.y,
                    entity.velocity.z,
                )?;
            }

            // イベント行
            for event in &frame.events {
                writeln!(
                    writer,
                    "{},{:.3},,,,,,,,{},{}",
                    frame.frame,
                    frame.timestamp,
                    event.type_name(),
                    event.to_csv_detail()
                )?;
            }
        }

        writer.flush()?;
        Ok(())
    }

    /// JSON形式でファイルに出力
    pub fn write_json<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "{{")?;
        writeln!(writer, "  \"frames\": [")?;

        for (i, frame) in self.frames.iter().enumerate() {
            writeln!(writer, "    {{")?;
            writeln!(writer, "      \"frame\": {},", frame.frame)?;
            writeln!(writer, "      \"timestamp\": {:.3},", frame.timestamp)?;

            // entities
            let entities_json: Vec<String> = frame.entities.iter().map(|e| e.to_json()).collect();
            writeln!(writer, "      \"entities\": [")?;
            write_json_array(&mut writer, &entities_json, "        ")?;
            writeln!(writer, "      ],")?;

            // events
            let events_json: Vec<String> = frame.events.iter().map(|e| e.to_json()).collect();
            writeln!(writer, "      \"events\": [")?;
            write_json_array(&mut writer, &events_json, "        ")?;
            writeln!(writer, "      ]")?;

            let comma = if i < self.frames.len() - 1 { "," } else { "" };
            writeln!(writer, "    }}{}", comma)?;
        }

        writeln!(writer, "  ]")?;
        writeln!(writer, "}}")?;

        writer.flush()?;
        Ok(())
    }

    /// JSONL形式でファイルに出力（1フレーム1行）
    pub fn write_jsonl<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        for frame in &self.frames {
            writeln!(writer, "{}", frame.to_json_line())?;
        }

        writer.flush()?;
        Ok(())
    }

    /// 指定されたパスに基づいて適切な形式で出力
    /// - .csv 拡張子: CSV形式
    /// - .json 拡張子: JSON形式
    /// - .jsonl 拡張子: JSONL形式（1フレーム1行）
    /// - それ以外: CSV と JSON の両方出力
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let path = path.as_ref();
        let extension = path.extension().and_then(|e| e.to_str());

        match extension {
            Some("csv") => self.write_csv(path),
            Some("json") => self.write_json(path),
            Some("jsonl") => self.write_jsonl(path),
            _ => {
                // 拡張子なし: CSV と JSON の両方出力
                let csv_path = path.with_extension("csv");
                let json_path = path.with_extension("json");
                self.write_csv(&csv_path)?;
                self.write_json(&json_path)?;
                Ok(())
            }
        }
    }
}
