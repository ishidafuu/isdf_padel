//! 弾道計算モジュール
//! @spec 30605_trajectory_calculation_spec.md
//!
//! 入力から着地地点を決定し、放物線公式で発射角度を逆算するシステム

use bevy::prelude::*;


use crate::core::CourtSide;
use crate::resource::config::{CourtConfig, GameConfig, ServeSide, TrajectoryConfig};
use crate::systems::fault_judgment::get_service_box;

/// CourtSide の符号を取得（計算用ヘルパー）
trait CourtSideExt {
    fn sign(&self) -> f32;
}

impl CourtSideExt for CourtSide {
    fn sign(&self) -> f32 {
        match self {
            CourtSide::Left => 1.0,
            CourtSide::Right => -1.0,
        }
    }
}

/// 弾道計算結果
/// @spec 30605_trajectory_calculation_spec.md
#[derive(Debug, Clone)]
pub struct TrajectoryResult {
    /// 発射角度（度）
    pub launch_angle: f32,
    /// 最終初速
    pub final_speed: f32,
    /// 発射方向ベクトル（正規化済み）
    pub direction: Vec3,
    /// 着地予定地点
    pub landing_position: Vec3,
}

/// 弾道計算コンテキスト
/// 計算に必要な入力パラメータをまとめる
#[derive(Debug, Clone)]
pub struct TrajectoryContext {
    /// 入力方向（X=左右, Y=前後）
    pub input: Vec2,
    /// コートサイド
    pub court_side: CourtSide,
    /// ボールの現在位置
    pub ball_position: Vec3,
    /// スピン値（-1.0〜+1.0）
    pub spin: f32,
    /// 基準初速（ショット属性から）
    pub base_speed: f32,
    /// 精度（ショット属性から）
    pub accuracy: f32,
}

/// 着地地点を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-010
/// @spec 30605_trajectory_calculation_spec.md#req-30605-011
/// @spec 30605_trajectory_calculation_spec.md#req-30605-012
/// @spec 30605_trajectory_calculation_spec.md#req-30605-013
pub fn calculate_landing_position(
    ctx: &TrajectoryContext,
    court_config: &CourtConfig,
    trajectory_config: &TrajectoryConfig,
) -> Vec3 {
    let margin = trajectory_config.landing_margin;
    let half_width = court_config.width / 2.0;
    let half_depth = court_config.depth / 2.0;

    // ネット位置
    let net_x = court_config.net_x;

    // コートサイドに応じた座標変換
    let (baseline_x, _net_side_x) = match ctx.court_side {
        CourtSide::Left => {
            // Left側は+X方向に打つ → 相手側ベースライン = +half_depth
            (half_depth, net_x)
        }
        CourtSide::Right => {
            // Right側は-X方向に打つ → 相手側ベースライン = -half_depth
            (-half_depth, net_x)
        }
    };

    // REQ-30605-010, REQ-30605-011: 前後入力による深さ調整
    // input.y: -1.0=ネット際, 0.0=サービスライン付近, +1.0=ベースライン際
    let target_x = if ctx.input.y.abs() < 0.01 {
        // ニュートラル: デフォルト着地深さを使用
        let depth = trajectory_config.default_landing_depth;
        match ctx.court_side {
            CourtSide::Left => net_x + depth,
            CourtSide::Right => net_x - depth,
        }
    } else {
        // 入力あり: 線形補間
        // Left側の場合: input.y=-1 → ネット際, input.y=+1 → ベースライン際
        let near = net_x + margin * ctx.court_side.sign();
        let far = baseline_x - margin * ctx.court_side.sign();
        let t = (ctx.input.y + 1.0) / 2.0; // -1..1 → 0..1
        lerp(near, far, t)
    };

    // REQ-30605-012: 左右入力によるコース調整
    // input.x: -1.0=左サイド, 0.0=中央, +1.0=右サイド
    let target_z = ctx.input.x * (half_width - margin);

    Vec3::new(target_x, 0.0, target_z)
}

