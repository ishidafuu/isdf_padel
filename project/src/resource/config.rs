//! ゲーム設定
//! @data 80101_game_constants.md

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    input::keyboard::KeyCode,
    prelude::*,
};
use serde::Deserialize;

/// ゲーム全体の設定
/// @data 80101_game_constants.md#gameconfig-構造
#[derive(Asset, TypePath, Resource, Deserialize, Clone, Debug)]
pub struct GameConfig {
    pub physics: PhysicsConfig,
    pub court: CourtConfig,
    pub player: PlayerConfig,
    pub ball: BallConfig,
    pub collision: CollisionConfig,
    pub knockback: KnockbackConfig,
    pub shot: ShotConfig,
    pub scoring: ScoringConfig,
    /// TODO: v0.2で入力バッファリング機能として使用予定
    #[allow(dead_code)]
    pub input: InputConfig,
    /// 入力キーバインド設定
    #[serde(default)]
    pub input_keys: InputKeysConfig,
    #[serde(default)]
    pub shadow: ShadowConfig,
    #[serde(default)]
    pub shot_attributes: ShotAttributesConfig,
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub visual_feedback: VisualFeedbackConfig,
    /// プレイヤービジュアル設定（色、サイズ）
    #[serde(default)]
    pub player_visual: PlayerVisualConfig,
    /// サーブ設定
    #[serde(default)]
    pub serve: ServeConfig,
    /// スピン物理パラメータ
    /// @data 80101_game_constants.md#spin-physics-config
    #[serde(default)]
    pub spin_physics: SpinPhysicsConfig,
}

/// 物理演算パラメータ
/// @data 80101_game_constants.md#physics-config
#[derive(Deserialize, Clone, Debug)]
pub struct PhysicsConfig {
    #[serde(default = "default_gravity")]
    pub gravity: f32,
    #[serde(default = "default_max_fall_speed")]
    pub max_fall_speed: f32,
}

fn default_gravity() -> f32 {
    -9.8
}
fn default_max_fall_speed() -> f32 {
    -20.0
}

/// コートサイズ・範囲
/// @data 80101_game_constants.md#court-config
#[derive(Deserialize, Clone, Debug)]
pub struct CourtConfig {
    #[serde(default = "default_court_width")]
    pub width: f32,
    #[serde(default = "default_court_depth")]
    pub depth: f32,
    #[serde(default = "default_ceiling_height")]
    pub ceiling_height: f32,
    /// TODO: v0.2でジャンプ高さ制限として使用予定
    #[allow(dead_code)]
    #[serde(default = "default_max_jump_height")]
    pub max_jump_height: f32,
    #[serde(default = "default_net_height")]
    pub net_height: f32,
    #[serde(default = "default_net_x")]
    pub net_x: f32,
    /// サービスボックスの奥行き（ネットからの距離）
    /// @spec 30902_fault_spec.md#req-30902-001
    #[serde(default = "default_service_box_depth")]
    pub service_box_depth: f32,
}

fn default_court_width() -> f32 {
    10.0
}
fn default_court_depth() -> f32 {
    6.0
}
fn default_ceiling_height() -> f32 {
    5.0
}
fn default_max_jump_height() -> f32 {
    5.0
}
fn default_net_height() -> f32 {
    1.0
}
fn default_net_x() -> f32 {
    0.0
}
fn default_service_box_depth() -> f32 {
    1.5
}

/// プレイヤー移動パラメータ
/// @data 80101_game_constants.md#player-config
#[derive(Deserialize, Clone, Debug)]
pub struct PlayerConfig {
    #[serde(default = "default_move_speed")]
    pub move_speed: f32,
    #[serde(default = "default_move_speed_z")]
    pub move_speed_z: f32,
    #[serde(default = "default_max_speed")]
    pub max_speed: f32,
    #[serde(default = "default_jump_force")]
    pub jump_force: f32,
    /// TODO: v0.2で移動システム改善として使用予定
    #[allow(dead_code)]
    #[serde(default = "default_friction")]
    pub friction: f32,
    /// TODO: v0.2で空中制御として使用予定
    #[allow(dead_code)]
    #[serde(default = "default_air_control")]
    pub air_control_factor: f32,
    #[serde(default = "default_x_min")]
    pub x_min: f32,
    #[serde(default = "default_x_max")]
    pub x_max: f32,
}

fn default_move_speed() -> f32 {
    5.0
}
fn default_move_speed_z() -> f32 {
    4.0
}
fn default_max_speed() -> f32 {
    10.0
}
fn default_jump_force() -> f32 {
    8.0
}
fn default_friction() -> f32 {
    0.9
}
fn default_air_control() -> f32 {
    0.5
}
fn default_x_min() -> f32 {
    -3.0
}
fn default_x_max() -> f32 {
    3.0
}

