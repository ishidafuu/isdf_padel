# ECS Overview

**Version**: 3.0.0
**Last Updated**: 2026-01-06
**Status**: Active

## 概要

Bevy ネイティブ ECS による疎結合設計。2.5D パデルゲームに特化した ECS 構成。

## 基本方針

### 1. Entity
- **ID ベース管理**: Entity は一意な ID で識別
- **Component の集合**: Entity 自体は Component のコンテナ
- **振る舞いを持たない**: ロジックは System が担当

### 2. Component
- **データ専用**: ロジックを含まない
- **小さく・単一責務**: `Position` と `Velocity` を分離
- **共有可能**: Player/Enemy/Ball で共通利用
- **derive マクロ**: `#[derive(Component)]` で定義

### 3. System
- **ロジックの実装**: Component を操作してゲームを進行
- **独立性**: 他 System への直接参照禁止、Event 経由で通信
- **更新順序の管理**: `.chain()` または `before()`/`after()` で制御

---

## 主要 Component

### 3D 移動関連

| Component | 役割 | Fields |
|-----------|------|--------|
| **Position** | 3D座標 | x (f32), y (f32), z (f32) |
| **Velocity** | 3D速度 | x (f32), y (f32), z (f32) |
| **Gravity** | 重力の影響 | value (f32, default: -9.8) |
| **GroundState** | 地面接地状態 | is_grounded (bool) |

**座標系の定義**:
- **X**: 左右移動（-court_width/2 ~ +court_width/2）
- **Y**: 高さ（0.0 ~ max_jump_height）、重力の影響を受ける
- **Z**: 奥行き移動（0.0 ~ court_depth）、無段階

### 当たり判定関連

| Component | 役割 | Fields |
|-----------|------|--------|
| **WallCollider** | 壁当たり判定 | bounce_factor (f32, default: 0.8) |
| **CharacterCollider** | キャラクター当たり判定 | radius (f32), z_tolerance (f32) |
| **BallCollider** | ボール当たり判定 | radius (f32) |

### ふっとばし関連

| Component | 役割 | Fields |
|-----------|------|--------|
| **KnockbackState** | ふっとばし状態 | direction (Vec3), speed (f32), duration (f32), invincibility_time (f32) |
| **Invincible** | 無敵状態 | remaining_time (f32) |
| **Controllable** | 操作可否 | is_controllable (bool) |

### レンダリング関連

| Component | 役割 | Fields |
|-----------|------|--------|
| **DepthOrder** | レンダリング順序 | order (i32) |

---

## 主要 System

### 1. movement_system

**責務**: 移動・重力処理

**操作する Component**: Position, Velocity, Gravity, GroundState

**処理フロー**:
```
1. 入力を Velocity に反映（input_system → movement_system）
2. 重力を適用: velocity.y += gravity.value * time.delta_secs()
3. Position を更新: position += velocity * time.delta_secs()
4. 地面接地判定: position.y <= 0 → is_grounded = true, velocity.y = 0
```

---

### 2. wall_reflection_system

**責務**: 壁・天井反射処理

**操作する Component**: Position, Velocity, WallCollider

**反射ルール**:

| 壁 | 条件 | 処理 |
|----|------|------|
| 左壁 | position.x < -court_width/2 | velocity.x = -velocity.x * bounce_factor |
| 右壁 | position.x > +court_width/2 | velocity.x = -velocity.x * bounce_factor |
| 前壁 | position.z < 0 | velocity.z = -velocity.z * bounce_factor |
| 後壁 | position.z > court_depth | velocity.z = -velocity.z * bounce_factor |
| 天井 | position.y > ceiling_height | velocity.y = -velocity.y * bounce_factor |

**イベント発行**:
- `WallHitEvent` を発行（SE再生、エフェクト表示用）

---

### 3. character_collision_system

**責務**: キャラクター当たり判定

**操作する Component**: Position, CharacterCollider

