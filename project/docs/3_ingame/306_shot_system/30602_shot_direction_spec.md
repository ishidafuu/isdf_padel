# Shot Direction Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

ショット入力から打球方向を計算し、ボールの初速度を設定します。入力方向、ジャンプ状態に応じて速度・角度を調整します。

## Core Requirements (MVP v0.1)

### REQ-30602-001: 水平方向の計算
**WHEN** ShotEvent を受信した
**THE SYSTEM SHALL** 打球の水平方向を計算する

**Z軸方向（前後）: 常に相手コート方向に固定**
- Player 1: Z正方向（+Z）
- Player 2: Z負方向（-Z）
- 上下入力は無視する（移動方向に関係なく相手コートに飛ぶ）

**X軸方向（左右）: 入力で調整可能**
- 左右入力がある場合: 入力方向に応じて左右に打ち分け
- 左右入力がない場合: ストレート（X = 0）

**水平方向ベクトル計算式**:
- X成分 = `Direction.x`（左右入力、-1.0〜1.0）
- Z成分 = プレイヤーに応じた固定値（Player1: +1.0, Player2: -1.0）
- 正規化して使用

**テスト**: TST-30604-007

---

### REQ-30602-002: 通常ショットの速度
**WHEN** プレイヤーが地上にいる（IsGrounded == true）
**THE SYSTEM SHALL** 通常ショット速度を設定する
- 速度: `config.Ball.NormalShotSpeed` (デフォルト: 10.0 m/s)
- 打球角度: 45度（水平面から）

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#ball-config)
**テスト**: TST-30604-008

---

### REQ-30602-003: ジャンプショットの速度
**WHEN** プレイヤーがジャンプ中である（Position.Y > 0.5m）
**THE SYSTEM SHALL** ジャンプショット速度を設定する
- 速度: `config.Ball.PowerShotSpeed` (デフォルト: 15.0 m/s)
- 打球角度: 30度（急角度）

**参照**: [30603_jump_shot_spec.md](30603_jump_shot_spec.md)
**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#ball-config)
**テスト**: TST-30604-009

---

### REQ-30602-004: 打球ベクトルの計算
**WHEN** 打球速度と角度が決定された
**THE SYSTEM SHALL** 3D速度ベクトルを計算する
- Velocity.X = `horizontalDir.x * speed * cos(angle)`
- Velocity.Y = `speed * sin(angle)`
- Velocity.Z = `horizontalDir.z * speed * cos(angle)`

**テスト**: TST-30604-010

---

### REQ-30602-005: ボール速度の設定
**WHEN** 打球ベクトルが計算された
**THE SYSTEM SHALL** ボールの速度を設定する
- Ball.Velocity = 計算された速度ベクトル
- Ball.State = Flying（飛行中状態）

**テスト**: TST-30604-011

---

### REQ-30602-006: クールダウンの開始
**WHEN** ショットが実行された
**THE SYSTEM SHALL** プレイヤーのクールダウンを開始する
- ShotState.CooldownTimer = `config.Shot.Cooldown`

**参照**: [30601_shot_input_spec.md](30601_shot_input_spec.md#req-30601-004)
**テスト**: TST-30604-012

---

### REQ-30602-007: ShotExecutedEvent の発行
**WHEN** ショットが実行された
**THE SYSTEM SHALL** ShotExecutedEvent を発行する
- イベントデータ：
  - PlayerId: プレイヤーID
  - ShotVelocity: 打球ベクトル
  - IsJumpShot: ジャンプショットか

**参照**: [20005_event_system.md](../../2_architecture/20005_event_system.md)
**テスト**: TST-30604-013

---

## Extended Requirements (v0.2)

### REQ-30602-050: スピンショット
**WHEN** プレイヤーが特定の方向入力でショットする
**THE SYSTEM SHALL** ボールにスピンを設定する
- スピン強度: `config.Shot.SpinStrength`

**テスト**: TST-30604-050

### REQ-30602-051: カーブショット
**WHEN** プレイヤーが斜め入力でショットする
**THE SYSTEM SHALL** ボールにカーブ軌道を設定する

**テスト**: TST-30604-051

---

## Future Requirements (v0.3+)

### REQ-30602-100: ショット種類選択
**WHEN** プレイヤーが異なるボタン組み合わせを入力する
**THE SYSTEM SHALL** ショット種類（ロブ、スマッシュ等）を選択する

**テスト**: TST-30604-100

---

## 制約（Design by Contract）

### 事前条件
- ShotEvent が発行されている
- ボールが存在する
- GameConfig が読み込まれている

### 事後条件
- ボールの速度が設定される
- ボールの状態が Flying になる
- ShotExecutedEvent が発行される
- プレイヤーのクールダウンが開始される

### 不変条件
- 打球速度は常に正の値
- 打球角度は 0～90度の範囲内

---

## データ参照

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| 通常ショット速度 | config.Ball.NormalShotSpeed | 10.0 m/s |
| ジャンプショット速度 | config.Ball.PowerShotSpeed | 15.0 m/s |
| クールダウン時間 | config.Shot.Cooldown | 0.5秒 |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - Ball, Shot パラメータ
- [30601_shot_input_spec.md](30601_shot_input_spec.md) - ShotEvent を受信
- [30603_jump_shot_spec.md](30603_jump_shot_spec.md) - ジャンプショット判定
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - ShotExecutedEvent を発行

### 依存元
- ボール物理システム（304_ball）- ボールの速度を受け取って軌道計算

---

## 備考

- 打球角度（45度、30度）は固定値（将来的に調整可能にする可能性）
- スピン、カーブなどの軌道変化は v0.2以降の実装
- **Z軸方向（前後）は常に相手コート方向に固定** - プレイヤーの移動方向に関係なく、ボールは必ず相手コートに飛ぶ
- X軸方向（左右）のみ入力で調整可能
