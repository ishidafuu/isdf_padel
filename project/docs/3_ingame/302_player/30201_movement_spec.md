# Player Movement Specification

**Version**: 2.0.0
**Status**: Draft
**Last Updated**: 2026-01-09

## 概要

プレイヤーの3軸移動（X: 打ち合い方向、Z: コート幅方向、Y: 上下）を制御します。入力に応じて即座に移動し、外壁までの範囲で自由に移動できます。コート外〜外壁までの移動が可能です（テニス：ボール追い）。

## Core Requirements (MVP v0.1)

### REQ-30201-001: 打ち合い方向移動（X軸）
**WHEN** プレイヤーが左右入力（A/D キーまたは左スティック）を行う
**THE SYSTEM SHALL** プレイヤーをX軸方向（打ち合い方向）に移動させる
- 速度 `config.Player.MoveSpeedX` (デフォルト: 5.0 m/s)
- 外壁 `-config.Court.OuterWallX` ～ 自コート側外壁まで移動可能
- コートライン外への移動を許可（ボール追い）

**テスト**: TST-30204-001
**イベント**: PlayerMoveEvent

---

### REQ-30201-002: コート幅方向移動（Z軸）
**WHEN** プレイヤーが前後入力（W/S キーまたは左スティック）を行う
**THE SYSTEM SHALL** プレイヤーをZ軸方向（コート幅方向）に移動させる
- 速度 `config.Player.MoveSpeedZ` (デフォルト: 4.0 m/s)
- 外壁 `-config.Court.OuterWallZ` ～ `+config.Court.OuterWallZ` まで移動可能
- サイドライン外への移動を許可（ボール追い）

**テスト**: TST-30204-002
**イベント**: PlayerMoveEvent

---

### REQ-30201-003: 移動速度の統一
**WHEN** プレイヤーが複数軸同時に移動する（例: 斜め移動）
**THE SYSTEM SHALL** 移動ベクトルを正規化する
- 移動速度が `max(config.Player.MoveSpeedX, config.Player.MoveSpeedZ)` を超えないように調整

**テスト**: TST-30204-003

---

### REQ-30201-004: 外壁での停止
**WHEN** プレイヤーが外壁に到達する
**THE SYSTEM SHALL** 外壁を超える方向の移動を停止する
**AND** 外壁に沿った移動は許可する

**備考**: コートライン外（アウトコート）への移動は自由に許可される
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
- プレイヤー位置が外壁内に収まる
- 移動速度が設定値以下である
- PlayerMoveEvent が発行される（位置変化時のみ）

### 不変条件
- `-config.Court.OuterWallX <= position.X <= 自コート側外壁`
- `-config.Court.OuterWallZ <= position.Z <= config.Court.OuterWallZ`
- `position.Y >= 0`（地面より下に行かない）
- プレイヤーはネットを越えて相手コートに入れない

---

## データ参照

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| 打ち合い方向移動速度 | config.Player.MoveSpeedX | 5.0 m/s |
| コート幅方向移動速度 | config.Player.MoveSpeedZ | 4.0 m/s |
| コート幅（Z方向） | config.Court.Width | 10.0 m |
| コート奥行き（X方向） | config.Court.Depth | 6.0 m |
| 外壁（Z方向） | config.Court.OuterWallZ | 8.0 m |
| 外壁（X方向） | config.Court.OuterWallX | 5.0 m |

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

### テニスの特徴
- プレイヤーはコートライン外〜外壁まで自由に移動可能
- ボールを追いかけるためにアウトコートへ出られる
- ネットを越えての相手コート侵入は禁止

### 座標系
- X軸: 打ち合い方向（-X: 1Pコート側、+X: 2Pコート側）
- Z軸: コート幅方向（-Z: 左、+Z: 右）
- Y軸: 高さ方向（0: 地面、+Y: 上）

---

## Change Log

### 2026-01-09 - v2.0.0（テニス仕様・座標系統一）

- **概要**: 座標系を実装に統一、コート外移動を許可
- **REQ-30201-001**: 「左右移動」→「打ち合い方向移動」、外壁まで移動可能
- **REQ-30201-002**: 「前後移動」→「コート幅方向移動」、外壁まで移動可能
- **REQ-30201-004**: 「境界での停止」→「外壁での停止」に変更
- **事後条件・不変条件**: コート境界内 → 外壁内に変更

### 2025-12-23 - v1.0.0（初版）

- 初版作成（パデルベース）
