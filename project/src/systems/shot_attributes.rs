//! ショット属性計算システム
//! @spec 30604_shot_attributes_spec.md

use bevy::prelude::*;

use crate::components::{Ball, BounceState, InputMode, ShotAttributes, ShotContext};
use crate::core::events::GroundBounceEvent;
use crate::resource::config::{
    ApproachCurvePoint, DistanceCurvePoint, HeightCurvePoint, ShotAttributesConfig,
    SpinCurvePoint, TimingCurvePoint,
};

// ============================================================================
// 線形補間ユーティリティ
// ============================================================================

/// 2点間で線形補間
/// @spec 30604_shot_attributes_spec.md
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// カーブポイントから値を線形補間で取得
/// ポイントはソート済みと仮定
fn interpolate_from_curve<T, F, G>(points: &[T], value: f32, get_key: F, get_value: G) -> f32
where
    F: Fn(&T) -> f32,
    G: Fn(&T) -> f32,
{
    if points.is_empty() {
        return 1.0;
    }
    if points.len() == 1 {
        return get_value(&points[0]);
    }

    // 範囲外の場合はクランプ
    let first_key = get_key(&points[0]);
    if value <= first_key {
        return get_value(&points[0]);
    }
    let last_key = get_key(&points[points.len() - 1]);
    if value >= last_key {
        return get_value(&points[points.len() - 1]);
    }

    // 2点間を探索して補間
    for window in points.windows(2) {
        let k0 = get_key(&window[0]);
        let k1 = get_key(&window[1]);
        if value >= k0 && value <= k1 {
            let t = (value - k0) / (k1 - k0);
            return lerp(get_value(&window[0]), get_value(&window[1]), t);
        }
    }

    // フォールバック
    get_value(&points[0])
}

// ============================================================================
// 入力方式判定
// @spec 30604_shot_attributes_spec.md#req-30604-050
// @spec 30604_shot_attributes_spec.md#req-30604-051
// ============================================================================

/// ボタン押下状態を追跡するリソース
/// @spec 30604_shot_attributes_spec.md#req-30604-051
/// @spec 30802_visual_feedback_spec.md#req-30802-001
#[derive(Resource, Default)]
pub struct ShotButtonState {
    /// Player 1 がボタンを押し続けている時間（ミリ秒）
    pub player1_hold_time: f32,
    /// Player 2 がボタンを押し続けている時間（ミリ秒）
    pub player2_hold_time: f32,
    /// Player 1 がボタンを押しているかどうか
    /// @spec 30802_visual_feedback_spec.md#req-30802-001
    pub player1_holding: bool,
    /// Player 2 がボタンを押しているかどうか
    /// @spec 30802_visual_feedback_spec.md#req-30802-001
    pub player2_holding: bool,
}

/// ボタン押下状態追跡システム
/// @spec 30604_shot_attributes_spec.md#req-30604-051
/// @spec 30802_visual_feedback_spec.md#req-30802-001
pub fn track_shot_button_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut button_state: ResMut<ShotButtonState>,
) {
    let delta_ms = time.delta_secs() * 1000.0;

    // Vキーの状態を追跡（Player1のみ）
    // Player2（AI）はキー入力を使わないため、ホールド状態は更新しない
    if keyboard.pressed(KeyCode::KeyV) {
        if !button_state.player1_holding {
            // 押し始め
            button_state.player1_holding = true;
            button_state.player1_hold_time = 0.0;
        } else {
            // 押し続けている
            button_state.player1_hold_time += delta_ms;
        }
    } else {
        // 離した
        button_state.player1_holding = false;
        // hold_time はショット実行時に参照されるのでリセットしない
    }
}

/// 入力方式を判定
/// @spec 30604_shot_attributes_spec.md#req-30604-050
/// @spec 30604_shot_attributes_spec.md#req-30604-051
pub fn determine_input_mode(
    button_state: &ShotButtonState,
    player_id: u8,
    config: &ShotAttributesConfig,
) -> (InputMode, f32, f32) {
    let hold_time = match player_id {
        1 => button_state.player1_hold_time,
        2 => button_state.player2_hold_time,
        _ => 0.0,
    };

    // ホールド時間が閾値以上ならホールド、そうでなければプッシュ
    if hold_time >= config.push_to_hold_threshold {
        (InputMode::Hold, 0.0, hold_time)
    } else {
        // プッシュの場合、タイミング差はhold_timeと等価
        (InputMode::Push, hold_time, hold_time)
    }
}

