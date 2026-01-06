# Event System

**Version**: 3.0.0
**Last Updated**: 2026-01-06
**Status**: Active

## 概要

Entity 間の疎結合な相互作用を実現するイベントシステム。2.5D パデルゲームに特化したイベント定義。Bevy ネイティブの Event システムを使用。

## 設計方針

### 疎結合の原則
- **Entity 間の直接参照を禁止**: Player/Enemy/Ball 間の直接参照を避ける
- **Bevy Event による通信**: すべての Entity 間相互作用は Event 経由
- **発行・購読パターン**: EventWriter は EventReader を知らない

---

## イベント定義

### 移動・当たり判定関連

#### BallHitEvent
**発生条件**: ボールがキャラクターに当たった

| Field | Type | Description |
|-------|------|-------------|
| ball_id | Entity | ボールの Entity ID |
| target_id | Entity | 当たったキャラクターの Entity ID |
| hit_point | Vec3 | 衝突点（X, Y, Z） |
| ball_velocity | Vec3 | 衝突時のボール速度 |

**発行者**: character_collision_system
**購読者**: knockback_system, score_system, sound_effect_system

```rust
#[derive(Event)]
pub struct BallHitEvent {
    pub ball_id: Entity,
    pub target_id: Entity,
    pub hit_point: Vec3,
    pub ball_velocity: Vec3,
}
```

---

#### WallHitEvent
**発生条件**: ボールが壁・天井に反射した

| Field | Type | Description |
|-------|------|-------------|
| entity_id | Entity | 反射した Entity の ID |
| wall_type | WallType | 壁の種類（Left, Right, Front, Back, Ceiling） |
| hit_point | Vec3 | 反射点 |
| reflected_velocity | Vec3 | 反射後の速度 |

**WallType 列挙型**:
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallType {
    Left,      // 左壁（x < -court_width/2）
    Right,     // 右壁（x > court_width/2）
    Front,     // 前壁（z < 0）
    Back,      // 後壁（z > court_depth）
    Ceiling,   // 天井（y > ceiling_height）
}

#[derive(Event)]
pub struct WallHitEvent {
    pub entity_id: Entity,
    pub wall_type: WallType,
    pub hit_point: Vec3,
    pub reflected_velocity: Vec3,
}
```

**発行者**: wall_reflection_system
**購読者**: sound_effect_system, effect_system

---

### ふっとばし関連

#### KnockbackStartEvent
**発生条件**: ふっとばしが開始された

| Field | Type | Description |
|-------|------|-------------|
| target_id | Entity | ふっとばされる Entity の ID |
| direction | Vec3 | ふっとばし方向 |
| speed | f32 | ふっとばし速度 |
| duration | f32 | ふっとばし継続時間（秒） |

```rust
#[derive(Event)]
pub struct KnockbackStartEvent {
    pub target_id: Entity,
    pub direction: Vec3,
    pub speed: f32,
    pub duration: f32,
}
```

**発行者**: knockback_system
**購読者**: animation_system, sound_effect_system

---

#### KnockbackEndEvent
**発生条件**: ふっとばしが終了した

| Field | Type | Description |
|-------|------|-------------|
| target_id | Entity | ふっとばされていた Entity の ID |

```rust
#[derive(Event)]
pub struct KnockbackEndEvent {
    pub target_id: Entity,
}
```

**発行者**: knockback_system
**購読者**: input_system（操作可能に戻す）

---

### ゲームフロー関連

#### ScoreEvent
**発生条件**: 得点が入った

| Field | Type | Description |
|-------|------|-------------|
| scoring_player | Entity | 得点したプレイヤー |
| points | i32 | 得点 |
| reason | ScoreReason | 得点理由 |

**ScoreReason 列挙型**:
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScoreReason {
    BallOut,       // ボールがアウト
    ServiceError,  // サーブミス
    Winner,        // ウィナーショット
}

#[derive(Event)]
pub struct ScoreEvent {
    pub scoring_player: Entity,
    pub points: i32,
    pub reason: ScoreReason,
}
```

**発行者**: score_system
**購読者**: ui_manager, game_flow_system

---

### 入力関連

#### InputEvent
**発生条件**: プレイヤー入力が発生した

| Field | Type | Description |
|-------|------|-------------|
| player_id | Entity | 入力したプレイヤー |
| action | InputAction | 入力アクション |
| value | Vec2 | 入力値（移動方向等） |

**InputAction 列挙型**:
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputAction {
    Move,   // 移動（左右、奥行き）
    Jump,   // ジャンプ
    Shot,   // ショット
    Dash,   // ダッシュ（将来実装）
}

