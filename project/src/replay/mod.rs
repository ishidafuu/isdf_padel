//! Replay Module
//! @spec 77103_replay_spec.md
//!
//! リプレイ機能を提供するモジュール。
//! - 入力の記録
//! - リプレイファイルの保存/読み込み
//! - リプレイの再生

pub mod data;
pub mod loader;
pub mod manager;
pub mod player;
pub mod recorder;

use bevy::{app::Last, ecs::message::MessageReader, prelude::*};

use crate::components::{HumanControlled, Player};
use crate::core::CourtSide;
use crate::resource::{GameRng, MatchFlowState};

pub use data::ControlType;
pub use manager::ReplayManager;
pub use recorder::{ReplayRecorder, StartReplayRecording, StopReplayRecording};

/// リプレイプラグイン（記録機能）
/// 通常のゲームプレイ時に使用
pub struct ReplayRecordPlugin;

impl Plugin for ReplayRecordPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReplayRecorder>()
            .init_resource::<ReplayManager>()
            .add_message::<StartReplayRecording>()
            .add_message::<StopReplayRecording>()
            .add_systems(Startup, manager::startup_cleanup_system)
            // 試合開始時にリプレイ記録を開始
            .add_systems(OnEnter(MatchFlowState::MatchStart), start_replay_on_match_start)
            .add_systems(
                Update,
                (
                    recorder::start_recording_system,
                    recorder::record_frame_system,
                    recorder::stop_recording_system,
                    auto_save_on_match_end_system,
                ),
            )
            // アプリ終了時にリプレイ保存（途中終了対応）
            .add_systems(Last, save_replay_on_exit);
    }
}

/// 試合開始時にリプレイ記録を開始
/// @spec REQ-77103-002
fn start_replay_on_match_start(
    mut recorder: ResMut<ReplayRecorder>,
    match_score: Res<crate::resource::MatchScore>,
    game_rng: Res<GameRng>,
    players: Query<(&Player, Option<&HumanControlled>)>,
) {
    // GameRng から現在のシードを取得
    let seed = game_rng.seed();

    // ゲーム状態から初期サーブ側を取得
    let initial_serve_side = match_score.server;

    // プレイヤーのコントロールタイプを判定
    let mut left_control = ControlType::Ai;
    let mut right_control = ControlType::Ai;

    for (player, human) in players.iter() {
        let control = if human.is_some() {
            ControlType::Human
        } else {
            ControlType::Ai
        };
        match player.court_side {
            CourtSide::Left => left_control = control,
            CourtSide::Right => right_control = control,
        }
    }

    recorder.start_recording(seed, initial_serve_side, left_control, right_control);
    info!(
        "Replay recording started with seed: {}, initial_serve_side: {:?}, left: {:?}, right: {:?}",
        seed, initial_serve_side, left_control, right_control
    );
}

/// 試合終了時に自動保存
/// @spec REQ-77103-003
fn auto_save_on_match_end_system(
    match_state: Res<State<MatchFlowState>>,
    mut prev_state: Local<Option<MatchFlowState>>,
    mut recorder: ResMut<ReplayRecorder>,
    manager: Res<ReplayManager>,
) {
    let current = *match_state.get();

    // 状態変化を検出
    let state_changed = prev_state.is_none_or(|prev| prev != current);
    *prev_state = Some(current);

    if !state_changed {
        return;
    }

    // MatchEnd に遷移したら保存
    if current == MatchFlowState::MatchEnd && recorder.is_recording() {
        recorder.stop_recording();

        if let Some(data) = recorder.take_data() {
            match manager.save_replay(&data) {
                Ok(path) => {
                    info!("Replay saved to {:?}", path);
                    // 上限チェック
                    if let Err(e) = manager.cleanup_excess_replays() {
                        warn!("Failed to cleanup excess replays: {}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to save replay: {}", e);
                }
            }
        }
    }
}

/// ゲーム終了時に記録中のリプレイを保存
fn save_replay_on_exit(
    mut exit_reader: MessageReader<AppExit>,
    mut recorder: ResMut<ReplayRecorder>,
    manager: Res<ReplayManager>,
) {
    for _ in exit_reader.read() {
        if recorder.is_recording() {
            recorder.stop_recording();
            if let Some(data) = recorder.take_data() {
                match manager.save_replay(&data) {
                    Ok(path) => info!("Replay saved on exit: {:?}", path),
                    Err(e) => error!("Failed to save replay on exit: {}", e),
                }
            }
        }
    }
}
