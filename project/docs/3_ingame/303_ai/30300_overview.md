# AI - Overview

**Version**: 2.0.0
**Last Updated**: 2026-01-08
**Status**: Draft

---

## 概要

AI対戦相手の思考・行動を定義します。

ボール追跡、返球判断、ポジショニング、難易度調整など、コンピュータ対戦相手に関する全ての機能を含みます。

---

## 管理する機能

| ID範囲 | 機能 | 説明 |
|--------|------|------|
| 303xx | AI挙動全般 | ボール追跡、返球判断、ポジショニング、難易度調整 |

### 主要機能

1. **ボール追跡**
   - ボールの軌道予測
   - 落下地点計算
   - 移動判断

2. **返球判断**
   - ショットタイミング判定
   - ショット種類選択（v0.2以降）
   - 打球方向決定

3. **ポジショニング**（v0.2以降）
   - コート内での位置取り
   - 戦略的な配球

4. **難易度調整**（v0.2以降）
   - 移動速度調整
   - 反応速度調整
   - ミス率調整

---

## MVP v0.1の範囲

### ✅ 含む機能

- **シンプルなボール追跡**: ボールの位置に向かって移動
- **基本的な返球**: ボールが近づいたらショット
- **固定難易度**: 調整なし（中級程度）

### ❌ 含まない機能（v0.2以降）

- **軌道予測**: ボールの落下地点を予測
- **戦略的配球**: 相手の位置を考慮した打ち分け
- **難易度調整**: 初級/中級/上級
- **学習機能**: プレイヤーの戦術への適応

---

## コンポーネント設計

### ECS Components

AIは以下のComponentsを持ちます：

| Component | 説明 | 参照 |
|-----------|------|------|
| `Position` | X/Y/Z軸の位置 | [20901_position.md](../../2_architecture/209_components/20901_position.md) |
| `Velocity` | X/Y/Z軸の速度 | [20902_velocity.md](../../2_architecture/209_components/20902_velocity.md) |
| `AIState` | AI状態（追跡中、待機中、返球中） | 未定義 |
| `DifficultyState` | 難易度パラメータ（v0.2以降） | 未定義 |

### Systems

AIに関連するSystemsは以下の通り：

| System | 責務 | 参照 |
|--------|------|------|
| `AIMovementSystem` | ボール追跡、移動判断 | 未定義 |
| `AIShotSystem` | 返球判断、ショット実行 | 未定義 |
| `AIPositioningSystem` | ポジショニング（v0.2以降） | 未定義 |
| `AIDifficultySystem` | 難易度調整（v0.2以降） | 未定義 |

---

## AI仕様（MVP v0.1）

### シンプルなボール追跡

```csharp
// AIMovementSystem
void UpdateAIMovement(aiEntityId, deltaTime) {
    var aiPos = GetComponent<Position>(aiEntityId);
    var ballPos = GetComponent<Position>(ballEntityId);

    // ボールの位置に向かって移動
    Vector2 direction = new Vector2(
        ballPos.X - aiPos.X,
        ballPos.Z - aiPos.Z
    ).normalized;

    // 速度設定（プレイヤーと同じ速度）
    var aiVelocity = GetComponent<Velocity>(aiEntityId);
    aiVelocity.X = direction.x * config.AI.MoveSpeed;  // デフォルト: 5.0 m/s
    aiVelocity.Z = direction.y * config.AI.MoveSpeed;
}
```

### 基本的な返球

```csharp
// AIShotSystem
void UpdateAIShot(aiEntityId) {
    // ボールが近くにあるか判定
    if (!IsNearBall(aiEntityId)) {
        return;
    }

    // クールダウン判定
    var aiState = GetComponent<AIState>(aiEntityId);
    if (aiState.ShotCooldown > 0) {
        return;
    }

    // ショット実行（相手コート中央に向けて打つ）
    Vector2 targetDirection = CalculateTargetDirection(aiEntityId);

    eventBus.Publish(new ShotEvent {
        playerId = aiEntityId,
        direction = targetDirection,
        jumpHeight = 0  // MVP v0.1ではジャンプショットなし
    });

    // クールダウン設定
    aiState.ShotCooldown = config.AI.ShotCooldown;  // デフォルト: 0.5秒
}

// 打球方向計算（シンプル版）
Vector2 CalculateTargetDirection(aiEntityId) {
    var aiPos = GetComponent<Position>(aiEntityId);

    // 相手コート中央に向けて打つ
    Vector2 targetPos = new Vector2(
        config.Court.OpponentCenter.X,
        config.Court.OpponentCenter.Z
    );

    return (targetPos - new Vector2(aiPos.X, aiPos.Z)).normalized;
}
```

---

## AI仕様（v0.2以降）

### 軌道予測

```csharp
// ボールの落下地点を予測
Vector3 PredictBallLanding(ballEntityId) {
    var ballPos = GetComponent<Position>(ballEntityId);
    var ballVel = GetComponent<Velocity>(ballEntityId);

    // 簡易な放物線計算
    float timeToGround = (-ballVel.Y - Mathf.Sqrt(
        ballVel.Y * ballVel.Y - 2 * config.Physics.Gravity * ballPos.Y
    )) / config.Physics.Gravity;

    Vector3 landingPos = new Vector3(
        ballPos.X + ballVel.X * timeToGround,
        0,
        ballPos.Z + ballVel.Z * timeToGround
    );

    return landingPos;
}
```

