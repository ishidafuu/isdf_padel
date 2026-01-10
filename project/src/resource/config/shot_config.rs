//! ショットシステムパラメータ
//! @data 80101_game_constants.md#shot-config
//! @spec 30604_shot_attributes_spec.md
//! @spec 30605_trajectory_calculation_spec.md

use serde::Deserialize;

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
    /// TODO: v0.4着地点逆算型弾道システムで使用予定
    #[allow(dead_code)]
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

/// 弾道計算パラメータ
/// @spec 30605_trajectory_calculation_spec.md
#[derive(Deserialize, Clone, Debug)]
pub struct TrajectoryConfig {
    /// 着地マージン（コート端からの距離）
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-011
    #[serde(default = "default_landing_margin")]
    pub landing_margin: f32,
    /// デフォルト着地深さ（ニュートラル時のサービスライン付近）
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-010
    #[serde(default = "default_landing_depth")]
    pub default_landing_depth: f32,
    /// 最小発射角度（度）
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-022
    #[serde(default = "default_min_launch_angle")]
    pub min_launch_angle: f32,
    /// 最大発射角度（度）
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-022
    #[serde(default = "default_max_launch_angle")]
    pub max_launch_angle: f32,
    /// フラット時の初速係数
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-031
    #[serde(default = "default_spin_speed_flat")]
    pub spin_speed_flat: f32,
    /// トップスピン時の初速係数
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-031
    #[serde(default = "default_spin_speed_topspin")]
    pub spin_speed_topspin: f32,
    /// スライス時の初速係数
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-031
    #[serde(default = "default_spin_speed_slice")]
    pub spin_speed_slice: f32,
    /// 近距離時の初速係数
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-032
    #[serde(default = "default_distance_speed_min")]
    pub distance_speed_min: f32,
    /// 遠距離時の初速係数
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-032
    #[serde(default = "default_distance_speed_max")]
    pub distance_speed_max: f32,
    /// 最大着地ズレ（精度100%以外での偏差）
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-040
    #[serde(default = "default_max_landing_deviation")]
    pub max_landing_deviation: f32,
}

impl Default for TrajectoryConfig {
    fn default() -> Self {
        Self {
            landing_margin: default_landing_margin(),
            default_landing_depth: default_landing_depth(),
            min_launch_angle: default_min_launch_angle(),
            max_launch_angle: default_max_launch_angle(),
            spin_speed_flat: default_spin_speed_flat(),
            spin_speed_topspin: default_spin_speed_topspin(),
            spin_speed_slice: default_spin_speed_slice(),
            distance_speed_min: default_distance_speed_min(),
            distance_speed_max: default_distance_speed_max(),
            max_landing_deviation: default_max_landing_deviation(),
        }
    }
}

fn default_landing_margin() -> f32 {
    0.5
}
fn default_landing_depth() -> f32 {
    4.0
}
fn default_min_launch_angle() -> f32 {
    -90.0 // 下限は動的計算（ネット通過角度）に任せるため、実質的に無効化
}
fn default_max_launch_angle() -> f32 {
    60.0
}
fn default_spin_speed_flat() -> f32 {
    1.0
}
fn default_spin_speed_topspin() -> f32 {
    0.92
}
fn default_spin_speed_slice() -> f32 {
    0.88
}
fn default_distance_speed_min() -> f32 {
    1.0
}
fn default_distance_speed_max() -> f32 {
    1.15
}
fn default_max_landing_deviation() -> f32 {
    1.0
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
    pub power_bonus: f32,
    pub stability_factor: f32,
    pub angle_offset: f32,
}

/// タイミングカーブのポイント
#[derive(Deserialize, Clone, Debug)]
pub struct TimingCurvePoint {
    pub elapsed: f32,
    pub power_bonus: f32,
    pub stability_factor: f32,
    pub angle_offset: f32,
}

