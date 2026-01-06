# Ball - Overview

**Version**: 1.0.0
**Last Updated**: 2025-12-23
**Status**: Draft

---

## 概要

ボールの物理挙動を定義します。

軌道計算、壁・天井反射、キャラクター当たり判定、アウト/イン判定など、ボールに関する全ての機能を含みます。

---

## 管理する機能

| ID範囲 | 機能 | 説明 |
|--------|------|------|
| 304xx | ボール物理全般 | 軌道、反射、当たり判定、アウト判定 |

### 主要機能

1. **軌道計算**
   - 3D空間での放物線運動
   - 重力による落下
   - 速度ベクトルの更新

2. **壁・天井反射**
   - 左右の壁（X軸）
   - 前後の壁（Z軸）
   - 天井（Y軸）
   - 入射角 = 反射角、バウンド係数適用

3. **キャラクター当たり判定**
   - ボールとプレイヤー/AIの距離計算
   - Z軸許容範囲の考慮
   - BallHitEvent発行

4. **コート判定**
   - アウト判定（コート外）
   - イン判定（コート内）
   - ポイント終了イベント発行

---

## MVP v0.1の範囲

### ✅ 含む機能

- **3D軌道**: X/Y/Z軸での放物線運動
- **壁・天井反射**: 4面の壁 + 天井、全て反射する
- **キャラクター当たり判定**: ボールがキャラクターに当たる
- **コート判定**: アウト/イン判定
- **ふっとばしイベント**: BallHitEvent発行

### ❌ 含まない機能（v0.2以降）

- **スピン**: 回転による軌道変化
- **ボール間衝突**: 複数ボールの衝突判定
- **特殊な反射**: 摩擦、スライド
- **ボールエフェクト**: 軌跡、残像

---

## コンポーネント設計

### ECS Components

ボールは以下のComponentsを持ちます：

| Component | 説明 | 参照 |
|-----------|------|------|
| `Position` | X/Y/Z軸の位置 | [20901_position.md](../../2_architecture/209_components/20901_position.md) |
| `Velocity` | X/Y/Z軸の速度 | [20902_velocity.md](../../2_architecture/209_components/20902_velocity.md) |
| `BallState` | ボール状態（飛行中、バウンド中、保持中） | 未定義 |
| `Collider` | 当たり判定（半径） | 未定義 |

### Systems

ボールに関連するSystemsは以下の通り：

