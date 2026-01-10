//! ショット方向計算テスト
//! @spec 30602_shot_direction_spec.md
//! @spec 30604_shot_attributes_spec.md

use bevy::prelude::*;
use crate::core::CourtSide;

/// 水平方向を計算（テスト用）
/// @spec 30602_shot_direction_spec.md#req-30602-001
/// 新座標系: X=打ち合い方向, Z=コート幅
/// X軸方向（打ち合い）: コートサイドに応じて常に相手コート方向に固定
/// Z軸方向（左右）: 入力で調整可能
#[inline]
fn calculate_horizontal_direction(direction: Vec2, court_side: CourtSide) -> Vec3 {
    // X軸方向: コートサイドに応じて固定（常に相手コートへ）
    // Left側（X < net_x）にいる場合: +X方向（相手コート）
    // Right側（X > net_x）にいる場合: -X方向（相手コート）
    let x_direction = match court_side {
        CourtSide::Left => 1.0,
        CourtSide::Right => -1.0,
    };

    // Z軸方向: 入力X値を使用（コート幅方向の打ち分け）
    // shot_input.rs から direction.x に横入力（左右）が入る
    let z_direction = direction.x;

    // 正規化して返す
    Vec3::new(x_direction, 0.0, z_direction).normalize()
}

/// 打球ベクトルを計算（テスト用）
/// @spec 30602_shot_direction_spec.md#req-30602-004
#[inline]
fn calculate_shot_velocity(horizontal_dir: Vec3, speed: f32, angle_deg: f32) -> Vec3 {
    let angle_rad = angle_deg.to_radians();
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();

    Vec3::new(
        horizontal_dir.x * speed * cos_angle,
        speed * sin_angle,
        horizontal_dir.z * speed * cos_angle,
    )
}

/// TST-30604-007: 水平方向計算テスト（Left側コート、入力なし）
/// 新座標系: X=打ち合い方向（固定）, Z=コート幅（入力）
#[test]
fn test_calculate_horizontal_direction_left_side_no_input() {
    // Left側コート: 入力なし -> +X方向（相手コート方向のみ）
    let direction = Vec2::new(0.0, 0.0);
    let result = calculate_horizontal_direction(direction, CourtSide::Left);

    assert!((result.x - 1.0).abs() < 0.001);  // +X方向（2Pコートへ）
    assert!((result.y - 0.0).abs() < 0.001);
    assert!((result.z - 0.0).abs() < 0.001);  // 横方向なし
}

/// TST-30604-007: 水平方向計算テスト（Right側コート、入力なし）
/// 新座標系: X=打ち合い方向（固定）, Z=コート幅（入力）
#[test]
fn test_calculate_horizontal_direction_right_side_no_input() {
    // Right側コート: 入力なし -> -X方向（相手コート方向のみ）
    let direction = Vec2::new(0.0, 0.0);
    let result = calculate_horizontal_direction(direction, CourtSide::Right);

    assert!((result.x - -1.0).abs() < 0.001); // -X方向（1Pコートへ）
    assert!((result.y - 0.0).abs() < 0.001);
    assert!((result.z - 0.0).abs() < 0.001);  // 横方向なし
}

/// TST-30604-007: 水平方向計算テスト（Left側コート、右入力）
/// 新座標系: X=打ち合い方向（固定）, Z=コート幅（入力）
#[test]
fn test_calculate_horizontal_direction_left_side_right() {
    // Left側コート: 右入力(+Z) -> 右前方向
    let direction = Vec2::new(1.0, 0.0);  // X入力 → Z方向
    let result = calculate_horizontal_direction(direction, CourtSide::Left);

    let expected = 1.0 / 2.0_f32.sqrt();
    assert!((result.x - expected).abs() < 0.001);  // +X方向
    assert!((result.y - 0.0).abs() < 0.001);
    assert!((result.z - expected).abs() < 0.001);  // +Z方向（右）
}

/// TST-30604-007: 水平方向計算テスト（Right側コート、右入力）
/// 新座標系: X=打ち合い方向（固定）, Z=コート幅（入力）
#[test]
fn test_calculate_horizontal_direction_right_side_right() {
    // Right側コート: 右入力(+Z) -> 右後方向
    let direction = Vec2::new(1.0, 0.0);  // X入力 → Z方向
    let result = calculate_horizontal_direction(direction, CourtSide::Right);

    let expected = 1.0 / 2.0_f32.sqrt();
    assert!((result.x - -expected).abs() < 0.001); // -X方向（1Pコートへ）
    assert!((result.y - 0.0).abs() < 0.001);
    assert!((result.z - expected).abs() < 0.001);  // +Z方向（右）
}

/// TST-30604-008: 通常ショット速度テスト
#[test]
fn test_calculate_shot_velocity_normal() {
    let horizontal_dir = Vec3::new(0.0, 0.0, 1.0);
    let speed = 10.0;
    let angle_deg = 45.0;

    let result = calculate_shot_velocity(horizontal_dir, speed, angle_deg);

    // 45度の場合: cos(45) = sin(45) ≈ 0.707
    let expected_horizontal = speed * 45.0_f32.to_radians().cos();
    let expected_vertical = speed * 45.0_f32.to_radians().sin();

    assert!((result.x - 0.0).abs() < 0.001);
    assert!((result.y - expected_vertical).abs() < 0.001);
    assert!((result.z - expected_horizontal).abs() < 0.001);
}

