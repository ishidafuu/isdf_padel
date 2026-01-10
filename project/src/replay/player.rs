#![allow(dead_code)]
//! Replay Player
//! @spec 77103_replay_spec.md
//!
//! リプレイの再生システム。
//! 記録された入力データをInputStateに注入する。

use bevy::prelude::*;

use crate::components::{InputState, Player};
use crate::core::CourtSide;

use super::data::{InputSnapshot, ReplayData};

/// リプレイ再生リソース
/// @spec REQ-77103-008
#[derive(Resource)]
pub struct ReplayPlayer {
    /// 再生中のリプレイデータ
    data: Option<ReplayData>,
    /// 現在のフレームインデックス
    current_frame: usize,
    /// 再生中かどうか
    is_playing: bool,
    /// 再生完了かどうか
    is_finished: bool,
}

impl Default for ReplayPlayer {
    fn default() -> Self {
        Self {
            data: None,
            current_frame: 0,
            is_playing: false,
            is_finished: false,
        }
    }
}

impl ReplayPlayer {
    /// 新しいプレイヤーを作成
    pub fn new() -> Self {
        Self::default()
    }

    /// リプレイデータを設定して再生開始
    pub fn start_playback(&mut self, data: ReplayData) {
        info!(
            "Starting replay playback: {} frames",
            data.frames.len()
        );
        self.data = Some(data);
        self.current_frame = 0;
        self.is_playing = true;
        self.is_finished = false;
    }

    /// 再生を停止
    pub fn stop_playback(&mut self) {
        self.is_playing = false;
        info!("Replay playback stopped at frame {}", self.current_frame);
    }

    /// 再生中かどうか
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// 再生完了かどうか
    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    /// 現在のフレーム入力を取得してフレームを進める
    pub fn advance_frame(&mut self) -> Option<(InputSnapshot, InputSnapshot)> {
        if !self.is_playing || self.is_finished {
            return None;
        }

        let data = self.data.as_ref()?;

        if self.current_frame >= data.frames.len() {
            self.is_finished = true;
            self.is_playing = false;
            info!("Replay playback finished");
            return None;
        }

        let frame = &data.frames[self.current_frame];
        self.current_frame += 1;

        Some((frame.p1, frame.p2))
    }

    /// 現在のフレーム番号
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    /// 総フレーム数
    pub fn total_frames(&self) -> usize {
        self.data.as_ref().map_or(0, |d| d.frames.len())
    }

    /// メタデータへの参照
    pub fn metadata(&self) -> Option<&super::data::ReplayMetadata> {
        self.data.as_ref().map(|d| &d.metadata)
    }

    /// シード値を取得
    pub fn seed(&self) -> Option<u64> {
        self.data.as_ref().map(|d| d.metadata.seed)
    }
}

/// 入力注入システム
/// @spec REQ-77103-008
/// ECS設計原則: court_sideベースでプレイヤーを識別（固定IDを排除）
pub fn replay_input_system(
    mut replay_player: ResMut<ReplayPlayer>,
    mut players: Query<(&Player, &mut InputState)>,
) {
    if !replay_player.is_playing() {
        return;
    }

    // 次のフレーム入力を取得
    // p1 = Left側, p2 = Right側
    let Some((left_snapshot, right_snapshot)) = replay_player.advance_frame() else {
        return;
    };

    // 各プレイヤーに入力を注入
    for (player, mut input) in players.iter_mut() {
        let snapshot = match player.court_side {
            CourtSide::Left => &left_snapshot,
            CourtSide::Right => &right_snapshot,
        };

        input.movement = snapshot.movement;
        input.jump_pressed = snapshot.jump_pressed;
        input.shot_pressed = snapshot.shot_pressed;
        input.holding = snapshot.holding;
        input.hold_time = snapshot.hold_time;
    }
}

/// リプレイ再生完了イベント
#[derive(bevy::ecs::message::Message)]
pub struct ReplayPlaybackFinished;

/// 再生完了検出システム
pub fn replay_finished_check_system(
    replay_player: Res<ReplayPlayer>,
    mut finished_events: bevy::ecs::message::MessageWriter<ReplayPlaybackFinished>,
    mut prev_finished: Local<bool>,
) {
    let is_finished = replay_player.is_finished();

    // 完了状態に変化があった場合のみイベント発火
    if is_finished && !*prev_finished {
        finished_events.write(ReplayPlaybackFinished);
        info!("Replay playback finished event sent");
    }

    *prev_finished = is_finished;
}
