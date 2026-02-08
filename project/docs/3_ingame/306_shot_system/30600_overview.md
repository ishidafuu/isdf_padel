# Shot System - Overview

**Version**: 2.1.0
**Last Updated**: 2026-01-09
**Status**: Draft

---

## 概要

ショットシステムは、プレイヤーがボールを打つ操作とロジックを定義します。

入力受付、打球方向計算、ジャンプショット、タイミング判定など、ショットに関する全ての機能を含みます。

## 仕様書一覧

| ID | ファイル | 内容 |
|----|---------|------|
| 30601 | [30601_shot_input_spec.md](30601_shot_input_spec.md) | ショット入力 |
| 30602 | [30602_shot_direction_spec.md](30602_shot_direction_spec.md) | 打球方向 |
| 30604 | [30604_shot_attributes_spec.md](30604_shot_attributes_spec.md) | ショット属性 |
| 30605 | [30605_trajectory_calculation_spec.md](30605_trajectory_calculation_spec.md) | 弾道計算 |
| 30606 | [30606_racket_contact_spec.md](30606_racket_contact_spec.md) | ラケット接触駆動ショット |

---

## 管理する機能

| ID範囲 | 機能 | 説明 |
|--------|------|------|
| 306xx | ショット操作全般 | 入力、方向計算、ジャンプショット、タイミング判定 |

### 主要機能

1. **ショット入力**
   - Bボタンでショット
   - 方向指定（十字キー）
   - タイミング判定（ボールとの距離）

2. **打球方向計算**
   - 入力方向 → ボール初速度ベクトル
   - 打球角度（水平方向）
   - 打球角度（垂直方向）

3. **ジャンプショット**
   - ジャンプ中のショット可能
   - Y軸位置に応じて軌道変化

4. **ショット属性システム**（v0.2）
   - 5要素による無段階ショット変化
   - 入力方式（プッシュ/ホールド）
   - 打点、タイミング、入り方、距離で属性決定

5. **必殺ショット**（v0.3以降）
   - 適切な高さで打つことで発生
   - 特殊な軌道、威力

---

## MVP v0.1の範囲

### ✅ 含む機能

- **基本ショット**: Bボタン、方向指定（十字キー）
- **ジャンプショット**: ジャンプ中のショット
- **タイミング判定**: ボールが近くにある時のみショット可能
- **打球方向計算**: 入力方向 → ボール初速度ベクトル

### ❌ 含まない機能（v0.4以降）

- **必殺ショット**: 特殊なショット
- **キャラ別カーブ係数**: キャラクター固有の係数

---

## v0.2の範囲（実装済み）

### ✅ 含む機能

- **ショット属性システム**: 5要素による無段階ショット変化
  - 入力方式（プッシュ精度/ホールド時間）
  - 打点の高さ（Z座標）
  - バウンド経過時間
  - ボールへの入り方（移動ベクトル）
  - ボールとの距離
- **属性計算**: 威力、安定性、角度、スピン、精度
- **ホールド/プッシュ**: 押しっぱなし vs タイミング合わせ

## v0.3の範囲（実装済み）

### ✅ 含む機能

- **スピン軌道影響**: スピン値に応じた軌道変化
- **スピン視覚化**: ボール色変更による視認性

### ❌ 含まない機能（v0.4以降）

- **必殺ショット**: 特殊なショット（高さ依存）
- **キャラ別カーブ係数**: キャラクター固有の係数

---

## コンポーネント設計

### ECS Components

ショットは以下のComponentsを扱います：

| Component | 説明 | 参照 |
|-----------|------|------|
| `Position` | プレイヤー、ボールの位置 | [20901_position.md](../../2_architecture/209_components/20901_position.md) |
| `Velocity` | ボールの速度（ショット後に設定） | [20902_velocity.md](../../2_architecture/209_components/20902_velocity.md) |
| `InputState` | 入力状態（方向キー、Bボタン） | 未定義 |
| `ShotState` | ショット状態（クールダウン、タイミング） | 未定義 |

