//! ラケット接触駆動ショットシステム
//! @spec 30606_racket_contact_spec.md

use bevy::prelude::*;

use crate::components::{Ball, BounceState, LogicalPosition, Player, ShotState, Velocity};
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
    ball_query: Query<(&LogicalPosition, &Velocity, &BounceState), With<Ball>>,
    mut player_query: Query<(&Player, &LogicalPosition, &mut ShotState), Without<Ball>>,
) {
    let Some((ball_pos, ball_vel, bounce_state)) = ball_query.iter().next() else {
        return;
    };
    let ball_pos = ball_pos.value;
    let ball_vel = ball_vel.value;
    let is_volley = bounce_state.time_since_bounce.is_none();
    let gravity = config.physics.gravity;
    let swing_cfg = &config.shot.racket_swing;

    for intent in intents.read() {
        for (player, player_pos, mut shot_state) in player_query.iter_mut() {
            if player.id != intent.player_id || shot_state.racket_swing.is_active {
                continue;
            }

            let (predicted_hit, hit_time) = predict_hit_point(
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
            let planned_hit = clamp_hit_to_reach(
                player_pos.value,
                predicted_hit,
                swing_cfg.reach_distance,
                swing_cfg.max_hit_height_diff,
            );

            let swing = &mut shot_state.racket_swing;
            let front_sign = front_sign(player.court_side);
            let lateral_sign = if intent.direction.y >= 0.0 { 1.0 } else { -1.0 };
            let is_slice_like = is_slice_like_hit(ball_vel, planned_hit, player_pos.value);
            let horizontal_distance = Vec2::new(
                planned_hit.x - player_pos.value.x,
                planned_hit.z - player_pos.value.z,
            )
            .length();
            let needs_direct_assist =
                hit_time < 0.20 || horizontal_distance > swing_cfg.reach_distance * 0.85;
            let use_direct_style = is_volley || is_slice_like || needs_direct_assist;

            swing.is_active = true;
            swing.contact_done = false;
            swing.elapsed_seconds = 0.0;
            swing.duration_seconds = swing_cfg.duration_seconds.max(0.01);
            swing.contact_time_seconds = swing_cfg
                .contact_time_seconds
                .clamp(0.01, swing.duration_seconds - 0.01);
            swing.input_direction = intent.direction;
            swing.hold_time_ms = intent.hold_time_ms;
            if use_direct_style {
                // ボレー/スライスは直線寄り（面を直接合わせる）軌道
                swing.planned_hit_position = planned_hit;
                swing.start_position =
                    player_pos.value + Vec3::new(-0.38 * front_sign, 1.00, -0.15 * lateral_sign);
                swing.pre_contact_position = swing.start_position
                    + (planned_hit - swing.start_position) * 0.65
                    + Vec3::new(-0.05 * front_sign, 0.08, -0.08 * lateral_sign);
                swing.post_contact_position =
                    planned_hit + Vec3::new(0.22 * front_sign, 0.10, 0.12 * lateral_sign);
                swing.end_position = swing.post_contact_position
                    + Vec3::new(0.12 * front_sign, -0.05, 0.03 * lateral_sign);
                swing.follow_through_control_position =
                    swing.end_position + Vec3::new(0.06 * front_sign, -0.04, 0.02 * lateral_sign);
            } else {
                // 通常は「後ろから巻き込む」遠心力寄りの軌道
                let travel_xz = Vec2::new(ball_vel.x, ball_vel.z)
                    .try_normalize()
                    .unwrap_or(Vec2::new(front_sign, 0.0));
                let behind_xz = -travel_xz;
                let side_xz = Vec2::new(-travel_xz.y, travel_xz.x) * lateral_sign;
                let behind = Vec3::new(behind_xz.x, 0.0, behind_xz.y);
                let side = Vec3::new(side_xz.x, 0.0, side_xz.y);
                let travel = -behind;

                swing.planned_hit_position =
                    planned_hit + behind * 0.10 + Vec3::new(0.0, -0.02, 0.0);
                swing.planned_hit_position.y = swing.planned_hit_position.y.max(0.25);
                swing.planned_hit_position = clamp_hit_to_reach(
                    player_pos.value,
                    swing.planned_hit_position,
                    swing_cfg.reach_distance,
                    swing_cfg.max_hit_height_diff,
                );

                swing.start_position =
                    player_pos.value + behind * 0.75 + side * 0.35 + Vec3::new(0.0, 1.25, 0.0);
                swing.pre_contact_position =
                    player_pos.value + behind * 0.45 + side * 0.75 + Vec3::new(0.0, 1.55, 0.0);
                swing.post_contact_position = swing.planned_hit_position
                    + side * 0.45
                    + travel * 0.25
                    + Vec3::new(0.0, 0.22, 0.0);
                swing.end_position = swing.planned_hit_position + travel * 0.48 - side * 0.14
                    + Vec3::new(0.0, 0.12, 0.0);
                swing.follow_through_control_position =
                    swing.end_position + travel * 0.18 - side * 0.08 + Vec3::new(0.0, -0.08, 0.0);
            }

            swing.previous_racket_position = swing.start_position;
            swing.current_racket_position = swing.start_position;

            info!(
                "Swing planned: player={}, style={}, hit_time={:.3}, hit=({:.2},{:.2},{:.2})",
                player.id,
                if use_direct_style { "direct" } else { "whip" },
                hit_time,
                swing.planned_hit_position.x,
                swing.planned_hit_position.y,
                swing.planned_hit_position.z
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

            // ラケット接触半径 + ボール半径で判定し、見た目と実接触のズレを減らす
            let contact_threshold = swing_cfg.contact_radius + config.ball.radius;
            if distance <= contact_threshold {
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

fn clamp_hit_to_reach(
    player_pos: Vec3,
    hit: Vec3,
    reach_distance: f32,
    max_hit_height_diff: f32,
) -> Vec3 {
    let mut clamped = hit;

    let horizontal = Vec2::new(hit.x - player_pos.x, hit.z - player_pos.z);
    if horizontal.length_squared() > f32::EPSILON {
        let limited = horizontal.clamp_length_max(reach_distance);
        clamped.x = player_pos.x + limited.x;
        clamped.z = player_pos.z + limited.y;
    } else {
        clamped.x = player_pos.x;
        clamped.z = player_pos.z;
    }

    let min_y = player_pos.y - max_hit_height_diff;
    let max_y = player_pos.y + max_hit_height_diff;
    clamped.y = hit.y.clamp(min_y, max_y).max(0.0);

    clamped
}

fn sample_swing_position(swing: &crate::components::RacketSwingState, time: f32) -> Vec3 {
    let t = (time / swing.duration_seconds).clamp(0.0, 1.0);
    let tc = (swing.contact_time_seconds / swing.duration_seconds).clamp(0.01, 0.99);

    if t <= tc {
        let u = (t / tc).clamp(0.0, 1.0);
        cubic_bezier(
            swing.start_position,
            swing.pre_contact_position,
            swing.post_contact_position,
            swing.planned_hit_position,
            u,
        )
    } else {
        let u = ((t - tc) / (1.0 - tc)).clamp(0.0, 1.0);
        cubic_bezier(
            swing.planned_hit_position,
            swing.post_contact_position,
            swing.follow_through_control_position,
            swing.end_position,
            u,
        )
    }
}

fn cubic_bezier(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    let omt = 1.0 - t;
    omt * omt * omt * p0 + 3.0 * omt * omt * t * p1 + 3.0 * omt * t * t * p2 + t * t * t * p3
}

fn is_slice_like_hit(ball_vel: Vec3, planned_hit: Vec3, player_pos: Vec3) -> bool {
    ball_vel.y < -0.6 && planned_hit.y <= player_pos.y + 1.3
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
