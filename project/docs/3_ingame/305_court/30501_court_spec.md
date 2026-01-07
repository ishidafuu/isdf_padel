# Court Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

パデルテニスのコート構造を定義します。コートは壁とネットで区切られた閉鎖空間であり、アウトは存在しません。

## Core Requirements (MVP v0.1)

### REQ-30501-001: コート座標系
**WHEN** システムが初期化される
**THE SYSTEM SHALL** コートの3D座標系を以下のように定義する
- X軸: 左右方向（-X: 左、+X: 右）
- Y軸: 高さ方向（0: 地面、+Y: 上）
- Z軸: 前後方向（-Z: 1Pコート側、+Z: 2Pコート側）

**参照**: なし（独自定義）
**テスト**: TST-30504-001

---

### REQ-30501-002: コート境界（左右）
**WHEN** コート境界を判定する
**THE SYSTEM SHALL** コートの左右端を以下の座標で定義する
- 左端: X = `-config.Court.Width / 2` (デフォルト: -5.0m)
- 右端: X = `+config.Court.Width / 2` (デフォルト: +5.0m)

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#court-config)
**テスト**: TST-30504-002

---

### REQ-30501-003: コート境界（前後）
**WHEN** コート境界を判定する
**THE SYSTEM SHALL** コートの前後端を以下の座標で定義する
- 後端（1Pコート側）: Z = `-config.Court.Depth / 2` (デフォルト: -3.0m)
- 後端（2Pコート側）: Z = `+config.Court.Depth / 2` (デフォルト: +3.0m)

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#court-config)
**テスト**: TST-30504-003

---

### REQ-30501-004: コート境界（天井）
**WHEN** コート境界を判定する
**THE SYSTEM SHALL** コートの天井高さを以下の座標で定義する
- 天井: Y = `config.Court.CeilingHeight` (デフォルト: 5.0m)

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#court-config)
**テスト**: TST-30504-004

---

### REQ-30501-005: ネット位置
**WHEN** コート境界を判定する
**THE SYSTEM SHALL** ネットをコート中央に配置する
- ネットX座標: X = 0（コート中央、左右に伸びる）
- ネットZ座標: Z = 0（コート中央、1P/2Pの境界）
- ネット高さ: Y = 0.88m（パデルテニス規格）

**参照**: パデルテニス規格
**テスト**: TST-30504-005

---

### REQ-30501-006: コート区分（1P/2P）
**WHEN** ボールまたはプレイヤーの位置を判定する
**THE SYSTEM SHALL** コートを1Pコートと2Pコートに区分する
- 1Pコート範囲: Z < 0（ネットより手前）
- 2Pコート範囲: Z > 0（ネットより奥）

**テスト**: TST-30504-006

---

### REQ-30501-007: アウトの不存在
**WHEN** ボールが壁に到達する
**THE SYSTEM SHALL** アウト判定を行わない
**BECAUSE** コートは壁で完全に囲まれており、ボールが外に出ることはない

**参照**: パデルテニスルール
**備考**: 失点条件はツーバウンド、ネット、自コート打球のみ

---

### REQ-30501-008: サービスライン描画
**WHEN** コートが描画される
**THE SYSTEM SHALL** サービスライン（サービスボックスの境界線）を描画する
- サービスライン位置（1P側）: Z = `-config.Court.ServiceBoxDepth` (デフォルト: -1.5m)
- サービスライン位置（2P側）: Z = `+config.Court.ServiceBoxDepth` (デフォルト: +1.5m)
- センターサービスライン: X = 0（ネットからサービスラインまでの中央線）
- 線の色: 白
- 線の太さ: コート境界線と同様

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#court-config)
**テスト**: TST-30504-008
**備考**: サービスボックスはサーブ着地判定（REQ-30902-001）で使用される

---

## Extended Requirements (v0.2)

