# Velocity Component

**Version**: 3.0.0
**Last Updated**: 2026-01-06
**Status**: Active

## 概要

Entity の3D速度を表すコンポーネント。2.5D パデルゲームの移動に対応。

## 使用機能
- [301_player](../../3_ingame/301_player/)
- [302_enemy](../../3_ingame/302_enemy/)
- [303_ball](../../3_ingame/303_ball/)

## 定義

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| x | f32 | 0.0 | 左右速度（m/s） |
| y | f32 | 0.0 | 上下速度（m/s、重力の影響を受ける） |
| z | f32 | 0.0 | 奥行き速度（m/s） |

## 速度の制御

### 移動速度（Player）

| 軸 | 最大速度 | 説明 |
|----|---------|------|
| X | 5.0 m/s | 左右移動（走り） |
| Y | 8.0 m/s | ジャンプ初速（上向き） |
| Z | 4.0 m/s | 奥行き移動（走り） |

### 重力（全 Entity）

```rust
// 毎フレーム適用
velocity.y += gravity.value * time.delta_secs();
// gravity.value = -9.8 m/s²（地球の重力）
```

### ボール速度

| 状態 | 最大速度 | 説明 |
|------|---------|------|
| 通常ショット | 10.0 m/s | 基本的なショット |
| 強打 | 15.0 m/s | ジャンプショット等 |
| 壁反射後 | 速度 × 0.8 | 反射時の減衰 |

## 使用例

### Player 移動
```rust
// input_system → movement_system
pub fn movement_system(
    time: Res<Time>,
    config: Res<GameConfig>,
    mut query: Query<(&mut Position, &Velocity, &mut GroundState)>,
) {
    for (mut pos, vel, mut ground_state) in &mut query {
        // Position を更新
        pos.x += vel.x * time.delta_secs();
        pos.y += vel.y * time.delta_secs();
        pos.z += vel.z * time.delta_secs();

        // 地面接地判定
        if pos.y <= 0.0 {
            pos.y = 0.0;
            ground_state.is_grounded = true;
        }
    }
}

pub fn gravity_system(
    time: Res<Time>,
    config: Res<GameConfig>,
    mut query: Query<(&mut Velocity, &Gravity, &GroundState)>,
) {
    for (mut vel, gravity, ground_state) in &mut query {
        if !ground_state.is_grounded {
            vel.y += gravity.value * time.delta_secs();
        }
    }
}
```

### ボール反射
```rust
// wall_reflection_system
pub fn wall_reflection_system(
    config: Res<GameConfig>,
    mut query: Query<(&Position, &mut Velocity, &WallCollider)>,
) {
    let court_width = config.court.width;
    let court_depth = config.court.depth;
    let ceiling_height = config.court.ceiling_height;

    for (pos, mut vel, collider) in &mut query {
        // 左右の壁
        if pos.x < -court_width / 2.0 || pos.x > court_width / 2.0 {
            vel.x = -vel.x * collider.bounce_factor;  // 反転 + 減衰
        }

        // 前後の壁
        if pos.z < 0.0 || pos.z > court_depth {
            vel.z = -vel.z * collider.bounce_factor;
        }

        // 天井
        if pos.y > ceiling_height {
            vel.y = -vel.y * collider.bounce_factor;
        }
    }
}
```

## 関連システム

| System | 操作内容 |
|--------|---------|
| movement_system | Velocity に基づいて Position を更新 |
| gravity_system | 重力を Velocity.y に適用 |
| wall_reflection_system | 壁反射時に Velocity を反転 |
| knockback_system | ふっとばし時に Velocity を上書き |

## 制約

### 最大速度制限
```rust
// 各軸で最大速度をクランプ
velocity.x = velocity.x.clamp(-max_speed_x, max_speed_x);
velocity.y = velocity.y.clamp(-max_fall_speed, max_jump_speed);
velocity.z = velocity.z.clamp(-max_speed_z, max_speed_z);
```

### 摩擦（地上のみ）
```rust
if ground_state.is_grounded {
    velocity.x *= 0.9;  // 左右の摩擦
    velocity.z *= 0.9;  // 奥行きの摩擦
}
```

## 実装例

```rust
// Rust 実装
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// 速度の大きさ（3D）
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// 正規化
    pub fn normalized(&self) -> Velocity {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Velocity::default();
        }
        Velocity {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    /// Bevy Vec3 への変換
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// 速度をクランプ
    pub fn clamp(&mut self, max_x: f32, max_y: f32, max_z: f32) {
        self.x = self.x.clamp(-max_x, max_x);
        self.y = self.y.clamp(-max_y, max_y);
        self.z = self.z.clamp(-max_z, max_z);
    }
}
```

## 参考資料

- [20000_overview.md](../20000_overview.md) - 座標系の定義
- [20004_ecs_overview.md](../20004_ecs_overview.md) - ECS 詳細
- [20901_position.md](20901_position.md) - Position コンポーネント