#[derive(Event)]
pub struct InputEvent {
    pub player_id: Entity,
    pub action: InputAction,
    pub value: Vec2,
}
```

**発行者**: input_system
**購読者**: movement_system, shot_system

---

## 発行・購読パターン

### 発行側（EventWriter）

**例**: wall_reflection_system が WallHitEvent を発行

```rust
pub fn wall_reflection_system(
    config: Res<GameConfig>,
    mut events: EventWriter<WallHitEvent>,
    mut query: Query<(Entity, &Position, &mut Velocity, &WallCollider)>,
) {
    for (entity, pos, mut vel, collider) in &mut query {
        // 壁衝突検出
        if let Some((wall_type, hit_point)) = detect_wall_collision(pos, &config) {
            // 反射処理
            reflect_velocity(&mut vel, wall_type, collider.bounce_factor);

            // イベント発行
            events.write(WallHitEvent {
                entity_id: entity,
                wall_type,
                hit_point,
                reflected_velocity: Vec3::new(vel.x, vel.y, vel.z),
            });
        }
    }
}
```

---

### 購読側（EventReader）

**例**: sound_effect_system が WallHitEvent を購読

```rust
pub fn sound_effect_system(
    mut events: EventReader<WallHitEvent>,
    audio: Res<Audio>,
    sounds: Res<SoundAssets>,
) {
    for event in events.read() {
        // 壁の種類に応じて効果音を再生
        let sound = match event.wall_type {
            WallType::Ceiling => &sounds.ceiling_hit,
            WallType::Left | WallType::Right => &sounds.side_wall_hit,
            _ => &sounds.wall_hit,
        };
        audio.play(sound.clone());
    }
}
```

---

## イベントフロー例

### ボールがキャラクターに当たる

```
1. character_collision_system（EventWriter）
   → 衝突検出
   → BallHitEvent 発行

2. knockback_system（EventReader）
   → BallHitEvent を受信
   → KnockbackState を追加
   → KnockbackStartEvent を発行

3. animation_system（EventReader）
   → KnockbackStartEvent を受信
   → ふっとばしアニメーション開始

4. sound_effect_system（EventReader）
   → BallHitEvent を受信
   → 衝突効果音を再生
```

---

## 設計原則

### 1. イベントは「事実」を伝える
```rust
// ✅ 良い例（何が起きたかを伝える）
#[derive(Event)]
pub struct BallHitEvent {
    pub ball_id: Entity,
    pub target_id: Entity,
    pub hit_point: Vec3,
}

// ❌ 悪い例（命令を含む）
// ApplyKnockbackCommand のような命令型イベントは避ける
```

### 2. イベントは最小限の情報のみ含む
```rust
// ✅ 良い例（必要な情報のみ）
#[derive(Event)]
pub struct WallHitEvent {
    pub entity_id: Entity,
    pub wall_type: WallType,
}

// ❌ 悪い例（不要な情報を含む）
// court_width, player_name などは含めない
```

### 3. System 間の通信は Event 経由
```rust
// ✅ 良い例（疎結合）
pub fn sound_effect_system(mut events: EventReader<WallHitEvent>) {
    for event in events.read() {
        // イベントを処理
    }
}

// ❌ 悪い例（密結合）
// wall_reflection_system への直接参照は禁止
```

---

## Bevy Event の特性

### 自動バッファリング

Bevy の Event システムは自動的にフレーム間でバッファリングされます。

```rust
// Bevy は同一フレーム内のイベントを自動的にバッファリング
// EventReader は次のフレームでまとめて読み取り可能
```

### イベント登録

```rust
impl Plugin for GameEventsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BallHitEvent>()
            .add_event::<WallHitEvent>()
            .add_event::<KnockbackStartEvent>()
            .add_event::<KnockbackEndEvent>()
            .add_event::<ScoreEvent>()
            .add_event::<InputEvent>();
    }
}
```

### 相打ち対策

**Bevy での対応**: 同一フレーム内の全イベントが EventReader で読み取れるため、自然に公平な処理が可能。

```rust
pub fn knockback_system(
    mut commands: Commands,
    mut events: EventReader<BallHitEvent>,
    config: Res<GameConfig>,
    mut query: Query<(Entity, &mut Velocity, Option<&Invincible>)>,
) {
    for event in events.read() {
        // 既に無敵状態ならスキップ
        if let Ok((_, _, Some(_))) = query.get(event.target_id) {
            continue;
        }

        // ふっとばし処理
        if let Ok((entity, mut velocity, _)) = query.get_mut(event.target_id) {
            let direction = calculate_direction(event.ball_velocity);
            let speed = calculate_speed(event.ball_velocity);

            // KnockbackState コンポーネントを追加
            commands.entity(entity).insert(KnockbackState {
                direction,
                speed,
                duration: config.knockback.duration,
                invincibility_time: config.knockback.invincibility_time,
            });

            // 無敵時間付与
            commands.entity(entity).insert(Invincible {
                remaining_time: config.knockback.invincibility_time,
            });
        }
    }
}
```

---

## 次のステップ

1. ✅ イベント定義（このドキュメント）
2. ✅ イベント処理のタイミング（Bevy 自動バッファリング）
3. ⏳ InputSystem の設計（`20006_input_system.md`）
4. ⏳ 各 System でのイベント購読実装

## 参考資料

- [20000_overview.md](20000_overview.md) - アーキテクチャ概要
- [20004_ecs_overview.md](20004_ecs_overview.md) - ECS 詳細
- [20006_input_system.md](20006_input_system.md) - 入力システム
- [../../docs/reference/design-decisions.md](../../docs/reference/design-decisions.md#イベント処理のタイミングバッファリング) - フレームワーク設計判断