/// ボールパラメータ
/// @data 80101_game_constants.md#ball-config
#[derive(Deserialize, Clone, Debug)]
pub struct BallConfig {
    #[serde(default = "default_normal_shot_speed")]
    pub normal_shot_speed: f32,
    /// TODO: v0.2ショット属性システムで使用予定
    #[allow(dead_code)]
    #[serde(default = "default_power_shot_speed")]
    pub power_shot_speed: f32,
    #[serde(default = "default_bounce_factor")]
    pub bounce_factor: f32,
    #[serde(default = "default_ball_radius")]
    pub radius: f32,
    /// 最小バウンド速度（Y速度が0の場合に適用）
    #[serde(default = "default_min_bounce_velocity")]
    pub min_bounce_velocity: f32,
    /// 壁反射係数（壁バウンド時の速度減衰）
    #[serde(default = "default_wall_bounce_factor")]
    pub wall_bounce_factor: f32,
}

fn default_normal_shot_speed() -> f32 {
    10.0
}
fn default_power_shot_speed() -> f32 {
    15.0
}
fn default_bounce_factor() -> f32 {
    0.8
}
fn default_ball_radius() -> f32 {
    0.2
}
fn default_min_bounce_velocity() -> f32 {
    1.0
}
fn default_wall_bounce_factor() -> f32 {
    0.8
}

/// 当たり判定パラメータ
/// @data 80101_game_constants.md#collision-config
#[derive(Deserialize, Clone, Debug)]
pub struct CollisionConfig {
    #[serde(default = "default_character_radius")]
    pub character_radius: f32,
    #[serde(default = "default_z_tolerance")]
    pub z_tolerance: f32,
}

fn default_character_radius() -> f32 {
    0.5
}
fn default_z_tolerance() -> f32 {
    0.3
}

/// ふっとばしパラメータ
/// @data 80101_game_constants.md#knockback-config
#[derive(Deserialize, Clone, Debug)]
pub struct KnockbackConfig {
    /// ふっとばし機能の有効/無効
    /// false の場合、被弾してもふっとばしが発生しない
    #[serde(default = "default_knockback_enabled")]
    pub enabled: bool,
    #[serde(default = "default_knockback_duration")]
    pub duration: f32,
    #[serde(default = "default_speed_multiplier")]
    pub speed_multiplier: f32,
    #[serde(default = "default_invincibility_time")]
    pub invincibility_time: f32,
}

fn default_knockback_enabled() -> bool {
    true // デフォルトは有効
}
fn default_knockback_duration() -> f32 {
    0.5
}
fn default_speed_multiplier() -> f32 {
    0.5
}
fn default_invincibility_time() -> f32 {
    1.0
}

/// ショットシステムパラメータ
/// @data 80101_game_constants.md#shot-config
#[derive(Deserialize, Clone, Debug)]
pub struct ShotConfig {
    #[serde(default = "default_max_distance")]
    pub max_distance: f32,
    #[serde(default = "default_max_height_diff")]
    pub max_height_diff: f32,
    #[serde(default = "default_cooldown_time")]
    pub cooldown_time: f32,
    #[serde(default = "default_normal_shot_angle")]
    pub normal_shot_angle: f32,
    /// TODO: v0.2ショット属性システムで使用予定
    #[allow(dead_code)]
    #[serde(default = "default_jump_shot_angle")]
    pub jump_shot_angle: f32,
    #[serde(default = "default_jump_threshold")]
    pub jump_threshold: f32,
}

fn default_max_distance() -> f32 {
    1.5
}
fn default_max_height_diff() -> f32 {
    2.0
}
fn default_cooldown_time() -> f32 {
    0.5
}
fn default_normal_shot_angle() -> f32 {
    45.0
}
fn default_jump_shot_angle() -> f32 {
    30.0
}
fn default_jump_threshold() -> f32 {
    0.5
}

/// スコアリングパラメータ
/// @data 80101_game_constants.md#scoring-config
/// @data 80701_point_config.md
/// @data 80703_set_config.md
#[derive(Deserialize, Clone, Debug)]
pub struct ScoringConfig {
    /// ポイント進行値 [0, 15, 30, 40]
    /// @spec 30701_point_spec.md#req-30701-001
    #[serde(default = "default_point_values")]
    pub point_values: Vec<u32>,
    /// 勝利に必要なゲーム数（6ゲーム先取でセット獲得）
    /// @data 80703_set_config.md#games_to_win_set
    #[serde(default = "default_games_to_win_set")]
    pub games_to_win_set: u32,
    /// 勝利に必要なセット数（1セット先取でマッチ勝利）
    /// @data 80703_set_config.md#sets_to_win_match
    #[serde(default = "default_sets_to_win_match")]
    pub sets_to_win_match: u32,
}