### 戦略的配球

```csharp
// 相手の位置を考慮した打ち分け
Vector2 CalculateStrategicDirection(aiEntityId) {
    var opponentPos = GetComponent<Position>(opponentEntityId);

    // 相手がいない方向に打つ
    Vector2 targetPos;
    if (opponentPos.X < 0) {
        // 相手が左側にいる場合、右側に打つ
        targetPos = new Vector2(config.Court.RightTarget.X, config.Court.RightTarget.Z);
    } else {
        // 相手が右側にいる場合、左側に打つ
        targetPos = new Vector2(config.Court.LeftTarget.X, config.Court.LeftTarget.Z);
    }

    var aiPos = GetComponent<Position>(aiEntityId);
    return (targetPos - new Vector2(aiPos.X, aiPos.Z)).normalized;
}
```

### 難易度調整

```csharp
// 難易度パラメータ
public class DifficultyConfig {
    public float MoveSpeed;      // 移動速度
    public float ReactionTime;   // 反応速度
    public float MissRate;       // ミス率
}

// 難易度設定
var difficultyConfigs = new Dictionary<Difficulty, DifficultyConfig> {
    { Difficulty.Easy, new DifficultyConfig {
        MoveSpeed = config.AI.EasyMoveSpeed,      // デフォルト: 3.0 m/s
        ReactionTime = config.AI.EasyReaction,    // デフォルト: 0.5秒
        MissRate = config.AI.EasyMissRate         // デフォルト: 0.3（30%）
    }},
    { Difficulty.Normal, new DifficultyConfig {
        MoveSpeed = config.AI.NormalMoveSpeed,    // デフォルト: 5.0 m/s
        ReactionTime = config.AI.NormalReaction,  // デフォルト: 0.3秒
        MissRate = config.AI.NormalMissRate       // デフォルト: 0.1（10%）
    }},
    { Difficulty.Hard, new DifficultyConfig {
        MoveSpeed = config.AI.HardMoveSpeed,      // デフォルト: 7.0 m/s
        ReactionTime = config.AI.HardReaction,    // デフォルト: 0.1秒
        MissRate = config.AI.HardMissRate         // デフォルト: 0.05（5%）
    }}
};
```

---

## データ定義

AIのパラメータは `8_data/80101_game_constants.md` に定義されます。

| パラメータ | 設定ファイル参照 | デフォルト値（参考） |
|-----------|----------------|------------------|
| 移動速度（MVP） | `config.AI.MoveSpeed` | 5.0 m/s |
| ショットクールダウン | `config.AI.ShotCooldown` | 0.5秒 |
| 初級移動速度 | `config.AI.EasyMoveSpeed` | 3.0 m/s（v0.2以降） |
| 中級移動速度 | `config.AI.NormalMoveSpeed` | 5.0 m/s（v0.2以降） |
| 上級移動速度 | `config.AI.HardMoveSpeed` | 7.0 m/s（v0.2以降） |
| 初級ミス率 | `config.AI.EasyMissRate` | 0.3（30%、v0.2以降） |

**重要**: 具体的な値は参考値です。実装時は必ず `config.AI.*` を参照してください。

---

## 参考資料

### ナムコテニス

- [AI Behavior](../../9_reference/901_reference_game/mechanics/90115_ai_behavior.md)
  - 確度: ★★☆☆☆
  - 参照推奨項目: ボール追跡、返球判断、難易度パラメータ（参考程度）

### アーキテクチャ

- [20000_overview.md](../../2_architecture/20000_overview.md) - アーキテクチャ概要
- [20004_ecs_overview.md](../../2_architecture/20004_ecs_overview.md) - ECS設計

### Padel Actionの特徴

- **シンプルなAI（MVP v0.1）**: ボールを追いかけて返球するだけ
- **戦略的なAI（v0.2以降）**: 配球、ポジショニング、難易度調整

---

## 詳細仕様書

| ファイル | 内容 | バージョン |
|---------|------|-----------|
| [30301_ai_movement_spec.md](30301_ai_movement_spec.md) | AI移動・ポジショニング | MVP + v0.2 |
| [30302_ai_shot_spec.md](30302_ai_shot_spec.md) | AIショット・難易度調整 | MVP + v0.2 |
| [30303_ai_tactics_spec.md](30303_ai_tactics_spec.md) | AI戦術選択（攻め/守り） | v0.4 |

---

## 次のステップ

1. ✅ AI全体の構造定義（このドキュメント）
2. ✅ 詳細仕様の策定
   - ✅ 30301_ai_movement_spec.md: ボール追跡・ポジショニング
   - ✅ 30302_ai_shot_spec.md: 返球判断・難易度調整
3. ⏳ データ定義の更新（8_data/80101_game_constants.md）
4. ⏳ 実装開始

---

## Change Log

### 2026-01-08 - v2.0.0

- 詳細仕様書へのリンク追加
- 30301_ai_movement_spec.md 作成
- 30302_ai_shot_spec.md 作成

### 2025-12-23 - v1.0.0（初版）

- AI機能の全体構造定義
- MVP v0.1の範囲設定（シンプルな追跡AI）
- Component/System設計の明確化
- AI仕様の定義（ボール追跡、返球判断）
