//! ラケット接触駆動ショットシステム
//! @spec 30606_racket_contact_spec.md

use bevy::prelude::*;

use crate::components::{Ball, LogicalPosition, Player, ShotState, Velocity};
use crate::core::events::{RacketContactEvent, ShotEvent, SwingIntentEvent};
use crate::core::CourtSide;
use crate::resource::config::GameConfig;
use crate::resource::FixedDeltaTime;

/// 入力意図からラケットスイング計画を作成
/// @spec 30606_racket_contact_spec.md#req-30606-002
#[allow(clippy::type_complexity)]
pub fn plan_racket_swing_system(
    config: Res<GameConfig>,
    mut intents: MessageReader<SwingIntentEvent>,
    ball_query: Query<(&LogicalPosition, &Velocity), With<Ball>>,
    mut player_query: Query<(&Player, &LogicalPosition, &mut ShotState), Without<Ball>>,
) {
    let Some((ball_pos, ball_vel)) = ball_query.iter().next() else {
        return;
    };
    let ball_pos = ball_pos.value;
    let ball_vel = ball_vel.value;
    let gravity = config.physics.gravity;
    let swing_cfg = &config.shot.racket_swing;

    for intent in intents.read() {
        for (player, player_pos, mut shot_state) in player_query.iter_mut() {
            if player.id != intent.player_id || shot_state.racket_swing.is_active {
                continue;
            }

            let (planned_hit, hit_time) = predict_hit_point(
                player_pos.value,
                ball_pos,
                ball_vel,
                gravity,
                swing_cfg.min_prediction_time,
                swing_cfg.max_prediction_time,
                swing_cfg.prediction_step,
                swing_cfg.reach_distance,
                swing_cfg.max_hit_height_diff,
            );

            let swing = &mut shot_state.racket_swing;
            let front_sign = front_sign(player.court_side);
            let lateral_sign = if intent.direction.y >= 0.0 { 1.0 } else { -1.0 };

            swing.is_active = true;
            swing.contact_done = false;
            swing.elapsed_seconds = 0.0;
            swing.duration_seconds = swing_cfg.duration_seconds.max(0.01);
            swing.contact_time_seconds = swing_cfg
                .contact_time_seconds
                .clamp(0.01, swing.duration_seconds - 0.01);
            swing.input_direction = intent.direction;
            swing.hold_time_ms = intent.hold_time_ms;
            swing.planned_hit_position = planned_hit;

            // スイング軌道をプレイヤー近傍で構築し、接触時にplanned_hitを通過させる
            swing.start_position =
                player_pos.value + Vec3::new(-0.45 * front_sign, 1.05, -0.20 * lateral_sign);
            swing.pre_contact_position = swing.start_position
                + (planned_hit - swing.start_position) * 0.55
                + Vec3::new(-0.12 * front_sign, 0.18, -0.18 * lateral_sign);
            swing.post_contact_position =
                planned_hit + Vec3::new(0.35 * front_sign, 0.20, 0.18 * lateral_sign);
            swing.end_position = swing.post_contact_position
                + Vec3::new(0.18 * front_sign, -0.08, 0.05 * lateral_sign);

            swing.previous_racket_position = swing.start_position;
            swing.current_racket_position = swing.start_position;

            info!(
                "Swing planned: player={}, hit_time={:.3}, hit=({:.2},{:.2},{:.2})",
                player.id, hit_time, planned_hit.x, planned_hit.y, planned_hit.z
            );
        }
    }
}