// ============================================================================
// 5要素の取得
// ============================================================================

/// ボールとの距離を計算（XZ平面）
/// @spec 30604_shot_attributes_spec.md#req-30604-061
#[inline]
pub fn calculate_ball_distance(player_pos: Vec3, ball_pos: Vec3) -> f32 {
    let dx = player_pos.x - ball_pos.x;
    let dz = player_pos.z - ball_pos.z;
    (dx * dx + dz * dz).sqrt()
}

/// 移動ベクトルとボール方向の内積を計算
/// @spec 30604_shot_attributes_spec.md#req-30604-059
pub fn calculate_approach_dot(player_velocity: Vec3, player_pos: Vec3, ball_pos: Vec3) -> f32 {
    let velocity_2d = Vec2::new(player_velocity.x, player_velocity.z);
    let to_ball = Vec2::new(ball_pos.x - player_pos.x, ball_pos.z - player_pos.z);

    if velocity_2d.length_squared() < f32::EPSILON || to_ball.length_squared() < f32::EPSILON {
        return 0.0;
    }

    velocity_2d.normalize().dot(to_ball.normalize())
}

// ============================================================================
// 係数計算
// ============================================================================

/// 打点高さから係数を取得
/// @spec 30604_shot_attributes_spec.md#req-30604-055
pub fn get_height_factors(
    height: f32,
    curve: &[HeightCurvePoint],
) -> (f32, f32, f32) {
    let power = interpolate_from_curve(curve, height, |p| p.height, |p| p.power_factor);
    let stability = interpolate_from_curve(curve, height, |p| p.height, |p| p.stability_factor);
    let angle = interpolate_from_curve(curve, height, |p| p.height, |p| p.angle_offset);
    (power, stability, angle)
}

/// バウンド経過時間から係数を取得
/// @spec 30604_shot_attributes_spec.md#req-30604-058
pub fn get_timing_factors(
    elapsed: f32,
    curve: &[TimingCurvePoint],
) -> (f32, f32, f32) {
    let power = interpolate_from_curve(curve, elapsed, |p| p.elapsed, |p| p.power_factor);
    let stability = interpolate_from_curve(curve, elapsed, |p| p.elapsed, |p| p.stability_factor);
    let angle = interpolate_from_curve(curve, elapsed, |p| p.elapsed, |p| p.angle_offset);
    (power, stability, angle)
}

/// 入り方から係数を取得
/// @spec 30604_shot_attributes_spec.md#req-30604-060
pub fn get_approach_factors(dot: f32, curve: &[ApproachCurvePoint]) -> (f32, f32) {
    let power = interpolate_from_curve(curve, dot, |p| p.dot, |p| p.power_factor);
    let angle = interpolate_from_curve(curve, dot, |p| p.dot, |p| p.angle_offset);
    (power, angle)
}

/// 距離から係数を取得
/// @spec 30604_shot_attributes_spec.md#req-30604-062
pub fn get_distance_factors(distance: f32, curve: &[DistanceCurvePoint]) -> (f32, f32, f32) {
    let power = interpolate_from_curve(curve, distance, |p| p.distance, |p| p.power_factor);
    let stability = interpolate_from_curve(curve, distance, |p| p.distance, |p| p.stability_factor);
    let accuracy = interpolate_from_curve(curve, distance, |p| p.distance, |p| p.accuracy_factor);
    (power, stability, accuracy)
}

/// スピン係数を取得（高さ）
/// @spec 30604_shot_attributes_spec.md#req-30604-066
pub fn get_spin_height_factor(height: f32, curve: &[SpinCurvePoint]) -> f32 {
    interpolate_from_curve(curve, height, |p| p.value, |p| p.spin_factor)
}

/// スピン係数を取得（タイミング）
/// @spec 30604_shot_attributes_spec.md#req-30604-066
pub fn get_spin_timing_factor(elapsed: f32, curve: &[SpinCurvePoint]) -> f32 {
    interpolate_from_curve(curve, elapsed, |p| p.value, |p| p.spin_factor)
}