fn default_point_values() -> Vec<u32> {
    vec![0, 15, 30, 40]
}
fn default_games_to_win_set() -> u32 {
    6
}
fn default_sets_to_win_match() -> u32 {
    1
}

/// 入力パラメータ
/// @data 80101_game_constants.md#input-config
/// TODO: v0.2で入力バッファリング機能として使用予定
#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct InputConfig {
    #[serde(default = "default_jump_buffer_time")]
    pub jump_buffer_time: f32,
    #[serde(default = "default_shot_buffer_time")]
    pub shot_buffer_time: f32,
    /// 斜め移動の正規化閾値（この値を超えると正規化）
    #[serde(default = "default_normalization_threshold")]
    pub normalization_threshold: f32,
    /// 入力感度（移動入力に乗算する係数、アナログ入力対応用）
    #[serde(default = "default_input_sensitivity")]
    pub input_sensitivity: f32,
}

fn default_jump_buffer_time() -> f32 {
    0.1
}
fn default_shot_buffer_time() -> f32 {
    0.05
}
fn default_normalization_threshold() -> f32 {
    1.0
}
fn default_input_sensitivity() -> f32 {
    1.0
}

/// 入力キーバインド設定
/// @data 80101_game_constants.md#input-keys-config
#[derive(Deserialize, Clone, Debug)]
pub struct InputKeysConfig {
    /// 上移動キー（デフォルト: W）
    #[serde(default = "default_key_move_up")]
    pub move_up: KeyCode,
    /// 下移動キー（デフォルト: S）
    #[serde(default = "default_key_move_down")]
    pub move_down: KeyCode,
    /// 左移動キー（デフォルト: A）
    #[serde(default = "default_key_move_left")]
    pub move_left: KeyCode,
    /// 右移動キー（デフォルト: D）
    #[serde(default = "default_key_move_right")]
    pub move_right: KeyCode,
    /// 上移動キー（代替: 矢印上）
    #[serde(default = "default_key_move_up_alt")]
    pub move_up_alt: KeyCode,
    /// 下移動キー（代替: 矢印下）
    #[serde(default = "default_key_move_down_alt")]
    pub move_down_alt: KeyCode,
    /// 左移動キー（代替: 矢印左）
    #[serde(default = "default_key_move_left_alt")]
    pub move_left_alt: KeyCode,
    /// 右移動キー（代替: 矢印右）
    #[serde(default = "default_key_move_right_alt")]
    pub move_right_alt: KeyCode,
    /// ジャンプキー（デフォルト: B）
    #[serde(default = "default_key_jump")]
    pub jump: KeyCode,
    /// ショットキー（デフォルト: V）
    #[serde(default = "default_key_shot")]
    pub shot: KeyCode,
}

impl Default for InputKeysConfig {
    fn default() -> Self {
        Self {
            move_up: default_key_move_up(),
            move_down: default_key_move_down(),
            move_left: default_key_move_left(),
            move_right: default_key_move_right(),
            move_up_alt: default_key_move_up_alt(),
            move_down_alt: default_key_move_down_alt(),
            move_left_alt: default_key_move_left_alt(),
            move_right_alt: default_key_move_right_alt(),
            jump: default_key_jump(),
            shot: default_key_shot(),
        }
    }
}

fn default_key_move_up() -> KeyCode {
    KeyCode::KeyW
}
fn default_key_move_down() -> KeyCode {
    KeyCode::KeyS
}
fn default_key_move_left() -> KeyCode {
    KeyCode::KeyA
}
fn default_key_move_right() -> KeyCode {
    KeyCode::KeyD
}
fn default_key_move_up_alt() -> KeyCode {
    KeyCode::ArrowUp
}
fn default_key_move_down_alt() -> KeyCode {
    KeyCode::ArrowDown
}
fn default_key_move_left_alt() -> KeyCode {
    KeyCode::ArrowLeft
}
fn default_key_move_right_alt() -> KeyCode {
    KeyCode::ArrowRight
}
fn default_key_jump() -> KeyCode {
    KeyCode::KeyB
}
fn default_key_shot() -> KeyCode {
    KeyCode::KeyV
}

/// サーブパラメータ
/// @spec 30102_serve_spec.md
#[derive(Deserialize, Clone, Debug)]
pub struct ServeConfig {
    /// ボール生成時のY方向オフセット（プレイヤー足元からの高さ）
    #[serde(default = "default_ball_spawn_offset_y")]
    pub ball_spawn_offset_y: f32,
    /// Player1のデフォルトサーブ方向（X軸：打ち合い方向）
    #[serde(default = "default_p1_default_direction_x")]
    pub p1_default_direction_x: f32,
    /// Player2のデフォルトサーブ方向（X軸：打ち合い方向）
    #[serde(default = "default_p2_default_direction_x")]
    pub p2_default_direction_x: f32,
}

