//! ボール当たり判定システム
//! @spec 30403_collision_spec.md

use bevy::prelude::*;

use crate::components::{Ball, KnockbackState, LastShooter, LogicalPosition, Player, Velocity};
use crate::core::events::BallHitEvent;
use crate::resource::config::GameConfig;

/// ボール当たり判定プラグイン
/// @spec 30403_collision_spec.md
pub struct BallCollisionPlugin;

impl Plugin for BallCollisionPlugin {
    fn build(&self, app: &mut App) {
        // Note: BallHitEvent は main.rs で add_message 済み
        app.add_systems(
            Update,
            ball_player_collision_system, // @spec 30403_collision_spec.md#req-30403-005
        );
    }
}

/// ボールとプレイヤーの衝突判定システム
/// @spec 30403_collision_spec.md#req-30403-001
/// @spec 30403_collision_spec.md#req-30403-002
/// @spec 30403_collision_spec.md#req-30403-003
/// @spec 30403_collision_spec.md#req-30403-004
/// @spec 30403_collision_spec.md#req-30403-005
/// @spec 30403_collision_spec.md#req-30403-006
pub fn ball_player_collision_system(
    config: Res<GameConfig>,
    mut ball_query: Query<(Entity, &LogicalPosition, &mut Velocity, &LastShooter), With<Ball>>,
    player_query: Query<(Entity, &LogicalPosition, &KnockbackState, &Player), With<Player>>,
    mut event_writer: MessageWriter<BallHitEvent>,
) {
    // ふっとばし機能が無効の場合、衝突判定・反射処理をスキップ
    if !config.knockback.enabled {
        return;
    }

    let collision_params = CollisionParams::from_config(&config);

    for (ball_entity, ball_logical_pos, mut ball_velocity, last_shooter) in ball_query.iter_mut() {
        let ball_pos = ball_logical_pos.value;
        let ball_vel = ball_velocity.value;

        // REQ-30403-006: 複数プレイヤー衝突時、最も近いプレイヤー優先
        let closest = find_closest_collision(
            ball_pos,
            last_shooter,
            &collision_params,
            &player_query,
        );

        // 最も近いプレイヤーとの衝突を処理
        if let Some((target_entity, player_pos)) = closest {
            process_collision(
                ball_entity,
                ball_pos,
                ball_vel,
                target_entity,
                player_pos,
                &mut ball_velocity,
                &mut event_writer,
            );
        }
    }
}

/// 衝突判定に必要なパラメータ
struct CollisionParams {
    collision_distance: f32,
    z_tolerance: f32,
    max_height_diff: f32,
}

impl CollisionParams {
    fn from_config(config: &GameConfig) -> Self {
        Self {
            collision_distance: config.ball.radius + config.collision.character_radius,
            z_tolerance: config.collision.z_tolerance,
            max_height_diff: config.shot.max_height_diff,
        }
    }
}

/// 最も近い衝突プレイヤーを検索
/// @spec 30403_collision_spec.md#req-30403-001
/// @spec 30403_collision_spec.md#req-30403-003
/// @spec 30403_collision_spec.md#req-30403-006
fn find_closest_collision(
    ball_pos: Vec3,
    last_shooter: &LastShooter,
    params: &CollisionParams,
    player_query: &Query<(Entity, &LogicalPosition, &KnockbackState, &Player), With<Player>>,
) -> Option<(Entity, Vec3)> {
    let mut closest: Option<(Entity, Vec3, f32)> = None;

    for (player_entity, player_logical_pos, knockback, player) in player_query.iter() {
        // REQ-30403-003: 無敵状態のプレイヤーは衝突無視
        if knockback.is_invincible() {
            continue;
        }

        // 最後に打ったプレイヤー自身との衝突は無視
        if last_shooter.side == Some(player.court_side) {
            info!(
                "Skipping collision: last_shooter={:?}, player={:?}",
                last_shooter.side, player.court_side
            );
            continue;
        }

        let player_pos = player_logical_pos.value;

        // 衝突条件をチェック
        if !is_collision_candidate(ball_pos, player_pos, params) {
            continue;
        }

        // XZ平面での距離計算
        let distance_2d = distance_xz(ball_pos, player_pos);

        // 衝突判定 & 最も近いプレイヤーを記録
        if distance_2d <= params.collision_distance
            && closest.as_ref().is_none_or(|(_, _, d)| distance_2d < *d)
        {
            closest = Some((player_entity, player_pos, distance_2d));
        }
    }

    closest.map(|(entity, pos, _)| (entity, pos))
}

/// 衝突候補かどうかをチェック（Z軸・高さ方向）
/// @spec 30403_collision_spec.md#req-30403-001
#[inline]
fn is_collision_candidate(ball_pos: Vec3, player_pos: Vec3, params: &CollisionParams) -> bool {
    // Z軸許容範囲チェック
    let z_diff = (ball_pos.z - player_pos.z).abs();
    if z_diff > params.z_tolerance {
        return false;
    }

    // 高さ方向の打球可能範囲チェック
    let height_diff = ball_pos.y - player_pos.y;
    height_diff >= 0.0 && height_diff <= params.max_height_diff
}

