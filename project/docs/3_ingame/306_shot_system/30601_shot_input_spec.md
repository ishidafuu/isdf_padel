# Shot Input Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

プレイヤーのショット入力（Bボタン）と、ショット可能条件（タイミング判定、クールダウン）を定義します。

## Core Requirements (MVP v0.1)

### REQ-30601-001: ショット入力
**WHEN** プレイヤーがBボタンを押す
**AND** プレイヤーがボールの近くにいる
**AND** クールダウンが完了している
**THE SYSTEM SHALL** ShotEvent を発行する
- イベントデータ：
  - PlayerId: プレイヤーID
  - Direction: 入力方向（十字キー）
  - JumpHeight: ジャンプ中の高さ（Position.Y）

**テスト**: TST-30604-001

---

### REQ-30601-002: タイミング判定（距離）
**WHEN** ショット入力を受け付ける
**THE SYSTEM SHALL** プレイヤーとボールの距離を判定する
- 水平距離: `Distance2D(PlayerPos, BallPos) < config.Shot.MaxDistance`
- MaxDistance: デフォルト 1.5 m

**AND** 距離が範囲外の場合、ショット入力を無視する

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md)
**テスト**: TST-30604-002

---

### REQ-30601-003: タイミング判定（高さ差）
**WHEN** ショット入力を受け付ける
**THE SYSTEM SHALL** プレイヤーとボールの高さ差を判定する
- 高さ差: `|PlayerPos.Y - BallPos.Y| < config.Shot.MaxHeightDiff`
- MaxHeightDiff: デフォルト 2.0 m

**AND** 高さ差が範囲外の場合、ショット入力を無視する

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md)
**テスト**: TST-30604-003

---

### REQ-30601-004: クールダウン管理
**WHEN** ショットを実行した
**THE SYSTEM SHALL** クールダウンタイマーを開始する
- CooldownTimer = `config.Shot.Cooldown` (デフォルト: 0.5秒)

**WHILE** CooldownTimer > 0
**THE SYSTEM SHALL** クールダウンタイマーを毎フレーム減算する
- `CooldownTimer -= deltaTime`
- クールダウン中はショット入力を無視する

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md)
**テスト**: TST-30604-004

---

### REQ-30601-005: ふっとばし中のショット禁止
**WHEN** プレイヤーがふっとばし状態である（Knockback.IsActive == true）
**THE SYSTEM SHALL** ショット入力を無視する

**参照**: [30203_knockback_spec.md](../302_player/30203_knockback_spec.md)
**テスト**: TST-30604-005

---

### REQ-30601-006: ShotEvent の発行
**WHEN** ショット条件を満たす
**THE SYSTEM SHALL** ShotEvent をイベントバスに発行する
- PlayerId: プレイヤーID
- Direction: 入力方向（Vector2、正規化済み）
- JumpHeight: プレイヤーのY座標

**参照**: [20005_event_system.md](../../2_architecture/20005_event_system.md)
**テスト**: TST-30604-006

---

## Extended Requirements (v0.2)

### REQ-30601-050: ボタン長押し（強さ調整）
**WHEN** プレイヤーがBボタンを長押しする
**THE SYSTEM SHALL** 長押し時間に応じてショット強度を調整する
- 最大長押し時間: `config.Shot.MaxChargeTime`

**テスト**: TST-30604-050

### REQ-30601-051: 入力バッファリング
**WHEN** ショット入力がタイミング外で行われる
**THE SYSTEM SHALL** 入力を一定時間バッファする
- バッファ時間: `config.Shot.InputBufferTime`

**テスト**: TST-30604-051

---

## Future Requirements (v0.3+)

### REQ-30601-100: 必殺ショット判定
**WHEN** 特定の条件でショットを実行する
**THE SYSTEM SHALL** 必殺ショットを発動する

**テスト**: TST-30604-100

---

## 制約（Design by Contract）

### 事前条件
- PlayerComponent が存在する
- InputComponent が存在する
- ShotStateComponent が存在する
- ボールが存在する
- GameConfig が読み込まれている

### 事後条件
- ショット成功時、ShotEvent が発行される
- ショット成功時、CooldownTimer が開始される
- ショット失敗時、何も発生しない（入力を無視）

### 不変条件
- `CooldownTimer >= 0`
- クールダウン中はショット不可

---

## データ参照

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| 打球可能距離 | config.Shot.MaxDistance | 1.5 m |
| 打球可能高さ差 | config.Shot.MaxHeightDiff | 2.0 m |
| クールダウン時間 | config.Shot.Cooldown | 0.5秒 |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - Shot パラメータ
- [30203_knockback_spec.md](../302_player/30203_knockback_spec.md) - ふっとばし中はショット禁止
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - ShotEvent を発行

### 依存元
- [30602_shot_direction_spec.md](30602_shot_direction_spec.md) - ShotEvent を受信して打球方向を計算

---

## 備考

- ショット入力は即座に処理される（入力バッファなし）
- 将来の拡張として、入力バッファリングを検討（v0.2以降）
- Bボタン長押しによる強打は v0.2以降の実装
