//! ボール着地予測システム
//! @spec 30301_ai_movement_spec.md#req-30301-v05

use bevy::prelude::*;

use crate::components::{Ball, BallPrediction, LogicalPosition, Velocity};
use crate::resource::config::GameConfig;

/// ボール着地予測システム
/// @spec 30301_ai_movement_spec.md#req-30301-v05
///
/// ボールの現在位置・速度から放物線運動の二次方程式を解いて
/// Y=0（着地）となる位置を予測する
pub fn ball_prediction_system(
    config: Res<GameConfig>,
    ball_query: Query<(&LogicalPosition, &Velocity), With<Ball>>,
    mut prediction_query: Query<&mut BallPrediction>,
) {
    // BallPredictionコンポーネントを持つエンティティが存在しない場合はスキップ
    let mut prediction = match prediction_query.iter_mut().next() {
        Some(p) => p,
        None => return,
    };

    // ボールが存在しない場合は予測を無効化
    let (ball_pos, ball_vel) = match ball_query.iter().next() {
        Some((pos, vel)) => (pos.value, vel.value),
        None => {
            prediction.is_valid = false;
            return;
        }
    };

    let gravity = config.physics.gravity; // 負の値（下向き）

    // 着地点を計算
    if let Some((landing_pos, time_to_landing)) =
        calculate_landing_position(ball_pos, ball_vel, gravity)
    {
        prediction.landing_position = landing_pos;
        prediction.time_to_landing = time_to_landing;
        prediction.is_valid = true;
    } else {
        // 着地できない場合（すでに地面上で上昇中など）は無効化
        prediction.is_valid = false;
    }
}

/// 着地位置を計算
/// 二次方程式を解いて Y=0 となる時間を求め、その時点でのXZ位置を返す
///
/// # 引数
/// - `position`: 現在位置
/// - `velocity`: 現在速度
/// - `gravity`: 重力加速度（負の値）
///
/// # 戻り値
/// - Some((landing_position, time_to_landing)): 着地位置と着地までの時間
/// - None: 着地できない場合
fn calculate_landing_position(
    position: Vec3,
    velocity: Vec3,
    gravity: f32,
) -> Option<(Vec3, f32)> {
    let y0 = position.y;
    let vy = velocity.y;
    let g = gravity;

    // すでに地面上または地面以下の場合
    if y0 <= 0.0 {
        return Some((position, 0.0));
    }

    // 二次方程式: 0.5 * g * t² + vy * t + y0 = 0
    // a = 0.5 * g, b = vy, c = y0
    // t = (-b ± √(b² - 4ac)) / 2a
    //   = (-vy ± √(vy² - 2 * g * y0)) / g

    let a = 0.5 * g;
    let b = vy;
    let c = y0;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        // 解なし（理論上は重力があれば常に着地するはずだが）
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let t1 = (-b - sqrt_d) / (2.0 * a);
    let t2 = (-b + sqrt_d) / (2.0 * a);

    // 正の時間のうち最も近い未来を選択
    let time_to_landing = if t1 > 0.0 && t2 > 0.0 {
        t1.min(t2)
    } else if t1 > 0.0 {
        t1
    } else if t2 > 0.0 {
        t2
    } else {
        // どちらも負（すでに過去）の場合
        return None;
    };

    // 着地時のXZ位置を計算（等速直線運動）
    let landing_x = position.x + velocity.x * time_to_landing;
    let landing_z = position.z + velocity.z * time_to_landing;

    Some((Vec3::new(landing_x, 0.0, landing_z), time_to_landing))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 基本的な着地計算テスト
    #[test]
    fn test_calculate_landing_basic() {
        // 位置: (0, 5, 0)、速度: (10, 0, 5)、重力: -10
        let position = Vec3::new(0.0, 5.0, 0.0);
        let velocity = Vec3::new(10.0, 0.0, 5.0);
        let gravity = -10.0;

        let result = calculate_landing_position(position, velocity, gravity);
        assert!(result.is_some());

        let (landing_pos, time) = result.unwrap();

        // t = √(2h/g) = √(2*5/10) = 1.0秒
        assert!((time - 1.0).abs() < 0.01);
        // X = 0 + 10 * 1.0 = 10.0
        assert!((landing_pos.x - 10.0).abs() < 0.1);
        // Z = 0 + 5 * 1.0 = 5.0
        assert!((landing_pos.z - 5.0).abs() < 0.1);
        // Y = 0
        assert!(landing_pos.y.abs() < 0.01);
    }

    /// 上向き速度ありの着地計算テスト
    #[test]
    fn test_calculate_landing_with_upward_velocity() {
        // 位置: (0, 2, 0)、速度: (5, 4, 0)、重力: -10
        let position = Vec3::new(0.0, 2.0, 0.0);
        let velocity = Vec3::new(5.0, 4.0, 0.0);
        let gravity = -10.0;

        let result = calculate_landing_position(position, velocity, gravity);
        assert!(result.is_some());

        let (landing_pos, time) = result.unwrap();

        // 二次方程式: -5t² + 4t + 2 = 0
        // t = (-4 ± √(16 + 40)) / (-10)
        // t = (-4 ± 7.48) / (-10)
        // t1 = (-4 - 7.48) / -10 = 1.148
        // t2 = (-4 + 7.48) / -10 = -0.348 (負なので無効)
        assert!((time - 1.148).abs() < 0.1);
        assert!(landing_pos.y.abs() < 0.01);
    }

    /// すでに地面上の場合
    #[test]
    fn test_calculate_landing_on_ground() {
        let position = Vec3::new(5.0, 0.0, 3.0);
        let velocity = Vec3::new(10.0, 5.0, 0.0);
        let gravity = -10.0;

        let result = calculate_landing_position(position, velocity, gravity);
        assert!(result.is_some());

        let (landing_pos, time) = result.unwrap();
        assert!(time.abs() < 0.01);
        assert!((landing_pos.x - 5.0).abs() < 0.01);
        assert!((landing_pos.z - 3.0).abs() < 0.01);
    }
}
