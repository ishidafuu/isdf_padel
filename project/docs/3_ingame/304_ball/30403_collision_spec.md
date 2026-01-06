# Ball Collision Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

ボールとプレイヤーの当たり判定を定義します。衝突時にプレイヤーをふっとばし、無敵時間を適用します。

## Core Requirements (MVP v0.1)

### REQ-30403-001: プレイヤーとの当たり判定
**WHEN** ボールとプレイヤーの距離が判定範囲以下になった
**AND** プレイヤーが無敵状態でない
**THE SYSTEM SHALL** 衝突を検出する
- 判定範囲: `config.Ball.Radius + config.Collision.CharacterRadius`
- Z軸許容範囲: `config.Collision.ZTolerance`

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md)
**テスト**: TST-30404-014

---

### REQ-30403-002: ふっとばしトリガー
**WHEN** ボールとプレイヤーの衝突を検出した
**THE SYSTEM SHALL** `BallHitEvent` を発行する
- イベントデータ：
  - PlayerId（被弾プレイヤー）
  - HitPosition（衝突位置）
  - BallVelocity（ボール速度）

**参照**: [30203_knockback_spec.md](../302_player/30203_knockback_spec.md)
**テスト**: TST-30404-015

---

### REQ-30403-003: 無敵時間中の衝突無視
**WHEN** プレイヤーが無敵状態である
**AND** ボールがプレイヤーに接触した
**THE SYSTEM SHALL** 衝突を無視する（ふっとばし発生せず）

**参照**: [30203_knockback_spec.md](../302_player/30203_knockback_spec.md#req-30203-005)
**テスト**: TST-30404-016

---

### REQ-30403-004: ボール反射（プレイヤー衝突時）
**WHEN** ボールがプレイヤーに衝突した
**THE SYSTEM SHALL** ボールの速度を反転させる
- 反射方向: プレイヤー→ボールの方向

**テスト**: TST-30404-017

---

### REQ-30403-005: 衝突判定頻度
**WHEN** 物理演算フレーム（`_PhysicsProcess`）が実行される
**THE SYSTEM SHALL** 全プレイヤーとの距離を計算し、衝突判定を行う

**テスト**: TST-30404-018

---

### REQ-30403-006: 複数プレイヤー衝突時の優先順位
**WHEN** ボールが複数プレイヤーと同時に衝突した
**THE SYSTEM SHALL** 最も近いプレイヤーとの衝突を優先する

**テスト**: TST-30404-019

---

## Extended Requirements (v0.2)

### REQ-30403-050: ボール間衝突
**WHEN** 複数のボールが同時に存在する
**AND** ボール同士が接触する
**THE SYSTEM SHALL** ボール間の衝突を処理する

**テスト**: TST-30404-050

### REQ-30403-051: スローモーション判定
**WHEN** 重要な衝突が発生する
**THE SYSTEM SHALL** スローモーション演出を開始する

**テスト**: TST-30404-051

---

## Future Requirements (v0.3+)

### REQ-30403-100: 衝突エフェクト
**WHEN** ボールがプレイヤーに衝突する
**THE SYSTEM SHALL** 衝突エフェクトを表示する

**テスト**: TST-30404-100

---

## 制約（Design by Contract）

### 事前条件
- 衝突判定前、ボールとプレイヤーは有効な位置にある
- 判定半径 > 0
- プレイヤーの無敵状態フラグが正しく設定されている

### 事後条件
- 衝突後、BallHitEvent が発行される（無敵時を除く）
- 衝突後、ボールの速度が更新される

### 不変条件
- 衝突判定は毎フレーム1回のみ実行
- 無敵時間中は衝突イベント発行禁止

---

## データ参照

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| ボール半径 | config.Ball.Radius | 0.2 m |
| キャラクター半径 | config.Collision.CharacterRadius | 0.5 m |
| Z軸許容範囲 | config.Collision.ZTolerance | 0.3 m |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - Ball, Collision パラメータ
- [30203_knockback_spec.md](../302_player/30203_knockback_spec.md) - プレイヤーふっとばし
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - イベント定義

---

## 備考

- 衝突判定は独自定義
- ふっとばし処理は 30203_knockback_spec.md が責務を持つ
- このファイルは衝突検出とイベント発行のみを定義