impl Default for ServeConfig {
    fn default() -> Self {
        Self {
            ball_spawn_offset_y: default_ball_spawn_offset_y(),
            p1_default_direction_x: default_p1_default_direction_x(),
            p2_default_direction_x: default_p2_default_direction_x(),
        }
    }
}

fn default_ball_spawn_offset_y() -> f32 {
    0.5
}
fn default_p1_default_direction_x() -> f32 {
    1.0 // +X方向（2Pコートへ）
}
fn default_p2_default_direction_x() -> f32 {
    -1.0 // -X方向（1Pコートへ）
}

/// 影パラメータ
/// @data 80101_game_constants.md#shadow-config
#[derive(Deserialize, Clone, Debug, Default)]
pub struct ShadowConfig {
    /// プレイヤー影のサイズ（幅, 高さ）
    #[serde(default = "default_player_shadow_size")]
    pub player_size: (f32, f32),
    /// プレイヤー影の透明度
    #[serde(default = "default_player_shadow_alpha")]
    pub player_alpha: f32,
    /// プレイヤー影のY方向オフセット（足元に表示するため）
    #[serde(default = "default_player_shadow_y_offset")]
    pub player_y_offset: f32,

    /// ボール影のサイズ（幅, 高さ）
    #[serde(default = "default_ball_shadow_size")]
    pub ball_size: (f32, f32),
    /// ボール影の透明度
    #[serde(default = "default_ball_shadow_alpha")]
    pub ball_alpha: f32,
    /// ボール影のY方向オフセット
    #[serde(default = "default_ball_shadow_y_offset")]
    pub ball_y_offset: f32,

    /// 影のZ深度（背面に表示）
    #[serde(default = "default_shadow_z_layer")]
    pub z_layer: f32,
}

fn default_player_shadow_size() -> (f32, f32) {
    (50.0, 20.0)
}
fn default_player_shadow_alpha() -> f32 {
    0.6
}
fn default_player_shadow_y_offset() -> f32 {
    30.0
}
fn default_ball_shadow_size() -> (f32, f32) {
    (25.0, 10.0)
}
fn default_ball_shadow_alpha() -> f32 {
    0.5
}
fn default_ball_shadow_y_offset() -> f32 {
    0.0
}
fn default_shadow_z_layer() -> f32 {
    -0.5
}

/// AIパラメータ
/// @spec 30301_ai_movement_spec.md
/// @spec 30302_ai_shot_spec.md
#[derive(Deserialize, Clone, Debug)]
pub struct AiConfig {
    /// AI移動速度（m/s）
    /// @spec 30301_ai_movement_spec.md#req-30301-001
    #[serde(default = "default_ai_move_speed")]
    pub move_speed: f32,
    /// ホームポジションX座標（m、打ち合い方向）
    /// @spec 30301_ai_movement_spec.md#req-30301-005
    #[serde(default = "default_ai_home_position_x")]
    pub home_position_x: f32,
    /// AIショットクールダウン（秒）
    /// @spec 30302_ai_shot_spec.md#req-30302-002
    /// @spec 30302_ai_shot_spec.md#req-30302-004
    #[serde(default = "default_ai_shot_cooldown")]
    pub shot_cooldown: f32,
    /// ホーム復帰時の停止距離（m）
    /// @spec 30301_ai_movement_spec.md#req-30301-005
    #[serde(default = "default_ai_home_return_stop_distance")]
    pub home_return_stop_distance: f32,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            move_speed: default_ai_move_speed(),
            home_position_x: default_ai_home_position_x(),
            shot_cooldown: default_ai_shot_cooldown(),
            home_return_stop_distance: default_ai_home_return_stop_distance(),
        }
    }
}

fn default_ai_move_speed() -> f32 {
    5.0
}
fn default_ai_home_position_x() -> f32 {
    5.0 // 2Pコート側（+X方向）
}
/// @spec 30302_ai_shot_spec.md#req-30302-002
fn default_ai_shot_cooldown() -> f32 {
    0.5 // デフォルト: 0.5秒
}
fn default_ai_home_return_stop_distance() -> f32 {
    0.3
}