/// サーブ用着地地点を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-050
/// @spec 30605_trajectory_calculation_spec.md#req-30605-051
/// @spec 30605_trajectory_calculation_spec.md#req-30605-052
pub fn calculate_serve_landing_position(
    input: Vec2,
    server: CourtSide,
    serve_side: ServeSide,
    config: &GameConfig,
) -> Vec3 {
    let service_box = get_service_box(server, serve_side, config);
    let margin = config.trajectory.landing_margin;

    // REQ-30605-051: 前後入力による深さ調整
    // input.y: -1.0=ネット際, 0.0=中央, +1.0=サービスライン際
    let depth_t = (input.y + 1.0) / 2.0; // -1..1 → 0..1
    let target_x = lerp(
        service_box.x_min + margin * server.sign(),
        service_box.x_max - margin * server.sign(),
        depth_t,
    );

    // REQ-30605-052: 左右入力によるコース調整
    // input.x: -1.0=左端, 0.0=中央, +1.0=右端
    let width_t = (input.x + 1.0) / 2.0; // -1..1 → 0..1
    let target_z = lerp(
        service_box.z_min + margin,
        service_box.z_max - margin,
        width_t,
    );

    Vec3::new(target_x, 0.0, target_z)
}

/// サーブ用弾道計算コンテキスト
#[derive(Debug, Clone)]
pub struct ServeTrajectoryContext {
    /// 入力方向（X=左右, Y=前後）
    pub input: Vec2,
    /// サーバーのコートサイド
    pub server: CourtSide,
    /// サーブサイド（デュース/アド）
    pub serve_side: ServeSide,
    /// 打点位置（トスボールの位置）
    pub hit_position: Vec3,
    /// 基準初速
    pub base_speed: f32,
}

/// サーブ用弾道を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-050
/// @spec 30605_trajectory_calculation_spec.md#req-30605-053
/// @spec 30605_trajectory_calculation_spec.md#req-30605-054
pub fn calculate_serve_trajectory(ctx: &ServeTrajectoryContext, config: &GameConfig) -> TrajectoryResult {
    let trajectory_config = &config.trajectory;
    let court_config = &config.court;

    // 1. サービスボックス内の着地地点を決定
    let landing_position = calculate_serve_landing_position(
        ctx.input,
        ctx.server,
        ctx.serve_side,
        config,
    );

    // 2. 有効重力を計算（サーブはフラット: spin = 0）
    let effective_gravity = calculate_effective_gravity(0.0, ctx.hit_position.y, config);

    // 3. 発射角度と調整後初速を計算（サーブは着地点調整なし）
    let (launch_angle, adjusted_speed, adjusted_landing) = calculate_launch_angle(
        ctx.hit_position,
        landing_position,
        ctx.base_speed,
        effective_gravity,
        trajectory_config,
        court_config.net_x,
        court_config.net_height,
    );

    // 4. 方向ベクトルを計算
    let direction = calculate_direction_vector(ctx.hit_position, adjusted_landing, launch_angle);

    TrajectoryResult {
        launch_angle,
        final_speed: adjusted_speed,
        direction,
        landing_position: adjusted_landing,
    }
}

/// 有効重力を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-020
pub fn calculate_effective_gravity(
    spin: f32,
    initial_height: f32,
    config: &GameConfig,
) -> f32 {
    let gravity = config.physics.gravity.abs();
    let spin_config = &config.spin_physics;

    // 飛行時間推定（簡易計算）
    let estimated_flight_time = if initial_height > 0.1 {
        2.0 * (initial_height / gravity).sqrt()
    } else {
        1.0 // デフォルト
    };

    // 平均スピン効果
    let avg_spin = spin * (1.0 - spin_config.spin_decay_rate * estimated_flight_time / 2.0);

    // 有効重力 = 基本重力 × (1 + スピン効果)
    // トップスピン(+) → 重力増加（落ちやすい）
    // スライス(-) → 重力減少（浮きやすい）
    gravity * (1.0 + avg_spin * spin_config.gravity_spin_factor)
}

