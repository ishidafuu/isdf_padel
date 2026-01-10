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

use bevy::prelude::*;

use crate::resource::MatchFlowState;

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
            );
    }
}

/// 試合開始時にリプレイ記録を開始
/// @spec REQ-77103-002
fn start_replay_on_match_start(
    mut recorder: ResMut<ReplayRecorder>,
    match_score: Res<crate::resource::MatchScore>,
) {
    // シードを生成（現在時刻ベース）
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);

    // ゲーム状態から初期サーブ側を取得
    let initial_serve_side = match_score.server;

    recorder.start_recording(seed, initial_serve_side);
    info!(
        "Replay recording started with seed: {}, initial_serve_side: {:?}",
        seed, initial_serve_side
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
    let state_changed = prev_state.map_or(true, |prev| prev != current);
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
