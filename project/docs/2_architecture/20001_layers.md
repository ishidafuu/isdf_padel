# Layers

**Version**: 3.0.0
**Last Updated**: 2026-01-06
**Status**: Active

## 概要

5層構造による責務分離。上位レイヤーは下位レイヤーのみ参照可能。

## レイヤー一覧

### Layer 1: Core（基盤層）

**責務**: エンジン非依存の共通機能

| 要素 | 役割 |
|------|------|
| Events | イベントの定義 |
| Utility | 数学関数、座標変換 |
| Traits | 共通トレイト |

**依存**: なし

**主要コード**:
```rust
// events.rs
#[derive(Event)]
pub struct BallHitEvent {
    pub ball_id: Entity,
    pub target_id: Entity,
    pub hit_point: Vec3,
}

// utils.rs
pub fn reflect(velocity: Vec3, normal: Vec3) -> Vec3 {
    velocity - 2.0 * velocity.dot(normal) * normal
}

pub fn distance_2d(a: Vec3, b: Vec3) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

pub fn distance_xz(a: Vec3, b: Vec3) -> f32 {
    ((a.x - b.x).powi(2) + (a.z - b.z).powi(2)).sqrt()
}
```

---

### Layer 2: Resource（設定・マスタデータ層）

**責務**: ゲーム設定、マスタデータの定義

| Resource | 役割 |
|----------|------|
| GameConfig | ゲーム全体の設定（物理、移動、サイズ等） |
| MasterData | エンティティのパラメータ（Player速度、Enemy HP等） |

**依存**: Core

**例**:
```rust
// config.rs
#[derive(Resource, Deserialize)]
pub struct GameConfig {
    pub physics: PhysicsConfig,
    pub court: CourtConfig,
    pub player: PlayerConfig,
    pub ball: BallConfig,
}

#[derive(Deserialize)]
pub struct PhysicsConfig {
    #[serde(default = "default_gravity")]
    pub gravity: f32,
    #[serde(default = "default_max_fall_speed")]
    pub max_fall_speed: f32,
}

fn default_gravity() -> f32 { -9.8 }
fn default_max_fall_speed() -> f32 { -20.0 }
```

**重要**: ハードコーディング禁止
- 調整可能な全てのパラメータは GameConfig に外部化
- System は GameConfig を `Res<GameConfig>` で受け取る
- データファイルは `project/docs/8_data/` に定義（RON 形式）

---

### Layer 3: Components（データ層）

**責務**: Entity のデータ構造定義

| Component | 役割 |
|-----------|------|
| Position | 3D座標（X, Y, Z） |
| Velocity | 3D速度（X, Y, Z） |
| KnockbackState | ふっとばし状態 |
| WallCollider | 壁当たり判定 |
| DepthOrder | レンダリング順序 |

**依存**: Core

**例**:
```rust
// position.rs
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Position {
    pub x: f32, // 左右
    pub y: f32, // 高さ
    pub z: f32, // 奥行き
}

// knockback.rs
#[derive(Component, Clone, Debug)]
pub struct KnockbackState {
    pub direction: Vec3,
    pub speed: f32,
    pub duration: f32,
    pub invincibility_time: f32,
}
```

---

### Layer 4: Systems（ロジック層）

**責務**: ゲームロジックの実装

| System | 役割 | 操作する Component |
|--------|------|-------------------|
| movement_system | 移動・重力処理 | Position, Velocity |
| wall_reflection_system | 壁・天井反射 | Position, Velocity, WallCollider |
| character_collision_system | キャラクター当たり判定 | Position |
| knockback_system | ふっとばし処理 | Position, Velocity, KnockbackState |
| depth_order_system | レンダリング順序更新 | Position, Transform |

**依存**: Core, Resource, Components