/// 視覚フィードバックパラメータ
/// @spec 30802_visual_feedback_spec.md
/// @data 80101_game_constants.md#visual-feedback-config
#[derive(Deserialize, Clone, Debug)]
pub struct VisualFeedbackConfig {
    /// ホールド中のプレイヤー色（RGBA）
    /// @spec 30802_visual_feedback_spec.md#req-30802-001
    #[serde(default = "default_hold_color")]
    pub hold_color: (f32, f32, f32, f32),
    /// トップスピン時のボール色（RGBA）
    /// @spec 30802_visual_feedback_spec.md#req-30802-003
    #[serde(default = "default_ball_color_topspin")]
    pub ball_color_topspin: (f32, f32, f32, f32),
    /// ニュートラル時のボール色（RGBA）
    /// @spec 30802_visual_feedback_spec.md#req-30802-003
    #[serde(default = "default_ball_color_neutral")]
    pub ball_color_neutral: (f32, f32, f32, f32),
    /// スライス時のボール色（RGBA）
    /// @spec 30802_visual_feedback_spec.md#req-30802-003
    #[serde(default = "default_ball_color_slice")]
    pub ball_color_slice: (f32, f32, f32, f32),
}

impl Default for VisualFeedbackConfig {
    fn default() -> Self {
        Self {
            hold_color: default_hold_color(),
            ball_color_topspin: default_ball_color_topspin(),
            ball_color_neutral: default_ball_color_neutral(),
            ball_color_slice: default_ball_color_slice(),
        }
    }
}

fn default_hold_color() -> (f32, f32, f32, f32) {
    (1.0, 0.5, 0.0, 1.0) // オレンジ
}
fn default_ball_color_topspin() -> (f32, f32, f32, f32) {
    (1.0, 0.2, 0.2, 1.0) // 赤
}
fn default_ball_color_neutral() -> (f32, f32, f32, f32) {
    (0.9, 0.9, 0.2, 1.0) // 黄色
}
fn default_ball_color_slice() -> (f32, f32, f32, f32) {
    (0.2, 0.4, 1.0, 1.0) // 青
}

/// プレイヤービジュアル設定
/// @data 80101_game_constants.md#player-visual-config
#[derive(Deserialize, Clone, Debug)]
pub struct PlayerVisualConfig {
    /// Player1（1Pコート側）の色（RGB）
    #[serde(default = "default_player1_color")]
    pub player1_color: (f32, f32, f32),
    /// Player2（2Pコート側）の色（RGB）
    #[serde(default = "default_player2_color")]
    pub player2_color: (f32, f32, f32),
    /// プレイヤーのサイズ（幅, 高さ）ピクセル
    #[serde(default = "default_player_size")]
    pub size: (f32, f32),
}

impl Default for PlayerVisualConfig {
    fn default() -> Self {
        Self {
            player1_color: default_player1_color(),
            player2_color: default_player2_color(),
            size: default_player_size(),
        }
    }
}

fn default_player1_color() -> (f32, f32, f32) {
    (0.2, 0.4, 0.8) // 青
}

fn default_player2_color() -> (f32, f32, f32) {
    (0.8, 0.2, 0.2) // 赤
}

fn default_player_size() -> (f32, f32) {
    (40.0, 60.0)
}

/// スピン物理パラメータ
/// @spec 30401_trajectory_spec.md#req-30401-100
/// @spec 30401_trajectory_spec.md#req-30401-101
/// @spec 30401_trajectory_spec.md#req-30401-102
/// @spec 30402_reflection_spec.md#req-30402-100
/// @data 80101_game_constants.md#spin-physics-config
#[derive(Deserialize, Clone, Debug)]
pub struct SpinPhysicsConfig {
    /// 重力に対するスピンの影響度（±30%時 = 0.3）
    #[serde(default = "default_gravity_spin_factor")]
    pub gravity_spin_factor: f32,

    /// バウンド時の水平方向へのスピンの影響度
    #[serde(default = "default_bounce_spin_horizontal_factor")]
    pub bounce_spin_horizontal_factor: f32,

    /// バウンド時の垂直方向へのスピンの影響度
    #[serde(default = "default_bounce_spin_vertical_factor")]
    pub bounce_spin_vertical_factor: f32,

    /// ベース空気抵抗（スピンなしでも適用）
    #[serde(default = "default_base_air_drag")]
    pub base_air_drag: f32,

    /// スピンによる追加空気抵抗係数
    #[serde(default = "default_spin_drag_factor")]
    pub spin_drag_factor: f32,

    /// スピン時間減衰率（1秒あたり）
    #[serde(default = "default_spin_decay_rate")]
    pub spin_decay_rate: f32,
}

impl Default for SpinPhysicsConfig {
    fn default() -> Self {
        Self {
            gravity_spin_factor: default_gravity_spin_factor(),
            bounce_spin_horizontal_factor: default_bounce_spin_horizontal_factor(),
            bounce_spin_vertical_factor: default_bounce_spin_vertical_factor(),
            base_air_drag: default_base_air_drag(),
            spin_drag_factor: default_spin_drag_factor(),
            spin_decay_rate: default_spin_decay_rate(),
        }
    }
}

