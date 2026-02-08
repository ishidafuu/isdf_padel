//! デバッグ制御用の外部設定
//! @spec 77210_debug_control.md

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;

use crate::resource::config::GameConfig;

/// 実行中上書き設定ファイル
/// @spec 77210_debug_control.md#req-77210-001
pub const DEBUG_RUNTIME_CONFIG_PATH: &str = "assets/config/debug_runtime.ron";
/// 起動時環境変数プロファイルファイル
/// @spec 77210_debug_control.md#req-77210-003
#[allow(dead_code)]
pub const DEBUG_ENV_CONFIG_PATH: &str = "assets/config/debug_env.ron";

const ENV_OVERRIDES_ENABLED: &str = "PADEL_DEBUG_OVERRIDES_ENABLED";
const ENV_PRACTICE_INFINITE_MODE: &str = "PADEL_PRACTICE_INFINITE_MODE";
const ENV_PLAYER_MOVE_SPEED: &str = "PADEL_PLAYER_MOVE_SPEED";
const ENV_PLAYER_MOVE_SPEED_Z: &str = "PADEL_PLAYER_MOVE_SPEED_Z";
const ENV_BALL_NORMAL_SHOT_SPEED: &str = "PADEL_BALL_NORMAL_SHOT_SPEED";
const ENV_BALL_POWER_SHOT_SPEED: &str = "PADEL_BALL_POWER_SHOT_SPEED";
const ENV_SERVE_SPEED: &str = "PADEL_SERVE_SPEED";
const ENV_GRAVITY: &str = "PADEL_GRAVITY";

/// 実行中上書き設定
/// @spec 77210_debug_control.md#req-77210-001
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(default)]
pub struct DebugRuntimeOverrides {
    /// false の場合は値が入っていても無効化
    pub enabled: bool,
    pub practice_infinite_mode: Option<bool>,
    pub player_move_speed: Option<f32>,
    pub player_move_speed_z: Option<f32>,
    pub ball_normal_shot_speed: Option<f32>,
    pub ball_power_shot_speed: Option<f32>,
    pub serve_speed: Option<f32>,
    pub gravity: Option<f32>,
}

impl Default for DebugRuntimeOverrides {
    fn default() -> Self {
        Self {
            enabled: false,
            practice_infinite_mode: None,
            player_move_speed: None,
            player_move_speed_z: None,
            ball_normal_shot_speed: None,
            ball_power_shot_speed: None,
            serve_speed: None,
            gravity: None,
        }
    }
}

impl DebugRuntimeOverrides {
    /// 起動時環境変数から上書き設定を生成
    /// @spec 77210_debug_control.md#req-77210-002
    pub fn from_env() -> Self {
        let mut overrides = Self {
            enabled: false,
            practice_infinite_mode: parse_bool_env(ENV_PRACTICE_INFINITE_MODE),
            player_move_speed: parse_f32_env(ENV_PLAYER_MOVE_SPEED),
            player_move_speed_z: parse_f32_env(ENV_PLAYER_MOVE_SPEED_Z),
            ball_normal_shot_speed: parse_f32_env(ENV_BALL_NORMAL_SHOT_SPEED),
            ball_power_shot_speed: parse_f32_env(ENV_BALL_POWER_SHOT_SPEED),
            serve_speed: parse_f32_env(ENV_SERVE_SPEED),
            gravity: parse_f32_env(ENV_GRAVITY),
        };

        let enabled_explicit = parse_bool_env(ENV_OVERRIDES_ENABLED);
        overrides.enabled = enabled_explicit.unwrap_or(overrides.has_any_value());
        overrides
    }

    #[inline]
    pub fn has_any_value(&self) -> bool {
        self.practice_infinite_mode.is_some()
            || self.player_move_speed.is_some()
            || self.player_move_speed_z.is_some()
            || self.ball_normal_shot_speed.is_some()
            || self.ball_power_shot_speed.is_some()
            || self.serve_speed.is_some()
            || self.gravity.is_some()
    }

    /// GameConfig に上書き適用
    /// @spec 77210_debug_control.md#req-77210-004
    pub fn apply_to_game_config(&self, config: &mut GameConfig) {
        if !self.enabled {
            return;
        }

        if let Some(v) = self.practice_infinite_mode {
            config.serve.practice_infinite_mode = v;
        }
        if let Some(v) = self.player_move_speed {
            config.player.move_speed = v;
        }
        if let Some(v) = self.player_move_speed_z {
            config.player.move_speed_z = v;
        }
        if let Some(v) = self.ball_normal_shot_speed {
            config.ball.normal_shot_speed = v;
        }
        if let Some(v) = self.ball_power_shot_speed {
            config.ball.power_shot_speed = v;
        }
        if let Some(v) = self.serve_speed {
            config.serve.serve_speed = v;
        }
        if let Some(v) = self.gravity {
            config.physics.gravity = v;
        }
    }
}

/// 起動時環境変数プロファイル
/// @spec 77210_debug_control.md#req-77210-003
#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(default)]
#[allow(dead_code)]
pub struct DebugEnvProfile {
    pub vars: BTreeMap<String, String>,
}

/// 実効設定を合成（ベース + 起動時環境変数 + 実行中上書き）
/// @spec 77210_debug_control.md#req-77210-004
pub fn compose_effective_config(
    base: &GameConfig,
    startup_env: &DebugRuntimeOverrides,
    runtime: &DebugRuntimeOverrides,
) -> GameConfig {
    let mut effective = base.clone();
    startup_env.apply_to_game_config(&mut effective);
    runtime.apply_to_game_config(&mut effective);
    effective
}

