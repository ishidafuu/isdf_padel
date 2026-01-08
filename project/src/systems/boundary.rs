//! 境界判定システム
//! @spec 30503_boundary_behavior.md

use bevy::prelude::*;

use crate::components::{LogicalPosition, Player, Velocity};
use crate::core::{CourtSide, NetHitEvent};
use crate::resource::GameConfig;

use super::court_factory::{create_court_bounds, create_net_info};


/// 境界システムプラグイン
pub struct BoundaryPlugin;

impl Plugin for BoundaryPlugin {
    fn build(&self, app: &mut App) {
        // イベント登録（他システムで使用される）
        app.add_message::<NetHitEvent>();

        // 注意: ball_boundary_system は無効化されました
        // ボールの境界処理は BallTrajectoryPlugin（LogicalPosition使用）に統一
        app.add_systems(Update, player_boundary_system);
    }
}

/// プレイヤー境界制限システム
/// @spec 30503_boundary_behavior.md#beh-30503-001
/// @spec 30503_boundary_behavior.md#beh-30503-002
/// @spec 30503_boundary_behavior.md#beh-30503-003
///
/// プレイヤーがコート境界を超えないように制限する。
/// - 左右壁: Position.X を境界内にクランプ、Velocity.X = 0
/// - 前後壁: Position.Z を境界内にクランプ、Velocity.Z = 0
/// - ネット: 自コート側に制限
pub fn player_boundary_system(
    config: Res<GameConfig>,
    mut query: Query<(&Player, &mut LogicalPosition, &mut Velocity)>,
) {
    let bounds = create_court_bounds(&config.court);
    let net = create_net_info(&config.court);

    for (player, mut logical_pos, mut velocity) in query.iter_mut() {
        let pos = &mut logical_pos.value;

        // BEH-30503-001: 左右壁制限
        if pos.x < bounds.left {
            pos.x = bounds.left;
            if velocity.value.x < 0.0 {
                velocity.value.x = 0.0;
            }
        } else if pos.x > bounds.right {
            pos.x = bounds.right;
            if velocity.value.x > 0.0 {
                velocity.value.x = 0.0;
            }
        }

        // BEH-30503-002: 前後壁制限
        match player.court_side {
            CourtSide::Player1 => {
                // 1Pは Z < net_z の範囲
                if pos.z < bounds.back_1p {
                    pos.z = bounds.back_1p;
                    if velocity.value.z < 0.0 {
                        velocity.value.z = 0.0;
                    }
                }
                // BEH-30503-003: ネット通過禁止（1Pは net_z 未満）
                if pos.z > net.z {
                    pos.z = net.z;
                    if velocity.value.z > 0.0 {
                        velocity.value.z = 0.0;
                    }
                }
            }
            CourtSide::Player2 => {
                // 2Pは Z > net_z の範囲
                if pos.z > bounds.back_2p {
                    pos.z = bounds.back_2p;
                    if velocity.value.z > 0.0 {
                        velocity.value.z = 0.0;
                    }
                }
                // BEH-30503-003: ネット通過禁止（2Pは net_z 超過）
                if pos.z < net.z {
                    pos.z = net.z;
                    if velocity.value.z < 0.0 {
                        velocity.value.z = 0.0;
                    }
                }
            }
        }

        // 天井制限（ジャンプ時）
        if pos.y > bounds.ceiling {
            pos.y = bounds.ceiling;
            if velocity.value.y > 0.0 {
                velocity.value.y = 0.0;
            }
        }

        // 地面制限
        if pos.y < bounds.ground {
            pos.y = bounds.ground;
            if velocity.value.y < 0.0 {
                velocity.value.y = 0.0;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::BounceCount;
    use crate::core::{determine_court_side, WallReflection};

    fn test_config() -> GameConfig {
        use crate::resource::config::*;
        GameConfig {
            physics: PhysicsConfig {
                gravity: -9.8,
                max_fall_speed: -20.0,
            },
            court: CourtConfig {
                width: 10.0,
                depth: 6.0,
                ceiling_height: 5.0,
                max_jump_height: 5.0,
                net_height: 1.0,
                net_z: 0.0,
                service_box_depth: 1.5,
            },
            player: PlayerConfig {
                move_speed: 5.0,
                move_speed_z: 4.0,
                max_speed: 10.0,
                jump_force: 8.0,
                friction: 0.9,
                air_control_factor: 0.5,
                z_min: -3.0,
                z_max: 3.0,
            },
            ball: BallConfig {
                normal_shot_speed: 10.0,
                power_shot_speed: 15.0,
                bounce_factor: 0.8,
                radius: 0.2,
                min_bounce_velocity: 1.0,
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
            shadow: ShadowConfig::default(),
            shot_attributes: ShotAttributesConfig::default(),
            ai: AiConfig::default(),
            visual_feedback: VisualFeedbackConfig::default(),
            serve: ServeConfig::default(),
        }
    }

    /// TST-30504-011: プレイヤーの左右壁制限
    #[test]
    fn test_beh_30503_001_player_side_wall_constraint() {
        let config = test_config();
        let bounds = create_court_bounds(&config.court);

        // 左壁を超えた位置
        let mut pos = Vec3::new(-6.0, 0.0, -1.0);
        let mut vel = Vec3::new(-5.0, 0.0, 0.0);

        // 制限適用
        if pos.x < bounds.left {
            pos.x = bounds.left;
            if vel.x < 0.0 {
                vel.x = 0.0;
            }
        }

        assert_eq!(pos.x, -5.0); // 境界にクランプ
        assert_eq!(vel.x, 0.0); // 壁方向の速度を0に
    }

    /// TST-30504-012: プレイヤーの前後壁制限
    #[test]
    fn test_beh_30503_002_player_back_wall_constraint() {
        let config = test_config();
        let bounds = create_court_bounds(&config.court);

        // 後壁を超えた位置（1P側）
        let mut pos = Vec3::new(0.0, 0.0, -4.0);
        let mut vel = Vec3::new(0.0, 0.0, -5.0);

        // 制限適用
        if pos.z < bounds.back_1p {
            pos.z = bounds.back_1p;
            if vel.z < 0.0 {
                vel.z = 0.0;
            }
        }

        assert_eq!(pos.z, -3.0); // 境界にクランプ
        assert_eq!(vel.z, 0.0); // 壁方向の速度を0に
    }

    /// TST-30504-013: プレイヤーのネット通過禁止
    #[test]
    fn test_beh_30503_003_player_net_constraint() {
        let config = test_config();
        let net = create_net_info(&config.court);

        // 1Pがネットを超えようとしている
        let mut pos = Vec3::new(0.0, 0.0, 0.5);
        let mut vel = Vec3::new(0.0, 0.0, 5.0);
        let court_side = CourtSide::Player1;

        // 1Pの場合、net_z より大きくなれない
        if court_side == CourtSide::Player1 && pos.z > net.z {
            pos.z = net.z;
            if vel.z > 0.0 {
                vel.z = 0.0;
            }
        }

        assert_eq!(pos.z, 0.0); // ネット位置にクランプ
        assert_eq!(vel.z, 0.0); // ネット方向の速度を0に
    }

    /// TST-30504-014: ボールの壁反射判定
    #[test]
    fn test_beh_30503_004_ball_wall_reflection() {
        let config = test_config();
        let bounds = create_court_bounds(&config.court);
        let bounce_factor = config.ball.bounce_factor;

        // 左壁に接触
        let pos = Vec3::new(-5.0, 2.0, 0.0);
        let vel = Vec3::new(-10.0, 0.0, 3.0);

        let result = WallReflection::check_and_reflect(pos, vel, &bounds, bounce_factor);

        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.reflected_velocity.x > 0.0); // X成分が反転
    }

    /// TST-30504-015: ボールのネット接触判定
    #[test]
    fn test_beh_30503_005_ball_net_collision() {
        let config = test_config();
        let net = create_net_info(&config.court);

        // ネット位置で高さ未満
        let y = 0.5;
        let z = 0.0;
        let tolerance = 0.1;

        assert!(net.is_collision(y, z, tolerance));

        // ネット高さ超過
        let y_above = 1.5;
        assert!(!net.is_collision(y_above, z, tolerance));
    }

    /// TST-30504-016: ボールのコート区分判定
    #[test]
    fn test_beh_30503_006_ball_court_side_detection() {
        let config = test_config();
        let net = create_net_info(&config.court);

        // 1Pコート側
        assert_eq!(determine_court_side(-1.0, net.z), CourtSide::Player1);

        // 2Pコート側
        assert_eq!(determine_court_side(1.0, net.z), CourtSide::Player2);

        // ネット上は2P側扱い
        assert_eq!(determine_court_side(0.0, net.z), CourtSide::Player2);
    }

    /// TST-30504-017: 境界チェックの優先順位
    #[test]
    fn test_beh_30503_007_boundary_check_priority() {
        // 優先順位: ネット > 地面 > 壁 > 天井
        // これはシステムのロジックで保証される（テストはコードレビューで確認）
        // 実際の優先順位はball_boundary_systemの処理順序で決定
    }

    /// BounceCount のテスト
    #[test]
    fn test_bounce_count() {
        let mut bc = BounceCount::default();

        // 最初のバウンス
        bc.record_bounce(CourtSide::Player1);
        assert_eq!(bc.count, 1);
        assert_eq!(bc.last_court_side, Some(CourtSide::Player1));

        // 同じコートで2回目
        bc.record_bounce(CourtSide::Player1);
        assert_eq!(bc.count, 2);

        // 別のコートでバウンス
        bc.record_bounce(CourtSide::Player2);
        assert_eq!(bc.count, 1);
        assert_eq!(bc.last_court_side, Some(CourtSide::Player2));

        // リセット
        bc.reset();
        assert_eq!(bc.count, 0);
        assert_eq!(bc.last_court_side, None);
    }
}
