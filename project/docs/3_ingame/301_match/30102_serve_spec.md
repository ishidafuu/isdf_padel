# Serve Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

サーブの処理を定義します。MVP v0.1では簡易実装（手動操作）とします。

## Core Requirements (MVP v0.1)

### REQ-30102-001: サーブ権の管理
**WHEN** ポイントが開始される
**THE SYSTEM SHALL** サーブ権を持つプレイヤーを決定する
- ゲーム開始時: Player1
- ゲーム終了時: サーブ権交代
**テスト**: TST-30104-006

### REQ-30102-002: サーブ入力
**WHEN** サーブ権を持つプレイヤーがBボタンを押す
**AND** MatchState == Serve
**THE SYSTEM SHALL** ボールを生成する
- 位置: プレイヤーの足元 + (0, 0.5, 0)
**AND** ShotEvent を発行する
**テスト**: TST-30104-007

### REQ-30102-003: サーブの打球
**WHEN** サーブプレイヤーがショットを実行する
**THE SYSTEM SHALL** ボールを相手コート方向に発射する
- 速度: `config.Ball.NormalShotSpeed`
- 角度: 45度
**AND** MatchState を Rally に遷移する
**テスト**: TST-30104-008

### REQ-30102-004: サーブ権交代
**WHEN** ゲームが終了する
**THE SYSTEM SHALL** サーブ権を相手プレイヤーに交代する
**テスト**: TST-30104-009

---

## Extended Requirements (v0.2)

### REQ-30102-050: ワンバウンドサーブ
**WHEN** サーブを打つ
**THE SYSTEM SHALL** ボールを一度地面にバウンドさせてから打つ
**AND** バウンド前の打球はフォルトとする
**テスト**: TST-30104-050

### REQ-30102-051: サーブフォルト
**WHEN** サーブがネットに当たる、または相手コートに届かない
**THE SYSTEM SHALL** フォルトと判定する
**AND** 2回連続フォルトでポイント喪失（ダブルフォルト）
**テスト**: TST-30104-051

### REQ-30102-052: 自動サーブ
**WHEN** サーブ権を持つプレイヤーがAIである
**THE SYSTEM SHALL** 自動的にサーブを実行する
**テスト**: TST-30104-052

---

## Future Requirements (v0.3+)

### REQ-30102-100: サーブアニメーション
**WHEN** サーブを実行する
**THE SYSTEM SHALL** サーブモーションのアニメーションを再生する
**テスト**: TST-30104-100

---

## データ参照
- [80101_game_constants.md](../../8_data/80101_game_constants.md#ball-config)

## 依存関係
- [30101_flow_spec.md](30101_flow_spec.md)
- [30601_shot_input_spec.md](../306_shot_system/30601_shot_input_spec.md)

## 備考
- MVP v0.1: 簡易実装（手動サーブ）
- v0.2: ワンバウンドサーブ、サーブ失敗（フォルト）