**処理フロー例（wall_reflection_system）**:
```rust
pub fn wall_reflection_system(
    config: Res<GameConfig>,
    mut events: EventWriter<WallHitEvent>,
    mut query: Query<(Entity, &Position, &mut Velocity, &WallCollider)>,
) {
    let court_width = config.court.width;
    let court_depth = config.court.depth;
    let ceiling_height = config.court.ceiling_height;

    for (entity, pos, mut vel, collider) in &mut query {
        // 左右の壁（設定ファイル参照）
        if pos.x < -court_width / 2.0 || pos.x > court_width / 2.0 {
            vel.x = -vel.x * collider.bounce_factor;
            events.write(WallHitEvent {
                entity_id: entity,
                wall_type: WallType::Side,
            });
        }

        // 前後の壁
        if pos.z < 0.0 || pos.z > court_depth {
            vel.z = -vel.z * collider.bounce_factor;
            events.write(WallHitEvent {
                entity_id: entity,
                wall_type: WallType::FrontBack,
            });
        }

        // 天井
        if pos.y > ceiling_height {
            vel.y = -vel.y * collider.bounce_factor;
            events.write(WallHitEvent {
                entity_id: entity,
                wall_type: WallType::Ceiling,
            });
        }
    }
}
```

---

### Layer 5: Presentation（表現層）

**責務**: Bevy Sprite、Transform、UI

| 要素 | 役割 |
|------|------|
| SpriteBundle | Entity の描画 |
| Transform | 位置・回転・スケール |
| UiBundle | UI表示 |

**依存**: Systems

**例**:
```rust
// sync_transform_system.rs（Position を Bevy Transform に同期）
pub fn sync_transform_system(
    mut query: Query<(&Position, &mut Transform)>,
) {
    for (pos, mut transform) in &mut query {
        // Position を Transform に反映
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
        // Z は深度ソート用（小さい値が手前）
        transform.translation.z = -pos.z * 0.01;
    }
}

// spawn_player.rs
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("player.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Position { x: 0.0, y: 0.0, z: 3.0 },
        Velocity::default(),
        CharacterCollider { radius: 0.5, z_tolerance: 0.3 },
        Controllable { is_controllable: true },
    ));
}
```

---

## 依存ルール

### 禁止パターン

| ❌ 禁止 | 理由 |
|---------|------|
| Components → Systems | データがロジックに依存してはいけない |
| Systems ↔ Systems | System 間の直接参照（Event 経由にする） |
| Core → 上位レイヤー | 基盤が上位に依存してはいけない |

### 許可パターン

| ✅ 許可 | 例 |
|---------|---|
| Systems → Components | `movement_system` が `Position` を操作 |
| Systems → Core | `wall_reflection_system` が Event を発行 |
| Presentation → Systems | `sync_transform_system` が `Position` を読み取る |

---

## 2.5D 座標系とレイヤーの対応

| 座標軸 | 処理レイヤー | 担当 System |
|--------|-------------|------------|
| X（左右） | Systems | movement_system, wall_reflection_system |
| Y（高さ） | Systems | movement_system（重力）, wall_reflection_system（天井） |
| Z（奥行き） | Systems | movement_system, depth_order_system |
| レンダリング順序 | Presentation | sync_transform_system |

---

## 実装時の注意点

### 1. Component の粒度
- **小さく・単一責務**: `Position` と `Velocity` を分離
- **再利用可能**: Player/Enemy/Ball で共通利用

### 2. System の独立性
- **他 System への直接参照禁止**: Event 経由で通信
- **単一責務**: `movement_system` は移動のみ、反射は `wall_reflection_system`

### 3. Bevy との統合
- **Position と Transform を分離**: ゲームロジックは Position、描画は Transform
- **sync_transform_system**: Position → Transform の同期を Presentation 層で実行

---

## 次のステップ

1. ✅ レイヤー定義（このドキュメント）
2. ⏳ 各 System の詳細設計（`20004_ecs_overview.md`）
3. ⏳ Component 定義の更新（`209_components/`）
4. ⏳ EventSystem の設計（`20005_event_system.md`）

## 参考資料

- [20000_overview.md](20000_overview.md) - アーキテクチャ概要
- [20004_ecs_overview.md](20004_ecs_overview.md) - ECS 詳細
- [20002_dependencies.md](20002_dependencies.md) - 依存関係ルール
