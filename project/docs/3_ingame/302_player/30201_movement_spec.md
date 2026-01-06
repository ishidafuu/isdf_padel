# Player Movement Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

プレイヤーの3軸移動（X: 左右、Z: 前後、Y: 上下）を制御します。入力に応じて即座に移動し、コート境界を越えないように制限されます。

## Core Requirements (MVP v0.1)

### REQ-30201-001: 左右移動（X軸）
**WHEN** プレイヤーが左右入力（A/D キーまたは左スティック）を行う
**THE SYSTEM SHALL** プレイヤーをX軸方向に移動させる
- 速度 `config.Player.MoveSpeedX` (デフォルト: 5.0 m/s)
- コート境界 `-config.Court.Width/2` ～ `+config.Court.Width/2` を超えない

**テスト**: TST-30204-001
**イベント**: PlayerMoveEvent

---

### REQ-30201-002: 前後移動（Z軸）
**WHEN** プレイヤーが前後入力（W/S キーまたは左スティック）を行う
**THE SYSTEM SHALL** プレイヤーをZ軸方向に移動させる
- 速度 `config.Player.MoveSpeedZ` (デフォルト: 4.0 m/s)
- コート境界（自コート内）を超えない

**テスト**: TST-30204-002
**イベント**: PlayerMoveEvent

---

### REQ-30201-003: 移動速度の統一
**WHEN** プレイヤーが複数軸同時に移動する（例: 斜め移動）
**THE SYSTEM SHALL** 移動ベクトルを正規化する
- 移動速度が `max(config.Player.MoveSpeedX, config.Player.MoveSpeedZ)` を超えないように調整

**テスト**: TST-30204-003

---

### REQ-30201-004: 境界での停止
**WHEN** プレイヤーがコート境界に到達する
**THE SYSTEM SHALL** 境界を超える方向の移動を停止する
**AND** 境界に沿った移動は許可する

**参照**: [30503_boundary_behavior.md](../305_court/30503_boundary_behavior.md)
**テスト**: TST-30204-004

---

### REQ-30201-005: ふっとばし中の移動制限
**WHEN** プレイヤーがふっとばし状態である（Knockback.IsActive == true）
**THE SYSTEM SHALL** プレイヤーの入力を無視する
**AND** ふっとばしベクトルに従った移動のみを許可する

**参照**: [30203_knockback_spec.md](30203_knockback_spec.md)
**テスト**: TST-30204-005

---

### REQ-30201-006: PlayerMoveEvent の発行
**WHEN** プレイヤーの位置が変化する
**THE SYSTEM SHALL** PlayerMoveEvent を発行する
- イベントデータ：
  - `PlayerId: int` - プレイヤーID
  - `Position: Vector3` - 新しい位置
  - `Velocity: Vector3` - 移動速度ベクトル

**参照**: [20005_event_system.md](../../2_architecture/20005_event_system.md)
**テスト**: TST-30204-006

---

## Extended Requirements (v0.2)

### REQ-30201-050: ダッシュ移動
**WHEN** プレイヤーがダッシュボタンを押しながら移動する
**THE SYSTEM SHALL** 移動速度を `config.Player.DashSpeedMultiplier` 倍にする
**AND** スタミナを消費する

**テスト**: TST-30204-050

### REQ-30201-051: 慣性移動
**WHEN** プレイヤーが入力を停止する
**THE SYSTEM SHALL** 徐々に減速する
- 減速係数: `config.Player.Friction`

**テスト**: TST-30204-051

---

## Future Requirements (v0.3+)

### REQ-30201-100: スライディング
**WHEN** プレイヤーが特定の条件で移動する
**THE SYSTEM SHALL** スライディング動作を実行する
**AND** 低姿勢での移動を可能にする

**テスト**: TST-30204-100

---

## 制約（Design by Contract）

### 事前条件
- PlayerComponent が存在する
- TransformComponent が存在する
- InputComponent が存在する
- GameConfig が読み込まれている
- コート境界が定義されている

### 事後条件
- プレイヤー位置がコート境界内に収まる
- 移動速度が設定値以下である
- PlayerMoveEvent が発行される（位置変化時のみ）

### 不変条件
- `-config.Court.Width/2 <= position.X <= config.Court.Width/2`
- `position.Y >= 0`（地面より下に行かない）
- プレイヤーは自コート内に留まる

---

## データ参照

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| 左右移動速度 | config.Player.MoveSpeedX | 5.0 m/s |
| 前後移動速度 | config.Player.MoveSpeedZ | 4.0 m/s |
| コート幅 | config.Court.Width | 10.0 m |
| コート奥行き | config.Court.Depth | 6.0 m |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先
- [30501_court_spec.md](../305_court/30501_court_spec.md) - コート座標系を使用
- [30503_boundary_behavior.md](../305_court/30503_boundary_behavior.md) - 境界判定に従う
- [30203_knockback_spec.md](30203_knockback_spec.md) - ふっとばし中は移動制限
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - PlayerMoveEvent を発行
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - Player パラメータを参照

---

## 備考

- 移動は即座（加速・減速なし）
- 慣性は将来の拡張として検討
- ジャンプ中も移動可能（空中制御あり）
