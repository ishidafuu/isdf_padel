# Serve Specification

**Version**: 2.0.0
**Status**: Draft
**Last Updated**: 2026-01-09

## 概要

サーブの処理を定義します。v0.4でテニス式オーバーハンドサーブを導入します。

## Core Requirements (MVP v0.1)

### REQ-30102-001: サーブ権の管理
**WHEN** ポイントが開始される
**THE SYSTEM SHALL** サーブ権を持つプレイヤーを決定する
- ゲーム開始時: Player1
- ゲーム終了時: サーブ権交代
**テスト**: TST-30104-006

### REQ-30102-002: サーブ入力（人間プレイヤー）
**WHEN** サーブ権を持つ人間プレイヤーがショットボタンを押す
**AND** MatchFlowState == Serve
**THE SYSTEM SHALL** オーバーハンドサーブを実行する
- ボール生成位置: プレイヤー位置 + (0, `config.serve.ball_spawn_offset_y`, 0)
- 速度: `config.serve.serve_speed`
- 角度: `config.serve.serve_angle`
**AND** ShotEvent を発行する
**テスト**: TST-30104-007

### REQ-30102-003: サーブの打球
**WHEN** サーブプレイヤーがショットを実行する
**THE SYSTEM SHALL** ボールを相手コート方向に発射する
- 速度: `config.serve.serve_speed`
- 角度: `config.serve.serve_angle`（度）
- 方向: `config.serve.p1_default_direction_x` / `p2_default_direction_x`
**AND** MatchFlowState を Rally に遷移する
**テスト**: TST-30104-008

### REQ-30102-004: サーブ権交代
**WHEN** ゲームが終了する
**THE SYSTEM SHALL** サーブ権を相手プレイヤーに交代する
**テスト**: TST-30104-009

---

## Extended Requirements (v0.4)

### REQ-30102-060: オーバーハンドサーブ
**WHEN** サーブを実行する
**THE SYSTEM SHALL** 高い打点からボールを発射する
- 打点高さ: `config.serve.ball_spawn_offset_y`（デフォルト: 2.0）
- 速度: `config.serve.serve_speed`（デフォルト: 4.0 m/s）
- 角度: `config.serve.serve_angle`（デフォルト: 20度）
**テスト**: TST-30104-060

### REQ-30102-070: AI自動サーブ
**WHEN** サーブ権を持つプレイヤーが AiController を持つ
**AND** MatchFlowState == Serve
**THE SYSTEM SHALL** 一定時間後に自動的にサーブを実行する
- 待機時間: `config.ai.serve_delay_min` ～ `config.ai.serve_delay_max` の範囲でランダム
- 待機時間デフォルト: 0.5秒 ～ 1.5秒
**テスト**: TST-30104-070

### REQ-30102-071: AIサーブ方向ランダム化
**WHEN** AIがサーブを実行する
**THE SYSTEM SHALL** サーブ方向にランダムなバリエーションを追加する
- Z方向: 中央 ± `config.ai.serve_direction_variance` の範囲でランダム
- バリエーションデフォルト: ±0.5
**テスト**: TST-30104-071

---

## Future Requirements (v0.5+)

### REQ-30102-100: サーブアニメーション
**WHEN** サーブを実行する
**THE SYSTEM SHALL** サーブモーションのアニメーションを再生する
**テスト**: TST-30104-100

### REQ-30102-110: サーブフォルト
**WHEN** サーブがネットに当たる、または相手コートに届かない
**THE SYSTEM SHALL** フォルトと判定する
**AND** 2回連続フォルトでポイント喪失（ダブルフォルト）
**テスト**: TST-30104-110

---

## データ参照

### サーブパラメータ

| パラメータ | データパス | デフォルト値 | 説明 |
|-----------|-----------|-------------|------|
| 打点高さ | `config.serve.ball_spawn_offset_y` | 2.0 | オーバーハンドサーブの打点Y座標オフセット |
| サーブ速度 | `config.serve.serve_speed` | 4.0 m/s | ボール初速度 |
| サーブ角度 | `config.serve.serve_angle` | 20.0 | 発射角度（度） |
| P1デフォルト方向X | `config.serve.p1_default_direction_x` | 1.0 | Player1のサーブ方向（X成分） |
| P2デフォルト方向X | `config.serve.p2_default_direction_x` | -1.0 | Player2のサーブ方向（X成分） |

### AIサーブパラメータ

| パラメータ | データパス | デフォルト値 | 説明 |
|-----------|-----------|-------------|------|
| 待機時間下限 | `config.ai.serve_delay_min` | 0.5 | サーブまでの最小待機秒数 |
| 待機時間上限 | `config.ai.serve_delay_max` | 1.5 | サーブまでの最大待機秒数 |
| 方向バリエーション | `config.ai.serve_direction_variance` | 0.5 | Z方向のランダム幅 |

- [80101_game_constants.md](../../8_data/80101_game_constants.md#ball-config)

## 依存関係

### 依存先
- [30101_flow_spec.md](30101_flow_spec.md) - 試合フロー管理
- [30601_shot_input_spec.md](../306_shot_system/30601_shot_input_spec.md) - ショット入力

### 依存元
- [30302_ai_shot_spec.md](../303_ai/30302_ai_shot_spec.md) - AIショット（Rally時）

---

## 備考

- MVP v0.1: 簡易実装（手動サーブ）
- v0.4: オーバーハンドサーブ、AI自動サーブ
- v0.5+: サーブアニメーション、サーブフォルト

---

## Change Log

### 2026-01-09 - v2.0.0
- オーバーハンドサーブ仕様追加（REQ-30102-060）
- AI自動サーブ仕様追加（REQ-30102-070, 071）
- ワンバウンドサーブ（REQ-30102-050/051）を削除、v0.5+に移動
- 打点高さを 0.5 → 2.0 に変更
- サーブ専用速度・角度パラメータ追加

### 2025-12-23 - v1.0.0
- 初版作成（MVP v0.1）