fn default_gravity_spin_factor() -> f32 {
    0.3
}
fn default_bounce_spin_horizontal_factor() -> f32 {
    0.3
}
fn default_bounce_spin_vertical_factor() -> f32 {
    0.2
}
fn default_base_air_drag() -> f32 {
    0.0
}
fn default_spin_drag_factor() -> f32 {
    0.3
}
fn default_spin_decay_rate() -> f32 {
    0.5
}

/// ショット属性パラメータ
/// @spec 30604_shot_attributes_spec.md
#[derive(Deserialize, Clone, Debug)]
pub struct ShotAttributesConfig {
    // === 入力方式パラメータ ===
    /// プッシュ完璧判定ウィンドウ（ミリ秒）
    /// @spec 30604_shot_attributes_spec.md#req-30604-050
    /// TODO: v0.2ショット属性システムで使用予定
    #[allow(dead_code)]
    #[serde(default = "default_push_perfect_window")]
    pub push_perfect_window: f32,
    /// プッシュ→ホールド閾値（ミリ秒）
    /// @spec 30604_shot_attributes_spec.md#req-30604-053
    #[serde(default = "default_push_to_hold_threshold")]
    pub push_to_hold_threshold: f32,
    /// ホールド安定化時間（ミリ秒）
    /// @spec 30604_shot_attributes_spec.md#req-30604-052
    #[serde(default = "default_hold_stable_time")]
    pub hold_stable_time: f32,
    /// ホールド威力係数
    /// @spec 30604_shot_attributes_spec.md#req-30604-051
    #[serde(default = "default_hold_power_factor")]
    pub hold_power_factor: f32,

    // === 距離パラメータ ===
    /// 最適距離（メートル）
    /// @spec 30604_shot_attributes_spec.md#req-30604-062
    /// TODO: v0.2ショット属性システムで使用予定
    #[allow(dead_code)]
    #[serde(default = "default_optimal_distance")]
    pub optimal_distance: f32,

    // === 安定性パラメータ ===
    /// 安定性閾値（これ未満でミスショット判定）
    /// @spec 30604_shot_attributes_spec.md#req-30604-069
    #[serde(default = "default_stability_threshold")]
    pub stability_threshold: f32,
    /// 最大方向ブレ（度）
    /// @spec 30604_shot_attributes_spec.md#req-30604-070
    #[serde(default = "default_max_direction_error")]
    pub max_direction_error: f32,

    // === ベース値 ===
    /// ベース威力（m/s）
    /// @spec 30604_shot_attributes_spec.md#req-30604-063
    #[serde(default = "default_base_power")]
    pub base_power: f32,
    /// ベース安定性
    /// @spec 30604_shot_attributes_spec.md#req-30604-064
    #[serde(default = "default_base_stability")]
    pub base_stability: f32,
    /// ベース角度（度）
    /// @spec 30604_shot_attributes_spec.md#req-30604-065
    #[serde(default = "default_base_angle")]
    pub base_angle: f32,
    /// ベース精度
    /// @spec 30604_shot_attributes_spec.md#req-30604-067
    #[serde(default = "default_base_accuracy")]
    pub base_accuracy: f32,

    // === カーブポイント ===
    /// 打点高さカーブ [(高さ, 威力係数, 安定性係数, 角度補正)]
    /// @spec 30604_shot_attributes_spec.md#req-30604-055
    #[serde(default = "default_height_curve")]
    pub height_curve: Vec<HeightCurvePoint>,
    /// タイミングカーブ [(経過時間, 威力係数, 安定性係数, 角度補正)]
    /// @spec 30604_shot_attributes_spec.md#req-30604-058
    #[serde(default = "default_timing_curve")]
    pub timing_curve: Vec<TimingCurvePoint>,
    /// 入り方カーブ [(内積, 威力係数, 角度補正)]
    /// @spec 30604_shot_attributes_spec.md#req-30604-060
    #[serde(default = "default_approach_curve")]
    pub approach_curve: Vec<ApproachCurvePoint>,
    /// 距離カーブ [(距離, 威力係数, 安定性係数, 精度係数)]
    /// @spec 30604_shot_attributes_spec.md#req-30604-062
    #[serde(default = "default_distance_curve")]
    pub distance_curve: Vec<DistanceCurvePoint>,
    /// ボレー補正
    /// @spec 30604_shot_attributes_spec.md#req-30604-057
    #[serde(default = "default_volley_factors")]
    pub volley_factors: VolleyFactors,
    /// スピンカーブ（高さ）[(高さ, スピン係数)]
    /// @spec 30604_shot_attributes_spec.md#req-30604-066
    #[serde(default = "default_spin_height_curve")]
    pub spin_height_curve: Vec<SpinCurvePoint>,
    /// スピンカーブ（タイミング）[(経過時間, スピン係数)]
    /// @spec 30604_shot_attributes_spec.md#req-30604-066
    #[serde(default = "default_spin_timing_curve")]
    pub spin_timing_curve: Vec<SpinCurvePoint>,
}

