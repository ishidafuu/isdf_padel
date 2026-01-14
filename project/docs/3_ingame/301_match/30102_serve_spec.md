# Serve Specification

**Version**: 3.0.0
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
**THE SYSTEM SHALL** 高い打点からボールを下向きに発射する
- 打点高さ: `config.serve.ball_spawn_offset_y`（デフォルト: 2.0）
- 速度: `config.serve.serve_speed`（デフォルト: 10.0 m/s）
- 角度: `config.serve.serve_angle`（デフォルト: -15度、負の値=下向き）
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

### REQ-30102-080: トス開始
**WHEN** サーブ権を持つ人間プレイヤーがショットボタンを押す
**AND** MatchFlowState == Serve
**AND** ServeSubPhase == Waiting
**THE SYSTEM SHALL** トスを開始する
- トスボール生成位置: プレイヤー位置 + (0, `config.serve.toss_start_offset_y`, 0)
- 初速度: (0, `config.serve.toss_velocity_y`, 0)
- ServeSubPhase を Tossing に遷移する
- TossBall マーカーを付与する
**テスト**: TST-30104-080

### REQ-30102-081: トス物理
**WHILE** ServeSubPhase == Tossing
**THE SYSTEM SHALL** トスボールに重力を適用する
- 重力: `config.physics.gravity`
- 上昇後、落下する放物線軌道
**テスト**: TST-30104-081

### REQ-30102-082: ヒット入力
**WHEN** サーブ権を持つ人間プレイヤーがショットボタンを押す
**AND** ServeSubPhase == Tossing
**AND** ボール高さが `config.serve.hit_height_min` 以上 `config.serve.hit_height_max` 以下
**THE SYSTEM SHALL** サーブヒットを実行する
- TossBall エンティティを削除
- ShotEvent を発行（is_serve=true, hit_position=トスボール位置）
- MatchFlowState を Rally に遷移する
- ボール生成と弾道計算は shot_direction_system で実行

