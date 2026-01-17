//! AIショットシステム v0.6
//! @spec 30302_ai_shot_spec.md

use bevy::prelude::*;

use crate::components::{
    AiController, Ball, BounceCount, KnockbackState, LastShooter, LogicalPosition, Player, ShotState,
};
use crate::core::events::ShotEvent;
use crate::resource::config::GameConfig;
use crate::resource::{GameRng, MatchScore, RallyPhase, RallyState};
use crate::simulation::DebugLogger;

/// ショット可能条件をチェック
/// @spec 30302_ai_shot_spec.md#req-30302-001
/// @spec 30302_ai_shot_spec.md#req-30302-002
///
/// 以下の条件をすべて満たす場合のみ true を返す:
/// - ふっとばし中でない
/// - サーブ時のリターナー制約を満たしている
/// - 自分が打ったボールでない
/// - クールダウン中でない
/// - ボールが射程距離内
#[allow(clippy::too_many_arguments)]
fn can_ai_shoot(
    knockback: &KnockbackState,
    rally_state: &RallyState,
    match_score: &MatchScore,
    bounce_count: &BounceCount,
    player: &Player,
    last_shooter: &LastShooter,
    shot_state: &ShotState,
    distance_3d: f32,
    max_distance: f32,
) -> bool {
    // ふっとばし中はショット禁止
    if knockback.is_knockback_active() {
        return false;
    }

    // サーブ中でボールがまだバウンドしていない場合、リターナーはショット禁止
    // パデルルール: サーブは必ず1バウンドしてからリターンする
    if rally_state.phase == RallyPhase::Serving && bounce_count.count == 0 {
        // リターナー（サーバーの相手側）のみブロック
        if player.court_side != match_score.server {
            return false;
        }
    }

    // 自分が打ったボールは打てない（相手が打ち返すまで待つ）
    if last_shooter.side == Some(player.court_side) {
        return false;
    }

    // REQ-30302-002: クールダウン中はショット禁止
    if shot_state.is_on_cooldown() {
        return false;
    }

    // REQ-30302-001: 球体判定（3D距離）
    if distance_3d > max_distance {
        return false;
    }

    true
}

/// 打球方向の制御値を計算
/// @spec 30302_ai_shot_spec.md#req-30302-003
/// @spec 30302_ai_shot_spec.md#req-30302-055
///
/// 戻り値の Vec2:
/// - x: 深さ制御 (-1.0=ネット際, 0.0=サービスライン付近, +1.0=ベースライン際)
/// - y: 横方向制御 (-1.0〜+1.0)
fn calculate_shot_direction(
    game_rng: &mut GameRng,
    direction_variance: f32,
) -> Vec2 {
    // ネットフォルトを避けるため、サービスライン〜ベースライン中間を狙う
    let base_depth = 0.3_f32; // やや浅め（安全マージン）
    let base_lateral = 0.0_f32; // コート中央

    // REQ-30302-055: ランダムブレを適用（制御値として）
    // direction_variance（度）を制御値範囲に変換
    let variance_factor = (direction_variance / 45.0).clamp(0.0, 1.0);
    let depth_offset = game_rng.random_range(-variance_factor..=variance_factor) * 0.2;
    let lateral_offset = game_rng.random_range(-variance_factor..=variance_factor) * 0.5;

    Vec2::new(
        (base_depth + depth_offset).clamp(-0.5, 1.0), // ネット際は避ける（-0.5以上）
        (base_lateral + lateral_offset).clamp(-1.0, 1.0),
    )
}