impl Default for ShotAttributesConfig {
    fn default() -> Self {
        Self {
            push_perfect_window: default_push_perfect_window(),
            push_to_hold_threshold: default_push_to_hold_threshold(),
            hold_stable_time: default_hold_stable_time(),
            hold_power_factor: default_hold_power_factor(),
            optimal_distance: default_optimal_distance(),
            stability_threshold: default_stability_threshold(),
            max_direction_error: default_max_direction_error(),
            base_power: default_base_power(),
            base_stability: default_base_stability(),
            base_angle: default_base_angle(),
            base_accuracy: default_base_accuracy(),
            height_curve: default_height_curve(),
            timing_curve: default_timing_curve(),
            approach_curve: default_approach_curve(),
            distance_curve: default_distance_curve(),
            volley_factors: default_volley_factors(),
            spin_height_curve: default_spin_height_curve(),
            spin_timing_curve: default_spin_timing_curve(),
        }
    }
}

/// 打点高さカーブのポイント
#[derive(Deserialize, Clone, Debug)]
pub struct HeightCurvePoint {
    pub height: f32,
    pub power_factor: f32,
    pub stability_factor: f32,
    pub angle_offset: f32,
}

/// タイミングカーブのポイント
#[derive(Deserialize, Clone, Debug)]
pub struct TimingCurvePoint {
    pub elapsed: f32,
    pub power_factor: f32,
    pub stability_factor: f32,
    pub angle_offset: f32,
}

/// 入り方カーブのポイント
#[derive(Deserialize, Clone, Debug)]
pub struct ApproachCurvePoint {
    pub dot: f32,
    pub power_factor: f32,
    pub angle_offset: f32,
}

/// 距離カーブのポイント
#[derive(Deserialize, Clone, Debug)]
pub struct DistanceCurvePoint {
    pub distance: f32,
    pub power_factor: f32,
    pub stability_factor: f32,
    pub accuracy_factor: f32,
}

/// ボレー補正
#[derive(Deserialize, Clone, Debug)]
pub struct VolleyFactors {
    pub power_factor: f32,
    pub stability_factor: f32,
    pub angle_offset: f32,
}

impl Default for VolleyFactors {
    fn default() -> Self {
        default_volley_factors()
    }
}

/// スピンカーブのポイント
#[derive(Deserialize, Clone, Debug)]
pub struct SpinCurvePoint {
    pub value: f32,
    pub spin_factor: f32,
}

// === デフォルト値関数 ===

fn default_push_perfect_window() -> f32 {
    50.0
}
fn default_push_to_hold_threshold() -> f32 {
    150.0
}
fn default_hold_stable_time() -> f32 {
    200.0
}
fn default_hold_power_factor() -> f32 {
    0.6
}
fn default_optimal_distance() -> f32 {
    1.0
}
fn default_stability_threshold() -> f32 {
    0.3
}
fn default_max_direction_error() -> f32 {
    15.0
}
fn default_base_power() -> f32 {
    15.0
}
fn default_base_stability() -> f32 {
    1.0
}
fn default_base_angle() -> f32 {
    15.0
}
fn default_base_accuracy() -> f32 {
    1.0
}

/// 打点高さカーブのデフォルト値
/// @spec 30604_shot_attributes_spec.md#req-30604-055
fn default_height_curve() -> Vec<HeightCurvePoint> {
    vec![
        HeightCurvePoint { height: 0.0, power_factor: 0.3, stability_factor: 0.5, angle_offset: 30.0 },
        HeightCurvePoint { height: 0.5, power_factor: 0.5, stability_factor: 0.7, angle_offset: 20.0 },
        HeightCurvePoint { height: 1.0, power_factor: 0.7, stability_factor: 1.0, angle_offset: 10.0 },
        HeightCurvePoint { height: 1.5, power_factor: 0.9, stability_factor: 0.9, angle_offset: 0.0 },
        HeightCurvePoint { height: 2.0, power_factor: 1.0, stability_factor: 0.8, angle_offset: -15.0 },
        HeightCurvePoint { height: 2.5, power_factor: 1.0, stability_factor: 0.7, angle_offset: -30.0 },
    ]
}

/// タイミングカーブのデフォルト値
/// @spec 30604_shot_attributes_spec.md#req-30604-058
fn default_timing_curve() -> Vec<TimingCurvePoint> {
    vec![
        TimingCurvePoint { elapsed: 0.0, power_factor: 1.0, stability_factor: 0.6, angle_offset: -5.0 },
        TimingCurvePoint { elapsed: 0.3, power_factor: 0.9, stability_factor: 0.8, angle_offset: 0.0 },
        TimingCurvePoint { elapsed: 0.5, power_factor: 0.8, stability_factor: 1.0, angle_offset: 0.0 },
        TimingCurvePoint { elapsed: 0.8, power_factor: 0.6, stability_factor: 0.9, angle_offset: 10.0 },
        TimingCurvePoint { elapsed: 1.0, power_factor: 0.4, stability_factor: 0.7, angle_offset: 20.0 },
    ]
}

