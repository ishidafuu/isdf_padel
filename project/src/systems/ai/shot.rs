//! AIショットシステム v0.6
//! @spec 30302_ai_shot_spec.md
//! @spec 30303_ai_tactics_spec.md

use bevy::prelude::*;

use crate::components::{
    AiController, Ball, BounceCount, KnockbackState, LastShooter, LogicalPosition, Player,
    ShotState, TacticsType,
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

/// 戦術を選択
/// @spec 30303_ai_tactics_spec.md#req-30303-010
/// @spec 30303_ai_tactics_spec.md#req-30303-011
///
/// 打点距離に応じて攻め/守りを決定し、確率調整を適用する
fn select_tactics(
    distance_to_ball: f32,
    optimal_distance: f32,
    offensive_probability: f32,
    game_rng: &mut GameRng,
) -> TacticsType {
    // REQ-30303-010: 距離が最適距離より大きい場合は守り
    if distance_to_ball > optimal_distance {
        return TacticsType::Defensive;
    }

    // REQ-30303-011: 距離が短くても確率で守りを選ぶ場合がある
    let roll: f32 = game_rng.random_range(0.0..=1.0);
    if roll < offensive_probability {
        TacticsType::Offensive
    } else {
        TacticsType::Defensive
    }
}

/// 打球方向の制御値を計算
/// @spec 30302_ai_shot_spec.md#req-30302-003
/// @spec 30302_ai_shot_spec.md#req-30302-055
/// @spec 30303_ai_tactics_spec.md#req-30303-020
/// @spec 30303_ai_tactics_spec.md#req-30303-021
///
/// 戻り値の Vec2:
/// - x: 深さ制御 (-1.0=ネット際, 0.0=サービスライン付近, +1.0=ベースライン際)
/// - y: 横方向制御 (-1.0〜+1.0)
fn calculate_shot_direction(
    tactics: TacticsType,
    court_width: f32,
    offensive_margin: f32,
    game_rng: &mut GameRng,
    direction_variance: f32,
) -> Vec2 {
    // 戦術に応じたベース値を設定
    let (base_depth, base_lateral) = match tactics {
        // REQ-30303-020: 守り → コート中央狙い
        TacticsType::Defensive => {
            (0.3_f32, 0.0_f32) // やや浅め、中央
        }
        // REQ-30303-021: 攻め → ライン際狙い
        TacticsType::Offensive => {
            // 深い位置（ベースライン際）を狙う
            let depth = 0.8_f32;
            // ライン際を狙う（Z座標の制御値に変換）
            // offensive_margin をコート幅半分からの比率で制御値に変換
            // margin=0.8m, width=12m → サイドライン(6m)から0.8m内側 → 5.2m
            // 制御値 = 5.2 / 6.0 ≈ 0.87
            let half_width = court_width / 2.0;
            let target_z_ratio = (half_width - offensive_margin) / half_width;
            // 左右ランダム選択
            let lateral = if game_rng.random_range(0..=1) == 0 {
                target_z_ratio
            } else {
                -target_z_ratio
            };
            (depth, lateral)
        }
    };

    // REQ-30302-055: ランダムブレを適用（制御値として）
    // direction_variance（度）を制御値範囲に変換
    let variance_factor = (direction_variance / 45.0).clamp(0.0, 1.0);
    let depth_offset = game_rng.random_range(-variance_factor..=variance_factor) * 0.2;
    let lateral_offset = game_rng.random_range(-variance_factor..=variance_factor) * 0.5;

    Vec2::new(
        (base_depth + depth_offset).clamp(-0.5_f32, 1.0_f32), // ネット際は避ける（-0.5以上）
        (base_lateral + lateral_offset).clamp(-1.0_f32, 1.0_f32),
    )
}

/// AIショットシステム v0.6
/// @spec 30302_ai_shot_spec.md#req-30302-001
/// @spec 30302_ai_shot_spec.md#req-30302-002
/// @spec 30302_ai_shot_spec.md#req-30302-003
/// @spec 30302_ai_shot_spec.md#req-30302-004
/// @spec 30302_ai_shot_spec.md#req-30302-005
/// @spec 30302_ai_shot_spec.md#req-30302-055
/// @spec 30303_ai_tactics_spec.md#req-30303-010
/// @spec 30303_ai_tactics_spec.md#req-30303-011
#[allow(clippy::too_many_arguments)]
pub fn ai_shot_system(
    config: Res<GameConfig>,
    mut game_rng: ResMut<GameRng>,
    rally_state: Res<RallyState>,
    match_score: Res<MatchScore>,
    mut debug_logger: Option<ResMut<DebugLogger>>,
    ball_query: Query<(&LogicalPosition, &LastShooter, &BounceCount), With<Ball>>,
    mut ai_query: Query<(
        &Player,
        &LogicalPosition,
        &mut ShotState,
        &KnockbackState,
        &mut AiController,
    )>,
    mut event_writer: MessageWriter<ShotEvent>,
) {
    // ボール位置、LastShooter、BounceCountを取得（存在しなければ何もしない）
    let Some((ball_logical_pos, last_shooter, bounce_count)) = ball_query.iter().next() else {
        return;
    };
    let ball_pos = ball_logical_pos.value;

    for (player, ai_pos, mut shot_state, knockback, mut ai_controller) in ai_query.iter_mut() {
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

        // REQ-30303-010, REQ-30303-011: 戦術を選択
        let tactics = select_tactics(
            distance_3d,
            config.ai.optimal_distance,
            config.ai.offensive_probability,
            &mut game_rng,
        );
        ai_controller.current_tactics = tactics;

        // 打球方向を計算（戦術に応じて変化）
        let direction = calculate_shot_direction(
            tactics,
            config.court.width,
            config.ai.offensive_margin,
            &mut game_rng,
            config.ai.direction_variance,
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
            serve_toss_velocity_y: None,
        });

        // AIショットログ出力（戦術情報を追加）
        if let Some(ref mut logger) = debug_logger {
            logger.log_ai(&format!(
                "P{} SHOT tactics={:?} distance_3d={:.2} dir=({:.2},{:.2}) cooldown={:.2}",
                player.id, tactics, distance_3d, direction.x, direction.y, config.ai.shot_cooldown
            ));
        }

        info!(
            "AI Player {} shot! tactics: {:?}, direction: {:?}, distance_3d: {:.2}",
            player.id, tactics, direction, distance_3d
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
        assert!(
            direction.x > 0.0,
            "Should aim towards +X (opponent's court)"
        );
        // 中央に打つのでZ方向は小さい
        assert!(direction.z.abs() < 0.001, "Z should be near zero");
    }

    /// REQ-30303-010: 戦術選択テスト（距離による選択）
    #[test]
    fn test_select_tactics_by_distance() {
        use crate::resource::GameRng;

        let mut rng = GameRng::from_seed(12345);
        let optimal_distance = 1.2;
        let offensive_probability = 1.0; // 確実に攻め

        // 距離が短い場合 → 攻め
        let tactics_close = select_tactics(1.0, optimal_distance, offensive_probability, &mut rng);
        assert_eq!(tactics_close, TacticsType::Offensive);

        // 距離が遠い場合 → 守り
        let tactics_far = select_tactics(2.0, optimal_distance, offensive_probability, &mut rng);
        assert_eq!(tactics_far, TacticsType::Defensive);
    }

    /// REQ-30303-011: 戦術選択テスト（確率による選択）
    #[test]
    fn test_select_tactics_by_probability() {
        use crate::resource::GameRng;

        let mut rng = GameRng::from_seed(12345);
        let optimal_distance = 1.2;

        // 攻め確率0%の場合 → 常に守り
        let tactics_0 = select_tactics(1.0, optimal_distance, 0.0, &mut rng);
        assert_eq!(tactics_0, TacticsType::Defensive);
    }

    /// REQ-30303-020: 守りショット方向テスト（コート中央）
    #[test]
    fn test_defensive_shot_direction() {
        use crate::resource::GameRng;

        let mut rng = GameRng::from_seed(12345);
        let court_width = 12.0;
        let offensive_margin = 0.8;

        // 守り戦術の場合、中央狙い（y ≈ 0）
        let dir = calculate_shot_direction(
            TacticsType::Defensive,
            court_width,
            offensive_margin,
            &mut rng,
            0.0, // ブレなし
        );
        assert!(
            (dir.y - 0.0).abs() < 0.001,
            "Defensive should aim center (y ≈ 0)"
        );
        assert!((dir.x - 0.3).abs() < 0.001, "Defensive depth should be 0.3");
    }

    /// REQ-30303-021: 攻めショット方向テスト（ライン際）
    #[test]
    fn test_offensive_shot_direction() {
        use crate::resource::GameRng;

        let mut rng = GameRng::from_seed(12345);
        let court_width = 12.0;
        let offensive_margin = 0.8;
        // 期待値: (6.0 - 0.8) / 6.0 = 5.2 / 6.0 ≈ 0.867

        // 攻め戦術の場合、ライン際狙い（|y| > 0.8）
        let dir = calculate_shot_direction(
            TacticsType::Offensive,
            court_width,
            offensive_margin,
            &mut rng,
            0.0, // ブレなし
        );
        assert!(
            dir.y.abs() > 0.8,
            "Offensive should aim sideline (|y| > 0.8)"
        );
        assert!((dir.x - 0.8).abs() < 0.001, "Offensive depth should be 0.8");
    }

    /// REQ-30302-055: 方向計算のランダムブレテスト
    #[test]
    fn test_shot_direction_variance() {
        use crate::resource::GameRng;

        let mut rng = GameRng::from_seed(12345);
        let court_width = 12.0;
        let offensive_margin = 0.8;

        // 誤差ありの場合は範囲内
        let dir_with_variance = calculate_shot_direction(
            TacticsType::Defensive,
            court_width,
            offensive_margin,
            &mut rng,
            15.0, // ブレあり
        );
        assert!(dir_with_variance.x >= -0.5 && dir_with_variance.x <= 1.0);
        assert!(dir_with_variance.y >= -1.0 && dir_with_variance.y <= 1.0);
    }
}
