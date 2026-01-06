# Court - Overview

**Version**: 1.0.0
**Last Updated**: 2025-12-23
**Status**: Draft

---

## 概要

コート構造（壁、天井、境界）を定義します。

コートサイズ、壁・天井の配置、境界判定、視覚表現など、コートに関する全ての機能を含みます。

---

## 管理する機能

| ID範囲 | 機能 | 説明 |
|--------|------|------|
| 305xx | コート構造全般 | サイズ、壁、天井、境界判定、視覚表現 |

### 主要機能

1. **コートサイズ**
   - 約10m x 20m（密閉空間）
   - X/Y/Z軸の範囲定義

2. **壁構造**
   - 左右の壁（X軸境界）
   - 前後の壁（Z軸境界）
   - 反射判定

3. **天井構造**
   - 天井の高さ（Y軸境界）
   - 反射判定（アウトではない）

4. **境界判定**
   - プレイヤーの移動範囲制限
   - ボールの反射判定
   - アウト判定（地面に落ちた時）

5. **視覚表現**
   - Godot Sceneでの描画
   - 壁・天井のSprite/Mesh

---

## MVP v0.1の範囲

### ✅ 含む機能

- **コートサイズ定義**: 約10m x 20m
- **4面の壁**: 左右、前後の壁、反射判定
- **天井**: 反射判定（アウトではない）
- **境界判定**: プレイヤー移動制限、ボール反射
- **簡易な視覚表現**: シンプルなSprite/Mesh

### ❌ 含まない機能（v0.2以降）

- **ネット**: パデルルールではネットがない（v0.2で要検討）
- **詳細なテクスチャ**: リアルな壁・床の表現
- **ライティング**: 影、光源
- **エフェクト**: 反射エフェクト、衝突エフェクト

---

## コンポーネント設計

### ECS Components

コートは以下のComponentsを持ちます（Entityとしては存在しないが、データとして定義）：

| Component | 説明 | 参照 |
|-----------|------|------|
| `CourtConfig` | コートサイズ、壁・天井の範囲 | 未定義 |
| `WallConfig` | 各壁の位置、反射係数 | 未定義 |

### Systems

コートに関連するSystemsは以下の通り：