/// ラケット軌道を進め、接触時に ShotEvent を発行する
/// @spec 30606_racket_contact_spec.md#req-30606-003
/// @spec 30606_racket_contact_spec.md#req-30606-004
#[allow(clippy::type_complexity)]
pub fn update_racket_swing_system(
    fixed_dt: Res<FixedDeltaTime>,
    config: Res<GameConfig>,
    ball_query: Query<(&LogicalPosition, &Velocity), With<Ball>>,
    mut player_query: Query<(&Player, &LogicalPosition, &mut ShotState), Without<Ball>>,
    mut shot_writer: MessageWriter<ShotEvent>,
    mut contact_writer: MessageWriter<RacketContactEvent>,
) {
    let Some((ball_pos, ball_vel)) = ball_query.iter().next() else {
        return;
    };
    let ball_pos = ball_pos.value;
    let ball_vel = ball_vel.value;
    let dt = fixed_dt.delta_secs();
    let swing_cfg = &config.shot.racket_swing;

    for (player, player_pos, mut shot_state) in player_query.iter_mut() {
        let swing = &mut shot_state.racket_swing;
        if !swing.is_active {
            continue;
        }

        swing.elapsed_seconds += dt;
        swing.previous_racket_position = swing.current_racket_position;
        swing.current_racket_position = sample_swing_position(swing, swing.elapsed_seconds);

        let within_contact_window = swing.elapsed_seconds
            >= swing.contact_time_seconds - swing_cfg.contact_window_seconds
            && swing.elapsed_seconds
                <= swing.contact_time_seconds + swing_cfg.contact_window_seconds;

        if !swing.contact_done && within_contact_window {
            let ball_prev = ball_pos - ball_vel * dt;
            let dist_now = distance_point_to_segment(
                ball_pos,
                swing.previous_racket_position,
                swing.current_racket_position,
            );
            let dist_prev = distance_point_to_segment(
                ball_prev,
                swing.previous_racket_position,
                swing.current_racket_position,
            );
            let distance = dist_now.min(dist_prev);

            if distance <= swing_cfg.contact_radius {
                swing.contact_done = true;
                let contact_point = closest_point_on_segment(
                    ball_pos,
                    swing.previous_racket_position,
                    swing.current_racket_position,
                );
                let racket_velocity =
                    (swing.current_racket_position - swing.previous_racket_position) / dt.max(1e-6);

                contact_writer.write(RacketContactEvent {
                    player_id: player.id,
                    court_side: player.court_side,
                    contact_point,
                    racket_velocity,
                });

                shot_writer.write(ShotEvent {
                    player_id: player.id,
                    court_side: player.court_side,
                    direction: swing.input_direction,
                    jump_height: player_pos.value.y,
                    is_serve: false,
                    hit_position: Some(contact_point),
                    serve_toss_velocity_y: None,
                });
            }
        }

        if swing.elapsed_seconds >= swing.duration_seconds {
            swing.clear();
        }
    }
}

fn front_sign(court_side: CourtSide) -> f32 {
    match court_side {
        CourtSide::Left => 1.0,
        CourtSide::Right => -1.0,
    }
}

fn predict_hit_point(
    player_pos: Vec3,
    ball_pos: Vec3,
    ball_vel: Vec3,
    gravity: f32,
    min_t: f32,
    max_t: f32,
    step_t: f32,
    reach_distance: f32,
    max_hit_height_diff: f32,
) -> (Vec3, f32) {
    let mut t = min_t.max(0.0);
    let max_t = max_t.max(t + step_t);
    let step_t = step_t.max(1e-3);

    let mut best_pos = predict_ball_position(ball_pos, ball_vel, gravity, t);
    let mut best_time = t;
    let mut best_score = f32::MAX;

    while t <= max_t {
        let p = predict_ball_position(ball_pos, ball_vel, gravity, t);
        let horizontal = Vec2::new(p.x - player_pos.x, p.z - player_pos.z).length();
        let height_diff = (p.y - player_pos.y).abs();

        let reach_penalty = (horizontal - reach_distance).max(0.0) * 2.0
            + (height_diff - max_hit_height_diff).max(0.0) * 2.0;
        let score = reach_penalty + t * 0.05;

        if score < best_score {
            best_score = score;
            best_pos = p;
            best_time = t;
        }

        if horizontal <= reach_distance && height_diff <= max_hit_height_diff {
            return (p, t);
        }

        t += step_t;
    }

    (best_pos, best_time)
}

fn predict_ball_position(ball_pos: Vec3, ball_vel: Vec3, gravity: f32, t: f32) -> Vec3 {
    ball_pos + ball_vel * t + Vec3::new(0.0, 0.5 * gravity * t * t, 0.0)
}

fn sample_swing_position(swing: &crate::components::RacketSwingState, time: f32) -> Vec3 {
    let t = (time / swing.duration_seconds).clamp(0.0, 1.0);
    let tc = (swing.contact_time_seconds / swing.duration_seconds).clamp(0.01, 0.99);

    if t <= tc {
        let u = (t / tc).clamp(0.0, 1.0);
        quadratic_bezier(
            swing.start_position,
            swing.pre_contact_position,
            swing.planned_hit_position,
            u,
        )
    } else {
        let u = ((t - tc) / (1.0 - tc)).clamp(0.0, 1.0);
        quadratic_bezier(
            swing.planned_hit_position,
            swing.post_contact_position,
            swing.end_position,
            u,
        )
    }
}

fn quadratic_bezier(p0: Vec3, p1: Vec3, p2: Vec3, t: f32) -> Vec3 {
    let omt = 1.0 - t;
    omt * omt * p0 + 2.0 * omt * t * p1 + t * t * p2
}

fn distance_point_to_segment(point: Vec3, seg_a: Vec3, seg_b: Vec3) -> f32 {
    point.distance(closest_point_on_segment(point, seg_a, seg_b))
}

fn closest_point_on_segment(point: Vec3, seg_a: Vec3, seg_b: Vec3) -> Vec3 {
    let ab = seg_b - seg_a;
    let len2 = ab.length_squared();
    if len2 <= f32::EPSILON {
        return seg_a;
    }
    let t = (point - seg_a).dot(ab) / len2;
    seg_a + ab * t.clamp(0.0, 1.0)
}