/// ネット通過に必要な最小角度を計算
/// 打点位置、速度、重力からネットを越えるために必要な最小発射角度を計算
fn calculate_min_angle_for_net_clearance(
    start_pos: Vec3,
    target_pos: Vec3,
    speed: f32,
    gravity: f32,
    net_x: f32,
    net_height: f32,
) -> f32 {
    let dx = target_pos.x - start_pos.x;
    
    // ネットを越えない方向の場合は制限不要
    let crosses_net = (dx > 0.0 && start_pos.x < net_x && target_pos.x > net_x)
        || (dx < 0.0 && start_pos.x > net_x && target_pos.x < net_x);
    
    if !crosses_net {
        return 0.0; // ネットを越えない場合は制限なし
    }
    
    // ネットまでの水平距離
    let dist_to_net = (net_x - start_pos.x).abs();
    
    // 打点とネット上端の高さの差（マージン込み）
    let net_clearance_margin = 0.3; // ネット上端からのマージン
    let required_height = net_height + net_clearance_margin - start_pos.y;
    
    // 打点がネットより十分高い場合は制限緩和
    if required_height < 0.0 {
        // 打点がネット上端より高い場合、低い角度でも通過可能
        // ただし、落下を考慮して最低限の角度は必要
        return -5.0; // 少し下向きでもOK
    }
    
    // 二分探索でネットを越える最小角度を求める
    let mut low = 0.0_f32;
    let mut high = 60.0_f32;
    
    for _ in 0..20 {
        let mid = (low + high) / 2.0;
        let mid_rad = mid.to_radians();
        let cos_a = mid_rad.cos();
        let sin_a = mid_rad.sin();
        
        if cos_a.abs() < 0.001 {
            low = mid;
            continue;
        }
        
        // ネット到達時刻
        let t_net = dist_to_net / (speed * cos_a);
        
        // ネット到達時の高さ
        let height_at_net = start_pos.y + speed * sin_a * t_net - 0.5 * gravity * t_net * t_net;
        
        if height_at_net >= net_height + net_clearance_margin {
            high = mid; // この角度で通過できる、もっと低い角度を試す
        } else {
            low = mid; // 通過できない、もっと高い角度が必要
        }
    }
    
    high // 安全側（高い方）を返す
}

/// 発射角度を逆算（着地点も調整）
/// @spec 30605_trajectory_calculation_spec.md#req-30605-021
/// @spec 30605_trajectory_calculation_spec.md#req-30605-022
/// @spec 30605_trajectory_calculation_spec.md#req-30605-024
/// 戻り値: (角度, 速度, 調整後の着地点)
pub fn calculate_launch_angle(
    start_pos: Vec3,
    target_pos: Vec3,
    base_speed: f32,
    effective_gravity: f32,
    trajectory_config: &TrajectoryConfig,
    net_x: f32,
    net_height: f32,
) -> (f32, f32, Vec3) {
    let dx = target_pos.x - start_pos.x;
    let dz = target_pos.z - start_pos.z;
    let horizontal_distance = (dx * dx + dz * dz).sqrt();

    // 高さの差（着地高さ - 打点高さ）※発射点基準
    let h = target_pos.y - start_pos.y;

    let v = base_speed;
    let g = effective_gravity;
    let d = horizontal_distance;
    let v2 = v * v;
    let v4 = v2 * v2;

    // 判別式: v^4 - g(g*d^2 + 2*h*v^2)
    let discriminant = v4 - g * (g * d * d + 2.0 * h * v2);

    if discriminant >= 0.0 {
        // 解がある場合: 目標地点に到達可能
        let sqrt_disc = discriminant.sqrt();
        let tan_theta_1 = (v2 - sqrt_disc) / (g * d);
        let tan_theta_2 = (v2 + sqrt_disc) / (g * d);

        // 角度が低い方を採用（テニス的な軌道）
        let angle_1 = tan_theta_1.atan().to_degrees();
        let angle_2 = tan_theta_2.atan().to_degrees();

        let angle = if angle_1.abs() < angle_2.abs() {
            angle_1
        } else {
            angle_2
        };

        // ネット通過に必要な最小角度を計算
        let min_net_angle = calculate_min_angle_for_net_clearance(
            start_pos, target_pos, v, g, net_x, net_height,
        );

        // ネット通過角度と計算角度の大きい方を採用
        let final_angle = angle.max(min_net_angle);

        // 上限のみ制限
        let clamped_angle = final_angle.min(trajectory_config.max_launch_angle);

        // 角度が変更された場合、目標着地点に到達するように速度を調整
        if (clamped_angle - angle).abs() > 0.1 {
            // 変更後の角度で目標着地点に到達する速度を計算
            let adjusted_speed = calculate_speed_for_target(
                clamped_angle, horizontal_distance, g, h,
            );
            if adjusted_speed > 0.0 {
                return (clamped_angle, adjusted_speed, target_pos);
            }
        }

        return (clamped_angle, v, target_pos);
    }

    // 解がない場合: パワー不足で目標地点に届かない
    // → 到達可能な最大距離に着地点を短縮（但しネットは越える）
    let max_distance = calculate_max_reachable_distance(base_speed, effective_gravity, h);

    if max_distance < 0.1 || horizontal_distance < 0.001 {
        // 到達距離がほぼ0の場合は最大角度で打つ
        return (trajectory_config.max_launch_angle, v, target_pos);
    }

    // 着地点を短縮
    let scale = (max_distance / horizontal_distance).min(0.95); // 95%を上限として安全マージン
    let mut new_x = start_pos.x + dx * scale;
    let new_z = start_pos.z + dz * scale;

    // ネットを越える最低位置を保証
    // ネット位置は X=0、margin=0.5（trajectory_config.landing_margin）
    let net_margin = trajectory_config.landing_margin;
    let target_crosses_net = if dx > 0.0 {
        // 右方向に打っている（Left側プレイヤー）
        target_pos.x > net_margin
    } else {
        // 左方向に打っている（Right側プレイヤー）
        target_pos.x < -net_margin
    };

    if target_crosses_net {
        // 目標がネットを越えている場合、短縮後もネットを越えるよう保証
        if dx > 0.0 && new_x < net_margin {
            new_x = net_margin; // ネット直後
        } else if dx < 0.0 && new_x > -net_margin {
            new_x = -net_margin; // ネット直後
        }
    }

    let new_target = Vec3::new(new_x, target_pos.y, new_z);

    // 短縮した着地点で角度を再計算
    let new_dx = new_target.x - start_pos.x;
    let new_dz = new_target.z - start_pos.z;
    let new_d = (new_dx * new_dx + new_dz * new_dz).sqrt();
    let new_discriminant = v4 - g * (g * new_d * new_d + 2.0 * h * v2);

    if new_discriminant >= 0.0 {
        let sqrt_disc = new_discriminant.sqrt();
        let tan_theta = (v2 - sqrt_disc) / (g * new_d);
        let angle = tan_theta.atan().to_degrees();

        // ネット通過角度を計算
        let min_net_angle = calculate_min_angle_for_net_clearance(
            start_pos, new_target, v, g, net_x, net_height,
        );
        let final_angle = angle.max(min_net_angle);
        let clamped = final_angle.min(trajectory_config.max_launch_angle);
        return (clamped, v, new_target);
    }

    // それでも解がない場合は最大角度を使用し、実際の到達距離を計算
    let max_angle_rad = trajectory_config.max_launch_angle.to_radians();
    let cos_angle = max_angle_rad.cos();
    let sin_angle = max_angle_rad.sin();

    // 最大角度で打った場合の水平速度成分
    let v_horizontal = v * cos_angle;
    let v_vertical = v * sin_angle;

    // 飛行時間を計算: h + v_y*t - 0.5*g*t^2 = 0
    // t = (v_y + sqrt(v_y^2 + 2*g*h)) / g （h < 0 の場合）
    let flight_time = if h >= 0.0 {
        // 打点より上に着地する場合
        (v_vertical + (v_vertical * v_vertical + 2.0 * g * h.abs()).sqrt()) / g
    } else {
        // 打点より下に着地する場合（通常）
        (v_vertical + (v_vertical * v_vertical + 2.0 * g * h.abs()).sqrt()) / g
    };

    // 実際の水平到達距離
    let actual_distance = v_horizontal * flight_time;

    // 着地点を実際の到達距離に基づいて更新
    let actual_scale = if horizontal_distance > 0.001 {
        (actual_distance / horizontal_distance).min(1.0)
    } else {
        1.0
    };

    let actual_target = Vec3::new(
        start_pos.x + dx * actual_scale,
        target_pos.y,
        start_pos.z + dz * actual_scale,
    );

    (trajectory_config.max_launch_angle, v, actual_target)
}

