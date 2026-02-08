//! Debug Control CLI
//! @spec 77210_debug_control.md

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::env;
use std::path::Path;
use std::process::{self, Command};

use padel_game::resource::{
    compose_effective_config, ensure_env_profile_file, ensure_runtime_overrides_file,
    load_env_profile, load_game_config, load_runtime_overrides, save_env_profile,
    save_runtime_overrides, DebugRuntimeOverrides, DEBUG_ENV_CONFIG_PATH,
    DEBUG_RUNTIME_CONFIG_PATH,
};

#[derive(Parser, Debug)]
#[command(author, version, about = "Padelデバッグ制御CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 現在のデバッグ状況を表示
    Status,
    /// 実行中上書き設定を更新
    Runtime(RuntimeArgs),
    /// 起動時環境変数プロファイルを更新
    Env(EnvArgs),
    /// 設定済み環境変数を適用してゲームを起動
    Launch(LaunchArgs),
    /// 別窓エディタで設定ファイルを開く
    OpenEditor(OpenEditorArgs),
}

#[derive(Args, Debug)]
struct RuntimeArgs {
    /// 既存値をいったん全クリア
    #[arg(long)]
    clear: bool,
    /// 上書き全体の有効/無効
    #[arg(long)]
    enabled: Option<bool>,
    /// サーブ無限化
    #[arg(long)]
    practice_infinite_mode: Option<bool>,
    /// プレイヤーX移動速度
    #[arg(long)]
    player_move_speed: Option<f32>,
    /// プレイヤーZ移動速度
    #[arg(long)]
    player_move_speed_z: Option<f32>,
    /// 通常ショット速度
    #[arg(long)]
    ball_normal_shot_speed: Option<f32>,
    /// 強打ショット速度
    #[arg(long)]
    ball_power_shot_speed: Option<f32>,
    /// サーブ速度
    #[arg(long)]
    serve_speed: Option<f32>,
    /// 重力
    #[arg(long)]
    gravity: Option<f32>,
}

#[derive(Args, Debug)]
struct EnvArgs {
    /// KEY=VALUE を設定（複数指定可）
    #[arg(long = "set", value_name = "KEY=VALUE")]
    set: Vec<String>,
    /// KEY を削除（複数指定可）
    #[arg(long = "unset", value_name = "KEY")]
    unset: Vec<String>,
    /// 既存値を全クリア
    #[arg(long)]
    clear: bool,
}

