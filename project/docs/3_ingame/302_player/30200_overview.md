# Player - Overview

**Version**: 1.1.0
**Last Updated**: 2026-01-09
**Status**: Draft

---

## 概要

プレイヤーキャラクターの操作・挙動を定義します。

3軸移動（X/Y/Z）、ジャンプ、ショット、ふっとばしなど、プレイヤーが直接操作する全ての機能を含みます。

---

## 管理する機能

| ID範囲 | 機能 | 説明 |
|--------|------|------|
| 302xx | プレイヤー操作全般 | 移動、ジャンプ、ショット、ふっとばし |

### 主要機能

1. **3軸移動**
   - X軸（左右）: 方向キー左右
   - Y軸（高さ）: Aボタンでジャンプ、重力で落下
   - Z軸（奥行き）: 方向キー上下

2. **ジャンプ**
   - Aボタンでジャンプ
   - 重力による落下
   - 地面判定、着地

3. **ショット**
   - Bボタンでショット
   - 方向指定（十字キー）
   - ジャンプ中のショット可能

4. **ふっとばし**
   - ボールがキャラクターに当たった時
   - ふっとばし方向・速度の計算
   - 操作不能時間、無敵時間

---

## MVP v0.1の範囲

### ✅ 含む機能

- **3軸移動**: X/Y/Z軸の基本移動
- **ジャンプ**: Y軸ジャンプ、重力、着地
- **ショット**: Bボタンでショット、方向指定
- **ジャンプショット**: ジャンプ中のショット
- **ふっとばし**: ボールが当たった時の吹っ飛び、操作不能時間

### ❌ 含まない機能（v0.4以降）

- **必殺ショット**: 特殊なショット（Shot Systemで管理）
- **アニメーション**: 簡易表示のみ

---

## コンポーネント設計

### ECS Componentsプレイヤーは以下のComponentsを持ちます：

| Component | 説明 | 参照 |
|-----------|------|------|
| `Position` | X/Y/Z軸の位置 | [20901_position.md](../../2_architecture/209_components/20901_position.md) |
| `Velocity` | X/Y/Z軸の速度 | [20902_velocity.md](../../2_architecture/209_components/20902_velocity.md) |
| `Height` | 高さ関連データ（旧仕様、Positionに統合予定） | [20903_height.md](../../2_architecture/209_components/20903_height.md) |
| `InputState` | 入力状態（方向キー、ボタン） | 未定義 |
| `KnockbackState` | ふっとばし状態 | 未定義 |
| `InvincibleState` | 無敵状態 | 未定義 |

### Systems

プレイヤーに関連するSystemsは以下の通り：

