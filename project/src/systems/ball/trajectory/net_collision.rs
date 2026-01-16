//! ネット衝突検出・反射システム
//! @spec 30503_boundary_behavior.md#beh-30503-005

use bevy::prelude::*;

use crate::components::{Ball, LogicalPosition, Velocity};
use crate::core::events::NetHitEvent;
use crate::resource::config::GameConfig;

/// ネット衝突検出・反射システム
///
/// ボールがネット位置を通過する際、高さがネット未満なら：
/// 1. NetHitEvent を発行
/// 2. ボールを跳ね返す（X方向の速度を反転・減衰）
///
/// @spec 30503_boundary_behavior.md#beh-30503-005
pub fn ball_net_collision_system(
    config: Res<GameConfig>,
    mut query: Query<(Entity, &mut Velocity, &mut LogicalPosition), With<Ball>>,
    mut event_writer: MessageWriter<NetHitEvent>,
) {
    let net_x = config.court.net_x;
    let net_height = config.court.net_height;
    // ネット接触判定の許容範囲（ボールがネットを通過する1フレーム分の距離をカバー）
    let net_tolerance = 0.3;
    // ネット反射時の減衰係数
    let net_bounce_factor = 0.3;

    for (entity, mut velocity, mut logical_pos) in query.iter_mut() {
        let pos = logical_pos.value;
        let vel = velocity.value;

        // ネット位置付近（net_x ± tolerance）にいるか
        let near_net = (pos.x - net_x).abs() < net_tolerance;

        // ネット高さ未満か
        let below_net_height = pos.y < net_height;

        // ネットに向かって移動中か（X方向の速度がネットに向かっている）
        let moving_towards_net = (pos.x < net_x && vel.x > 0.0) || (pos.x > net_x && vel.x < 0.0);

        if near_net && below_net_height && moving_towards_net {
            info!(
                "Ball hit net at {:?}, height: {:.2} < {:.2}",
                pos, pos.y, net_height
            );

            // 1. NetHitEvent 発行
            event_writer.write(NetHitEvent {
                ball: entity,
                contact_point: Vec3::new(net_x, pos.y, pos.z),
            });

            // 2. X方向の速度を反転・減衰
            velocity.value.x = -vel.x * net_bounce_factor;
            // Y, Z方向も減衰
            velocity.value.y *= net_bounce_factor;
            velocity.value.z *= net_bounce_factor;

            // 3. 位置をネット手前に戻す（めり込み防止）
            if pos.x < net_x {
                logical_pos.value.x = net_x - net_tolerance;
            } else {
                logical_pos.value.x = net_x + net_tolerance;
            }
        }
    }
}