### REQ-30501-050: ネット衝突判定
**WHEN** ボールがネットに接触する
**THE SYSTEM SHALL** ボールとネットの衝突を判定する
- ネット位置: Z = `config.Court.NetZ` (デフォルト: 0.0m)
- ネット高さ: Y < `config.Court.NetHeight` (デフォルト: 0.88m)
- 衝突時: ボールは反射せず、ネットを越えた側に失点

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#court-config)
**テスト**: TST-30504-050

---

### REQ-30501-051: コート表面タイプ
**WHEN** ゲーム設定でコート表面タイプを選択する
**THE SYSTEM SHALL** コート表面タイプに応じてボール物理パラメータを変更する
- ハードコート: デフォルトの反射係数
- クレイコート: 反射係数減少、摩擦増加
- グラスコート: 反射係数増加、摩擦減少

**データ**: 新規 `config.Court.SurfaceType`
**テスト**: TST-30504-051

---

### REQ-30501-052: 環境照明
**WHEN** コートが描画される
**THE SYSTEM SHALL** 環境照明を設定する
- 昼間: 明るい照明、影あり
- 夜間: ナイター照明、強い影
- 室内: 均一照明、影なし

**データ**: 新規 `config.Court.LightingMode`
**テスト**: TST-30504-052

---

## Future Requirements (v0.3+)

### REQ-30501-100: 観客席・背景
**WHEN** コートが描画される
**THE SYSTEM SHALL** 観客席と背景を描画する
- 観客席: コート周囲に配置
- 背景: スタジアム、屋外、室内などのバリエーション
- アニメーション: 観客の歓声、動き

**データ**: 新規 `config.Court.BackgroundType`
**テスト**: TST-30504-100

---

### REQ-30501-101: 環境音
**WHEN** ゲームプレイ中
**THE SYSTEM SHALL** 環境音を再生する
- ボール音: 壁反射音、地面バウンド音
- 観客音: 歓声、拍手
- 環境音: 風、鳥の鳴き声（屋外の場合）

**データ**: 新規 `config.Audio.EnvironmentalSounds`
**テスト**: TST-30504-101

---

### REQ-30501-102: 天候システム
**WHEN** 屋外コートでゲームプレイ中
**THE SYSTEM SHALL** 天候システムを適用する
- 晴れ: デフォルト
- 雨: ボール速度減少、滑りやすい
- 風: ボール軌道に影響

**データ**: 新規 `config.Court.WeatherType`
**テスト**: TST-30504-102

---

## 制約（Design by Contract）

### 事前条件
- `config.Court.Width > 0`
- `config.Court.Depth > 0`
- `config.Court.CeilingHeight > 0`
- ネット高さ < 天井高さ

### 事後条件
- 全てのゲームオブジェクト（Player, Ball）はコート座標系内に配置される
- ネットはコート中央（X=0, Z=0）に配置される

### 不変条件
- コート座標系は変更されない（試合中不変）
- ネット位置は変更されない（試合中不変）

---

## データ参照

| パラメータ | データソース | デフォルト値 |
|----------|------------|------------|
| Court.Width | config.Court.Width | 10.0m |
| Court.Depth | config.Court.Depth | 6.0m |
| Court.CeilingHeight | config.Court.CeilingHeight | 5.0m |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md#court-config)

---

## 依存関係

### 依存先
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - ゲーム定数定義

### 依存元
- [30502_wall_design.md](30502_wall_design.md) - 壁の反射設計（コート境界を参照）
- [30503_boundary_behavior.md](30503_boundary_behavior.md) - 境界判定動作（コート区分を参照）

---

## 備考

### パデルテニスの特徴
- 壁で完全に囲まれており、アウトは存在しない
- 壁反射を活用したプレイが特徴
- ネットを超えて相手コートに入れば、その後の壁反射は自由

### 座標系の方向
- Z軸の正方向（+Z）が2Pコート側
- プレイヤーは通常、自コート側からネット方向（Z=0）に向かって移動