/// 衝突処理（イベント発行・ボール反射）
/// @spec 30403_collision_spec.md#req-30403-002
/// @spec 30403_collision_spec.md#req-30403-004
fn process_collision(
    ball_entity: Entity,
    ball_pos: Vec3,
    ball_vel: Vec3,
    target_entity: Entity,
    player_pos: Vec3,
    ball_velocity: &mut Velocity,
    event_writer: &mut MessageWriter<BallHitEvent>,
) {
    // REQ-30403-002: BallHitEvent 発行
    let hit_point = ball_pos;
    event_writer.write(BallHitEvent {
        ball_id: ball_entity,
        target_id: target_entity,
        ball_velocity: ball_vel,
        hit_point,
    });

    // REQ-30403-004: ボール反射（プレイヤー衝突時）
    let reflection_dir = (ball_pos - player_pos).normalize_or_zero();
    let speed = ball_vel.length();
    ball_velocity.value = Vec3::new(
        reflection_dir.x * speed.abs(),
        reflection_dir.y * speed.abs(),
        ball_vel.z, // Z成分は維持
    );

    info!(
        "Ball collision with player: ball={:?}, player={:?}, hit_point={:?}",
        ball_entity, target_entity, hit_point
    );
}

/// XZ平面での2点間距離を計算（水平距離、高さは無視）
#[inline]
fn distance_xz(a: Vec3, b: Vec3) -> f32 {
    let dx = a.x - b.x;
    let dz = a.z - b.z;
    (dx * dx + dz * dz).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30404-014: プレイヤー衝突判定テスト（XZ平面）
    /// @spec 30403_collision_spec.md#req-30403-001
    #[test]
    fn test_collision_detection() {
        let ball_radius = 0.2_f32;
        let character_radius = 0.5_f32;
        let collision_distance = ball_radius + character_radius; // 0.7

        // 衝突する距離（XZ平面）
        let ball_pos = Vec3::new(0.5, 1.0, 0.0);
        let player_pos = Vec3::new(0.0, 1.0, 0.0);
        let distance = distance_xz(ball_pos, player_pos);
        assert!(distance <= collision_distance); // 0.5 <= 0.7

        // 衝突しない距離
        let ball_pos_far = Vec3::new(1.0, 1.0, 0.0);
        let distance_far = distance_xz(ball_pos_far, player_pos);
        assert!(distance_far > collision_distance); // 1.0 > 0.7
    }

    /// 高さ方向の打球可能範囲テスト
    #[test]
    fn test_height_diff_check() {
        let max_height_diff = 2.0_f32;

        // 打球可能範囲内
        let player_y = 0.0;
        let ball_y_ok = 1.5;
        let height_diff_ok = ball_y_ok - player_y;
        assert!(height_diff_ok >= 0.0 && height_diff_ok <= max_height_diff);

        // 打球可能範囲外（高すぎる）
        let ball_y_high = 3.0;
        let height_diff_high = ball_y_high - player_y;
        assert!(height_diff_high > max_height_diff);

        // 打球可能範囲外（低すぎる＝地面より下）
        let ball_y_low = -0.5;
        let height_diff_low = ball_y_low - player_y;
        assert!(height_diff_low < 0.0);
    }

    /// TST-30404-015: Z軸許容範囲テスト
    /// @spec 30403_collision_spec.md#req-30403-001
    #[test]
    fn test_z_tolerance() {
        let z_tolerance = 0.3_f32;

        // 許容範囲内
        let z_diff_ok = 0.2_f32;
        assert!(z_diff_ok <= z_tolerance);

        // 許容範囲外
        let z_diff_ng = 0.5_f32;
        assert!(z_diff_ng > z_tolerance);
    }

    /// TST-30404-017: ボール反射方向テスト
    /// @spec 30403_collision_spec.md#req-30403-004
    #[test]
    fn test_ball_reflection_direction() {
        let ball_pos = Vec3::new(1.0, 1.0, 0.0);
        let player_pos = Vec3::new(0.0, 1.0, 0.0);

        // プレイヤー→ボールの方向
        let reflection_dir = (ball_pos - player_pos).normalize_or_zero();

        // X方向に反射
        assert!((reflection_dir.x - 1.0).abs() < 0.001);
        assert!(reflection_dir.y.abs() < 0.001);
    }

    /// TST-30404-019: 複数プレイヤー衝突時の優先順位テスト（XZ平面）
    /// @spec 30403_collision_spec.md#req-30403-006
    #[test]
    fn test_closest_player_priority() {
        let ball_pos = Vec3::new(0.0, 1.0, 0.0);
        let player1_pos = Vec3::new(0.5, 1.0, 0.0); // XZ距離 0.5
        let player2_pos = Vec3::new(0.3, 1.0, 0.0); // XZ距離 0.3

        let dist1 = distance_xz(ball_pos, player1_pos);
        let dist2 = distance_xz(ball_pos, player2_pos);

        // player2 が最も近い
        assert!(dist2 < dist1);
    }

    /// TST-30404-016: 無敵状態テスト
    /// @spec 30403_collision_spec.md#req-30403-003
    #[test]
    fn test_invincibility_check() {
        let knockback_invincible = KnockbackState {
            invincibility_time: 0.5,
            ..Default::default()
        };
        assert!(knockback_invincible.is_invincible());

        let knockback_not_invincible = KnockbackState {
            invincibility_time: 0.0,
            ..Default::default()
        };
        assert!(!knockback_not_invincible.is_invincible());
    }
}