### Systems

ショットに関連するSystemsは以下の通り：

| System | 責務 | 参照 |
|--------|------|------|
| `ShotInputSystem` | Bボタン入力 → ShotEvent発行 | 未定義 |
| `ShotTimingSystem` | ボールとの距離判定、タイミング判定 | 未定義 |
| `ShotDirectionSystem` | 入力方向 → 打球ベクトル計算 | 未定義 |
| `ShotExecuteSystem` | ShotEvent → ボール速度設定 | 未定義 |

---

## ショット仕様

### ショット入力

```csharp
// ShotInputSystem
void OnShotInput(entityId, input) {
    // タイミング判定
    if (!IsNearBall(entityId)) {
        return;  // ボールが近くにない
    }

    // クールダウン判定
    if (shotState.CooldownTimer > 0) {
        return;  // まだクールダウン中
    }

    // ShotEvent発行
    eventBus.Publish(new ShotEvent {
        playerId = entityId,
        direction = input.Direction,  // 十字キー入力
        jumpHeight = position.Y       // ジャンプ中の高さ
    });
}
```

### タイミング判定

```csharp
// ShotTimingSystem
bool IsNearBall(playerId) {
    var playerPos = GetComponent<Position>(playerId);
    var ballPos = GetComponent<Position>(ballEntityId);

    float distance = Vector2.Distance(
        new Vector2(playerPos.X, playerPos.Z),
        new Vector2(ballPos.X, ballPos.Z)
    );

    float heightDiff = Mathf.Abs(playerPos.Y - ballPos.Y);

    // ボールが打てる範囲内か
    return distance < config.Shot.MaxDistance  // デフォルト: 1.5 m
        && heightDiff < config.Shot.MaxHeightDiff;  // デフォルト: 2.0 m
}
```

### 打球方向計算

```csharp
// ShotDirectionSystem
Vector3 CalculateShotVelocity(input, jumpHeight) {
    // 水平方向（入力方向）
    Vector2 horizontalDir = input.Direction.normalized;

    // 打球角度（垂直方向）
    float verticalAngle = config.Shot.DefaultAngle;  // デフォルト: 45度

    // ジャンプショットの場合、角度調整
    if (jumpHeight > 0.5f) {
        verticalAngle = config.Shot.JumpAngle;  // デフォルト: 30度
    }

    // 初速度ベクトル
    float speed = config.Shot.DefaultSpeed;  // デフォルト: 15.0 m/s
    Vector3 velocity = new Vector3(
        horizontalDir.x * speed * Mathf.Cos(verticalAngle),
        speed * Mathf.Sin(verticalAngle),
        horizontalDir.y * speed * Mathf.Cos(verticalAngle)
    );

    return velocity;
}
```

### ショット実行

```csharp
// ShotExecuteSystem
void OnShotEvent(ShotEvent e) {
    // 打球方向計算
    Vector3 shotVelocity = CalculateShotVelocity(e.direction, e.jumpHeight);

    // ボールの速度を設定
    var ballVelocity = GetComponent<Velocity>(ballEntityId);
    ballVelocity.Value = shotVelocity;

    // ボール状態を更新
    var ballState = GetComponent<BallState>(ballEntityId);
    ballState.State = BallStateType.Flying;

    // プレイヤーのクールダウン開始
    var shotState = GetComponent<ShotState>(e.playerId);
    shotState.CooldownTimer = config.Shot.Cooldown;  // デフォルト: 0.5秒
}
```

---

## ジャンプショット

### 特徴

- **ジャンプ中のショット可能**: Y軸位置 > 0.5m
- **打球角度変化**: 通常45度 → ジャンプ時30度（急角度）
- **速度変化**: `config.Shot.JumpSpeed`（デフォルト: 18.0 m/s、通常より速い）

### 処理