**参照**: [30602_shot_direction_spec.md](../306_shot_system/30602_shot_direction_spec.md#req-30602-031)
**参照**: [30605_trajectory_calculation_spec.md](../306_shot_system/30605_trajectory_calculation_spec.md#req-30605-050)
**テスト**: TST-30104-082

### REQ-30102-083: ヒット可能範囲
**IF** ボール高さが `config.serve.hit_height_min` 未満
**OR** ボール高さが `config.serve.hit_height_max` 超過
**WHEN** プレイヤーがショットボタンを押す
**THE SYSTEM SHALL** ヒットを無視する（何も起こらない）
**テスト**: TST-30104-083

### REQ-30102-084: トス打ち直し（let）
**WHEN** ServeSubPhase == Tossing
**AND** トス開始から `config.serve.toss_timeout` 秒経過
**OR** ボールが `config.serve.hit_height_min` 未満に落下
**THE SYSTEM SHALL** 打ち直し（let）にする
- トスボールを削除
- fault_count は変更しない
- ServeSubPhase を Waiting に戻す
**テスト**: TST-30104-084

### REQ-30102-085: トス中移動禁止
**WHILE** ServeSubPhase == Tossing
**THE SYSTEM SHALL** サーバーの移動入力を無視する
- 完全静止（X, Z 両方向）
**テスト**: TST-30104-085

### REQ-30102-086: ベースライン制限
**WHILE** MatchFlowState == Serve
**AND** ServeSubPhase == Waiting
**THE SYSTEM SHALL** サーバーの移動を制限する
- X座標をベースラインに固定
  - Player1: `config.serve.serve_baseline_x_p1`
  - Player2: `config.serve.serve_baseline_x_p2`
- Z座標をセンターライン（Z=0）を越えないように制限（サイド方向はサイドウォールまで）
  - Left側 + デュースサイド: 0 ≤ Z ≤ `config.court.width / 2.0`
  - Left側 + アドサイド: `-config.court.width / 2.0` ≤ Z ≤ 0
  - Right側 + デュースサイド: `-config.court.width / 2.0` ≤ Z ≤ 0
  - Right側 + アドサイド: 0 ≤ Z ≤ `config.court.width / 2.0`
**テスト**: TST-30104-086

### REQ-30102-087: AIトス実行
**WHEN** サーブ権を持つ AI プレイヤーがサーブを開始する
**AND** ServeSubPhase == Waiting
**THE SYSTEM SHALL** 待機時間後にトスを自動実行する
- 待機時間: `config.ai.serve_delay_min` ～ `config.ai.serve_delay_max`
**テスト**: TST-30104-087

### REQ-30102-088: AIヒット実行
**WHEN** ServeSubPhase == Tossing
**AND** AI がサーバー
**AND** ボール高さが `config.serve.hit_height_optimal` ± `config.serve.ai_hit_tolerance`
**THE SYSTEM SHALL** サーブヒットを自動実行する
- TossBall エンティティを削除
- ShotEvent を発行（is_serve=true, hit_position=トスボール位置）
- MatchFlowState を Rally に遷移する
- ボール生成と弾道計算は shot_direction_system で実行

**参照**: [30602_shot_direction_spec.md](../306_shot_system/30602_shot_direction_spec.md#req-30602-031)
**テスト**: TST-30104-088

### REQ-30102-089: ダブルフォルト
**WHEN** fault_count が 2 に達する
**THE SYSTEM SHALL** 相手プレイヤーに 1 ポイントを加算する
- fault_count を 0 にリセット
- 次のポイントを開始
**テスト**: TST-30104-089

### REQ-30102-090: サーブ開始時のプレイヤー位置配置
**WHEN** MatchFlowState が Serve に遷移する
**THE SYSTEM SHALL** サーバーとレシーバーを対角線上（クロスポジション）に配置する
- サーブサイドZ座標: `config.court.width / 4.0`（コート幅の1/4、デフォルト3.0m）
- サーバー: サーブサイドに配置
  - デュースサイド: Z = +serve_z
  - アドサイド: Z = -serve_z
- レシーバー: サーバーと対角線上（クロス）に配置
  - サーバーがデュースサイドの場合: Z = -serve_z
  - サーバーがアドサイドの場合: Z = +serve_z
- X位置は各プレイヤーのベースライン位置を維持
**テスト**: TST-30104-090
**データ**: `80101_game_constants.md#court_config`

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
| サーブ速度 | `config.serve.serve_speed` | 10.0 m/s | ボール初速度 |
| サーブ角度 | `config.serve.serve_angle` | -15.0 | 発射角度（度、負の値=下向き） |
| P1デフォルト方向X | `config.serve.p1_default_direction_x` | 1.0 | Player1のサーブ方向（X成分） |
| P2デフォルト方向X | `config.serve.p2_default_direction_x` | -1.0 | Player2のサーブ方向（X成分） |
| トス開始高さ | `config.serve.toss_start_offset_y` | 1.0 | トスボール生成高さ（手元） |
| トス初速度 | `config.serve.toss_velocity_y` | 3.5 m/s | トス上向き初速度 |
| トスタイムアウト | `config.serve.toss_timeout` | 3.0 | トス失敗までの時間（秒） |
| ヒット可能最低高さ | `config.serve.hit_height_min` | 1.8 | ヒット可能な最低高さ（m） |
| ヒット可能最高高さ | `config.serve.hit_height_max` | 2.7 | ヒット可能な最高高さ（m） |
| ヒット最適高さ | `config.serve.hit_height_optimal` | 2.2 | AI用ヒット最適高さ（m） |
| AIヒット許容範囲 | `config.serve.ai_hit_tolerance` | 0.1 | AI用ヒット許容範囲（m） |
| P1ベースラインX | `config.serve.serve_baseline_x_p1` | -8.5 | Player1のベースライン位置（ベースライン外側） |
| P2ベースラインX | `config.serve.serve_baseline_x_p2` | 8.5 | Player2のベースライン位置（ベースライン外側） |

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

### 2026-01-09 - v3.0.0
- トス→ヒット方式追加（REQ-30102-080〜089）
- トス物理、ヒット可能範囲、タイムアウト、ダブルフォルト仕様
- ベースライン制限、トス中移動禁止仕様
- AIトス・ヒット自動実行仕様
- ServeConfigに新規パラメータ追加

### 2026-01-09 - v2.0.0
- オーバーハンドサーブ仕様追加（REQ-30102-060）
- AI自動サーブ仕様追加（REQ-30102-070, 071）
- ワンバウンドサーブ（REQ-30102-050/051）を削除、v0.5+に移動
- 打点高さを 0.5 → 2.0 に変更
- サーブ専用速度・角度パラメータ追加

### 2025-12-23 - v1.0.0
- 初版作成（MVP v0.1）