/// 指定した角度・初速・重力での水平飛距離を計算
/// h: 着地高さ - 発射高さ（負の値 = 着地点が低い）
fn calculate_landing_distance_for_angle(angle_deg: f32, speed: f32, gravity: f32, h: f32) -> f32 {
    let angle_rad = angle_deg.to_radians();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    if cos_a.abs() < 0.001 {
        return 0.0; // ほぼ真上に打つ場合
    }

    let v_horizontal = speed * cos_a;
    let v_vertical = speed * sin_a;

    // 飛行時間を計算: y(t) = v_y*t - 0.5*g*t² + h = 0
    // 0.5*g*t² - v_y*t - h = 0
    // t = (v_y + sqrt(v_y² + 2*g*h)) / g (hが負なら +2gh は正)
    let discriminant = v_vertical * v_vertical + 2.0 * gravity * (-h);

    if discriminant < 0.0 {
        return 0.0; // 到達不可能
    }

    let flight_time = (v_vertical + discriminant.sqrt()) / gravity;

    if flight_time < 0.0 {
        return 0.0;
    }

    v_horizontal * flight_time
}

/// 指定した角度で目標地点に到達するために必要な初速を計算
/// 公式: v = √[ g·d² / (2·cos²(θ)·(d·tan(θ) - h)) ]
fn calculate_speed_for_target(
    angle_deg: f32,
    horizontal_distance: f32,
    gravity: f32,
    height_diff: f32, // 着地高さ - 発射高さ（負の値 = 着地点が低い）
) -> f32 {
    let angle_rad = angle_deg.to_radians();
    let cos_a = angle_rad.cos();
    let tan_a = angle_rad.tan();

    if cos_a.abs() < 0.001 {
        return 0.0; // ほぼ真上に打つ場合
    }

    // 解の存在条件: d·tan(θ) - h > 0
    let denominator = horizontal_distance * tan_a - height_diff;

    if denominator <= 0.0 {
        return 0.0; // その角度では到達不可能
    }

    let v_squared = gravity * horizontal_distance * horizontal_distance
        / (2.0 * cos_a * cos_a * denominator);

    if v_squared <= 0.0 {
        return 0.0;
    }

    v_squared.sqrt()
}