| System | 責務 | 参照 |
|--------|------|------|
| `InputSystem` | 入力検知、InputStateの更新 | [20006_input_system.md](../../2_architecture/20006_input_system.md) |
| `MovementSystem` | Velocity → Position更新、重力適用 | 未定義 |
| `JumpSystem` | ジャンプ入力 → Velocity.Y設定 | 未定義 |
| `ShotInputSystem` | ショット入力 → ShotEvent発行 | 未定義 |
| `KnockbackSystem` | BallHitEvent → ふっとばし処理 | 未定義 |
| `DepthOrderSystem` | Z値 → 描画順序更新 | [20000_overview.md](../../2_architecture/20000_overview.md#4-深度レンダリングシステムdepthordersystem) |

---

## 操作仕様

### 入力マッピング

| 入力 | 動作 | 備考 |
|------|------|------|
| **方向キー左** | X軸負方向に移動 | 画面左 |
| **方向キー右** | X軸正方向に移動 | 画面右 |
| **方向キー上** | Z軸正方向に移動 | 画面奥 |
| **方向キー下** | Z軸負方向に移動 | 画面手前 |
| **Aボタン** | ジャンプ | Y軸正方向、地面にいる時のみ |
| **Bボタン** | ショット | ボールが近くにある時 |

**重要**: 当初のテニス想定（A=ショット）と逆になりました。**A=ジャンプ、B=ショット**。

### 移動速度

```csharp
// ❌ ハードコーディング禁止
velocity.X = 5.0f;

// ✅ 必須: GameConfig参照
velocity.X = input.X * config.Player.MoveSpeed;
velocity.Z = input.Z * config.Player.MoveSpeed;
```

**設定ファイル参照**: `config.Player.MoveSpeed`（デフォルト: 5.0 m/s）

### ジャンプ

```csharp
// 地面にいる時のみジャンプ可能
if (isGrounded && input.JumpPressed) {
    velocity.Y = config.Player.JumpForce;  // デフォルト: 10.0 m/s
}

// 毎フレーム重力を適用
velocity.Y += config.Physics.Gravity * deltaTime;  // デフォルト: -9.8 m/s²
```

### ふっとばし

```csharp
// BallHitEvent購読
on BallHitEvent(ballId, targetId, hitPoint):
    1. KnockbackStateコンポーネントを追加
    2. ふっとばし方向 = ボールの速度ベクトル正規化
    3. ふっとばし速度 = config.Player.KnockbackSpeed  // デフォルト: 8.0 m/s
    4. 操作不能フラグをセット
    5. 無敵時間タイマーを開始（config.Player.InvincibleTime）  // デフォルト: 1.5秒
```

---

## データ定義

プレイヤーのパラメータは `8_data/80101_game_constants.md` に定義されます。

| パラメータ | 設定ファイル参照 | デフォルト値（参考） |
|-----------|----------------|------------------|
| 移動速度 | `config.Player.MoveSpeed` | 5.0 m/s |
| ジャンプ力 | `config.Player.JumpForce` | 10.0 m/s |
| ふっとばし速度 | `config.Player.KnockbackSpeed` | 8.0 m/s |
| 無敵時間 | `config.Player.InvincibleTime` | 1.5秒 |
| コリジョン半径 | `config.Player.Radius` | 0.5 m |
| Z軸許容範囲 | `config.Player.ZTolerance` | 0.3 m |

**重要**: 具体的な値は参考値です。実装時は必ず `config.Player.*` を参照してください。

---

## 参考資料

### ナムコテニス

- [Player Movement](../../9_reference/901_reference_game/mechanics/90111_player_movement.md)
  - 確度: ★★★☆☆
  - 参照推奨項目: 基本移動、コート範囲制限

### アーキテクチャ

- [20000_overview.md](../../2_architecture/20000_overview.md) - アーキテクチャ概要
- [20004_ecs_overview.md](../../2_architecture/20004_ecs_overview.md) - ECS設計
- [20006_input_system.md](../../2_architecture/20006_input_system.md) - 入力システム

### Padel Actionの特徴

- **2.5D横視点**: Z軸が奥行き、無段階移動
- **A=ジャンプ、B=ショット**: ナムコテニスとは異なる操作系
- **キャラクター当たり判定**: ボールがキャラクターに直接当たる
- **ふっとばし**: ボールが当たると吹っ飛ぶ

---

## 次のステップ

1. ✅ Player全体の構造定義（このドキュメント）
2. ⏳ 詳細仕様の策定
   - 30201_movement_spec.md: 3軸移動の詳細
   - 30202_jump_spec.md: ジャンプの詳細
   - 30203_knockback_spec.md: ふっとばしの詳細
3. ⏳ データ定義の更新（8_data/80101_game_constants.md）
4. ⏳ 実装開始

---

## Change Log

### 2026-01-09 - v1.1.0

- 含まない機能をv0.4以降に変更
- ダッシュ、スタミナ削除（テニスでは不要）

### 2025-12-23 - v1.0.0（初版）

- Player機能の全体構造定義
- MVP v0.1の範囲設定
- Component/System設計の明確化
- 操作仕様の定義
