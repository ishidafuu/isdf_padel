//! フォーマット処理 - CSV/JSON/JSONL出力用のフォーマッタ
//! @spec 77100_headless_sim.md

use super::events::GameEvent;
use super::types::{EntityTrace, FrameTrace};

impl GameEvent {
    /// CSV形式の詳細文字列を取得
    pub(crate) fn to_csv_detail(&self) -> String {
        match self {
            GameEvent::BallHit { player, shot_type } => {
                format!("player={},type={}", player, shot_type)
            }
            GameEvent::Bounce { position, court_side } => {
                format!(
                    "pos=({:.2},{:.2},{:.2}),side={:?}",
                    position.x, position.y, position.z, court_side
                )
            }
            GameEvent::WallReflect { position, wall_type } => {
                format!(
                    "pos=({:.2},{:.2},{:.2}),type={}",
                    position.x, position.y, position.z, wall_type
                )
            }
            GameEvent::Point { winner, reason } => {
                format!("winner={},reason={}", winner, reason)
            }
            GameEvent::Fault { fault_type } => {
                format!("type={}", fault_type)
            }
            GameEvent::StateChange { from, to } => {
                format!("from={},to={}", from, to)
            }
            GameEvent::ShotAttributesCalculated {
                player_id,
                input_mode,
                hit_height,
                bounce_elapsed,
                approach_dot,
                ball_distance,
                height_factors,
                timing_factors,
                approach_factors,
                distance_factors,
                final_power,
                final_stability,
                final_angle,
                final_spin,
                final_accuracy,
            } => {
                let bounce_str = bounce_elapsed
                    .map(|v| format!("{:.3}", v))
                    .unwrap_or_else(|| "none".to_string());
                format!(
                    "player={},mode={},height={:.2},bounce={},approach={:.2},dist={:.2},\
                     hf=({:.2},{:.2},{:.2}),tf=({:.2},{:.2},{:.2}),af=({:.2},{:.2}),df=({:.2},{:.2},{:.2}),\
                     power={:.2},stability={:.2},angle={:.2},spin={:.2},accuracy={:.2}",
                    player_id,
                    input_mode,
                    hit_height,
                    bounce_str,
                    approach_dot,
                    ball_distance,
                    height_factors.0,
                    height_factors.1,
                    height_factors.2,
                    timing_factors.0,
                    timing_factors.1,
                    timing_factors.2,
                    approach_factors.0,
                    approach_factors.1,
                    distance_factors.0,
                    distance_factors.1,
                    distance_factors.2,
                    final_power,
                    final_stability,
                    final_angle,
                    final_spin,
                    final_accuracy
                )
            }
            GameEvent::AiMovementDecision {
                player_id,
                movement_state,
                ball_coming_to_me,
                reaction_timer,
                landing_time,
                landing_position,
                trajectory_line_z,
                arrival_distance,
                target_position,
            } => {
                let land_time_str = landing_time
                    .map(|v| format!("{:.3}", v))
                    .unwrap_or_else(|| "none".to_string());
                let land_pos_str = landing_position
                    .map(|p| format!("({:.2},{:.2},{:.2})", p.x, p.y, p.z))
                    .unwrap_or_else(|| "none".to_string());
                format!(
                    "player={},state={},coming={},react={:.3},land_t={},land_p={},\
                     traj_z={:.2},arr_dist={:.2},target=({:.2},{:.2},{:.2})",
                    player_id,
                    movement_state,
                    ball_coming_to_me,
                    reaction_timer,
                    land_time_str,
                    land_pos_str,
                    trajectory_line_z,
                    arrival_distance,
                    target_position.x,
                    target_position.y,
                    target_position.z
                )
            }
            GameEvent::PhysicsAnomaly {
                anomaly_type,
                position,
                velocity,
                expected_value,
                actual_value,
                severity,
            } => {
                format!(
                    "type={},pos=({:.2},{:.2},{:.2}),vel=({:.2},{:.2},{:.2}),\
                     expected={:.2},actual={:.2},severity={}",
                    anomaly_type,
                    position.x,
                    position.y,
                    position.z,
                    velocity.x,
                    velocity.y,
                    velocity.z,
                    expected_value,
                    actual_value,
                    severity
                )
            }
        }
    }