// ============================================================================
// 入力方式による係数
// ============================================================================

/// プッシュ精度による威力係数を計算
/// @spec 30604_shot_attributes_spec.md#req-30604-053
pub fn calculate_push_power_factor(timing_diff: f32, config: &ShotAttributesConfig) -> f32 {
    if timing_diff >= config.push_to_hold_threshold {
        // ホールド扱い
        config.hold_power_factor
    } else {
        // 威力係数 = 1.0 - (timing_diff / threshold) * 0.3
        1.0 - (timing_diff / config.push_to_hold_threshold) * 0.3
    }
}

/// ホールド時間による安定性係数を計算
/// @spec 30604_shot_attributes_spec.md#req-30604-052
pub fn calculate_hold_stability_factor(hold_time: f32, config: &ShotAttributesConfig) -> f32 {
    // stability_factor = min(1.0, 0.5 + (hold_time / hold_stable_time) * 0.5)
    (0.5 + (hold_time / config.hold_stable_time) * 0.5).min(1.0)
}

// ============================================================================
// 属性計算メイン
// ============================================================================

/// ShotContext から ShotAttributes を計算
/// @spec 30604_shot_attributes_spec.md#req-30604-063
/// @spec 30604_shot_attributes_spec.md#req-30604-064
/// @spec 30604_shot_attributes_spec.md#req-30604-065
/// @spec 30604_shot_attributes_spec.md#req-30604-066
/// @spec 30604_shot_attributes_spec.md#req-30604-067
pub fn calculate_shot_attributes(context: &ShotContext, config: &ShotAttributesConfig) -> ShotAttributes {
    // 入力方式による係数
    let (input_power_factor, input_stability_factor) = match context.input_mode {
        InputMode::Push => {
            let power = calculate_push_power_factor(context.push_timing_diff, config);
            // プッシュは安定性が低い（精度と引き換えに威力）
            let stability = 0.8 - (1.0 - power) * 0.5;
            (power, stability)
        }
        InputMode::Hold => {
            let power = config.hold_power_factor;
            let stability = calculate_hold_stability_factor(context.hold_duration, config);
            (power, stability)
        }
    };

    // 打点高さによる係数
    let (height_power, height_stability, height_angle) =
        get_height_factors(context.hit_height, &config.height_curve);

    // タイミング（バウンド経過時間）による係数
    let (timing_power, timing_stability, timing_angle) = match context.bounce_elapsed {
        Some(elapsed) => get_timing_factors(elapsed, &config.timing_curve),
        None => {
            // ボレー
            (
                config.volley_factors.power_factor,
                config.volley_factors.stability_factor,
                config.volley_factors.angle_offset,
            )
        }
    };

    // 入り方による係数
    let (approach_power, approach_angle) =
        get_approach_factors(context.approach_dot, &config.approach_curve);

    // 距離による係数
    let (distance_power, distance_stability, distance_accuracy) =
        get_distance_factors(context.ball_distance, &config.distance_curve);

    // 威力の最終計算
    // @spec 30604_shot_attributes_spec.md#req-30604-063
    let power = config.base_power
        * input_power_factor
        * height_power
        * timing_power
        * approach_power
        * distance_power;

    // 安定性の最終計算
    // @spec 30604_shot_attributes_spec.md#req-30604-064
    let stability = config.base_stability
        * input_stability_factor
        * height_stability
        * timing_stability
        * distance_stability;

    // 角度の最終計算
    // @spec 30604_shot_attributes_spec.md#req-30604-065
    let angle = config.base_angle + height_angle + timing_angle + approach_angle;

    // スピンの計算
    // @spec 30604_shot_attributes_spec.md#req-30604-066
    let spin_height = get_spin_height_factor(context.hit_height, &config.spin_height_curve);
    let spin_timing = match context.bounce_elapsed {
        Some(elapsed) => get_spin_timing_factor(elapsed, &config.spin_timing_curve),
        None => 0.0, // ボレーはスピンなし
    };
    let spin = (spin_height + spin_timing).clamp(-1.0, 1.0);

    // 精度の最終計算
    // @spec 30604_shot_attributes_spec.md#req-30604-067
    let accuracy = config.base_accuracy * distance_accuracy;

    ShotAttributes {
        power,
        stability,
        angle,
        spin,
        accuracy,
    }
}

