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

/// 打球方向にランダムブレを追加
/// @spec 30302_ai_shot_spec.md#req-30302-055
///
/// 2D回転行列で方向ベクトルを回転させる
fn apply_direction_variance(base_direction: Vec2, variance_deg: f32, game_rng: &mut GameRng) -> Vec2 {
    if variance_deg <= 0.0 {
        return base_direction;
    }

    let variance_rad = variance_deg.to_radians();
    let offset = game_rng.random_range(-variance_rad..=variance_rad);

    // 2D回転行列を適用
    let cos = offset.cos();
    let sin = offset.sin();
    Vec2::new(
        base_direction.x * cos - base_direction.y * sin,
        base_direction.x * sin + base_direction.y * cos,
    )
}

/// AIショットシステム v0.6
/// @spec 30302_ai_shot_spec.md#req-30302-001
/// @spec 30302_ai_shot_spec.md#req-30302-002
/// @spec 30302_ai_shot_spec.md#req-30302-003
/// @spec 30302_ai_shot_spec.md#req-30302-004
/// @spec 30302_ai_shot_spec.md#req-30302-005
/// @spec 30302_ai_shot_spec.md#req-30302-055
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
        // ふっとばし中はショット禁止
        if knockback.is_knockback_active() {
            continue;
        }

        // サーブ中でボールがまだバウンドしていない場合、リターナーはショット禁止
        // パデルルール: サーブは必ず1バウンドしてからリターンする
        if rally_state.phase == RallyPhase::Serving && bounce_count.count == 0 {
            // リターナー（サーバーの相手側）のみブロック
            if player.court_side != match_score.server {
                continue;
            }
        }

        // 自分が打ったボールは打てない（相手が打ち返すまで待つ）
        if last_shooter.side == Some(player.court_side) {
            continue;
        }

        // REQ-30302-002: クールダウン中はショット禁止
        if shot_state.is_on_cooldown() {
            continue;
        }

        let ai_position = ai_pos.value;

        // REQ-30302-001: ショット可能判定（距離）
        let distance_2d = distance_xz(ai_position, ball_pos);
        if distance_2d > config.shot.max_distance {
            continue;
        }

        // REQ-30302-001: ショット可能判定（高さ）
        let height_diff = (ai_position.y - ball_pos.y).abs();
        if height_diff > config.shot.max_height_diff {
            continue;
        }

        // REQ-30302-003: 打球方向を制御値として設定
        // ShotEvent.direction は制御値（-1.0〜+1.0）として解釈される
        // direction.x: 深さ制御 (-1.0=ネット際, 0.0=サービスライン付近, +1.0=ベースライン際)
        // direction.y: 横方向制御 (-1.0〜+1.0)
        //
        // 注意: 両サイドのプレイヤーとも同じ制御値セマンティクスを使用
        // landing_position.rs がコートサイドに応じて適切に変換する

        // ネットフォルトを避けるため、サービスライン〜ベースライン中間を狙う
        let base_depth = 0.3_f32; // やや浅め（安全マージン）
        let base_lateral = 0.0_f32; // コート中央

        // REQ-30302-055: ランダムブレを適用（制御値として）
        // direction_variance（度）を制御値範囲に変換
        let variance_factor = (config.ai.direction_variance / 45.0).clamp(0.0, 1.0);
        let depth_offset = game_rng.random_range(-variance_factor..=variance_factor) * 0.2;
        let lateral_offset = game_rng.random_range(-variance_factor..=variance_factor) * 0.5;

        let direction = Vec2::new(
            (base_depth + depth_offset).clamp(-0.5, 1.0), // ネット際は避ける（-0.5以上）
            (base_lateral + lateral_offset).clamp(-1.0, 1.0),
        );

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
                "P{} SHOT distance={:.2} height_diff={:.2} dir=({:.2},{:.2}) cooldown={:.2}",
                player.id, distance_2d, height_diff, direction.x, direction.y, config.ai.shot_cooldown
            ));
        }

        info!(
            "AI Player {} shot! direction: {:?}, distance: {:.2}",
            player.id, direction, distance_2d
        );
    }
}

/// XZ平面での2D距離計算
/// @spec 30302_ai_shot_spec.md#req-30302-001
#[inline]
fn distance_xz(a: Vec3, b: Vec3) -> f32 {
    let dx = a.x - b.x;
    let dz = a.z - b.z;
    (dx * dx + dz * dz).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// REQ-30302-001: 距離計算テスト
    #[test]
    fn test_distance_xz() {
        let ai_pos = Vec3::new(0.0, 1.0, 5.0);
        let ball_pos = Vec3::new(1.0, 2.0, 5.0);

        let distance = distance_xz(ai_pos, ball_pos);
        // Y軸は無視されるので、距離は1.0
        assert!((distance - 1.0).abs() < 0.001);
    }

    /// REQ-30302-001: 距離計算テスト（XZ両方に距離がある場合）
    #[test]
    fn test_distance_xz_diagonal() {
        let ai_pos = Vec3::new(0.0, 0.0, 0.0);
        let ball_pos = Vec3::new(3.0, 5.0, 4.0);

        let distance = distance_xz(ai_pos, ball_pos);
        // sqrt(3^2 + 4^2) = 5.0
        assert!((distance - 5.0).abs() < 0.001);
    }

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
}
