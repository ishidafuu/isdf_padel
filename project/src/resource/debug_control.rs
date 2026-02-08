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

pub const SUPPORTED_OVERRIDE_KEYS: &[&str] = &[
    "serve.practice_infinite_mode",
    "physics.gravity",
    "physics.max_fall_speed",
    "player.move_speed",
    "player.move_speed_z",
    "player.max_speed",
    "player.jump_force",
    "ball.normal_shot_speed",
    "ball.power_shot_speed",
    "ball.bounce_factor",
    "ball.radius",
    "ball.min_bounce_velocity",
    "ball.wall_bounce_factor",
    "serve.serve_speed",
    "serve.serve_angle",
    "serve.toss_velocity_y",
    "serve.toss_velocity_min_y",
    "serve.toss_velocity_max_y",
    "serve.toss_hold_max_secs",
    "serve.toss_depth_shift",
    "serve.toss_launch_angle_bonus_deg",
    "serve.toss_timeout",
    "serve.hit_height_min",
    "serve.hit_height_max",
    "serve.hit_height_optimal",
    "serve.ai_hit_tolerance",
    "shot.max_distance",
    "shot.cooldown_time",
    "shot.jump_threshold",
    "ai.move_speed",
    "ai.shot_cooldown",
    "ai.prediction_accuracy",
    "ai.prediction_error",
    "ai.direction_variance",
    "ai.reaction_delay",
    "ai.offensive_probability",
    "ai.serve_offensive_probability",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DebugOverrideValue {
    Bool(bool),
    Float(f32),
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebugOverrideType {
    Bool,
    Float,
}

/// 実行中上書き設定
/// @spec 77210_debug_control.md#req-77210-001
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(default)]
pub struct DebugRuntimeOverrides {
    /// false の場合は値が入っていても無効化
    pub enabled: bool,

    // === bool ===
    pub practice_infinite_mode: Option<bool>,

    // === physics ===
    pub gravity: Option<f32>,
    pub physics_max_fall_speed: Option<f32>,

    // === player ===
    pub player_move_speed: Option<f32>,
    pub player_move_speed_z: Option<f32>,
    pub player_max_speed: Option<f32>,
    pub player_jump_force: Option<f32>,

    // === ball ===
    pub ball_normal_shot_speed: Option<f32>,
    pub ball_power_shot_speed: Option<f32>,
    pub ball_bounce_factor: Option<f32>,
    pub ball_radius: Option<f32>,
    pub ball_min_bounce_velocity: Option<f32>,
    pub ball_wall_bounce_factor: Option<f32>,

    // === serve ===
    pub serve_speed: Option<f32>,
    pub serve_angle: Option<f32>,
    pub toss_velocity_y: Option<f32>,
    pub toss_velocity_min_y: Option<f32>,
    pub toss_velocity_max_y: Option<f32>,
    pub toss_hold_max_secs: Option<f32>,
    pub toss_depth_shift: Option<f32>,
    pub toss_launch_angle_bonus_deg: Option<f32>,
    pub toss_timeout: Option<f32>,
    pub hit_height_min: Option<f32>,
    pub hit_height_max: Option<f32>,
    pub hit_height_optimal: Option<f32>,
    pub ai_hit_tolerance: Option<f32>,

    // === shot ===
    pub shot_max_distance: Option<f32>,
    pub shot_cooldown_time: Option<f32>,
    pub shot_jump_threshold: Option<f32>,

    // === ai ===
    pub ai_move_speed: Option<f32>,
    pub ai_shot_cooldown: Option<f32>,
    pub ai_prediction_accuracy: Option<f32>,
    pub ai_prediction_error: Option<f32>,
    pub ai_direction_variance: Option<f32>,
    pub ai_reaction_delay: Option<f32>,
    pub ai_offensive_probability: Option<f32>,
    pub ai_serve_offensive_probability: Option<f32>,
}

impl Default for DebugRuntimeOverrides {
    fn default() -> Self {
        Self {
            enabled: false,
            practice_infinite_mode: None,
            gravity: None,
            physics_max_fall_speed: None,
            player_move_speed: None,
            player_move_speed_z: None,
            player_max_speed: None,
            player_jump_force: None,
            ball_normal_shot_speed: None,
            ball_power_shot_speed: None,
            ball_bounce_factor: None,
            ball_radius: None,
            ball_min_bounce_velocity: None,
            ball_wall_bounce_factor: None,
            serve_speed: None,
            serve_angle: None,
            toss_velocity_y: None,
            toss_velocity_min_y: None,
            toss_velocity_max_y: None,
            toss_hold_max_secs: None,
            toss_depth_shift: None,
            toss_launch_angle_bonus_deg: None,
            toss_timeout: None,
            hit_height_min: None,
            hit_height_max: None,
            hit_height_optimal: None,
            ai_hit_tolerance: None,
            shot_max_distance: None,
            shot_cooldown_time: None,
            shot_jump_threshold: None,
            ai_move_speed: None,
            ai_shot_cooldown: None,
            ai_prediction_accuracy: None,
            ai_prediction_error: None,
            ai_direction_variance: None,
            ai_reaction_delay: None,
            ai_offensive_probability: None,
            ai_serve_offensive_probability: None,
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
            ..Default::default()
        };

        let enabled_explicit = parse_bool_env(ENV_OVERRIDES_ENABLED);
        overrides.enabled = enabled_explicit.unwrap_or(overrides.has_any_value());
        overrides
    }

    #[inline]
    pub fn has_any_value(&self) -> bool {
        SUPPORTED_OVERRIDE_KEYS
            .iter()
            .any(|key| get_override_value(self, key).is_some())
    }

    /// GameConfig に上書き適用
    /// @spec 77210_debug_control.md#req-77210-004
    pub fn apply_to_game_config(&self, config: &mut GameConfig) {
        if !self.enabled {
            return;
        }

        for key in SUPPORTED_OVERRIDE_KEYS {
            if let Some(value) = get_override_value(self, key) {
                let _ = apply_override_to_config(config, key, value);
            }
        }
    }
}

#[allow(dead_code)]
pub fn override_field_type(key: &str) -> Option<DebugOverrideType> {
    match key {
        "serve.practice_infinite_mode" => Some(DebugOverrideType::Bool),
        "physics.gravity"
        | "physics.max_fall_speed"
        | "player.move_speed"
        | "player.move_speed_z"
        | "player.max_speed"
        | "player.jump_force"
        | "ball.normal_shot_speed"
        | "ball.power_shot_speed"
        | "ball.bounce_factor"
        | "ball.radius"
        | "ball.min_bounce_velocity"
        | "ball.wall_bounce_factor"
        | "serve.serve_speed"
        | "serve.serve_angle"
        | "serve.toss_velocity_y"
        | "serve.toss_velocity_min_y"
        | "serve.toss_velocity_max_y"
        | "serve.toss_hold_max_secs"
        | "serve.toss_depth_shift"
        | "serve.toss_launch_angle_bonus_deg"
        | "serve.toss_timeout"
        | "serve.hit_height_min"
        | "serve.hit_height_max"
        | "serve.hit_height_optimal"
        | "serve.ai_hit_tolerance"
        | "shot.max_distance"
        | "shot.cooldown_time"
        | "shot.jump_threshold"
        | "ai.move_speed"
        | "ai.shot_cooldown"
        | "ai.prediction_accuracy"
        | "ai.prediction_error"
        | "ai.direction_variance"
        | "ai.reaction_delay"
        | "ai.offensive_probability"
        | "ai.serve_offensive_probability" => Some(DebugOverrideType::Float),
        _ => None,
    }
}

pub fn get_override_value(
    overrides: &DebugRuntimeOverrides,
    key: &str,
) -> Option<DebugOverrideValue> {
    match key {
        "serve.practice_infinite_mode" => overrides
            .practice_infinite_mode
            .map(DebugOverrideValue::Bool),
        "physics.gravity" => overrides.gravity.map(DebugOverrideValue::Float),
        "physics.max_fall_speed" => overrides
            .physics_max_fall_speed
            .map(DebugOverrideValue::Float),
        "player.move_speed" => overrides.player_move_speed.map(DebugOverrideValue::Float),
        "player.move_speed_z" => overrides.player_move_speed_z.map(DebugOverrideValue::Float),
        "player.max_speed" => overrides.player_max_speed.map(DebugOverrideValue::Float),
        "player.jump_force" => overrides.player_jump_force.map(DebugOverrideValue::Float),
        "ball.normal_shot_speed" => overrides
            .ball_normal_shot_speed
            .map(DebugOverrideValue::Float),
        "ball.power_shot_speed" => overrides
            .ball_power_shot_speed
            .map(DebugOverrideValue::Float),
        "ball.bounce_factor" => overrides.ball_bounce_factor.map(DebugOverrideValue::Float),
        "ball.radius" => overrides.ball_radius.map(DebugOverrideValue::Float),
        "ball.min_bounce_velocity" => overrides
            .ball_min_bounce_velocity
            .map(DebugOverrideValue::Float),
        "ball.wall_bounce_factor" => overrides
            .ball_wall_bounce_factor
            .map(DebugOverrideValue::Float),
        "serve.serve_speed" => overrides.serve_speed.map(DebugOverrideValue::Float),
        "serve.serve_angle" => overrides.serve_angle.map(DebugOverrideValue::Float),
        "serve.toss_velocity_y" => overrides.toss_velocity_y.map(DebugOverrideValue::Float),
        "serve.toss_velocity_min_y" => overrides.toss_velocity_min_y.map(DebugOverrideValue::Float),
        "serve.toss_velocity_max_y" => overrides.toss_velocity_max_y.map(DebugOverrideValue::Float),
        "serve.toss_hold_max_secs" => overrides.toss_hold_max_secs.map(DebugOverrideValue::Float),
        "serve.toss_depth_shift" => overrides.toss_depth_shift.map(DebugOverrideValue::Float),
        "serve.toss_launch_angle_bonus_deg" => overrides
            .toss_launch_angle_bonus_deg
            .map(DebugOverrideValue::Float),
        "serve.toss_timeout" => overrides.toss_timeout.map(DebugOverrideValue::Float),
        "serve.hit_height_min" => overrides.hit_height_min.map(DebugOverrideValue::Float),
        "serve.hit_height_max" => overrides.hit_height_max.map(DebugOverrideValue::Float),
        "serve.hit_height_optimal" => overrides.hit_height_optimal.map(DebugOverrideValue::Float),
        "serve.ai_hit_tolerance" => overrides.ai_hit_tolerance.map(DebugOverrideValue::Float),
        "shot.max_distance" => overrides.shot_max_distance.map(DebugOverrideValue::Float),
        "shot.cooldown_time" => overrides.shot_cooldown_time.map(DebugOverrideValue::Float),
        "shot.jump_threshold" => overrides.shot_jump_threshold.map(DebugOverrideValue::Float),
        "ai.move_speed" => overrides.ai_move_speed.map(DebugOverrideValue::Float),
        "ai.shot_cooldown" => overrides.ai_shot_cooldown.map(DebugOverrideValue::Float),
        "ai.prediction_accuracy" => overrides
            .ai_prediction_accuracy
            .map(DebugOverrideValue::Float),
        "ai.prediction_error" => overrides.ai_prediction_error.map(DebugOverrideValue::Float),
        "ai.direction_variance" => overrides
            .ai_direction_variance
            .map(DebugOverrideValue::Float),
        "ai.reaction_delay" => overrides.ai_reaction_delay.map(DebugOverrideValue::Float),
        "ai.offensive_probability" => overrides
            .ai_offensive_probability
            .map(DebugOverrideValue::Float),
        "ai.serve_offensive_probability" => overrides
            .ai_serve_offensive_probability
            .map(DebugOverrideValue::Float),
        _ => None,
    }
}

#[allow(dead_code)]
pub fn set_override_value(
    overrides: &mut DebugRuntimeOverrides,
    key: &str,
    value: Option<DebugOverrideValue>,
) -> Result<(), String> {
    match (key, value) {
        ("serve.practice_infinite_mode", Some(DebugOverrideValue::Bool(v))) => {
            overrides.practice_infinite_mode = Some(v)
        }
        ("serve.practice_infinite_mode", None) => overrides.practice_infinite_mode = None,

        ("physics.gravity", Some(DebugOverrideValue::Float(v))) => overrides.gravity = Some(v),
        ("physics.gravity", None) => overrides.gravity = None,
        ("physics.max_fall_speed", Some(DebugOverrideValue::Float(v))) => {
            overrides.physics_max_fall_speed = Some(v)
        }
        ("physics.max_fall_speed", None) => overrides.physics_max_fall_speed = None,

        ("player.move_speed", Some(DebugOverrideValue::Float(v))) => {
            overrides.player_move_speed = Some(v)
        }
        ("player.move_speed", None) => overrides.player_move_speed = None,
        ("player.move_speed_z", Some(DebugOverrideValue::Float(v))) => {
            overrides.player_move_speed_z = Some(v)
        }
        ("player.move_speed_z", None) => overrides.player_move_speed_z = None,
        ("player.max_speed", Some(DebugOverrideValue::Float(v))) => {
            overrides.player_max_speed = Some(v)
        }
        ("player.max_speed", None) => overrides.player_max_speed = None,
        ("player.jump_force", Some(DebugOverrideValue::Float(v))) => {
            overrides.player_jump_force = Some(v)
        }
        ("player.jump_force", None) => overrides.player_jump_force = None,

        ("ball.normal_shot_speed", Some(DebugOverrideValue::Float(v))) => {
            overrides.ball_normal_shot_speed = Some(v)
        }
        ("ball.normal_shot_speed", None) => overrides.ball_normal_shot_speed = None,
        ("ball.power_shot_speed", Some(DebugOverrideValue::Float(v))) => {
            overrides.ball_power_shot_speed = Some(v)
        }
        ("ball.power_shot_speed", None) => overrides.ball_power_shot_speed = None,
        ("ball.bounce_factor", Some(DebugOverrideValue::Float(v))) => {
            overrides.ball_bounce_factor = Some(v)
        }
        ("ball.bounce_factor", None) => overrides.ball_bounce_factor = None,
        ("ball.radius", Some(DebugOverrideValue::Float(v))) => overrides.ball_radius = Some(v),
        ("ball.radius", None) => overrides.ball_radius = None,
        ("ball.min_bounce_velocity", Some(DebugOverrideValue::Float(v))) => {
            overrides.ball_min_bounce_velocity = Some(v)
        }
        ("ball.min_bounce_velocity", None) => overrides.ball_min_bounce_velocity = None,
        ("ball.wall_bounce_factor", Some(DebugOverrideValue::Float(v))) => {
            overrides.ball_wall_bounce_factor = Some(v)
        }
        ("ball.wall_bounce_factor", None) => overrides.ball_wall_bounce_factor = None,

        ("serve.serve_speed", Some(DebugOverrideValue::Float(v))) => {
            overrides.serve_speed = Some(v)
        }
        ("serve.serve_speed", None) => overrides.serve_speed = None,
        ("serve.serve_angle", Some(DebugOverrideValue::Float(v))) => {
            overrides.serve_angle = Some(v)
        }
        ("serve.serve_angle", None) => overrides.serve_angle = None,
        ("serve.toss_velocity_y", Some(DebugOverrideValue::Float(v))) => {
            overrides.toss_velocity_y = Some(v)
        }
        ("serve.toss_velocity_y", None) => overrides.toss_velocity_y = None,
        ("serve.toss_velocity_min_y", Some(DebugOverrideValue::Float(v))) => {
            overrides.toss_velocity_min_y = Some(v)
        }
        ("serve.toss_velocity_min_y", None) => overrides.toss_velocity_min_y = None,
        ("serve.toss_velocity_max_y", Some(DebugOverrideValue::Float(v))) => {
            overrides.toss_velocity_max_y = Some(v)
        }
        ("serve.toss_velocity_max_y", None) => overrides.toss_velocity_max_y = None,
        ("serve.toss_hold_max_secs", Some(DebugOverrideValue::Float(v))) => {
            overrides.toss_hold_max_secs = Some(v)
        }
        ("serve.toss_hold_max_secs", None) => overrides.toss_hold_max_secs = None,
        ("serve.toss_depth_shift", Some(DebugOverrideValue::Float(v))) => {
            overrides.toss_depth_shift = Some(v)
        }
        ("serve.toss_depth_shift", None) => overrides.toss_depth_shift = None,
        ("serve.toss_launch_angle_bonus_deg", Some(DebugOverrideValue::Float(v))) => {
            overrides.toss_launch_angle_bonus_deg = Some(v)
        }
        ("serve.toss_launch_angle_bonus_deg", None) => overrides.toss_launch_angle_bonus_deg = None,
        ("serve.toss_timeout", Some(DebugOverrideValue::Float(v))) => {
            overrides.toss_timeout = Some(v)
        }
        ("serve.toss_timeout", None) => overrides.toss_timeout = None,
        ("serve.hit_height_min", Some(DebugOverrideValue::Float(v))) => {
            overrides.hit_height_min = Some(v)
        }
        ("serve.hit_height_min", None) => overrides.hit_height_min = None,
        ("serve.hit_height_max", Some(DebugOverrideValue::Float(v))) => {
            overrides.hit_height_max = Some(v)
        }
        ("serve.hit_height_max", None) => overrides.hit_height_max = None,
        ("serve.hit_height_optimal", Some(DebugOverrideValue::Float(v))) => {
            overrides.hit_height_optimal = Some(v)
        }
        ("serve.hit_height_optimal", None) => overrides.hit_height_optimal = None,
        ("serve.ai_hit_tolerance", Some(DebugOverrideValue::Float(v))) => {
            overrides.ai_hit_tolerance = Some(v)
        }
        ("serve.ai_hit_tolerance", None) => overrides.ai_hit_tolerance = None,

        ("shot.max_distance", Some(DebugOverrideValue::Float(v))) => {
            overrides.shot_max_distance = Some(v)
        }
        ("shot.max_distance", None) => overrides.shot_max_distance = None,
        ("shot.cooldown_time", Some(DebugOverrideValue::Float(v))) => {
            overrides.shot_cooldown_time = Some(v)
        }
        ("shot.cooldown_time", None) => overrides.shot_cooldown_time = None,
        ("shot.jump_threshold", Some(DebugOverrideValue::Float(v))) => {
            overrides.shot_jump_threshold = Some(v)
        }
        ("shot.jump_threshold", None) => overrides.shot_jump_threshold = None,

        ("ai.move_speed", Some(DebugOverrideValue::Float(v))) => overrides.ai_move_speed = Some(v),
        ("ai.move_speed", None) => overrides.ai_move_speed = None,
        ("ai.shot_cooldown", Some(DebugOverrideValue::Float(v))) => {
            overrides.ai_shot_cooldown = Some(v)
        }
        ("ai.shot_cooldown", None) => overrides.ai_shot_cooldown = None,
        ("ai.prediction_accuracy", Some(DebugOverrideValue::Float(v))) => {
            overrides.ai_prediction_accuracy = Some(v)
        }
        ("ai.prediction_accuracy", None) => overrides.ai_prediction_accuracy = None,
        ("ai.prediction_error", Some(DebugOverrideValue::Float(v))) => {
            overrides.ai_prediction_error = Some(v)
        }
        ("ai.prediction_error", None) => overrides.ai_prediction_error = None,
        ("ai.direction_variance", Some(DebugOverrideValue::Float(v))) => {
            overrides.ai_direction_variance = Some(v)
        }
        ("ai.direction_variance", None) => overrides.ai_direction_variance = None,
        ("ai.reaction_delay", Some(DebugOverrideValue::Float(v))) => {
            overrides.ai_reaction_delay = Some(v)
        }
        ("ai.reaction_delay", None) => overrides.ai_reaction_delay = None,
        ("ai.offensive_probability", Some(DebugOverrideValue::Float(v))) => {
            overrides.ai_offensive_probability = Some(v)
        }
        ("ai.offensive_probability", None) => overrides.ai_offensive_probability = None,
        ("ai.serve_offensive_probability", Some(DebugOverrideValue::Float(v))) => {
            overrides.ai_serve_offensive_probability = Some(v)
        }
        ("ai.serve_offensive_probability", None) => overrides.ai_serve_offensive_probability = None,

        (_, Some(v)) => {
            return Err(format!(
                "Unsupported key or value type: key='{}', value={:?}",
                key, v
            ));
        }
        (_, None) => return Err(format!("Unsupported key: '{}'", key)),
    }

    Ok(())
}

#[allow(dead_code)]
pub fn get_config_value(config: &GameConfig, key: &str) -> Option<DebugOverrideValue> {
    match key {
        "serve.practice_infinite_mode" => Some(DebugOverrideValue::Bool(
            config.serve.practice_infinite_mode,
        )),
        "physics.gravity" => Some(DebugOverrideValue::Float(config.physics.gravity)),
        "physics.max_fall_speed" => Some(DebugOverrideValue::Float(config.physics.max_fall_speed)),
        "player.move_speed" => Some(DebugOverrideValue::Float(config.player.move_speed)),
        "player.move_speed_z" => Some(DebugOverrideValue::Float(config.player.move_speed_z)),
        "player.max_speed" => Some(DebugOverrideValue::Float(config.player.max_speed)),
        "player.jump_force" => Some(DebugOverrideValue::Float(config.player.jump_force)),
        "ball.normal_shot_speed" => Some(DebugOverrideValue::Float(config.ball.normal_shot_speed)),
        "ball.power_shot_speed" => Some(DebugOverrideValue::Float(config.ball.power_shot_speed)),
        "ball.bounce_factor" => Some(DebugOverrideValue::Float(config.ball.bounce_factor)),
        "ball.radius" => Some(DebugOverrideValue::Float(config.ball.radius)),
        "ball.min_bounce_velocity" => {
            Some(DebugOverrideValue::Float(config.ball.min_bounce_velocity))
        }
        "ball.wall_bounce_factor" => {
            Some(DebugOverrideValue::Float(config.ball.wall_bounce_factor))
        }
        "serve.serve_speed" => Some(DebugOverrideValue::Float(config.serve.serve_speed)),
        "serve.serve_angle" => Some(DebugOverrideValue::Float(config.serve.serve_angle)),
        "serve.toss_velocity_y" => Some(DebugOverrideValue::Float(config.serve.toss_velocity_y)),
        "serve.toss_velocity_min_y" => {
            Some(DebugOverrideValue::Float(config.serve.toss_velocity_min_y))
        }
        "serve.toss_velocity_max_y" => {
            Some(DebugOverrideValue::Float(config.serve.toss_velocity_max_y))
        }
        "serve.toss_hold_max_secs" => {
            Some(DebugOverrideValue::Float(config.serve.toss_hold_max_secs))
        }
        "serve.toss_depth_shift" => Some(DebugOverrideValue::Float(config.serve.toss_depth_shift)),
        "serve.toss_launch_angle_bonus_deg" => Some(DebugOverrideValue::Float(
            config.serve.toss_launch_angle_bonus_deg,
        )),
        "serve.toss_timeout" => Some(DebugOverrideValue::Float(config.serve.toss_timeout)),
        "serve.hit_height_min" => Some(DebugOverrideValue::Float(config.serve.hit_height_min)),
        "serve.hit_height_max" => Some(DebugOverrideValue::Float(config.serve.hit_height_max)),
        "serve.hit_height_optimal" => {
            Some(DebugOverrideValue::Float(config.serve.hit_height_optimal))
        }
        "serve.ai_hit_tolerance" => Some(DebugOverrideValue::Float(config.serve.ai_hit_tolerance)),
        "shot.max_distance" => Some(DebugOverrideValue::Float(config.shot.max_distance)),
        "shot.cooldown_time" => Some(DebugOverrideValue::Float(config.shot.cooldown_time)),
        "shot.jump_threshold" => Some(DebugOverrideValue::Float(config.shot.jump_threshold)),
        "ai.move_speed" => Some(DebugOverrideValue::Float(config.ai.move_speed)),
        "ai.shot_cooldown" => Some(DebugOverrideValue::Float(config.ai.shot_cooldown)),
        "ai.prediction_accuracy" => Some(DebugOverrideValue::Float(config.ai.prediction_accuracy)),
        "ai.prediction_error" => Some(DebugOverrideValue::Float(config.ai.prediction_error)),
        "ai.direction_variance" => Some(DebugOverrideValue::Float(config.ai.direction_variance)),
        "ai.reaction_delay" => Some(DebugOverrideValue::Float(config.ai.reaction_delay)),
        "ai.offensive_probability" => {
            Some(DebugOverrideValue::Float(config.ai.offensive_probability))
        }
        "ai.serve_offensive_probability" => Some(DebugOverrideValue::Float(
            config.ai.serve_offensive_probability,
        )),
        _ => None,
    }
}

fn apply_override_to_config(
    config: &mut GameConfig,
    key: &str,
    value: DebugOverrideValue,
) -> Result<(), String> {
    match (key, value) {
        ("serve.practice_infinite_mode", DebugOverrideValue::Bool(v)) => {
            config.serve.practice_infinite_mode = v
        }
        ("physics.gravity", DebugOverrideValue::Float(v)) => config.physics.gravity = v,
        ("physics.max_fall_speed", DebugOverrideValue::Float(v)) => {
            config.physics.max_fall_speed = v
        }
        ("player.move_speed", DebugOverrideValue::Float(v)) => config.player.move_speed = v,
        ("player.move_speed_z", DebugOverrideValue::Float(v)) => config.player.move_speed_z = v,
        ("player.max_speed", DebugOverrideValue::Float(v)) => config.player.max_speed = v,
        ("player.jump_force", DebugOverrideValue::Float(v)) => config.player.jump_force = v,
        ("ball.normal_shot_speed", DebugOverrideValue::Float(v)) => {
            config.ball.normal_shot_speed = v
        }
        ("ball.power_shot_speed", DebugOverrideValue::Float(v)) => config.ball.power_shot_speed = v,
        ("ball.bounce_factor", DebugOverrideValue::Float(v)) => config.ball.bounce_factor = v,
        ("ball.radius", DebugOverrideValue::Float(v)) => config.ball.radius = v,
        ("ball.min_bounce_velocity", DebugOverrideValue::Float(v)) => {
            config.ball.min_bounce_velocity = v
        }
        ("ball.wall_bounce_factor", DebugOverrideValue::Float(v)) => {
            config.ball.wall_bounce_factor = v
        }
        ("serve.serve_speed", DebugOverrideValue::Float(v)) => config.serve.serve_speed = v,
        ("serve.serve_angle", DebugOverrideValue::Float(v)) => config.serve.serve_angle = v,
        ("serve.toss_velocity_y", DebugOverrideValue::Float(v)) => config.serve.toss_velocity_y = v,
        ("serve.toss_velocity_min_y", DebugOverrideValue::Float(v)) => {
            config.serve.toss_velocity_min_y = v
        }
        ("serve.toss_velocity_max_y", DebugOverrideValue::Float(v)) => {
            config.serve.toss_velocity_max_y = v
        }
        ("serve.toss_hold_max_secs", DebugOverrideValue::Float(v)) => {
            config.serve.toss_hold_max_secs = v
        }
        ("serve.toss_depth_shift", DebugOverrideValue::Float(v)) => {
            config.serve.toss_depth_shift = v
        }
        ("serve.toss_launch_angle_bonus_deg", DebugOverrideValue::Float(v)) => {
            config.serve.toss_launch_angle_bonus_deg = v
        }
        ("serve.toss_timeout", DebugOverrideValue::Float(v)) => config.serve.toss_timeout = v,
        ("serve.hit_height_min", DebugOverrideValue::Float(v)) => config.serve.hit_height_min = v,
        ("serve.hit_height_max", DebugOverrideValue::Float(v)) => config.serve.hit_height_max = v,
        ("serve.hit_height_optimal", DebugOverrideValue::Float(v)) => {
            config.serve.hit_height_optimal = v
        }
        ("serve.ai_hit_tolerance", DebugOverrideValue::Float(v)) => {
            config.serve.ai_hit_tolerance = v
        }
        ("shot.max_distance", DebugOverrideValue::Float(v)) => config.shot.max_distance = v,
        ("shot.cooldown_time", DebugOverrideValue::Float(v)) => config.shot.cooldown_time = v,
        ("shot.jump_threshold", DebugOverrideValue::Float(v)) => config.shot.jump_threshold = v,
        ("ai.move_speed", DebugOverrideValue::Float(v)) => config.ai.move_speed = v,
        ("ai.shot_cooldown", DebugOverrideValue::Float(v)) => config.ai.shot_cooldown = v,
        ("ai.prediction_accuracy", DebugOverrideValue::Float(v)) => {
            config.ai.prediction_accuracy = v
        }
        ("ai.prediction_error", DebugOverrideValue::Float(v)) => config.ai.prediction_error = v,
        ("ai.direction_variance", DebugOverrideValue::Float(v)) => config.ai.direction_variance = v,
        ("ai.reaction_delay", DebugOverrideValue::Float(v)) => config.ai.reaction_delay = v,
        ("ai.offensive_probability", DebugOverrideValue::Float(v)) => {
            config.ai.offensive_probability = v
        }
        ("ai.serve_offensive_probability", DebugOverrideValue::Float(v)) => {
            config.ai.serve_offensive_probability = v
        }
        _ => {
            return Err(format!(
                "Unsupported key/value for apply: key='{}', value={:?}",
                key, value
            ));
        }
    }

    Ok(())
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
            ..Default::default()
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
            ..Default::default()
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

    #[test]
    fn test_override_key_helpers() {
        let mut overrides = DebugRuntimeOverrides::default();
        set_override_value(
            &mut overrides,
            "serve.practice_infinite_mode",
            Some(DebugOverrideValue::Bool(true)),
        )
        .expect("set bool failed");
        set_override_value(
            &mut overrides,
            "player.move_speed",
            Some(DebugOverrideValue::Float(4.5)),
        )
        .expect("set float failed");

        assert_eq!(
            get_override_value(&overrides, "serve.practice_infinite_mode"),
            Some(DebugOverrideValue::Bool(true))
        );
        assert_eq!(
            get_override_value(&overrides, "player.move_speed"),
            Some(DebugOverrideValue::Float(4.5))
        );
    }
}