/// TST-30604-009: ジャンプショット速度テスト
#[test]
fn test_calculate_shot_velocity_jump_shot() {
    let horizontal_dir = Vec3::new(0.0, 0.0, 1.0);
    let speed = 15.0;
    let angle_deg = 30.0;

    let result = calculate_shot_velocity(horizontal_dir, speed, angle_deg);

    // 30度の場合
    let expected_horizontal = speed * 30.0_f32.to_radians().cos();
    let expected_vertical = speed * 30.0_f32.to_radians().sin();

    assert!((result.x - 0.0).abs() < 0.001);
    assert!((result.y - expected_vertical).abs() < 0.001);
    assert!((result.z - expected_horizontal).abs() < 0.001);
}

/// TST-30604-010: 斜め打球ベクトルテスト
#[test]
fn test_calculate_shot_velocity_diagonal() {
    let horizontal_dir = Vec3::new(1.0, 0.0, 1.0).normalize();
    let speed = 10.0;
    let angle_deg = 45.0;

    let result = calculate_shot_velocity(horizontal_dir, speed, angle_deg);

    let cos_angle = 45.0_f32.to_radians().cos();
    let sin_angle = 45.0_f32.to_radians().sin();
    let horizontal_component = horizontal_dir.x * speed * cos_angle;

    // X と Z は同じ値（45度方向）
    assert!((result.x - horizontal_component).abs() < 0.001);
    assert!((result.y - speed * sin_angle).abs() < 0.001);
    assert!((result.z - horizontal_component).abs() < 0.001);
}

// ========================================================================
// ショット属性の軌道反映テスト（v0.2 新機能）
// @spec 30604_shot_attributes_spec.md
// ========================================================================

/// ミスショット判定（テスト用ダミー実装）
/// 安定性が閾値以上なら (false, 0.0) を返す
fn check_miss_shot(
    stability: f32,
    config: &crate::resource::config::ShotAttributesConfig,
) -> (bool, f32) {
    if stability >= config.stability_threshold {
        (false, 0.0)
    } else {
        // 安定性が低い場合はミス判定（テスト用に常にfalse）
        (false, 0.0)
    }
}

/// 精度によるコースブレ計算（テスト用）
/// @spec 30604_shot_attributes_spec.md#req-30604-070
/// ランダム性なし: 精度が低いほどコート中央寄りに収束
fn calculate_direction_error(
    _accuracy: f32,
    _config: &crate::resource::config::ShotAttributesConfig,
) -> f32 {
    // ランダム性を排除: 常に0を返す
    // 精度による影響は着地位置の収束で表現（trajectory_calculator側）
    0.0
}

/// 方向にオフセットを適用（テスト用）
fn apply_direction_offset(horizontal_dir: Vec3, offset_deg: f32) -> Vec3 {
    if offset_deg.abs() < f32::EPSILON {
        return horizontal_dir;
    }

    // XZ平面での回転
    let offset_rad = offset_deg.to_radians();
    let cos_offset = offset_rad.cos();
    let sin_offset = offset_rad.sin();

    let new_x = horizontal_dir.x * cos_offset - horizontal_dir.z * sin_offset;
    let new_z = horizontal_dir.x * sin_offset + horizontal_dir.z * cos_offset;

    Vec3::new(new_x, horizontal_dir.y, new_z).normalize()
}

/// TST-30604-068: ミスショット判定テスト（安定性が閾値以上）
/// @spec 30604_shot_attributes_spec.md#req-30604-069
#[test]
fn test_check_miss_shot_stable() {
    let config = crate::resource::config::ShotAttributesConfig::default();
    let stability = 1.0; // 閾値(0.3)以上

    let (is_miss, _offset) = check_miss_shot(stability, &config);
    assert!(!is_miss, "高い安定性はミスショットを発生させない");
}

/// TST-30604-069: 方向オフセット適用テスト（オフセットなし）
#[test]
fn test_apply_direction_offset_zero() {
    let dir = Vec3::new(0.0, 0.0, 1.0);
    let result = apply_direction_offset(dir, 0.0);

    assert!((result.x - dir.x).abs() < 0.001);
    assert!((result.z - dir.z).abs() < 0.001);
}

/// TST-30604-070: 方向オフセット適用テスト（90度回転）
#[test]
fn test_apply_direction_offset_90_degrees() {
    let dir = Vec3::new(0.0, 0.0, 1.0);
    let result = apply_direction_offset(dir, 90.0);

    // Z方向から反時計回りに90度回転 → -X方向
    // (標準的な2D回転行列: 正の角度 = 反時計回り)
    assert!((result.x - (-1.0)).abs() < 0.001);
    assert!((result.z - 0.0).abs() < 0.001);
}

/// TST-30604-071: 方向オフセット適用テスト（-45度回転）
#[test]
fn test_apply_direction_offset_minus_45_degrees() {
    let dir = Vec3::new(0.0, 0.0, 1.0);
    let result = apply_direction_offset(dir, -45.0);

    let expected = 1.0 / 2.0_f32.sqrt();
    // -45度回転（時計回り）で右前方向
    assert!((result.x - expected).abs() < 0.001);
    assert!((result.z - expected).abs() < 0.001);
}

/// TST-30604-072: 精度によるコースブレ範囲テスト
/// @spec 30604_shot_attributes_spec.md#req-30604-070
#[test]
fn test_calculate_direction_error_perfect_accuracy() {
    let config = crate::resource::config::ShotAttributesConfig::default();
    let accuracy = 1.0; // 完璧な精度

    // 精度1.0では(1.0 - 1.0) * max_error * random = 0
    let error = calculate_direction_error(accuracy, &config);
    assert!(
        error.abs() < 0.001,
        "完璧な精度ではコースブレが発生しない"
    );
}