// ============================================================================
// BounceState 更新システム
// ============================================================================

/// BounceState のタイマーを更新するシステム
/// @spec 30604_shot_attributes_spec.md#req-30604-056
/// v0.2 で使用予定
#[allow(dead_code)]
pub fn update_bounce_state_timer_system(time: Res<Time>, mut query: Query<&mut BounceState, With<Ball>>) {
    let delta = time.delta_secs();

    for mut bounce_state in query.iter_mut() {
        if let Some(ref mut elapsed) = bounce_state.time_since_bounce {
            *elapsed += delta;
        }
    }
}

/// GroundBounceEvent を受けて BounceState をリセットするシステム
/// @spec 30604_shot_attributes_spec.md#req-30604-056
/// v0.2 で使用予定
#[allow(dead_code)]
pub fn handle_ground_bounce_event_system(
    mut reader: MessageReader<GroundBounceEvent>,
    mut query: Query<&mut BounceState, With<Ball>>,
) {
    for _event in reader.read() {
        for mut bounce_state in query.iter_mut() {
            // バウンドしたらタイマーをリセット
            bounce_state.time_since_bounce = Some(0.0);
            info!("Ball bounced, reset bounce timer");
        }
    }
}

/// ショットされたときに BounceState をリセットするシステム
/// @spec 30604_shot_attributes_spec.md#req-30604-056
/// v0.2 で使用予定
#[allow(dead_code)]
pub fn reset_bounce_state_on_shot_system(mut query: Query<&mut BounceState, With<Ball>>) {
    // このシステムはショットイベント時に手動で呼び出すか、
    // ShotExecutedEvent をトリガーにする
    for mut bounce_state in query.iter_mut() {
        bounce_state.time_since_bounce = None;
    }
}

// ============================================================================
// ShotContext 構築ヘルパー
// ============================================================================

/// 現在の状態から ShotContext を構築する
/// @spec 30604_shot_attributes_spec.md
pub fn build_shot_context(
    button_state: &ShotButtonState,
    player_id: u8,
    player_pos: Vec3,
    player_velocity: Vec3,
    ball_pos: Vec3,
    ball_bounce_state: &BounceState,
    config: &ShotAttributesConfig,
) -> ShotContext {
    let (input_mode, push_timing_diff, hold_duration) =
        determine_input_mode(button_state, player_id, config);

    let ball_distance = calculate_ball_distance(player_pos, ball_pos);
    let approach_dot = calculate_approach_dot(player_velocity, player_pos, ball_pos);

    ShotContext {
        input_mode,
        push_timing_diff,
        hold_duration,
        hit_height: ball_pos.y,
        bounce_elapsed: ball_bounce_state.time_since_bounce,
        approach_dot,
        ball_distance,
    }
}