#[derive(Args, Debug)]
struct LaunchArgs {
    /// cargo run --release で起動
    #[arg(long)]
    release: bool,
    /// 起動時に追加する環境変数（保存はしない）
    #[arg(long = "env", value_name = "KEY=VALUE")]
    env: Vec<String>,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum EditorTarget {
    Runtime,
    Env,
}

#[derive(Args, Debug)]
struct OpenEditorArgs {
    /// 開く対象
    #[arg(long, value_enum, default_value_t = EditorTarget::Runtime)]
    target: EditorTarget,
    /// 使用するエディタコマンド（例: "code -n", "zed")
    #[arg(long)]
    editor: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command.unwrap_or(Commands::Status) {
        Commands::Status => run_status(),
        Commands::Runtime(args) => run_runtime(args),
        Commands::Env(args) => run_env(args),
        Commands::Launch(args) => run_launch(args),
        Commands::OpenEditor(args) => run_open_editor(args),
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn run_status() -> Result<(), String> {
    let base = load_game_config("assets/config/game_config.ron")?;
    let startup_env = DebugRuntimeOverrides::from_env();
    let runtime = load_runtime_overrides(DEBUG_RUNTIME_CONFIG_PATH)?;
    let env_profile = load_env_profile(DEBUG_ENV_CONFIG_PATH)?;

    let effective = compose_effective_config(&base, &startup_env, &runtime);

    println!("=== Debug Status ===");
    println!("runtime file: {}", DEBUG_RUNTIME_CONFIG_PATH);
    println!("env file: {}", DEBUG_ENV_CONFIG_PATH);
    println!();

    println!("[Runtime Overrides]");
    println!("enabled: {}", runtime.enabled);
    print_optional_bool("practice_infinite_mode", runtime.practice_infinite_mode);
    print_optional_f32("player_move_speed", runtime.player_move_speed);
    print_optional_f32("player_move_speed_z", runtime.player_move_speed_z);
    print_optional_f32("ball_normal_shot_speed", runtime.ball_normal_shot_speed);
    print_optional_f32("ball_power_shot_speed", runtime.ball_power_shot_speed);
    print_optional_f32("serve_speed", runtime.serve_speed);
    print_optional_f32("gravity", runtime.gravity);
    println!();

    println!("[Startup Env Overrides (this process)]");
    println!("enabled: {}", startup_env.enabled);
    print_optional_bool("practice_infinite_mode", startup_env.practice_infinite_mode);
    print_optional_f32("player_move_speed", startup_env.player_move_speed);
    print_optional_f32("player_move_speed_z", startup_env.player_move_speed_z);
    print_optional_f32("ball_normal_shot_speed", startup_env.ball_normal_shot_speed);
    print_optional_f32("ball_power_shot_speed", startup_env.ball_power_shot_speed);
    print_optional_f32("serve_speed", startup_env.serve_speed);
    print_optional_f32("gravity", startup_env.gravity);
    println!();

    println!("[Launch Env Profile]");
    if env_profile.vars.is_empty() {
        println!("(empty)");
    } else {
        for (k, v) in &env_profile.vars {
            println!("{}={}", k, v);
        }
    }
    println!();

    println!("[Effective Game Values]");
    println!(
        "serve.practice_infinite_mode: {}",
        effective.serve.practice_infinite_mode
    );
    println!("player.move_speed: {:.3}", effective.player.move_speed);
    println!("player.move_speed_z: {:.3}", effective.player.move_speed_z);
    println!(
        "ball.normal_shot_speed: {:.3}",
        effective.ball.normal_shot_speed
    );
    println!(
        "ball.power_shot_speed: {:.3}",
        effective.ball.power_shot_speed
    );
    println!("serve.serve_speed: {:.3}", effective.serve.serve_speed);
    println!("physics.gravity: {:.3}", effective.physics.gravity);

    Ok(())
}

fn run_runtime(args: RuntimeArgs) -> Result<(), String> {
    let mut runtime = load_runtime_overrides(DEBUG_RUNTIME_CONFIG_PATH)?;

    if args.clear {
        runtime = DebugRuntimeOverrides::default();
    }

    let has_new_values = args.practice_infinite_mode.is_some()
        || args.player_move_speed.is_some()
        || args.player_move_speed_z.is_some()
        || args.ball_normal_shot_speed.is_some()
        || args.ball_power_shot_speed.is_some()
        || args.serve_speed.is_some()
        || args.gravity.is_some();

    if let Some(v) = args.enabled {
        runtime.enabled = v;
    } else if has_new_values {
        // 値指定があるときは自動的に有効化（明示無効化を避ける）
        runtime.enabled = true;
    }

    if let Some(v) = args.practice_infinite_mode {
        runtime.practice_infinite_mode = Some(v);
    }
    if let Some(v) = args.player_move_speed {
        runtime.player_move_speed = Some(v);
    }
    if let Some(v) = args.player_move_speed_z {
        runtime.player_move_speed_z = Some(v);
    }
    if let Some(v) = args.ball_normal_shot_speed {
        runtime.ball_normal_shot_speed = Some(v);
    }
    if let Some(v) = args.ball_power_shot_speed {
        runtime.ball_power_shot_speed = Some(v);
    }
    if let Some(v) = args.serve_speed {
        runtime.serve_speed = Some(v);
    }
    if let Some(v) = args.gravity {
        runtime.gravity = Some(v);
    }

    save_runtime_overrides(DEBUG_RUNTIME_CONFIG_PATH, &runtime)?;
    println!("Updated {}", DEBUG_RUNTIME_CONFIG_PATH);
    run_status()
}

fn run_env(args: EnvArgs) -> Result<(), String> {
    let mut profile = load_env_profile(DEBUG_ENV_CONFIG_PATH)?;

    if args.clear {
        profile.vars.clear();
    }

    for key in args.unset {
        profile.vars.remove(&key);
    }

    for item in args.set {
        let (k, v) = parse_env_assignment(&item)?;
        profile.vars.insert(k, v);
    }

    save_env_profile(DEBUG_ENV_CONFIG_PATH, &profile)?;
    println!("Updated {}", DEBUG_ENV_CONFIG_PATH);
    run_status()
}

fn run_launch(args: LaunchArgs) -> Result<(), String> {
    let profile = load_env_profile(DEBUG_ENV_CONFIG_PATH)?;

    let mut cmd = Command::new("cargo");
    cmd.arg("run");
    if args.release {
        cmd.arg("--release");
    }
    cmd.arg("--bin").arg("padel_game");

    for (k, v) in &profile.vars {
        cmd.env(k, v);
    }

    for item in args.env {
        let (k, v) = parse_env_assignment(&item)?;
        cmd.env(k, v);
    }

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute cargo run: {}", e))?;
    if !status.success() {
        return Err(format!("padel_game exited with status {}", status));
    }
    Ok(())
}

fn run_open_editor(args: OpenEditorArgs) -> Result<(), String> {
    let target_path = match args.target {
        EditorTarget::Runtime => Path::new(DEBUG_RUNTIME_CONFIG_PATH),
        EditorTarget::Env => Path::new(DEBUG_ENV_CONFIG_PATH),
    };

    match args.target {
        EditorTarget::Runtime => ensure_runtime_overrides_file(target_path)?,
        EditorTarget::Env => ensure_env_profile_file(target_path)?,
    }

    let editor = args
        .editor
        .or_else(|| env::var("PADEL_DEBUG_EDITOR").ok())
        .or_else(|| env::var("EDITOR").ok())
        .unwrap_or_else(|| "code -n".to_string());

    let escaped = target_path.to_string_lossy().replace('"', "\\\"");
    let command_line = format!("{} \"{}\"", editor, escaped);
    let status = Command::new("sh")
        .arg("-lc")
        .arg(command_line)
        .status()
        .map_err(|e| format!("Failed to launch editor: {}", e))?;
    if !status.success() {
        return Err(format!("Editor command exited with status {}", status));
    }

    println!("Opened {}", target_path.display());
    Ok(())
}

fn parse_env_assignment(raw: &str) -> Result<(String, String), String> {
    let mut parts = raw.splitn(2, '=');
    let key = parts
        .next()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("Invalid env assignment '{}'", raw))?;
    let value = parts
        .next()
        .ok_or_else(|| format!("Missing '=' in env assignment '{}'", raw))?;
    Ok((key.to_string(), value.to_string()))
}

fn print_optional_bool(label: &str, value: Option<bool>) {
    match value {
        Some(v) => println!("{}: {}", label, v),
        None => println!("{}: (none)", label),
    }
}

fn print_optional_f32(label: &str, value: Option<f32>) {
    match value {
        Some(v) => println!("{}: {:.3}", label, v),
        None => println!("{}: (none)", label),
    }
}
