# Boundary Behavior

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

コート境界におけるプレイヤーとボールの動作を定義します。プレイヤーは壁を超えて移動できず、ボールは壁で反射します。

## 動作定義（EARS記法）

### BEH-30503-001: プレイヤーの左右壁制限
**WHEN** プレイヤーが左右壁に接触する
**THE SYSTEM SHALL** プレイヤーの移動を停止させる
- Position.X を壁の座標に制限
- Velocity.X = 0（壁方向の速度成分を0に）

**参照**: config.Court.Width
**テスト**: TST-30504-011

---

### BEH-30503-002: プレイヤーの前後壁制限
**WHEN** プレイヤーが前後壁に接触する
**THE SYSTEM SHALL** プレイヤーの移動を停止させる
- Position.Z を壁の座標に制限
- Velocity.Z = 0（壁方向の速度成分を0に）

**参照**: config.Court.Depth
**テスト**: TST-30504-012

---

### BEH-30503-003: プレイヤーのネット通過禁止
**WHEN** プレイヤーがネットに接触する
**THE SYSTEM SHALL** プレイヤーの移動を停止させる
- Position.Z を自コート側に制限
- 1Pの場合: Position.Z < 0（ネットより手前）
- 2Pの場合: Position.Z > 0（ネットより奥）

**備考**: プレイヤーは相手コートに入れない
**テスト**: TST-30504-013

---

### BEH-30503-004: ボールの壁反射判定
**WHEN** ボールが壁境界に接触する
**THE SYSTEM SHALL** 壁の種類を判定する
- 左壁: Position.X ≤ `-config.Court.Width / 2`
- 右壁: Position.X ≥ `+config.Court.Width / 2`
- 後壁（1P側）: Position.Z ≤ `-config.Court.Depth / 2`
- 後壁（2P側）: Position.Z ≥ `+config.Court.Depth / 2`
- 天井: Position.Y ≥ `config.Court.CeilingHeight`

**AND THE SYSTEM SHALL** 反射処理を実行する

**備考**: 壁反射ロジックは本ドキュメント BEH-30503-004 〜 BEH-30503-006 で定義
**テスト**: TST-30504-014

---

### BEH-30503-005: ボールのネット接触判定
**WHEN** ボールがネット位置を通過する
**AND** ボールの高さがネット高さ未満である
**THE SYSTEM SHALL** ネット直撃失点を判定する
- 接触条件: |Position.Z| < 0.1m かつ Position.Y < 0.88m

**AND THE SYSTEM SHALL** NetHitEvent を発行する

**テスト**: TST-30504-015

---

### BEH-30503-006: ボールのコート区分判定
**WHEN** ボールがバウンドする
**THE SYSTEM SHALL** ボールがどちらのコートにいるかを判定する
- 1Pコート: Position.Z < 0
- 2Pコート: Position.Z > 0
- ネット上: Position.Z == 0（稀なケース、直前のコートを継続）

**備考**: ツーバウンド判定に使用
**テスト**: TST-30504-016

---

### BEH-30503-007: 境界チェックの優先順位
**WHILE** 境界判定を実行する
**THE SYSTEM SHALL** 以下の順序で判定する
1. ネット接触（失点判定優先）
2. 地面接触（バウンド判定）
3. 壁接触（反射処理）
4. 天井接触（反射処理）

**備考**: 1フレームで複数接触する場合の処理順序
**テスト**: TST-30504-017

---

## 制約（Design by Contract）

### 事前条件
- プレイヤーおよびボールの Position が有効な座標
- config.Court.* パラメータが正しく設定されている

### 事後条件
- プレイヤーは常にコート内に存在する
- ボールは壁反射またはネット失点により、適切に処理される

### 不変条件
- プレイヤーの Position.X は `[-Court.Width/2, +Court.Width/2]` の範囲内
- プレイヤーの Position.Z は自コート側の範囲内
- プレイヤーの Position.Y は `[0, Court.CeilingHeight]` の範囲内（ジャンプ時）

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
- [30501_court_spec.md](30501_court_spec.md) - コート境界定義
- 壁反射ロジック: 本ドキュメント BEH-30503-004 〜 BEH-30503-006 で定義
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - ゲーム定数

### 依存元
- プレイヤー移動システム（302_player）- プレイヤー移動制限
- ボール物理システム（304_ball）- ボール壁反射

---

## 備考

### 境界判定の精度
- 境界判定は各フレームで実行され、境界を超えた場合に即座に補正
- 高速移動時の壁すり抜けを防ぐため、前フレームとの線分交差判定も必要（将来実装）

### ネット接触の扱い
- ネット接触は失点条件のため、壁反射より優先して判定
- ネット上部を超えた場合はプレイ続行