/// 実行中上書き設定を読み込む
pub fn load_runtime_overrides<P: AsRef<Path>>(path: P) -> Result<DebugRuntimeOverrides, String> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(DebugRuntimeOverrides::default());
    }

    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read runtime overrides file: {}", e))?;
    ron::from_str(&content).map_err(|e| format!("Failed to parse runtime overrides RON: {}", e))
}

/// 実行中上書き設定を書き込む
pub fn save_runtime_overrides<P: AsRef<Path>>(
    path: P,
    overrides: &DebugRuntimeOverrides,
) -> Result<(), String> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create parent directories for {}: {}",
                path.display(),
                e
            )
        })?;
    }

    let pretty = ron::ser::PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true);
    let payload = ron::ser::to_string_pretty(overrides, pretty)
        .map_err(|e| format!("Failed to serialize runtime overrides: {}", e))?;
    fs::write(path, payload).map_err(|e| {
        format!(
            "Failed to write runtime overrides file {}: {}",
            path.display(),
            e
        )
    })
}

/// 起動時環境変数プロファイルを読み込む
#[allow(dead_code)]
pub fn load_env_profile<P: AsRef<Path>>(path: P) -> Result<DebugEnvProfile, String> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(DebugEnvProfile::default());
    }

    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read env profile file: {}", e))?;
    ron::from_str(&content).map_err(|e| format!("Failed to parse env profile RON: {}", e))
}

/// 起動時環境変数プロファイルを書き込む
#[allow(dead_code)]
pub fn save_env_profile<P: AsRef<Path>>(path: P, profile: &DebugEnvProfile) -> Result<(), String> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create parent directories for {}: {}",
                path.display(),
                e
            )
        })?;
    }

    let pretty = ron::ser::PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true);
    let payload = ron::ser::to_string_pretty(profile, pretty)
        .map_err(|e| format!("Failed to serialize env profile: {}", e))?;
    fs::write(path, payload)
        .map_err(|e| format!("Failed to write env profile file {}: {}", path.display(), e))
}

/// ファイルが存在しない場合にデフォルト内容で作成
#[allow(dead_code)]
pub fn ensure_runtime_overrides_file<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let path = path.as_ref();
    if path.exists() {
        return Ok(());
    }
    save_runtime_overrides(path, &DebugRuntimeOverrides::default())
}

/// ファイルが存在しない場合にデフォルト内容で作成
#[allow(dead_code)]
pub fn ensure_env_profile_file<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let path = path.as_ref();
    if path.exists() {
        return Ok(());
    }
    save_env_profile(path, &DebugEnvProfile::default())
}

fn parse_bool_env(key: &str) -> Option<bool> {
    let raw = env::var(key).ok()?;
    parse_bool_str(&raw)
}

fn parse_f32_env(key: &str) -> Option<f32> {
    let raw = env::var(key).ok()?;
    raw.trim().parse::<f32>().ok()
}

fn parse_bool_str(raw: &str) -> Option<bool> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "on" | "yes" => Some(true),
        "0" | "false" | "off" | "no" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::config::load_game_config;
    use tempfile::tempdir;

    #[test]
    fn test_parse_bool_str() {
        assert_eq!(parse_bool_str("true"), Some(true));
        assert_eq!(parse_bool_str("1"), Some(true));
        assert_eq!(parse_bool_str("on"), Some(true));
        assert_eq!(parse_bool_str("false"), Some(false));
        assert_eq!(parse_bool_str("0"), Some(false));
        assert_eq!(parse_bool_str("off"), Some(false));
        assert_eq!(parse_bool_str("maybe"), None);
    }

    #[test]
    fn test_runtime_overrides_roundtrip() {
        let dir = tempdir().expect("tempdir create failed");
        let path = dir.path().join("debug_runtime.ron");

        let expected = DebugRuntimeOverrides {
            enabled: true,
            practice_infinite_mode: Some(false),
            player_move_speed: Some(4.2),
            player_move_speed_z: Some(3.1),
            ball_normal_shot_speed: Some(6.0),
            ball_power_shot_speed: Some(8.0),
            serve_speed: Some(9.0),
            gravity: Some(-3.5),
        };

        save_runtime_overrides(&path, &expected).expect("save runtime overrides failed");
        let actual = load_runtime_overrides(&path).expect("load runtime overrides failed");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_apply_to_game_config() {
        let mut config =
            load_game_config("assets/config/game_config.ron").expect("game_config load failed");
        let overrides = DebugRuntimeOverrides {
            enabled: true,
            practice_infinite_mode: Some(false),
            player_move_speed: Some(7.5),
            player_move_speed_z: Some(6.5),
            ball_normal_shot_speed: Some(9.1),
            ball_power_shot_speed: Some(11.2),
            serve_speed: Some(12.3),
            gravity: Some(-5.0),
        };

        overrides.apply_to_game_config(&mut config);

        assert!(!config.serve.practice_infinite_mode);
        assert_eq!(config.player.move_speed, 7.5);
        assert_eq!(config.player.move_speed_z, 6.5);
        assert_eq!(config.ball.normal_shot_speed, 9.1);
        assert_eq!(config.ball.power_shot_speed, 11.2);
        assert_eq!(config.serve.serve_speed, 12.3);
        assert_eq!(config.physics.gravity, -5.0);
    }
}
