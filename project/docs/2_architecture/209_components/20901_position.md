# Position Component

**Version**: 3.0.0
**Last Updated**: 2026-01-06
**Status**: Active

## 概要

Entity の3D座標を表すコンポーネント。2.5D パデルゲームの座標系に対応。

## 使用機能
- [301_player](../../3_ingame/301_player/)
- [302_enemy](../../3_ingame/302_enemy/)
- [303_ball](../../3_ingame/303_ball/)

## 座標系の定義（IMPORTANT）

| 軸 | 意味 | 範囲 | 備考 |
|----|------|------|------|
| **X** | 左右移動 | -court_width/2 ~ +court_width/2 | 画面の横方向 |
| **Y** | 高さ（ジャンプ） | 0.0 ~ max_jump_height | 重力の影響を受ける |
| **Z** | 奥行き移動 | 0.0 ~ court_depth | 無段階、くにおくん方式 |

**レンダリング**:
- Z値が大きい（奥にいる）Entity は後ろに描画
- depth_order_system が Position.z から Bevy の Transform.translation.z を計算

## 定義

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| x | f32 | 0.0 | 左右座標（-court_width/2 ~ +court_width/2） |
| y | f32 | 0.0 | 高さ座標（0.0 ~ max_jump_height） |
| z | f32 | 3.0 | 奥行き座標（0.0 ~ court_depth） |

## 使用例

### Player Entity
```rust
commands.spawn((
    Position {
        x: 0.0,   // コート中央（左右）
        y: 0.0,   // 地面
        z: 3.0,   // コート中央（奥行き）
    },
    Velocity::default(),
    // ... 他のコンポーネント
));
```

### Ball Entity
```rust
commands.spawn((
    Position {
        x: 0.0,
        y: 1.5,   // サーブ高さ
        z: 5.0,   // 奥側
    },
    Velocity { x: 3.0, y: 2.0, z: -1.0 },
    // ... 他のコンポーネント
));
```

## 関連システム

| System | 操作内容 |
|--------|---------|
| movement_system | Position を Velocity に基づいて更新 |
| wall_reflection_system | 壁・天井の境界チェックに Position を使用 |
| character_collision_system | Position 間の距離計算で衝突判定 |
| depth_order_system | Position.z から Transform.translation.z を計算 |

## 制約

### コート範囲
```rust
// X軸（左右）
const COURT_WIDTH: f32 = 10.0;
// -COURT_WIDTH/2 <= position.x <= COURT_WIDTH/2

// Y軸（高さ）
const MAX_JUMP_HEIGHT: f32 = 5.0;
// 0.0 <= position.y <= MAX_JUMP_HEIGHT

// Z軸（奥行き）
const COURT_DEPTH: f32 = 6.0;
// 0.0 <= position.z <= COURT_DEPTH
```

### 境界処理
- **壁**: x, z が範囲外 → wall_reflection_system が Velocity を反転
- **地面**: y < 0 → movement_system が y = 0 に修正
- **天井**: y > ceiling_height → wall_reflection_system が velocity.y を反転

## 実装例

```rust
// Rust 実装
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Bevy Vec2（X-Y平面）への変換
    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// Bevy Vec3 への変換
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// 2点間の X-Y 平面距離
    pub fn distance_2d(&self, other: &Position) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// 2点間の X-Z 平面距離
    pub fn distance_xz(&self, other: &Position) -> f32 {
        ((self.x - other.x).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}
```

## 参考資料

- [20000_overview.md](../20000_overview.md) - 座標系の定義
- [20004_ecs_overview.md](../20004_ecs_overview.md) - ECS 詳細
- [20902_velocity.md](20902_velocity.md) - Velocity コンポーネント