**衝突判定条件**:
```
distance_2d(ball, character) < (ball_radius + character_radius)
&& (ball.z - character.z).abs() < z_tolerance
→ 衝突とみなす
```

**イベント発行**:
- `BallHitEvent` を発行（ふっとばし処理用）

---

### 4. knockback_system

**責務**: ふっとばし処理

**購読するイベント**: `BallHitEvent`

**処理フロー**:
```
1. BallHitEvent を受信
2. KnockbackState コンポーネントを追加
3. ふっとばし方向・速度を計算（ボールの速度に基づく）
4. Invincible コンポーネントを追加（無敵時間）
5. 操作不能フラグをセット
```

---

### 5. depth_order_system

**責務**: レンダリング順序の更新

**操作する Component**: Position, Transform

**処理**: Z値を Transform.translation.z に変換

```rust
fn depth_order_system(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut query {
        // Bevy は Z が小さいほど手前に描画
        transform.translation.z = -pos.z * 0.01;
    }
}
```

---

## System 実行順序

**重要**: System の実行順序は依存関係に基づいて決定。

```
1. input_system（入力 → Velocity に反映）
2. movement_system（Velocity → Position に反映、重力適用）
3. character_collision_system（当たり判定、イベント発行）
4. knockback_system（イベント購読、ふっとばし処理）
5. wall_reflection_system（壁反射、Velocity 調整）
6. depth_order_system（Position → Transform.z に変換）
7. sync_transform_system（Position → Transform.x/y に反映）
```

**Bevy での順序制御**:
```rust
app.add_systems(Update, (
    input_system,
    movement_system,
    character_collision_system,
    knockback_system,
    wall_reflection_system,
    depth_order_system,
    sync_transform_system,
).chain());
```

**理由**:
- 入力 → 移動 → 当たり判定 → 反射 → レンダリング の順序で自然な挙動

---

## Entity 構成例

### Player Entity

| Component | 値 |
|-----------|---|
| Position | x=0.0, y=0.0, z=3.0 |
| Velocity | x=0.0, y=0.0, z=0.0 |
| Gravity | value=-9.8 |
| GroundState | is_grounded=true |
| CharacterCollider | radius=0.5, z_tolerance=0.3 |
| Controllable | is_controllable=true |
| Transform | translation=(0.0, 0.0, -0.03) |
| Sprite | ... |

### Ball Entity

| Component | 値 |
|-----------|---|
| Position | x=0.0, y=1.5, z=5.0 |
| Velocity | x=3.0, y=2.0, z=-1.0 |
| Gravity | value=-9.8 |
| WallCollider | bounce_factor=0.8 |
| BallCollider | radius=0.2 |
| Transform | translation=(0.0, 1.5, -0.05) |
| Sprite | ... |

---

## 設計原則

### 1. Component は純粋なデータ
```rust
// ✅ 良い例
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// ❌ 悪い例（ロジックを含む）
impl Position {
    pub fn move_to(&mut self, new_x: f32) { ... }
}
```

### 2. System 間の通信は Event 経由
```rust
// ✅ 良い例
events.write(BallHitEvent { ... });

// ❌ 悪い例（直接参照）
knockback_system.apply_knockback(target_id, direction);
```

### 3. System は関数として定義
```rust
// ✅ 良い例（Bevy System）
pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Position, &Velocity)>,
) {
    for (mut pos, vel) in &mut query {
        pos.x += vel.x * time.delta_secs();
        pos.y += vel.y * time.delta_secs();
        pos.z += vel.z * time.delta_secs();
    }
}
```

---

## 次のステップ

1. ✅ ECS 設計（このドキュメント）
2. ⏳ Component 定義の詳細化（`209_components/`）
3. ⏳ EventSystem の設計（`20005_event_system.md`）
4. ⏳ InputSystem の設計（`20006_input_system.md`）

## 参考資料

- [20000_overview.md](20000_overview.md) - アーキテクチャ概要
- [20001_layers.md](20001_layers.md) - レイヤー構成
- [20005_event_system.md](20005_event_system.md) - イベントシステム
