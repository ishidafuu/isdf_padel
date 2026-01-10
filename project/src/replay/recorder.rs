#![allow(dead_code)]
//! Replay Recorder
//! @spec 77103_replay_spec.md
//!
//! 試合中の入力を記録するシステム。

use bevy::ecs::message::MessageReader;
use bevy::prelude::*;

use crate::components::{InputState, Player};
use crate::core::CourtSide;
use crate::resource::MatchFlowState;

use super::data::{FrameInput, InputSnapshot, ReplayData, ReplayMetadata};

/// リプレイ記録リソース
/// @spec REQ-77103-001, REQ-77103-002
#[derive(Resource, Default)]
pub struct ReplayRecorder {
    /// 記録中のリプレイデータ
    data: Option<ReplayData>,
    /// 現在のフレーム番号
    frame_count: u32,
    /// 記録中かどうか
    is_recording: bool,
}

impl ReplayRecorder {
    /// 新しいレコーダーを作成
    pub fn new() -> Self {
        Self::default()
    }

    /// 記録を開始
    /// @spec REQ-77103-002
    pub fn start_recording(&mut self, seed: u64, initial_serve_side: CourtSide) {
        let metadata = ReplayMetadata::new(seed, initial_serve_side);
        self.data = Some(ReplayData::new(metadata));
        self.frame_count = 0;
        self.is_recording = true;
        info!("Replay recording started");
    }

    /// 記録を停止
    pub fn stop_recording(&mut self) {
        self.is_recording = false;
        info!(
            "Replay recording stopped. Total frames: {}",
            self.frame_count
        );
    }

    /// 記録中かどうか
    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    /// フレーム入力を記録
    /// @spec REQ-77103-001
    pub fn record_frame(&mut self, p1_input: &InputState, p2_input: &InputState) {
        if !self.is_recording {
            return;
        }

        if let Some(ref mut data) = self.data {
            let frame = FrameInput::new(
                self.frame_count,
                InputSnapshot::from_input_state(p1_input),
                InputSnapshot::from_input_state(p2_input),
            );
            data.push_frame(frame);
            self.frame_count += 1;
        }
    }

    /// 記録したリプレイデータを取得（消費）
    pub fn take_data(&mut self) -> Option<ReplayData> {
        self.is_recording = false;
        self.frame_count = 0;
        self.data.take()
    }

    /// 記録したリプレイデータへの参照
    pub fn data(&self) -> Option<&ReplayData> {
        self.data.as_ref()
    }

    /// 現在のフレーム数
    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }
}

/// 記録開始イベント
#[derive(bevy::ecs::message::Message)]
pub struct StartReplayRecording {
    pub seed: u64,
    pub initial_serve_side: CourtSide,
}

/// 記録停止イベント
#[derive(bevy::ecs::message::Message)]
pub struct StopReplayRecording;

/// 記録開始システム
/// @spec REQ-77103-002
pub fn start_recording_system(
    mut recorder: ResMut<ReplayRecorder>,
    mut events: MessageReader<StartReplayRecording>,
) {
    for event in events.read() {
        recorder.start_recording(event.seed, event.initial_serve_side);
    }
}

/// フレーム入力記録システム
/// @spec REQ-77103-001
/// ECS設計原則: court_sideベースでプレイヤーを識別（固定IDを排除）
pub fn record_frame_system(
    mut recorder: ResMut<ReplayRecorder>,
    match_state: Res<State<MatchFlowState>>,
    players: Query<(&Player, &InputState)>,
) {
    // Rally または Serve 状態でのみ記録
    let should_record = matches!(
        match_state.get(),
        MatchFlowState::Rally | MatchFlowState::Serve
    );

    if !should_record || !recorder.is_recording() {
        return;
    }

    // Left側, Right側 の入力を取得
    let mut left_input: Option<&InputState> = None;
    let mut right_input: Option<&InputState> = None;

    for (player, input) in players.iter() {
        match player.court_side {
            CourtSide::Left => left_input = Some(input),
            CourtSide::Right => right_input = Some(input),
        }
    }

    // 両方の入力が揃ったら記録
    // p1 = Left側, p2 = Right側 として保存
    if let (Some(left), Some(right)) = (left_input, right_input) {
        recorder.record_frame(left, right);
    }
}

/// 記録停止システム
pub fn stop_recording_system(
    mut recorder: ResMut<ReplayRecorder>,
    mut events: MessageReader<StopReplayRecording>,
) {
    for _ in events.read() {
        recorder.stop_recording();
    }
}