// ============================================================================
// テスト
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30604-001: 線形補間テスト
    #[test]
    fn test_lerp() {
        assert!((lerp(0.0, 10.0, 0.5) - 5.0).abs() < 0.001);
        assert!((lerp(0.0, 10.0, 0.0) - 0.0).abs() < 0.001);
        assert!((lerp(0.0, 10.0, 1.0) - 10.0).abs() < 0.001);
    }

    /// TST-30604-002: カーブ補間テスト
    #[test]
    fn test_interpolate_from_curve() {
        let points = vec![
            HeightCurvePoint {
                height: 0.0,
                power_factor: 0.3,
                stability_factor: 0.5,
                angle_offset: 30.0,
            },
            HeightCurvePoint {
                height: 1.0,
                power_factor: 0.7,
                stability_factor: 1.0,
                angle_offset: 10.0,
            },
            HeightCurvePoint {
                height: 2.0,
                power_factor: 1.0,
                stability_factor: 0.8,
                angle_offset: -15.0,
            },
        ];

        // 境界値
        let power_at_0 = interpolate_from_curve(&points, 0.0, |p| p.height, |p| p.power_factor);
        assert!((power_at_0 - 0.3).abs() < 0.001);

        // 中間値
        let power_at_0_5 = interpolate_from_curve(&points, 0.5, |p| p.height, |p| p.power_factor);
        assert!((power_at_0_5 - 0.5).abs() < 0.001); // (0.3 + 0.7) / 2 = 0.5

        // 範囲外（下限）
        let power_below = interpolate_from_curve(&points, -1.0, |p| p.height, |p| p.power_factor);
        assert!((power_below - 0.3).abs() < 0.001);

        // 範囲外（上限）
        let power_above = interpolate_from_curve(&points, 3.0, |p| p.height, |p| p.power_factor);
        assert!((power_above - 1.0).abs() < 0.001);
    }

    /// TST-30604-003: プッシュ威力係数テスト
    #[test]
    fn test_push_power_factor() {
        let config = ShotAttributesConfig::default();

        // 完璧タイミング
        let perfect = calculate_push_power_factor(0.0, &config);
        assert!((perfect - 1.0).abs() < 0.001);

        // 閾値ちょうど
        let threshold = calculate_push_power_factor(config.push_to_hold_threshold, &config);
        assert!((threshold - config.hold_power_factor).abs() < 0.001);

        // 中間
        let mid = calculate_push_power_factor(config.push_to_hold_threshold / 2.0, &config);
        assert!((mid - 0.85).abs() < 0.001);
    }

    /// TST-30604-004: ホールド安定性係数テスト
    #[test]
    fn test_hold_stability_factor() {
        let config = ShotAttributesConfig::default();

        // 0ms
        let at_0 = calculate_hold_stability_factor(0.0, &config);
        assert!((at_0 - 0.5).abs() < 0.001);

        // 頭打ち
        let at_max = calculate_hold_stability_factor(config.hold_stable_time * 2.0, &config);
        assert!((at_max - 1.0).abs() < 0.001);

        // 中間
        let at_half = calculate_hold_stability_factor(config.hold_stable_time / 2.0, &config);
        assert!((at_half - 0.75).abs() < 0.001);
    }

    /// TST-30604-005: 距離計算テスト
    #[test]
    fn test_ball_distance() {
        let player = Vec3::new(0.0, 0.0, 0.0);
        let ball = Vec3::new(1.0, 5.0, 0.0); // Y は無視

        let distance = calculate_ball_distance(player, ball);
        assert!((distance - 1.0).abs() < 0.001);
    }

    /// TST-30604-006: 内積計算テスト
    #[test]
    fn test_approach_dot() {
        let player_pos = Vec3::new(0.0, 0.0, 0.0);
        let ball_pos = Vec3::new(1.0, 0.0, 0.0);

        // 前進
        let forward_vel = Vec3::new(1.0, 0.0, 0.0);
        let forward_dot = calculate_approach_dot(forward_vel, player_pos, ball_pos);
        assert!((forward_dot - 1.0).abs() < 0.001);

        // 後退
        let backward_vel = Vec3::new(-1.0, 0.0, 0.0);
        let backward_dot = calculate_approach_dot(backward_vel, player_pos, ball_pos);
        assert!((backward_dot - (-1.0)).abs() < 0.001);

        // 静止
        let zero_vel = Vec3::ZERO;
        let zero_dot = calculate_approach_dot(zero_vel, player_pos, ball_pos);
        assert!((zero_dot - 0.0).abs() < 0.001);
    }

    /// TST-30604-007: 属性計算統合テスト
    #[test]
    fn test_calculate_shot_attributes() {
        let config = ShotAttributesConfig::default();

        // 標準的なコンテキスト
        let context = ShotContext {
            input_mode: InputMode::Push,
            push_timing_diff: 0.0, // 完璧
            hold_duration: 0.0,
            hit_height: 1.0,  // 最適高さ
            bounce_elapsed: Some(0.5), // 頂点
            approach_dot: 0.0, // 静止
            ball_distance: 1.0, // 最適距離
        };

        let attrs = calculate_shot_attributes(&context, &config);

        // 値が妥当な範囲にあることを確認
        assert!(attrs.power > 0.0);
        assert!(attrs.stability > 0.0);
        assert!(attrs.accuracy > 0.0);
        assert!(attrs.spin >= -1.0 && attrs.spin <= 1.0);
    }
}