    /// JSON形式の文字列を取得
    pub(crate) fn to_json(&self) -> String {
        match self {
            GameEvent::BallHit { player, shot_type } => {
                format!(
                    "{{\"type\": \"BallHit\", \"player\": {}, \"shot_type\": \"{}\"}}",
                    player, shot_type
                )
            }
            GameEvent::Bounce { position, court_side } => {
                format!(
                    "{{\"type\": \"Bounce\", \"position\": [{:.2}, {:.2}, {:.2}], \"court_side\": \"{:?}\"}}",
                    position.x, position.y, position.z, court_side
                )
            }
            GameEvent::WallReflect { position, wall_type } => {
                format!(
                    "{{\"type\": \"WallReflect\", \"position\": [{:.2}, {:.2}, {:.2}], \"wall_type\": \"{}\"}}",
                    position.x, position.y, position.z, wall_type
                )
            }
            GameEvent::Point { winner, reason } => {
                format!(
                    "{{\"type\": \"Point\", \"winner\": {}, \"reason\": \"{}\"}}",
                    winner, reason
                )
            }
            GameEvent::Fault { fault_type } => {
                format!("{{\"type\": \"Fault\", \"fault_type\": \"{}\"}}", fault_type)
            }
            GameEvent::StateChange { from, to } => {
                format!(
                    "{{\"type\": \"StateChange\", \"from\": \"{}\", \"to\": \"{}\"}}",
                    from, to
                )
            }
            GameEvent::ShotAttributesCalculated {
                player_id,
                input_mode,
                hit_height,
                bounce_elapsed,
                approach_dot,
                ball_distance,
                height_factors,
                timing_factors,
                approach_factors,
                distance_factors,
                final_power,
                final_stability,
                final_angle,
                final_spin,
                final_accuracy,
            } => {
                let bounce_json = bounce_elapsed
                    .map(|v| format!("{:.3}", v))
                    .unwrap_or_else(|| "null".to_string());
                format!(
                    "{{\"type\": \"ShotAttributesCalculated\", \"player_id\": {}, \"input_mode\": \"{}\", \
                     \"hit_height\": {:.2}, \"bounce_elapsed\": {}, \"approach_dot\": {:.2}, \"ball_distance\": {:.2}, \
                     \"height_factors\": [{:.2}, {:.2}, {:.2}], \"timing_factors\": [{:.2}, {:.2}, {:.2}], \
                     \"approach_factors\": [{:.2}, {:.2}], \"distance_factors\": [{:.2}, {:.2}, {:.2}], \
                     \"final_power\": {:.2}, \"final_stability\": {:.2}, \"final_angle\": {:.2}, \
                     \"final_spin\": {:.2}, \"final_accuracy\": {:.2}}}",
                    player_id,
                    input_mode,
                    hit_height,
                    bounce_json,
                    approach_dot,
                    ball_distance,
                    height_factors.0, height_factors.1, height_factors.2,
                    timing_factors.0, timing_factors.1, timing_factors.2,
                    approach_factors.0, approach_factors.1,
                    distance_factors.0, distance_factors.1, distance_factors.2,
                    final_power,
                    final_stability,
                    final_angle,
                    final_spin,
                    final_accuracy
                )
            }
            GameEvent::AiMovementDecision {
                player_id,
                movement_state,
                ball_coming_to_me,
                reaction_timer,
                landing_time,
                landing_position,
                trajectory_line_z,
                arrival_distance,
                target_position,
            } => {
                let land_time_json = landing_time
                    .map(|v| format!("{:.3}", v))
                    .unwrap_or_else(|| "null".to_string());
                let land_pos_json = landing_position
                    .map(|p| format!("[{:.2}, {:.2}, {:.2}]", p.x, p.y, p.z))
                    .unwrap_or_else(|| "null".to_string());
                format!(
                    "{{\"type\": \"AiMovementDecision\", \"player_id\": {}, \"movement_state\": \"{}\", \
                     \"ball_coming_to_me\": {}, \"reaction_timer\": {:.3}, \"landing_time\": {}, \
                     \"landing_position\": {}, \"trajectory_line_z\": {:.2}, \"arrival_distance\": {:.2}, \
                     \"target_position\": [{:.2}, {:.2}, {:.2}]}}",
                    player_id,
                    movement_state,
                    ball_coming_to_me,
                    reaction_timer,
                    land_time_json,
                    land_pos_json,
                    trajectory_line_z,
                    arrival_distance,
                    target_position.x, target_position.y, target_position.z
                )
            }
            GameEvent::PhysicsAnomaly {
                anomaly_type,
                position,
                velocity,
                expected_value,
                actual_value,
                severity,
            } => {
                format!(
                    "{{\"type\": \"PhysicsAnomaly\", \"anomaly_type\": \"{}\", \
                     \"position\": [{:.2}, {:.2}, {:.2}], \"velocity\": [{:.2}, {:.2}, {:.2}], \
                     \"expected_value\": {:.2}, \"actual_value\": {:.2}, \"severity\": \"{}\"}}",
                    anomaly_type,
                    position.x, position.y, position.z,
                    velocity.x, velocity.y, velocity.z,
                    expected_value,
                    actual_value,
                    severity
                )
            }
        }
    }
}

impl EntityTrace {
    /// JSON形式の文字列を取得
    pub(crate) fn to_json(&self) -> String {
        format!(
            "{{\"type\": \"{}\", \"position\": [{:.2}, {:.2}, {:.2}], \"velocity\": [{:.2}, {:.2}, {:.2}]}}",
            self.entity_type.as_str(),
            self.position.x, self.position.y, self.position.z,
            self.velocity.x, self.velocity.y, self.velocity.z
        )
    }
}

impl FrameTrace {
    /// JSONL形式用の1行JSON文字列を取得
    pub(crate) fn to_json_line(&self) -> String {
        let entities_json: Vec<String> = self.entities.iter().map(|e| e.to_json()).collect();
        let events_json: Vec<String> = self.events.iter().map(|e| e.to_json()).collect();

        format!(
            "{{\"frame\": {}, \"timestamp\": {:.3}, \"entities\": [{}], \"events\": [{}]}}",
            self.frame,
            self.timestamp,
            entities_json.join(", "),
            events_json.join(", ")
        )
    }
}
