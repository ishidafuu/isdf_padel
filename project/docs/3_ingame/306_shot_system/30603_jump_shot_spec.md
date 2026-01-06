# Jump Shot Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

ジャンプ中のショット（ジャンプショット）の特別な処理を定義します。ジャンプショットは通常ショットより速く、急角度で打ち下ろします。

## Core Requirements (MVP v0.1)

### REQ-30603-001: ジャンプ判定
**WHEN** ショットを実行する
**THE SYSTEM SHALL** プレイヤーがジャンプ中かを判定する
- ジャンプ中: `Position.Y > config.Shot.JumpThreshold`
- JumpThreshold: デフォルト 0.5 m

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md)
**テスト**: TST-30604-014

---

### REQ-30603-002: ジャンプショットの速度増加
**WHEN** プレイヤーがジャンプ中である
**THE SYSTEM SHALL** ジャンプショット速度を適用する
- 速度: `config.Ball.PowerShotSpeed` (デフォルト: 15.0 m/s)
- 通常ショットより高速（10.0 m/s → 15.0 m/s）

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#ball-config)
**テスト**: TST-30604-015

---

### REQ-30603-003: ジャンプショットの角度変化
**WHEN** プレイヤーがジャンプ中である
**THE SYSTEM SHALL** 打球角度を急角度に変更する
- 角度: 30度（通常45度より急）
- 打ち下ろすイメージ

**テスト**: TST-30604-016

---

### REQ-30603-004: ジャンプショット中の空中制御
**WHEN** ジャンプショットを実行する
**THE SYSTEM SHALL** ジャンプ中の移動を許可する
- プレイヤーは空中でも左右移動可能

**参照**: [30201_movement_spec.md](../302_player/30201_movement_spec.md)
**テスト**: TST-30604-017

---

### REQ-30603-005: ジャンプショットの視覚フィードバック
**WHEN** ジャンプショットを実行する
**THE SYSTEM SHALL** 特別なエフェクトを表示する（UI担当）
- ジャンプショットを示すエフェクト
- 通常ショットとの区別

**テスト**: TST-30604-018

---

### REQ-30603-006: JumpShotEvent の発行
**WHEN** ジャンプショットが実行された
**THE SYSTEM SHALL** JumpShotEvent を発行する
- イベントデータ：
  - PlayerId: プレイヤーID
  - JumpHeight: ジャンプ高さ
  - ShotVelocity: 打球ベクトル

**参照**: [20005_event_system.md](../../2_architecture/20005_event_system.md)
**テスト**: TST-30604-019

---

## Extended Requirements (v0.2)

### REQ-30603-050: 高さに応じた威力変化
**WHEN** ジャンプショットを実行する
**THE SYSTEM SHALL** ジャンプ高さに応じて威力を増加させる
- 威力倍率 = `1.0 + (jumpHeight / config.Court.CeilingHeight) * config.Shot.HeightBonus`

**テスト**: TST-30604-050

### REQ-30603-051: 必殺ショット判定
**WHEN** 特定の高さ範囲でジャンプショットを実行する
**THE SYSTEM SHALL** 必殺ショットを発動する
- 発動高さ範囲: `config.Shot.SpecialHeightMin` ~ `config.Shot.SpecialHeightMax`

**テスト**: TST-30604-051

---

## Future Requirements (v0.3+)

### REQ-30603-100: ジャンプショットアニメーション
**WHEN** ジャンプショットを実行する
**THE SYSTEM SHALL** 専用のアニメーションを再生する

**テスト**: TST-30604-100

---

## 制約（Design by Contract）

### 事前条件
- プレイヤーがジャンプ中である（Position.Y > JumpThreshold）
- ShotEvent が発行されている
- GameConfig が読み込まれている

### 事後条件
- ジャンプショット速度が適用される
- 打球角度が急角度になる
- JumpShotEvent が発行される

### 不変条件
- ジャンプショット速度 > 通常ショット速度
- ジャンプショット角度 < 通常ショット角度（急角度）

---

## データ参照

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| ジャンプ判定閾値 | config.Shot.JumpThreshold | 0.5 m |
| ジャンプショット速度 | config.Ball.PowerShotSpeed | 15.0 m/s |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - Ball, Shot パラメータ
- [30201_movement_spec.md](../302_player/30201_movement_spec.md) - 空中移動
- [30202_jump_spec.md](../302_player/30202_jump_spec.md) - ジャンプ判定
- [30602_shot_direction_spec.md](30602_shot_direction_spec.md) - 打球方向計算
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - JumpShotEvent を発行

---

## 備考

- ジャンプショットは「くにおくんドッジボール部」のジャンプシュートを参考
- 高さに応じた威力変化は v0.2以降の実装
- 必殺ショット（特定高さで発動）は v0.2以降の実装