/// 入り方カーブのポイント
#[derive(Deserialize, Clone, Debug)]
pub struct ApproachCurvePoint {
    pub dot: f32,
    pub power_bonus: f32,
    pub angle_offset: f32,
}

/// 距離カーブのポイント
#[derive(Deserialize, Clone, Debug)]
pub struct DistanceCurvePoint {
    pub distance: f32,
    pub power_bonus: f32,
    pub stability_factor: f32,
    pub accuracy_factor: f32,
}

/// ボレー補正
#[derive(Deserialize, Clone, Debug)]
pub struct VolleyFactors {
    pub power_bonus: f32,
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
        HeightCurvePoint { height: 0.0, power_bonus: -3.0, stability_factor: 0.5, angle_offset: 30.0 },
        HeightCurvePoint { height: 0.5, power_bonus: -2.0, stability_factor: 0.7, angle_offset: 20.0 },
        HeightCurvePoint { height: 1.0, power_bonus: -1.0, stability_factor: 1.0, angle_offset: 10.0 },
        HeightCurvePoint { height: 1.5, power_bonus: 0.0, stability_factor: 0.9, angle_offset: 0.0 },
        HeightCurvePoint { height: 2.0, power_bonus: 2.0, stability_factor: 0.8, angle_offset: -15.0 },
        HeightCurvePoint { height: 2.5, power_bonus: 3.0, stability_factor: 0.7, angle_offset: -30.0 },
    ]
}

/// タイミングカーブのデフォルト値
/// @spec 30604_shot_attributes_spec.md#req-30604-058
fn default_timing_curve() -> Vec<TimingCurvePoint> {
    vec![
        TimingCurvePoint { elapsed: 0.0, power_bonus: 2.0, stability_factor: 0.6, angle_offset: -5.0 },
        TimingCurvePoint { elapsed: 0.3, power_bonus: 1.0, stability_factor: 0.8, angle_offset: 0.0 },
        TimingCurvePoint { elapsed: 0.5, power_bonus: 0.0, stability_factor: 1.0, angle_offset: 0.0 },
        TimingCurvePoint { elapsed: 0.8, power_bonus: -1.0, stability_factor: 0.9, angle_offset: 10.0 },
        TimingCurvePoint { elapsed: 1.0, power_bonus: -2.0, stability_factor: 0.7, angle_offset: 20.0 },
    ]
}

/// 入り方カーブのデフォルト値
/// @spec 30604_shot_attributes_spec.md#req-30604-060
fn default_approach_curve() -> Vec<ApproachCurvePoint> {
    vec![
        ApproachCurvePoint { dot: -1.0, power_bonus: -2.0, angle_offset: 20.0 },
        ApproachCurvePoint { dot: 0.0, power_bonus: 0.0, angle_offset: 0.0 },
        ApproachCurvePoint { dot: 1.0, power_bonus: 3.0, angle_offset: -10.0 },
    ]
}

/// 距離カーブのデフォルト値
/// @spec 30604_shot_attributes_spec.md#req-30604-062
fn default_distance_curve() -> Vec<DistanceCurvePoint> {
    vec![
        DistanceCurvePoint { distance: 0.5, power_bonus: 1.0, stability_factor: 1.1, accuracy_factor: 1.1 },
        DistanceCurvePoint { distance: 1.0, power_bonus: 0.0, stability_factor: 1.0, accuracy_factor: 1.0 },
        DistanceCurvePoint { distance: 1.5, power_bonus: -1.5, stability_factor: 0.7, accuracy_factor: 0.7 },
        DistanceCurvePoint { distance: 2.0, power_bonus: -3.0, stability_factor: 0.4, accuracy_factor: 0.4 },
    ]
}

/// ボレー補正のデフォルト値
/// @spec 30604_shot_attributes_spec.md#req-30604-057
fn default_volley_factors() -> VolleyFactors {
    VolleyFactors {
        power_bonus: -1.0,
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