/// 与えられた初速と重力で到達可能な最大水平距離を計算
/// 判別式 v⁴ - g(g*d² + 2h*v²) >= 0 を満たす最大の d を求める
/// d_max = sqrt((v⁴ - 2gh*v²) / g²) = (v²/g) * sqrt(1 - 2gh/v²)
fn calculate_max_reachable_distance(base_speed: f32, effective_gravity: f32, height_diff: f32) -> f32 {
    let v2 = base_speed * base_speed;
    let g = effective_gravity;
    let h = height_diff;

    // v² > 2gh が必要（そうでないと到達不可能）
    let discriminant_factor = 1.0 - 2.0 * g * h / v2;

    if discriminant_factor <= 0.0 {
        // 初速が低すぎて上に打っても落ちてくる（到達距離ほぼ0）
        return 0.1; // 最小値
    }

    (v2 / g) * discriminant_factor.sqrt()
}

/// 球種・距離による初速係数を計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-031
/// @spec 30605_trajectory_calculation_spec.md#req-30605-032
/// @spec 30605_trajectory_calculation_spec.md#req-30605-033
pub fn calculate_speed_factors(
    spin: f32,
    horizontal_distance: f32,
    max_court_distance: f32,
    trajectory_config: &TrajectoryConfig,
) -> f32 {
    // 球種係数
    let spin_factor = if spin > 0.1 {
        trajectory_config.spin_speed_topspin
    } else if spin < -0.1 {
        trajectory_config.spin_speed_slice
    } else {
        trajectory_config.spin_speed_flat
    };

    // 距離係数
    let distance_ratio = (horizontal_distance / max_court_distance).clamp(0.0, 1.0);
    let distance_factor = lerp(
        trajectory_config.distance_speed_min,
        trajectory_config.distance_speed_max,
        distance_ratio,
    );

    spin_factor * distance_factor
}

/// 精度による着地位置の収束を適用（決定的）
/// @spec 30605_trajectory_calculation_spec.md#req-30605-040
/// @spec 30604_shot_attributes_spec.md#req-30604-070
/// ランダム性なし: 精度が低いほどコート中央（X軸方向のネット側）に収束
pub fn apply_landing_deviation(
    target: Vec3,
    accuracy: f32,
    _trajectory_config: &TrajectoryConfig,
) -> Vec3 {
    // 精度が低いほど狙った位置に打てない = 中央寄りに収束
    let convergence = (1.0 - accuracy.clamp(0.0, 1.0)) * 0.3;

    if convergence < 0.001 {
        return target;
    }

    // X軸: ネット方向（X=0）に向かって収束
    // Z軸: コート中央（Z=0）に向かって収束
    let converged_x = target.x * (1.0 - convergence);
    let converged_z = target.z * (1.0 - convergence);

    Vec3::new(converged_x, target.y, converged_z)
}

/// 方向ベクトルを計算
/// @spec 30605_trajectory_calculation_spec.md#req-30605-023
pub fn calculate_direction_vector(
    start_pos: Vec3,
    target_pos: Vec3,
    launch_angle: f32,
) -> Vec3 {
    let dx = target_pos.x - start_pos.x;
    let dz = target_pos.z - start_pos.z;
    let horizontal_distance = (dx * dx + dz * dz).sqrt();

    if horizontal_distance < 0.001 {
        // ほぼ同じ位置の場合はデフォルト方向
        return Vec3::new(1.0, launch_angle.to_radians().sin(), 0.0).normalize();
    }

    // 水平方向の単位ベクトル
    let horizontal_dir_x = dx / horizontal_distance;
    let horizontal_dir_z = dz / horizontal_distance;

    let angle_rad = launch_angle.to_radians();
    let cos_angle = angle_rad.cos();
    let sin_angle = angle_rad.sin();

    Vec3::new(
        horizontal_dir_x * cos_angle,
        sin_angle,
        horizontal_dir_z * cos_angle,
    )
}