/// AIショットシステム v0.6
/// @spec 30302_ai_shot_spec.md#req-30302-001
/// @spec 30302_ai_shot_spec.md#req-30302-002
/// @spec 30302_ai_shot_spec.md#req-30302-003
/// @spec 30302_ai_shot_spec.md#req-30302-004
/// @spec 30302_ai_shot_spec.md#req-30302-005
/// @spec 30302_ai_shot_spec.md#req-30302-055
#[allow(clippy::too_many_arguments)]
pub fn ai_shot_system(
    config: Res<GameConfig>,
    mut game_rng: ResMut<GameRng>,
    rally_state: Res<RallyState>,
    match_score: Res<MatchScore>,
    mut debug_logger: Option<ResMut<DebugLogger>>,
    ball_query: Query<(&LogicalPosition, &LastShooter, &BounceCount), With<Ball>>,
    mut ai_query: Query<
        (
            &Player,
            &LogicalPosition,
            &mut ShotState,
            &KnockbackState,
        ),
        With<AiController>,
    >,
    mut event_writer: MessageWriter<ShotEvent>,
) {
    // ボール位置、LastShooter、BounceCountを取得（存在しなければ何もしない）
    let Some((ball_logical_pos, last_shooter, bounce_count)) = ball_query.iter().next() else {
        return;
    };
    let ball_pos = ball_logical_pos.value;

    for (player, ai_pos, mut shot_state, knockback) in ai_query.iter_mut() {
        let ai_position = ai_pos.value;
        let distance_3d = (ai_position - ball_pos).length();

        // ショット可能条件をチェック
        if !can_ai_shoot(
            knockback,
            &rally_state,
            &match_score,
            bounce_count,
            player,
            last_shooter,
            &shot_state,
            distance_3d,
            config.shot.max_distance,
        ) {
            continue;
        }

        // 打球方向を計算
        let direction = calculate_shot_direction(&mut game_rng, config.ai.direction_variance);

        // REQ-30302-004: クールダウン開始
        shot_state.start_cooldown(config.ai.shot_cooldown);

        // REQ-30302-005: ジャンプショット禁止（MVP）- jumpHeight = 0
        let jump_height = 0.0;

        // ShotEvent を発行（通常ショット: is_serve = false）
        // @spec 30602_shot_direction_spec.md#req-30602-032
        event_writer.write(ShotEvent {
            player_id: player.id,
            court_side: player.court_side,
            direction,
            jump_height,
            is_serve: false,
            hit_position: None,
        });

        // AIショットログ出力
        if let Some(ref mut logger) = debug_logger {
            logger.log_ai(&format!(
                "P{} SHOT distance_3d={:.2} dir=({:.2},{:.2}) cooldown={:.2}",
                player.id, distance_3d, direction.x, direction.y, config.ai.shot_cooldown
            ));
        }

        info!(
            "AI Player {} shot! direction: {:?}, distance_3d: {:.2}",
            player.id, direction, distance_3d
        );
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// REQ-30302-003: 打球方向計算テスト
    /// X軸が打ち合い方向、Z軸がサイド方向
    #[test]
    fn test_direction_to_opponent_court() {
        // Leftコート（X<0側）にいるAIは、Rightコート（X>0側）を狙う
        let ai_position = Vec3::new(-5.0, 0.0, 0.0);
        let court_depth = 16.0;
        // 相手コート中央はX = +4.0（depth/4）
        let opponent_court_center = Vec3::new(court_depth / 4.0, 0.0, 0.0);
        let direction = (opponent_court_center - ai_position).normalize();

        // AIは Left側にいるので、相手コートは +X方向
        assert!(direction.x > 0.0, "Should aim towards +X (opponent's court)");
        // 中央に打つのでZ方向は小さい
        assert!(direction.z.abs() < 0.001, "Z should be near zero");
    }

    /// REQ-30302-055: 方向計算のランダムブレテスト
    #[test]
    fn test_shot_direction_variance() {
        use crate::resource::GameRng;

        let mut rng = GameRng::from_seed(12345);

        // 誤差0の場合は基本値
        let dir_no_variance = calculate_shot_direction(&mut rng, 0.0);
        assert!((dir_no_variance.x - 0.3).abs() < 0.001);
        assert!((dir_no_variance.y - 0.0).abs() < 0.001);

        // 誤差ありの場合は範囲内
        let dir_with_variance = calculate_shot_direction(&mut rng, 15.0);
        assert!(dir_with_variance.x >= -0.5 && dir_with_variance.x <= 1.0);
        assert!(dir_with_variance.y >= -1.0 && dir_with_variance.y <= 1.0);
    }
}