| System | 責務 | 参照 |
|--------|------|------|
| `WallReflectionSystem` | 壁・天井反射判定 | [20000_overview.md](../../2_architecture/20000_overview.md#2-壁天井反射システムwallreflectionsystem) |
| `CourtBoundarySystem` | プレイヤー移動制限、アウト判定 | 未定義 |
| `CourtRenderSystem` | 壁・天井の描画 | 未定義 |

---

## コート仕様

### コートサイズ

| 軸 | 範囲 | 説明 |
|----|------|------|
| **X軸** | -10.0 ~ +10.0 m | 左右の幅（約20m） |
| **Y軸** | 0.0 ~ 6.0 m | 地面 ~ 天井（約6m） |
| **Z軸** | 0.0 ~ 10.0 m | 手前 ~ 奥（約10m） |

**設定ファイル参照**:
```csharp
config.Court.MinX = -10.0f;  // デフォルト: -10.0 m
config.Court.MaxX = +10.0f;  // デフォルト: +10.0 m
config.Court.MinZ = 0.0f;    // デフォルト: 0.0 m
config.Court.MaxZ = 10.0f;   // デフォルト: 10.0 m
config.Court.CeilingHeight = 6.0f;  // デフォルト: 6.0 m
```

### 壁構造

| 壁 | 位置 | 反射軸 |
|----|------|--------|
| **左壁** | X = config.Court.MinX | X軸反転 |
| **右壁** | X = config.Court.MaxX | X軸反転 |
| **前壁** | Z = config.Court.MinZ | Z軸反転 |
| **後壁** | Z = config.Court.MaxZ | Z軸反転 |

### 天井構造

| 要素 | 位置 | 反射軸 |
|------|------|--------|
| **天井** | Y = config.Court.CeilingHeight | Y軸反転 |

**重要**: 天井反射は**アウトではない**（パデルルール）

### 境界判定

#### プレイヤー移動制限

```csharp
// CourtBoundarySystem
void ClampPlayerPosition(playerId) {
    var position = GetComponent<Position>(playerId);

    // X軸制限
    position.X = Mathf.Clamp(position.X, config.Court.MinX, config.Court.MaxX);

    // Z軸制限
    position.Z = Mathf.Clamp(position.Z, config.Court.MinZ, config.Court.MaxZ);

    // Y軸制限（地面のみ、天井は制限しない）
    position.Y = Mathf.Max(position.Y, 0);
}
```

#### ボール反射判定

```csharp
// WallReflectionSystem（詳細は20000_overview.mdを参照）
void CheckWallReflection(ballId) {
    var ball = GetComponent<Position>(ballId);
    var velocity = GetComponent<Velocity>(ballId);

    // 左壁
    if (ball.X < config.Court.MinX) {
        ball.X = config.Court.MinX;
        velocity.X *= -config.Ball.BounceFactor;
    }

    // 右壁
    if (ball.X > config.Court.MaxX) {
        ball.X = config.Court.MaxX;
        velocity.X *= -config.Ball.BounceFactor;
    }

    // 前壁
    if (ball.Z < config.Court.MinZ) {
        ball.Z = config.Court.MinZ;
        velocity.Z *= -config.Ball.BounceFactor;
    }

    // 後壁
    if (ball.Z > config.Court.MaxZ) {
        ball.Z = config.Court.MaxZ;
        velocity.Z *= -config.Ball.BounceFactor;
    }

    // 天井
    if (ball.Y > config.Court.CeilingHeight) {
        ball.Y = config.Court.CeilingHeight;
        velocity.Y *= -config.Ball.BounceFactor;
    }
}
```

#### アウト判定

```csharp
// CourtBoundarySystem
void CheckOutOfBounds(ballId) {
    var ball = GetComponent<Position>(ballId);

    // 地面に落ちた時のみ判定
    if (ball.Y <= 0) {
        bool isOut = ball.X < config.Court.MinX
                  || ball.X > config.Court.MaxX
                  || ball.Z < config.Court.MinZ
                  || ball.Z > config.Court.MaxZ;

        if (isOut) {
            eventBus.Publish(new PointEndEvent {
                reason = PointEndReason.Out,
                winner = GetOpponent(currentServerId)
            });
        }
    }
}
```

---

## 視覚表現（Godot Scene）

### シーン構造

```
Court (Node2D)
  ├─ Floor (Sprite)
  ├─ WallLeft (Sprite)
  ├─ WallRight (Sprite)
  ├─ WallFront (Sprite)
  ├─ WallBack (Sprite)
  └─ Ceiling (Sprite)
```

### 描画順序

| 要素 | Z-Index | 備考 |
|------|---------|------|
| **床** | -100 | 最背面 |
| **壁（奥）** | 0 | プレイヤー/ボールより後ろ |
| **プレイヤー/ボール** | 0～100 | Z値に応じて動的に変化 |
| **壁（手前）** | 200 | 最前面 |
| **天井** | -50 | 床の上、壁の後ろ |

### 2.5D表現

- **Z値 → Z-Index**: `DepthOrderSystem` が毎フレーム更新
- **奥にいる Entity は後ろに描画**
- **手前にいる Entity は前に描画**

---

## データ定義

コートのパラメータは `8_data/80101_game_constants.md` に定義されます。

| パラメータ | 設定ファイル参照 | デフォルト値（参考） |
|-----------|----------------|------------------|
| 左端 | `config.Court.MinX` | -10.0 m |
| 右端 | `config.Court.MaxX` | +10.0 m |
| 手前端 | `config.Court.MinZ` | 0.0 m |
| 奥端 | `config.Court.MaxZ` | 10.0 m |
| 天井高さ | `config.Court.CeilingHeight` | 6.0 m |
| ボール反射係数 | `config.Ball.BounceFactor` | 0.8 |

**重要**: 具体的な値は参考値です。実装時は必ず `config.Court.*` を参照してください。

---

## Padel Actionの特徴

### 密閉空間

- **4面の壁 + 天井**: 全て反射する
- **オープンコートではない**: テニスとの違い

### 天井反射

- **天井はアウトではない**: パデルルール
- **反射係数適用**: `config.Ball.BounceFactor`（デフォルト: 0.8）

### 2.5D横視点

- **Z軸が奥行き**: くにおくんドッジボール風
- **Y軸が高さ**: ジャンプ、重力

---

## 参考資料

### ゲームコンセプト

- [10001_concept.md](../../1_project/10001_concept.md#court-structure) - コート構造の説明

### アーキテクチャ

- [20000_overview.md](../../2_architecture/20000_overview.md) - アーキテクチャ概要
- [20001_layers.md](../../2_architecture/20001_layers.md) - 5層構造（Presentation層）

---

## 次のステップ

1. ✅ Court全体の構造定義（このドキュメント）
2. ⏳ 詳細仕様の策定
   - 30501_size_spec.md: コートサイズの詳細
   - 30502_wall_spec.md: 壁構造の詳細
   - 30503_ceiling_spec.md: 天井構造の詳細
   - 30504_visual_spec.md: 視覚表現の詳細
3. ⏳ データ定義の更新（8_data/80101_game_constants.md）
4. ⏳ Godot Sceneの作成
5. ⏳ 実装開始

---

## Change Log

### 2025-12-23 - v1.0.0（初版）

- Court機能の全体構造定義
- MVP v0.1の範囲設定
- コートサイズ、壁・天井構造の明確化
- 境界判定仕様の定義
- 視覚表現（Godot Scene）の設計
