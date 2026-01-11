use std::fs;
use std::path::Path;

fn main() {
    // リプレイフォルダのクリア
    let replay_dir = Path::new("assets/replays");
    if replay_dir.exists() {
        if let Ok(entries) = fs::read_dir(replay_dir) {
            for entry in entries.flatten() {
                let _ = fs::remove_file(entry.path());
            }
        }
    }

    // リプレイフォルダが変更されたら再実行
    println!("cargo:rerun-if-changed=assets/replays");
}