/// 弾道を計算（メイン関数）
/// @spec 30605_trajectory_calculation_spec.md
pub fn calculate_trajectory(ctx: &TrajectoryContext, config: &GameConfig) -> TrajectoryResult {
    let court_config = &config.court;
    let trajectory_config = &config.trajectory;

    // 1. 着地地点を決定
    let raw_landing = calculate_landing_position(ctx, court_config, trajectory_config);

    // 2. 精度によるズレを適用
    let landing_with_deviation = apply_landing_deviation(raw_landing, ctx.accuracy, trajectory_config);

    // 3. 有効重力を計算
    let effective_gravity = calculate_effective_gravity(ctx.spin, ctx.ball_position.y, config);

    // 4. 発射角度と調整後初速を計算（着地点も調整される可能性あり）
    let (launch_angle, adjusted_speed, landing_position) = calculate_launch_angle(
        ctx.ball_position,
        landing_with_deviation,
        ctx.base_speed,
        effective_gravity,
        trajectory_config,
        court_config.net_x,
        court_config.net_height,
    );

    // 6. 最終初速（角度計算と一貫性を保つため、speed_factor は適用しない）
    // 注: speed_factor を適用すると、角度計算時の速度と実際の速度が乖離し、
    //     着地点予測と実際の着地位置にズレが生じる
    let final_speed = adjusted_speed;

    // 8. 方向ベクトルを計算
    let direction = calculate_direction_vector(ctx.ball_position, landing_position, launch_angle);

    TrajectoryResult {
        launch_angle,
        final_speed,
        direction,
        landing_position,
    }
}