| System | 責務 | 参照 |
|--------|------|------|
| `BallPhysicsSystem` | 重力適用、Velocity → Position更新 | 未定義 |
| `WallReflectionSystem` | 壁・天井反射判定 | [20000_overview.md](../../2_architecture/20000_overview.md#2-壁天井反射システムwallreflectionsystem) |
| `CharacterCollisionSystem` | キャラクター当たり判定 | [20000_overview.md](../../2_architecture/20000_overview.md#3-キャラクター当たり判定システムcharactercollisionsystem) |
| `CourtBoundarySystem` | アウト/イン判定 | 未定義 |
| `DepthOrderSystem` | Z値 → 描画順序更新 | [20000_overview.md](../../2_architecture/20000_overview.md#4-深度レンダリングシステムdepthordersystem) |

---

## 物理仕様

### 軌道計算

```csharp
// 毎フレーム実行
void UpdateBallPhysics(deltaTime) {
    // 重力適用
    velocity.Y += config.Physics.Gravity * deltaTime;  // デフォルト: -9.8 m/s²

    // 位置更新
    position += velocity * deltaTime;
}
```

### 壁・天井反射

| 壁 | 反射軸 | 条件 | 処理 |
|----|--------|------|------|
| **左壁** | X軸反転 | `position.X < config.Court.MinX` | `velocity.X *= -config.Ball.BounceFactor` |
| **右壁** | X軸反転 | `position.X > config.Court.MaxX` | `velocity.X *= -config.Ball.BounceFactor` |
| **前壁** | Z軸反転 | `position.Z < config.Court.MinZ` | `velocity.Z *= -config.Ball.BounceFactor` |
| **後壁** | Z軸反転 | `position.Z > config.Court.MaxZ` | `velocity.Z *= -config.Ball.BounceFactor` |
| **天井** | Y軸反転 | `position.Y > config.Court.CeilingHeight` | `velocity.Y *= -config.Ball.BounceFactor` |

**重要**: 天井反射は**アウトではない**（パデルルール）

```csharp
// 壁反射の例
if (ball.Position.X < config.Court.MinX) {
    ball.Position.X = config.Court.MinX;  // 壁内に戻す
    ball.Velocity.X *= -config.Ball.BounceFactor;  // 反射、減衰
}
```

### キャラクター当たり判定

```csharp
// 衝突判定
bool IsColliding(ballPos, characterPos) {
    float distance = Vector2.Distance(
        new Vector2(ballPos.X, ballPos.Z),
        new Vector2(characterPos.X, characterPos.Z)
    );

    float zDiff = Mathf.Abs(ballPos.Y - characterPos.Y);  // Y軸は高さ

    return distance < (config.Ball.Radius + config.Character.Radius)
        && zDiff < config.Character.ZTolerance;
}

// 衝突時
if (IsColliding(ball, player)) {
    eventBus.Publish(new BallHitEvent {
        ballId = ballEntityId,
        targetId = playerEntityId,
        hitPoint = ball.Position
    });
}
```

### コート判定

```csharp
// アウト判定（地面に落ちた時）
if (ball.Position.Y <= 0) {
    bool isOut = ball.Position.X < config.Court.MinX
              || ball.Position.X > config.Court.MaxX
              || ball.Position.Z < config.Court.MinZ
              || ball.Position.Z > config.Court.MaxZ;

    if (isOut) {
        eventBus.Publish(new PointEndEvent {
            reason = PointEndReason.Out,
            winner = oppositePlayer
        });
    }
}
```

---

## データ定義

ボールのパラメータは `8_data/80101_game_constants.md` に定義されます。

| パラメータ | 設定ファイル参照 | デフォルト値（参考） |
|-----------|----------------|------------------|
| ボール半径 | `config.Ball.Radius` | 0.2 m |
| バウンド係数 | `config.Ball.BounceFactor` | 0.8（減衰はあるが強め） |
| 初速度（通常ショット） | `config.Shot.DefaultSpeed` | 15.0 m/s |
| 初速度（ジャンプショット） | `config.Shot.JumpSpeed` | 18.0 m/s |
| 重力 | `config.Physics.Gravity` | -9.8 m/s² |

**重要**: 具体的な値は参考値です。実装時は必ず `config.*` を参照してください。

---

## Padel Actionの特徴

### 壁・天井反射

- **4面の壁**: 左右、前後の壁、全て反射する
- **天井**: 壁の上部、**反射する**（アウトではない）
- **バウンド係数**: `config.Ball.BounceFactor`（デフォルト: 0.8、ラリー継続のため強め）

### キャラクター当たり判定

- **ボールがキャラクターに当たる**: くにおくんドッジボール風
- **ふっとばし**: ボールが当たるとキャラクターが吹っ飛ぶ
- **Z軸許容範囲**: `config.Character.ZTolerance`（デフォルト: 0.3 m、奥行き方向の当たり判定許容範囲）

### 2.5D座標系

- **X軸**: 左右（画面の横方向）
- **Y軸**: 高さ（ジャンプ、重力の影響）
- **Z軸**: 奥行き（無段階、くにおくん方式）

---

## 参考資料

### ナムコテニス

- [Ball Physics](../../9_reference/901_reference_game/mechanics/90113_ball_physics.md)
  - 確度: ★★★☆☆
  - 参照推奨項目: 放物線軌道、バウンド処理、コート判定

### アーキテクチャ

- [20000_overview.md](../../2_architecture/20000_overview.md) - アーキテクチャ概要
- [20004_ecs_overview.md](../../2_architecture/20004_ecs_overview.md) - ECS設計
- [20001_layers.md](../../2_architecture/20001_layers.md) - 5層構造

---

## 次のステップ

1. ✅ Ball全体の構造定義（このドキュメント）
2. ⏳ 詳細仕様の策定
   - 30401_physics_spec.md: 物理演算の詳細
   - 30402_reflection_spec.md: 壁・天井反射の詳細
   - 30403_collision_spec.md: 当たり判定の詳細
3. ⏳ データ定義の更新（8_data/80101_game_constants.md）
4. ⏳ 実装開始

---

## Change Log

### 2025-12-23 - v1.0.0（初版）

- Ball機能の全体構造定義
- MVP v0.1の範囲設定
- Component/System設計の明確化
- 物理仕様の定義（壁・天井反射、当たり判定）
