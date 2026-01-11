//! Trace Systems - トレースデータ記録システム
//! @spec 77100_headless_sim.md
//!
//! EventTracer にデータを記録するBevyシステム群。

use bevy::prelude::*;

use crate::components::{Ball, LogicalPosition, Player, Velocity};
use crate::core::events::{
    FaultEvent, GroundBounceEvent, PointScoredEvent, RallyEndEvent, ShotExecutedEvent,
    WallReflectionEvent,
};
use crate::resource::FixedDeltaTime;

use super::event_tracer::{EntityTrace, EntityType, EventTracer, GameEvent};

/// フレームカウントを進めるシステム
/// 毎フレーム呼ばれる
pub fn trace_frame_advance_system(mut tracer: ResMut<EventTracer>) {
    if tracer.enabled {
        tracer.advance_frame();
    }
}

/// 位置・速度・イベントを記録するシステム
/// interval_frames ごとに呼ばれる
pub fn trace_positions_system(
    mut tracer: ResMut<EventTracer>,
    fixed_dt: Res<FixedDeltaTime>,
    players: Query<(&Player, &LogicalPosition, &Velocity)>,
    balls: Query<(&LogicalPosition, &Velocity), With<Ball>>,
) {
    if !tracer.should_record_frame() {
        return;
    }

    let mut entities = Vec::new();

    // 位置記録が有効な場合のみエンティティを収集
    if tracer.config.position {
        let record_velocity = tracer.config.velocity;

        // プレイヤーの位置・速度を記録
        for (player, pos, vel) in players.iter() {
            let entity_type = if player.id == 1 {
                EntityType::Player1
            } else {
                EntityType::Player2
            };
            entities.push(EntityTrace {
                entity_type,
                position: pos.value,
                velocity: if record_velocity { vel.value } else { Vec3::ZERO },
            });
        }

        // ボールの位置・速度を記録
        for (pos, vel) in balls.iter() {
            entities.push(EntityTrace {
                entity_type: EntityType::Ball,
                position: pos.value,
                velocity: if record_velocity { vel.value } else { Vec3::ZERO },
            });
        }
    }

    let timestamp = tracer.current_frame() as f32 * fixed_dt.delta_secs();
    tracer.record_positions(timestamp, entities);
}

/// ショット実行イベントを記録するシステム
pub fn trace_shot_events_system(
    mut tracer: ResMut<EventTracer>,
    mut shot_events: MessageReader<ShotExecutedEvent>,
) {
    if !tracer.enabled || !tracer.config.events {
        return;
    }

    for event in shot_events.read() {
        let shot_type = if event.is_jump_shot {
            "jump_shot".to_string()
        } else {
            "normal".to_string()
        };
        tracer.record_event(GameEvent::BallHit {
            player: event.player_id,
            shot_type,
        });
    }
}

/// 地面バウンスイベントを記録するシステム
pub fn trace_bounce_events_system(
    mut tracer: ResMut<EventTracer>,
    mut bounce_events: MessageReader<GroundBounceEvent>,
) {
    if !tracer.enabled || !tracer.config.events {
        return;
    }

    for event in bounce_events.read() {
        tracer.record_event(GameEvent::Bounce {
            position: event.bounce_point,
            court_side: event.court_side,
        });
    }
}

/// 壁反射イベントを記録するシステム
pub fn trace_wall_events_system(
    mut tracer: ResMut<EventTracer>,
    mut wall_events: MessageReader<WallReflectionEvent>,
) {
    if !tracer.enabled || !tracer.config.events {
        return;
    }

    for event in wall_events.read() {
        tracer.record_event(GameEvent::WallReflect {
            position: event.contact_point,
            wall_type: format!("{:?}", event.wall_type),
        });
    }
}

/// ポイント獲得イベントを記録するシステム
pub fn trace_point_events_system(
    mut tracer: ResMut<EventTracer>,
    mut rally_events: MessageReader<RallyEndEvent>,
    mut point_events: MessageReader<PointScoredEvent>,
) {
    // PointScoredEvent は RallyEndEvent で記録するため消費のみ
    point_events.read().count();

    if !tracer.enabled || !tracer.config.events {
        return;
    }

    for event in rally_events.read() {
        let winner = match event.winner {
            crate::core::CourtSide::Left => 1,
            crate::core::CourtSide::Right => 2,
        };
        tracer.record_event(GameEvent::Point {
            winner,
            reason: format!("{:?}", event.reason),
        });
    }
}

/// フォールトイベントを記録するシステム
pub fn trace_fault_events_system(
    mut tracer: ResMut<EventTracer>,
    mut fault_events: MessageReader<FaultEvent>,
) {
    if !tracer.enabled || !tracer.config.events {
        return;
    }

    for event in fault_events.read() {
        tracer.record_event(GameEvent::Fault {
            fault_type: format!("{:?}", event.reason),
        });
    }
}

/// トレースシステムのプラグイン
pub struct TraceSystemPlugin;

impl Plugin for TraceSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                trace_frame_advance_system,
                trace_positions_system,
                trace_shot_events_system,
                trace_bounce_events_system,
                trace_wall_events_system,
                trace_point_events_system,
                trace_fault_events_system,
            )
                .chain(),
        );
    }
}
