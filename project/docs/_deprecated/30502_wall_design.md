# Wall Reflection Design

> **⚠️ DEPRECATED (2026-01-08)**
>
> この仕様書は廃止されました。
>
> **理由**: パデルからテニスへのルール変更に伴い、壁反射機能は不要となりました。
> テニスではオープンコート（壁・天井なし）となり、コート外に落ちたボールはアウト判定となります。
>
> **代替**: [30501_court_spec.md](../3_ingame/305_court/30501_court_spec.md) のREQ-30501-007（アウト境界）を参照してください。

**Version**: 1.0.0
**Status**: ~~Draft~~ **DEPRECATED**
**Last Updated**: 2025-12-23
**Deprecated**: 2026-01-08

## 概要

コートを囲む壁（左右壁、前後壁、天井）の反射ロジックを定義します。パデルテニスでは壁反射がゲームプレイの重要な要素です。

## 設計（データ構造）

### DES-30502-001: Wall Component
```csharp
// @spec REQ-30501-002, REQ-30501-003, REQ-30501-004
// @data 80101_game_constants.md
public struct Wall : IComponent
{
    public WallType Type;          // 壁の種類
    public Vector3 Normal;         // 壁の法線ベクトル（反射計算用）
}
```

**説明**: 壁の種類と法線ベクトルを保持するコンポーネント

---

### DES-30502-002: WallType Enum
```csharp
public enum WallType
{
    LeftWall,    // 左壁（X = -Court.Width/2）
    RightWall,   // 右壁（X = +Court.Width/2）
    BackWall1P,  // 後壁（1Pコート側、Z = -Court.Depth/2）
    BackWall2P,  // 後壁（2Pコート側、Z = +Court.Depth/2）
    Ceiling      // 天井（Y = Court.CeilingHeight）
}
```

**説明**: 壁の種類を区別するための列挙型

---

### DES-30502-003: WallReflectionEvent
```csharp
// @spec BEH-30502-005
public struct WallReflectionEvent : IEvent
{
    public Entity Ball;              // 反射したボール
    public WallType WallType;        // 反射した壁の種類
    public Vector3 ContactPoint;     // 接触点
    public Vector3 IncidentVelocity; // 入射速度
    public Vector3 ReflectedVelocity;// 反射後の速度
}
```

**説明**: 壁反射イベント

---

## 動作定義（EARS記法）

### BEH-30502-001: 左右壁の反射
**WHEN** ボールが左右壁に接触する
**THE SYSTEM SHALL** ボールの速度X成分を反転させる
- 反射後速度.X = `-速度.X * config.Ball.BounceFactor` (デフォルト係数: 0.8)
- 反射後速度.Y = `速度.Y * config.Ball.BounceFactor`
- 反射後速度.Z = `速度.Z * config.Ball.BounceFactor`

**AND THE SYSTEM SHALL** WallReflectionEvent を発行する

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#ball-config)
**テスト**: TST-30504-007

---

### BEH-30502-002: 前後壁の反射
**WHEN** ボールが前後壁に接触する
**THE SYSTEM SHALL** ボールの速度Z成分を反転させる
- 反射後速度.X = `速度.X * config.Ball.BounceFactor`
- 反射後速度.Y = `速度.Y * config.Ball.BounceFactor`
- 反射後速度.Z = `-速度.Z * config.Ball.BounceFactor`

**AND THE SYSTEM SHALL** WallReflectionEvent を発行する

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#ball-config)
**テスト**: TST-30504-008

---

### BEH-30502-003: 天井の反射
**WHEN** ボールが天井に接触する
**THE SYSTEM SHALL** ボールの速度Y成分を反転させる
- 反射後速度.X = `速度.X * config.Ball.BounceFactor`
- 反射後速度.Y = `-速度.Y * config.Ball.BounceFactor`
- 反射後速度.Z = `速度.Z * config.Ball.BounceFactor`

**AND THE SYSTEM SHALL** WallReflectionEvent を発行する

**データ**: [80101_game_constants.md](../../8_data/80101_game_constants.md#ball-config)
**テスト**: TST-30504-009

---

### BEH-30502-004: 壁反射の優先順位
**WHEN** ボールが複数の壁に同時に接触する可能性がある
**THE SYSTEM SHALL** 最も近い壁との接触を優先する
- 接触判定順序: 左右壁 → 前後壁 → 天井

**備考**: コーナーへの同時接触を防ぐ

---

### BEH-30502-005: 反射イベントの発行
**WHEN** ボールが壁に反射する
**THE SYSTEM SHALL** WallReflectionEvent をイベントバスに発行する
- Ball: 反射したボールのEntity
- WallType: 反射した壁の種類
- ContactPoint: 接触点の座標
- IncidentVelocity: 反射前の速度
- ReflectedVelocity: 反射後の速度

**依存**: [20005_event_system.md](../../2_architecture/20005_event_system.md)
**テスト**: TST-30504-010

---

## 制約（Design by Contract）

### 事前条件
- ボールがコート境界に接触している
- ボールの速度が0でない（静止していない）

### 事後条件
- 反射後、ボールはコート内に戻る方向に速度が変化する
- 反射後の速度の大きさは、入射速度の大きさ * BounceFactor 以下

### 不変条件
- Wall.Normal は常に単位ベクトル（長さ1）
- BounceFactor は 0 < x ≤ 1 の範囲内

---

## データ参照

| パラメータ | データソース | デフォルト値 |
|----------|------------|------------|
| Ball.BounceFactor | config.Ball.BounceFactor | 0.8 |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md#ball-config)

---

## 依存関係

### 依存先
- [30501_court_spec.md](30501_court_spec.md) - コート境界定義
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - 壁パラメータ
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - イベントシステム

### 依存元
- ボール物理システム（304_ball）- ボールの壁反射ロジック

---

## 備考

### 反射の物理
- 完全弾性衝突ではなく、BounceFactor によって速度が減衰
- 実際のパデルテニスでは壁の材質によって反発係数が異なるが、本実装では全壁共通

### 将来の拡張性
- 壁ごとに異なる BounceFactor を設定可能にする可能性
- 壁の材質によるスピンの変化を実装する可能性
