//! ふっとばしシステム
//! @spec 30203_knockback_spec.md

use bevy::prelude::*;

use crate::components::{GroundedState, KnockbackState, Player, Velocity};
use crate::core::court::CourtBounds;
use crate::core::events::{BallHitEvent, PlayerKnockbackEvent};
use crate::resource::config::GameConfig;

/// ふっとばし開始システム
/// @spec 30203_knockback_spec.md#req-30203-001
/// @spec 30203_knockback_spec.md#req-30203-007
pub fn knockback_start_system(
    config: Res<GameConfig>,
    mut ball_hit_events: MessageReader<BallHitEvent>,
    mut query: Query<(Entity, &Player, &Transform, &mut KnockbackState)>,
    mut knockback_event_writer: MessageWriter<PlayerKnockbackEvent>,
) {
    for event in ball_hit_events.read() {
        // 被弾したプレイヤーを検索
        for (entity, player, transform, mut knockback) in query.iter_mut() {
            if entity != event.target_id {
                continue;
            }

            // 無敵中は被弾しない
            // @spec 30203_knockback_spec.md#req-30203-005
            if knockback.is_invincible() {
                continue;
            }

            // ふっとばし方向：ボール→プレイヤーの単位ベクトル
            // @spec 30203_knockback_spec.md#req-30203-001
            let direction = (transform.translation - event.hit_point).normalize_or_zero();

            // ふっとばし速度：ボール速度 * SpeedMultiplier
            let knockback_speed = event.ball_velocity.length() * config.knockback.speed_multiplier;
            let knockback_velocity = direction * knockback_speed;

            // ふっとばし開始
            knockback.start(
                knockback_velocity,
                config.knockback.duration,
                config.knockback.invincibility_time,
            );

            // PlayerKnockbackEvent を発行
            // @spec 30203_knockback_spec.md#req-30203-007
            knockback_event_writer.write(PlayerKnockbackEvent {
                player_id: player.id,
                knockback_velocity,
                duration: config.knockback.duration,
                invincibility_time: config.knockback.invincibility_time,
            });

            info!(
                "Player {} knockback started: velocity={:?}, duration={}",
                player.id, knockback_velocity, config.knockback.duration
            );
        }
    }
}

/// ふっとばし移動システム
/// @spec 30203_knockback_spec.md#req-30203-002
/// @spec 30203_knockback_spec.md#req-30203-003
/// @spec 30203_knockback_spec.md#req-30203-008
pub fn knockback_movement_system(
    time: Res<Time>,
    config: Res<GameConfig>,
    mut query: Query<(
        &Player,
        &mut Transform,
        &mut KnockbackState,
        &mut Velocity,
        &GroundedState,
    )>,
) {
    let bounds = CourtBounds::from_config(&config.court);
    let delta = time.delta_secs();

    for (player, mut transform, mut knockback, mut velocity, grounded) in query.iter_mut() {
        if !knockback.is_knockback_active() {
            continue;
        }

        // REQ-30203-008: ふっとばし中の重力適用
        if !grounded.is_grounded {
            knockback.velocity.y += config.physics.gravity * delta;
            // 最大落下速度制限
            knockback.velocity.y = knockback.velocity.y.max(config.physics.max_fall_speed);
        }

        // REQ-30203-002: ふっとばし移動
        let mut new_position = transform.translation + knockback.velocity * delta;

        // REQ-30203-003: 境界制限
        let old_x = new_position.x;
        new_position.x = bounds.clamp_x(new_position.x);
        if new_position.x != old_x {
            // 境界に達したらX成分を0にリセット
            knockback.velocity.x = 0.0;
        }

        // Z軸境界（プレイヤーごと）
        let (z_min, z_max) = get_player_z_bounds(player.id, &config);
        let old_z = new_position.z;
        new_position.z = new_position.z.clamp(z_min, z_max);
        if new_position.z != old_z {
            knockback.velocity.z = 0.0;
        }

        // Y軸境界（地面）
        if new_position.y < 0.0 {
            new_position.y = 0.0;
            knockback.velocity.y = 0.0;
        }

        // 位置更新
        transform.translation = new_position;

        // Velocity コンポーネントにも反映（他システムとの整合性）
        velocity.value = knockback.velocity;
    }
}

/// ふっとばし時間・無敵時間更新システム
/// @spec 30203_knockback_spec.md#req-30203-004
/// @spec 30203_knockback_spec.md#req-30203-005
/// @spec 30203_knockback_spec.md#req-30203-006
pub fn knockback_timer_system(time: Res<Time>, mut query: Query<(&Player, &mut KnockbackState)>) {
    let delta = time.delta_secs();

    for (player, mut knockback) in query.iter_mut() {
        // 無敵時間の減算
        // @spec 30203_knockback_spec.md#req-30203-005
        if knockback.invincibility_time > 0.0 {
            knockback.invincibility_time -= delta;
            if knockback.invincibility_time < 0.0 {
                knockback.invincibility_time = 0.0;
            }
        }

        // ふっとばし時間の減算
        // @spec 30203_knockback_spec.md#req-30203-004
        if knockback.is_active {
            knockback.remaining_time -= delta;

            // REQ-30203-004: ふっとばし終了
            if knockback.remaining_time <= 0.0 {
                knockback.end();
                info!("Player {} knockback ended", player.id);
            }
        }
    }
}

/// プレイヤーごとのZ軸境界を取得
fn get_player_z_bounds(player_id: u8, config: &GameConfig) -> (f32, f32) {
    match player_id {
        1 => (config.player.z_min, 0.0),
        2 => (0.0, config.player.z_max),
        _ => (config.player.z_min, config.player.z_max),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TST-30204-015: ふっとばし開始テスト
    #[test]
    fn test_knockback_state_start() {
        let mut knockback = KnockbackState::default();
        let velocity = Vec3::new(5.0, 2.0, 3.0);
        let duration = 0.5;
        let invincibility_time = 1.0;

        knockback.start(velocity, duration, invincibility_time);

        assert!(knockback.is_active);
        assert_eq!(knockback.velocity, velocity);
        assert_eq!(knockback.remaining_time, duration);
        assert_eq!(knockback.invincibility_time, invincibility_time);
    }

    /// TST-30204-018: ふっとばし終了テスト
    #[test]
    fn test_knockback_state_end() {
        let mut knockback = KnockbackState {
            is_active: true,
            remaining_time: 0.5,
            invincibility_time: 1.0,
            velocity: Vec3::new(5.0, 2.0, 3.0),
        };

        knockback.end();

        assert!(!knockback.is_active);
        assert_eq!(knockback.velocity, Vec3::ZERO);
        assert_eq!(knockback.remaining_time, 0.0);
    }

    /// TST-30204-019: 無敵状態チェックテスト
    #[test]
    fn test_is_invincible() {
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

    /// TST-30204-020: 操作不能状態チェックテスト
    #[test]
    fn test_is_knockback_active() {
        let knockback_active = KnockbackState {
            is_active: true,
            remaining_time: 0.5,
            ..Default::default()
        };
        assert!(knockback_active.is_knockback_active());

        let knockback_expired = KnockbackState {
            is_active: true,
            remaining_time: 0.0,
            ..Default::default()
        };
        assert!(!knockback_expired.is_knockback_active());

        let knockback_inactive = KnockbackState {
            is_active: false,
            remaining_time: 0.5,
            ..Default::default()
        };
        assert!(!knockback_inactive.is_knockback_active());
    }
}