/// 線形補間
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_config() -> GameConfig {
        // テスト用の最小限の設定
        use crate::resource::config::*;
        GameConfig {
            physics: PhysicsConfig {
                gravity: -9.8,
                max_fall_speed: -20.0,
            },
            court: CourtConfig {
                width: 12.0,
                depth: 16.0,
                ceiling_height: 100.0,
                max_jump_height: 5.0,
                net_height: 1.0,
                net_x: 0.0,
                service_box_depth: 5.0,
                outer_wall_z: 10.0,
                outer_wall_x: 12.0,
            },
            player: PlayerConfig {
                move_speed: 5.0,
                move_speed_z: 4.0,
                max_speed: 10.0,
                jump_force: 8.0,
                friction: 0.9,
                air_control_factor: 0.5,
                x_min: -3.0,
                x_max: 3.0,
            },
            ball: BallConfig {
                normal_shot_speed: 10.0,
                power_shot_speed: 15.0,
                bounce_factor: 0.8,
                radius: 0.2,
                min_bounce_velocity: 1.0,
                wall_bounce_factor: 0.8,
            },
            collision: CollisionConfig {
                character_radius: 0.5,
                z_tolerance: 0.3,
            },
            knockback: KnockbackConfig {
                enabled: true,
                duration: 0.5,
                speed_multiplier: 0.5,
                invincibility_time: 1.0,
            },
            shot: ShotConfig {
                max_distance: 1.5,
                max_height_diff: 2.0,
                cooldown_time: 0.5,
                normal_shot_angle: 45.0,
                jump_shot_angle: 30.0,
                jump_threshold: 0.5,
            },
            scoring: ScoringConfig {
                point_values: vec![0, 15, 30, 40],
                games_to_win_set: 6,
                sets_to_win_match: 1,
            },
            input: InputConfig {
                jump_buffer_time: 0.1,
                shot_buffer_time: 0.05,
                normalization_threshold: 1.0,
                input_sensitivity: 1.0,
            },
            input_keys: InputKeysConfig::default(),
            gamepad_buttons: GamepadButtonsConfig::default(),
            shadow: ShadowConfig::default(),
            shot_attributes: ShotAttributesConfig::default(),
            ai: AiConfig::default(),
            visual_feedback: VisualFeedbackConfig::default(),
            player_visual: PlayerVisualConfig::default(),
            serve: ServeConfig::default(),
            spin_physics: SpinPhysicsConfig::default(),
            trajectory: TrajectoryConfig::default(),
            character: CharacterConfig::default(),
        }
    }

    /// TST-30605-001: ニュートラル着地テスト
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-010
    #[test]
    fn test_neutral_landing_position() {
        let config = make_test_config();
        let ctx = TrajectoryContext {
            input: Vec2::ZERO,
            court_side: CourtSide::Left,
            ball_position: Vec3::new(-5.0, 1.0, 0.0),
            spin: 0.0,
            base_speed: 15.0,
            accuracy: 1.0,
        };

        let landing = calculate_landing_position(&ctx, &config.court, &config.trajectory);

        // ニュートラル時: X = net_x + default_landing_depth = 0 + 4.0 = 4.0
        assert!(
            (landing.x - 4.0).abs() < 0.1,
            "Expected X near 4.0, got {}",
            landing.x
        );
        // 中央
        assert!(
            landing.z.abs() < 0.1,
            "Expected Z near 0, got {}",
            landing.z
        );
    }

    /// TST-30605-002: 前入力（ネット際）着地テスト
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-011
    #[test]
    fn test_forward_landing_position() {
        let config = make_test_config();
        let ctx = TrajectoryContext {
            input: Vec2::new(0.0, -1.0), // ネット際
            court_side: CourtSide::Left,
            ball_position: Vec3::new(-5.0, 1.0, 0.0),
            spin: 0.0,
            base_speed: 15.0,
            accuracy: 1.0,
        };

        let landing = calculate_landing_position(&ctx, &config.court, &config.trajectory);

        // ネット際: X = net_x + margin = 0 + 0.5 = 0.5
        assert!(
            landing.x < 2.0,
            "Expected X near net, got {}",
            landing.x
        );
    }

    /// TST-30605-003: 後入力（ベースライン際）着地テスト
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-011
    #[test]
    fn test_backward_landing_position() {
        let config = make_test_config();
        let ctx = TrajectoryContext {
            input: Vec2::new(0.0, 1.0), // ベースライン際
            court_side: CourtSide::Left,
            ball_position: Vec3::new(-5.0, 1.0, 0.0),
            spin: 0.0,
            base_speed: 15.0,
            accuracy: 1.0,
        };

        let landing = calculate_landing_position(&ctx, &config.court, &config.trajectory);

        // ベースライン際: X = baseline - margin = 8.0 - 0.5 = 7.5
        assert!(
            landing.x > 6.0,
            "Expected X near baseline, got {}",
            landing.x
        );
    }

    /// TST-30605-004: 左右入力による着地テスト
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-012
    #[test]
    fn test_side_landing_position() {
        let config = make_test_config();

        // 右入力
        let ctx_right = TrajectoryContext {
            input: Vec2::new(1.0, 0.0),
            court_side: CourtSide::Left,
            ball_position: Vec3::new(-5.0, 1.0, 0.0),
            spin: 0.0,
            base_speed: 15.0,
            accuracy: 1.0,
        };
        let landing_right = calculate_landing_position(&ctx_right, &config.court, &config.trajectory);

        // 右サイド: Z = (width/2 - margin) = 6.0 - 0.5 = 5.5
        assert!(
            landing_right.z > 4.0,
            "Expected Z positive for right input, got {}",
            landing_right.z
        );

        // 左入力
        let ctx_left = TrajectoryContext {
            input: Vec2::new(-1.0, 0.0),
            court_side: CourtSide::Left,
            ball_position: Vec3::new(-5.0, 1.0, 0.0),
            spin: 0.0,
            base_speed: 15.0,
            accuracy: 1.0,
        };
        let landing_left = calculate_landing_position(&ctx_left, &config.court, &config.trajectory);

        assert!(
            landing_left.z < -4.0,
            "Expected Z negative for left input, got {}",
            landing_left.z
        );
    }

    /// TST-30605-005: Right側の着地計算テスト
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-013
    #[test]
    fn test_right_side_landing_position() {
        let config = make_test_config();

        // Right側のニュートラル入力
        let ctx = TrajectoryContext {
            input: Vec2::ZERO,
            court_side: CourtSide::Right,
            ball_position: Vec3::new(5.0, 1.0, 0.0),
            spin: 0.0,
            base_speed: 15.0,
            accuracy: 1.0,
        };

        let landing = calculate_landing_position(&ctx, &config.court, &config.trajectory);

        // Right側のニュートラル時: X = net_x - default_landing_depth = 0 - 4.0 = -4.0
        assert!(
            (landing.x - (-4.0)).abs() < 0.1,
            "Expected X near -4.0 for Right side neutral, got {}",
            landing.x
        );

        // 中央: Z = 0
        assert!(
            landing.z.abs() < 0.5,
            "Expected Z near 0 for neutral input, got {}",
            landing.z
        );
    }

    /// TST-30605-010: 発射角度逆算テスト
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-021
    #[test]
    fn test_launch_angle_calculation() {
        let config = make_test_config();
        let trajectory_config = &config.trajectory;
        let court_config = &config.court;

        let start = Vec3::new(-5.0, 1.0, 0.0);
        let target = Vec3::new(5.0, 0.0, 0.0);
        let base_speed = 15.0;
        let effective_gravity = 9.8;

        let (angle, _speed, _landing) =
            calculate_launch_angle(
                start, target, base_speed, effective_gravity, trajectory_config,
                court_config.net_x, court_config.net_height,
            );

        // 角度が有効範囲内
        assert!(
            angle >= trajectory_config.min_launch_angle,
            "Angle {} below min",
            angle
        );
        assert!(
            angle <= trajectory_config.max_launch_angle,
            "Angle {} above max",
            angle
        );
    }

    /// TST-30605-011: 有効重力テスト（トップスピン）
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-020
    #[test]
    fn test_effective_gravity_topspin() {
        let config = make_test_config();
        let _base_gravity = config.physics.gravity.abs();

        let g_topspin = calculate_effective_gravity(0.5, 1.0, &config);
        let g_neutral = calculate_effective_gravity(0.0, 1.0, &config);

        // トップスピンは重力増加
        assert!(
            g_topspin > g_neutral,
            "Topspin gravity {} should be > neutral {}",
            g_topspin,
            g_neutral
        );
    }

    /// TST-30605-012: 有効重力テスト（スライス）
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-020
    #[test]
    fn test_effective_gravity_slice() {
        let config = make_test_config();

        let g_slice = calculate_effective_gravity(-0.5, 1.0, &config);
        let g_neutral = calculate_effective_gravity(0.0, 1.0, &config);

        // スライスは重力減少
        assert!(
            g_slice < g_neutral,
            "Slice gravity {} should be < neutral {}",
            g_slice,
            g_neutral
        );
    }

    /// TST-30605-020: 球種初速係数テスト（フラット）
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-031
    #[test]
    fn test_spin_speed_factor_flat() {
        let config = make_test_config();
        let factor = calculate_speed_factors(0.0, 5.0, 16.0, &config.trajectory);

        // フラット係数 × 距離係数
        let expected_min = config.trajectory.spin_speed_flat * config.trajectory.distance_speed_min;
        let expected_max = config.trajectory.spin_speed_flat * config.trajectory.distance_speed_max;

        assert!(
            factor >= expected_min && factor <= expected_max,
            "Factor {} out of range [{}, {}]",
            factor,
            expected_min,
            expected_max
        );
    }

    /// TST-30605-021: 球種初速係数テスト（トップスピン）
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-031
    #[test]
    fn test_spin_speed_factor_topspin() {
        let config = make_test_config();
        let factor_topspin = calculate_speed_factors(0.5, 5.0, 16.0, &config.trajectory);
        let factor_flat = calculate_speed_factors(0.0, 5.0, 16.0, &config.trajectory);

        // トップスピンは遅い
        assert!(
            factor_topspin < factor_flat,
            "Topspin factor {} should be < flat {}",
            factor_topspin,
            factor_flat
        );
    }

    /// TST-30605-022: 距離初速係数テスト
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-032
    #[test]
    fn test_distance_speed_factor() {
        let config = make_test_config();

        let factor_near = calculate_speed_factors(0.0, 2.0, 16.0, &config.trajectory);
        let factor_far = calculate_speed_factors(0.0, 14.0, 16.0, &config.trajectory);

        // 遠距離は速い
        assert!(
            factor_far > factor_near,
            "Far factor {} should be > near {}",
            factor_far,
            factor_near
        );
    }

    /// TST-30605-030: ズレ計算テスト
    /// @spec 30605_trajectory_calculation_spec.md#req-30605-040
    #[test]
    fn test_landing_deviation_perfect_accuracy() {
        let config = make_test_config();
        let target = Vec3::new(5.0, 0.0, 2.0);

        let result = apply_landing_deviation(target, 1.0, &config.trajectory);

        // 精度100%ではズレなし
        assert!(
            (result.x - target.x).abs() < 0.001,
            "X should not deviate with perfect accuracy"
        );
        assert!(
            (result.z - target.z).abs() < 0.001,
            "Z should not deviate with perfect accuracy"
        );
    }
}