/// 入り方カーブのデフォルト値
/// @spec 30604_shot_attributes_spec.md#req-30604-060
fn default_approach_curve() -> Vec<ApproachCurvePoint> {
    vec![
        ApproachCurvePoint { dot: -1.0, power_factor: 0.7, angle_offset: 20.0 },
        ApproachCurvePoint { dot: 0.0, power_factor: 1.0, angle_offset: 0.0 },
        ApproachCurvePoint { dot: 1.0, power_factor: 1.2, angle_offset: -10.0 },
    ]
}

/// 距離カーブのデフォルト値
/// @spec 30604_shot_attributes_spec.md#req-30604-062
fn default_distance_curve() -> Vec<DistanceCurvePoint> {
    vec![
        DistanceCurvePoint { distance: 0.5, power_factor: 1.1, stability_factor: 1.1, accuracy_factor: 1.1 },
        DistanceCurvePoint { distance: 1.0, power_factor: 1.0, stability_factor: 1.0, accuracy_factor: 1.0 },
        DistanceCurvePoint { distance: 1.5, power_factor: 0.7, stability_factor: 0.7, accuracy_factor: 0.7 },
        DistanceCurvePoint { distance: 2.0, power_factor: 0.4, stability_factor: 0.4, accuracy_factor: 0.4 },
    ]
}

/// ボレー補正のデフォルト値
/// @spec 30604_shot_attributes_spec.md#req-30604-057
fn default_volley_factors() -> VolleyFactors {
    VolleyFactors {
        power_factor: 0.8,
        stability_factor: 0.7,
        angle_offset: 0.0,
    }
}

/// スピンカーブ（高さ）のデフォルト値
/// @spec 30604_shot_attributes_spec.md#req-30604-066
fn default_spin_height_curve() -> Vec<SpinCurvePoint> {
    vec![
        SpinCurvePoint { value: 0.5, spin_factor: -0.5 },
        SpinCurvePoint { value: 1.0, spin_factor: 0.0 },
        SpinCurvePoint { value: 2.0, spin_factor: 0.5 },
    ]
}

/// スピンカーブ（タイミング）のデフォルト値
/// @spec 30604_shot_attributes_spec.md#req-30604-066
fn default_spin_timing_curve() -> Vec<SpinCurvePoint> {
    vec![
        SpinCurvePoint { value: 0.0, spin_factor: 0.3 },
        SpinCurvePoint { value: 0.3, spin_factor: 0.15 },
        SpinCurvePoint { value: 0.5, spin_factor: 0.0 },
        SpinCurvePoint { value: 0.8, spin_factor: -0.15 },
        SpinCurvePoint { value: 1.0, spin_factor: -0.3 },
    ]
}

/// サーブサイド
/// @spec 30903_serve_authority_spec.md#req-30903-003
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Deserialize)]
pub enum ServeSide {
    /// デュースサイド（右側）- ポイント合計が偶数
    #[default]
    Deuce,
    /// アドバンテージサイド（左側）- ポイント合計が奇数
    Ad,
}

impl ServeSide {
    /// ポイント合計からサーブサイドを判定
    /// @spec 30903_serve_authority_spec.md#req-30903-003
    #[inline]
    pub fn from_point_total(total: usize) -> Self {
        if total % 2 == 0 {
            ServeSide::Deuce
        } else {
            ServeSide::Ad
        }
    }
}

/// RONファイルからGameConfigをロード
pub fn load_game_config(path: &str) -> Result<GameConfig, String> {
    let config_str =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;
    ron::from_str(&config_str).map_err(|e| format!("Failed to parse config: {}", e))
}

// ============================================================================
// ホットリロード対応
// @spec 30026: GameConfig ホットリロード対応
// ============================================================================

/// GameConfig のハンドルを保持するリソース
#[derive(Resource)]
pub struct GameConfigHandle(pub Handle<GameConfig>);

/// GameConfig の RON ファイルをロードするカスタム AssetLoader
#[derive(Default)]
pub struct GameConfigLoader;

/// GameConfigLoader のエラー型
#[derive(Debug, thiserror::Error)]
pub enum GameConfigLoaderError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

impl AssetLoader for GameConfigLoader {
    type Asset = GameConfig;
    type Settings = ();
    type Error = GameConfigLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let config: GameConfig = ron::de::from_bytes(&bytes)?;
        Ok(config)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