```csharp
// ジャンプ中の判定
bool IsJumping(playerId) {
    var position = GetComponent<Position>(playerId);
    return position.Y > config.Shot.JumpThreshold;  // デフォルト: 0.5 m
}

// ジャンプショット時の速度計算
if (IsJumping(playerId)) {
    speed = config.Shot.JumpSpeed;  // 18.0 m/s
    verticalAngle = config.Shot.JumpAngle;  // 30度
} else {
    speed = config.Shot.DefaultSpeed;  // 15.0 m/s
    verticalAngle = config.Shot.DefaultAngle;  // 45度
}
```

---

## 必殺ショット（v0.4以降）

### 発動条件（仮説）

- **適切な高さで打つ**: `config.Shot.SpecialHeightMin` ~ `config.Shot.SpecialHeightMax`
- **タイミング**: ボールの高さと一致
- **ボタン**: Bボタン長押し（v0.2で検討）

### 効果（未定）

- 特殊な軌道（壁反射無視、貫通など）
- 高威力（ふっとばし距離増加）
- エフェクト表示

**詳細は実装しながら固める**

---

## データ定義

ショットのパラメータは `8_data/80101_game_constants.md` に定義されます。

| パラメータ | 設定ファイル参照 | デフォルト値（参考） |
|-----------|----------------|------------------|
| 打球可能距離 | `config.Shot.MaxDistance` | 1.5 m |
| 打球可能高さ差 | `config.Shot.MaxHeightDiff` | 2.0 m |
| 通常ショット速度 | `config.Shot.DefaultSpeed` | 15.0 m/s |
| ジャンプショット速度 | `config.Shot.JumpSpeed` | 18.0 m/s |
| 通常打球角度 | `config.Shot.DefaultAngle` | 45度 |
| ジャンプ打球角度 | `config.Shot.JumpAngle` | 30度 |
| ジャンプ判定閾値 | `config.Shot.JumpThreshold` | 0.5 m |
| クールダウン時間 | `config.Shot.Cooldown` | 0.5秒 |

**重要**: 具体的な値は参考値です。実装時は必ず `config.Shot.*` を参照してください。

---

## 参考資料

### ナムコテニス

- [Shot System](../../9_reference/901_reference_game/mechanics/90112_shot_system.md)
  - 確度: ★★★☆☆
  - 参照推奨項目: 2ボタンによるショット使い分け、タイミング判定システム

### アーキテクチャ

- [20000_overview.md](../../2_architecture/20000_overview.md) - アーキテクチャ概要
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - イベントシステム
- [20006_input_system.md](../../2_architecture/20006_input_system.md) - 入力システム

### Padel Actionの特徴

- **ジャンプショット**: ジャンプ中のショット可能（くにおくん風）
- **必殺ショット**: 適切な高さで打つことで発生（v0.2以降）
- **B=ショット**: A=ジャンプ、B=ショット（ナムコテニスとは異なる）

---

## 次のステップ

1. ✅ Shot System全体の構造定義（このドキュメント）
2. ⏳ 詳細仕様の策定
   - 30601_shot_input_spec.md: ショット入力の詳細
   - 30602_shot_direction_spec.md: 打球方向計算の詳細
   - 30603_jump_shot_spec.md: ジャンプショットの詳細
3. ⏳ データ定義の更新（8_data/80101_game_constants.md）
4. ⏳ 実装開始

---

## Change Log

### 2026-01-09 - v2.1.0

- v0.2/v0.3を実装済みに変更
- 含まない機能をv0.4以降に変更
- 必殺ショットをv0.4以降に移動

### 2026-01-07 - v2.0.0

- v0.2ショット属性システムの追加
- 5要素による無段階ショット変化の定義
- 30604_shot_attributes_spec.md の参照追加

### 2025-12-23 - v1.0.0（初版）

- Shot System機能の全体構造定義
- MVP v0.1の範囲設定
- Component/System設計の明確化
- ショット仕様の定義（入力、タイミング、方向計算）
- ジャンプショットの仕様定義
