# Game Constants

**Version**: 2.0.0
**Last Updated**: 2026-01-06
**Status**: Active

## 概要

ゲーム全体の調整可能なパラメータ定義。**全ての値はハードコーディング禁止**。

## GameConfig 構造

```rust
#[derive(Resource, Deserialize, Clone, Debug)]
pub struct GameConfig {
    pub physics: PhysicsConfig,
    pub court: CourtConfig,
    pub player: PlayerConfig,
    pub ball: BallConfig,
    pub collision: CollisionConfig,
    pub knockback: KnockbackConfig,
    pub shot: ShotConfig,
    pub scoring: ScoringConfig,
    pub input: InputConfig,
}
```

---

## Physics Config

物理演算パラメータ

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| gravity | f32 | -9.8 | 重力加速度（m/s²） |
| max_fall_speed | f32 | -20.0 | 最大落下速度（m/s） |

```rust
#[derive(Deserialize, Clone, Debug)]
pub struct PhysicsConfig {
    #[serde(default = "default_gravity")]
    pub gravity: f32,
    #[serde(default = "default_max_fall_speed")]
    pub max_fall_speed: f32,
}

fn default_gravity() -> f32 { -9.8 }
fn default_max_fall_speed() -> f32 { -20.0 }
```

**使用例**:
```rust
velocity.y += config.physics.gravity * time.delta_secs();
```

---

## Court Config

コートサイズ・範囲

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| width | f32 | 10.0 | コート幅（m） |
| depth | f32 | 6.0 | コート奥行き（m） |
| ceiling_height | f32 | 5.0 | 天井高さ（m） |
| max_jump_height | f32 | 5.0 | 最大ジャンプ高さ（m） |
| net_height | f32 | 1.0 | ネット高さ（m） |
| net_z | f32 | 0.0 | ネットのZ座標位置（m） |

```rust
#[derive(Deserialize, Clone, Debug)]
pub struct CourtConfig {
    #[serde(default = "default_court_width")]
    pub width: f32,
    #[serde(default = "default_court_depth")]
    pub depth: f32,
    #[serde(default = "default_ceiling_height")]
    pub ceiling_height: f32,
    #[serde(default = "default_max_jump_height")]
    pub max_jump_height: f32,
    #[serde(default = "default_net_height")]
    pub net_height: f32,
    #[serde(default = "default_net_z")]
    pub net_z: f32,
}

fn default_court_width() -> f32 { 10.0 }
fn default_court_depth() -> f32 { 6.0 }
fn default_ceiling_height() -> f32 { 5.0 }
fn default_max_jump_height() -> f32 { 5.0 }
fn default_net_height() -> f32 { 1.0 }
fn default_net_z() -> f32 { 0.0 }
```

**使用例**:
```rust
if pos.x < -config.court.width / 2.0 { ... }
if ball_pos.y < config.court.net_height && ball_pos.z == config.court.net_z { ... }
```

---

## Player Config

プレイヤー移動パラメータ

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| move_speed | f32 | 5.0 | 移動速度（m/s） |
| move_speed_z | f32 | 4.0 | 奥行き移動速度（m/s） |
| max_speed | f32 | 10.0 | 最大水平速度（m/s） |
| jump_force | f32 | 8.0 | ジャンプ初速（m/s、上向き） |
| friction | f32 | 0.9 | 地上摩擦係数（0.0～1.0） |
| air_control_factor | f32 | 0.5 | 空中制御係数（0.0～1.0） |
| z_min | f32 | -3.0 | Z軸最小値（m） |
| z_max | f32 | 3.0 | Z軸最大値（m） |

```rust
#[derive(Deserialize, Clone, Debug)]
pub struct PlayerConfig {
    #[serde(default = "default_move_speed")]
    pub move_speed: f32,
    #[serde(default = "default_move_speed_z")]
    pub move_speed_z: f32,
    #[serde(default = "default_max_speed")]
    pub max_speed: f32,
    #[serde(default = "default_jump_force")]
    pub jump_force: f32,
    #[serde(default = "default_friction")]
    pub friction: f32,
    #[serde(default = "default_air_control")]
    pub air_control_factor: f32,
    #[serde(default = "default_z_min")]
    pub z_min: f32,
    #[serde(default = "default_z_max")]
    pub z_max: f32,
}

fn default_move_speed() -> f32 { 5.0 }
fn default_move_speed_z() -> f32 { 4.0 }
fn default_max_speed() -> f32 { 10.0 }
fn default_jump_force() -> f32 { 8.0 }
fn default_friction() -> f32 { 0.9 }
fn default_air_control() -> f32 { 0.5 }
fn default_z_min() -> f32 { -3.0 }
fn default_z_max() -> f32 { 3.0 }
```

**使用例**:
```rust
velocity.x = input_x * config.player.move_speed;
velocity.y = config.player.jump_force;  // ジャンプ時
if velocity.length() > config.player.max_speed { ... }
if pos.z < config.player.z_min { pos.z = config.player.z_min; }
```

---

## Ball Config

ボールパラメータ

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| normal_shot_speed | f32 | 10.0 | 通常ショット速度（m/s） |
| power_shot_speed | f32 | 15.0 | 強打速度（m/s） |
| bounce_factor | f32 | 0.8 | 壁反射時の減衰係数（0.0～1.0） |
| radius | f32 | 0.2 | ボール半径（m） |

```rust
#[derive(Deserialize, Clone, Debug)]
pub struct BallConfig {
    #[serde(default = "default_normal_shot_speed")]
    pub normal_shot_speed: f32,
    #[serde(default = "default_power_shot_speed")]
    pub power_shot_speed: f32,
    #[serde(default = "default_bounce_factor")]
    pub bounce_factor: f32,
    #[serde(default = "default_ball_radius")]
    pub radius: f32,
}

fn default_normal_shot_speed() -> f32 { 10.0 }
fn default_power_shot_speed() -> f32 { 15.0 }
fn default_bounce_factor() -> f32 { 0.8 }
fn default_ball_radius() -> f32 { 0.2 }
```

**使用例**:
```rust
velocity.x = -velocity.x * config.ball.bounce_factor;  // 壁反射
```

---

## Collision Config

当たり判定パラメータ

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| character_radius | f32 | 0.5 | キャラクター半径（m） |
| z_tolerance | f32 | 0.3 | Z軸衝突許容範囲（m） |

```rust
#[derive(Deserialize, Clone, Debug)]
pub struct CollisionConfig {
    #[serde(default = "default_character_radius")]
    pub character_radius: f32,
    #[serde(default = "default_z_tolerance")]
    pub z_tolerance: f32,
}

fn default_character_radius() -> f32 { 0.5 }
fn default_z_tolerance() -> f32 { 0.3 }
```

**使用例**:
```rust
let distance = distance_2d(ball_pos, char_pos);
let z_diff = (ball_pos.z - char_pos.z).abs();
if distance < (config.ball.radius + config.collision.character_radius) &&
   z_diff < config.collision.z_tolerance {
    // 衝突
}
```

---

## Knockback Config

ふっとばしパラメータ

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| duration | f32 | 0.5 | ふっとばし時間（秒） |
| speed_multiplier | f32 | 0.5 | ボール速度に対する倍率 |
| invincibility_time | f32 | 1.0 | 無敵時間（秒） |

```rust
#[derive(Deserialize, Clone, Debug)]
pub struct KnockbackConfig {
    #[serde(default = "default_knockback_duration")]
    pub duration: f32,
    #[serde(default = "default_speed_multiplier")]
    pub speed_multiplier: f32,
    #[serde(default = "default_invincibility_time")]
    pub invincibility_time: f32,
}

fn default_knockback_duration() -> f32 { 0.5 }
fn default_speed_multiplier() -> f32 { 0.5 }
fn default_invincibility_time() -> f32 { 1.0 }
```

**使用例**:
```rust
let knockback_speed = ball_velocity.length() * config.knockback.speed_multiplier;
commands.entity(entity).insert(KnockbackState {
    speed: knockback_speed,
    duration: config.knockback.duration,
    invincibility_time: config.knockback.invincibility_time,
    ..default()
});
```

---

## Shot Config

ショットシステムパラメータ

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| max_distance | f32 | 1.5 | ショット可能な最大距離（m） |
| max_height_diff | f32 | 2.0 | ショット可能な最大高さ差（m） |
| cooldown_time | f32 | 0.5 | ショット後のクールダウン時間（秒） |
| normal_shot_angle | f32 | 45.0 | 通常ショットの打球角度（度） |
| jump_shot_angle | f32 | 30.0 | ジャンプショットの打球角度（度） |
| jump_threshold | f32 | 0.5 | ジャンプショット判定の高さ閾値（m） |

```rust
#[derive(Deserialize, Clone, Debug)]
pub struct ShotConfig {
    #[serde(default = "default_max_distance")]
    pub max_distance: f32,
    #[serde(default = "default_max_height_diff")]
    pub max_height_diff: f32,
    #[serde(default = "default_cooldown_time")]
    pub cooldown_time: f32,
    #[serde(default = "default_normal_shot_angle")]
    pub normal_shot_angle: f32,
    #[serde(default = "default_jump_shot_angle")]
    pub jump_shot_angle: f32,
    #[serde(default = "default_jump_threshold")]
    pub jump_threshold: f32,
}

fn default_max_distance() -> f32 { 1.5 }
fn default_max_height_diff() -> f32 { 2.0 }
fn default_cooldown_time() -> f32 { 0.5 }
fn default_normal_shot_angle() -> f32 { 45.0 }
fn default_jump_shot_angle() -> f32 { 30.0 }
fn default_jump_threshold() -> f32 { 0.5 }
```

**使用例**:
```rust
let distance = distance_2d(player_pos, ball_pos);
if distance <= config.shot.max_distance {
    // ショット可能
}

let is_jump_shot = player_pos.y > config.shot.jump_threshold;
let angle = if is_jump_shot {
    config.shot.jump_shot_angle
} else {
    config.shot.normal_shot_angle
};
```

---

## Input Config

入力パラメータ

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| jump_buffer_time | f32 | 0.1 | ジャンプバッファ時間（秒） |
| shot_buffer_time | f32 | 0.05 | ショットバッファ時間（秒） |

```rust
#[derive(Deserialize, Clone, Debug)]
pub struct InputConfig {
    #[serde(default = "default_jump_buffer_time")]
    pub jump_buffer_time: f32,
    #[serde(default = "default_shot_buffer_time")]
    pub shot_buffer_time: f32,
}

fn default_jump_buffer_time() -> f32 { 0.1 }
fn default_shot_buffer_time() -> f32 { 0.05 }
```

---

## Scoring Config

スコアリングパラメータ

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| game_point | i32 | 4 | ゲーム獲得に必要なポイント数 |
| set_game | i32 | 6 | セット獲得に必要なゲーム数 |
| match_set | i32 | 1 | マッチ勝利に必要なセット数（MVP v0.1） |

```rust
#[derive(Deserialize, Clone, Debug)]
pub struct ScoringConfig {
    #[serde(default = "default_game_point")]
    pub game_point: i32,
    #[serde(default = "default_set_game")]
    pub set_game: i32,
    #[serde(default = "default_match_set")]
    pub match_set: i32,
}

fn default_game_point() -> i32 { 4 }
fn default_set_game() -> i32 { 6 }
fn default_match_set() -> i32 { 1 }
```

**使用例**:
```rust
if score_state.point[player_id] >= config.scoring.game_point {
    // ゲーム獲得
}

if score_state.game[player_id] >= config.scoring.set_game {
    // セット獲得
}

if score_state.set[player_id] >= config.scoring.match_set {
    // マッチ終了
}
```

---

## 実装ファイル

### RON ファイル（Rust Object Notation）

**ファイル**: `assets/config/game_config.ron`

```ron
GameConfig(
    physics: PhysicsConfig(
        gravity: -9.8,
        max_fall_speed: -20.0,
    ),
    court: CourtConfig(
        width: 10.0,
        depth: 6.0,
        ceiling_height: 5.0,
        max_jump_height: 5.0,
        net_height: 1.0,
        net_z: 0.0,
    ),
    player: PlayerConfig(
        move_speed: 5.0,
        move_speed_z: 4.0,
        max_speed: 10.0,
        jump_force: 8.0,
        friction: 0.9,
        air_control_factor: 0.5,
        z_min: -3.0,
        z_max: 3.0,
    ),
    ball: BallConfig(
        normal_shot_speed: 10.0,
        power_shot_speed: 15.0,
        bounce_factor: 0.8,
        radius: 0.2,
    ),
    collision: CollisionConfig(
        character_radius: 0.5,
        z_tolerance: 0.3,
    ),
    knockback: KnockbackConfig(
        duration: 0.5,
        speed_multiplier: 0.5,
        invincibility_time: 1.0,
    ),
    shot: ShotConfig(
        max_distance: 1.5,
        max_height_diff: 2.0,
        cooldown_time: 0.5,
        normal_shot_angle: 45.0,
        jump_shot_angle: 30.0,
        jump_threshold: 0.5,
    ),
    input: InputConfig(
        jump_buffer_time: 0.1,
        shot_buffer_time: 0.05,
    ),
    scoring: ScoringConfig(
        game_point: 4,
        set_game: 6,
        match_set: 1,
    ),
)
```

### ロード方法

```rust
use bevy::prelude::*;
use serde::Deserialize;

// 起動時にロード
fn load_config(mut commands: Commands) {
    let config_str = std::fs::read_to_string("assets/config/game_config.ron")
        .expect("Failed to read config file");
    let config: GameConfig = ron::from_str(&config_str)
        .expect("Failed to parse config");
    commands.insert_resource(config);
}

// または Bevy Asset System を使用
#[derive(Asset, TypePath, Deserialize)]
pub struct GameConfigAsset(pub GameConfig);

fn setup_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<GameConfigAsset> = asset_server.load("config/game_config.ron");
    // ...
}
```

---

## 環境別設定

開発・テスト・本番で異なる値を使用する場合：

```
assets/config/
├── game_config.ron           # デフォルト（本番用）
├── game_config_dev.ron       # 開発用（デバッグ値）
└── game_config_test.ron      # テスト用（自動テスト向け）
```

```rust
fn load_config() -> GameConfig {
    let config_path = if cfg!(debug_assertions) {
        "assets/config/game_config_dev.ron"
    } else {
        "assets/config/game_config.ron"
    };

    let config_str = std::fs::read_to_string(config_path)
        .expect("Failed to read config file");
    ron::from_str(&config_str).expect("Failed to parse config")
}
```

---

## Visual Feedback Config

視覚フィードバックパラメータ

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| hold_color | (f32, f32, f32, f32) | (1.0, 0.5, 0.0, 1.0) | ホールド中の色（RGBA） |
| ball_color_topspin | (f32, f32, f32, f32) | (1.0, 0.2, 0.2, 1.0) | トップスピン時の色（赤系） |
| ball_color_neutral | (f32, f32, f32, f32) | (0.9, 0.9, 0.2, 1.0) | ニュートラル時の色（黄色） |
| ball_color_slice | (f32, f32, f32, f32) | (0.2, 0.4, 1.0, 1.0) | スライス時の色（青系） |

```rust
#[derive(Deserialize, Clone, Debug)]
pub struct VisualFeedbackConfig {
    /// ホールド中のプレイヤー色（RGBA）
    #[serde(default = "default_hold_color")]
    pub hold_color: (f32, f32, f32, f32),
    /// トップスピン時のボール色（RGBA）
    #[serde(default = "default_ball_color_topspin")]
    pub ball_color_topspin: (f32, f32, f32, f32),
    /// ニュートラル時のボール色（RGBA）
    #[serde(default = "default_ball_color_neutral")]
    pub ball_color_neutral: (f32, f32, f32, f32),
    /// スライス時のボール色（RGBA）
    #[serde(default = "default_ball_color_slice")]
    pub ball_color_slice: (f32, f32, f32, f32),
}

fn default_hold_color() -> (f32, f32, f32, f32) { (1.0, 0.5, 0.0, 1.0) }
fn default_ball_color_topspin() -> (f32, f32, f32, f32) { (1.0, 0.2, 0.2, 1.0) }
fn default_ball_color_neutral() -> (f32, f32, f32, f32) { (0.9, 0.9, 0.2, 1.0) }
fn default_ball_color_slice() -> (f32, f32, f32, f32) { (0.2, 0.4, 1.0, 1.0) }
```

**使用例**:
```rust
// ホールド中の色
let hold_color = Color::srgba(
    config.visual_feedback.hold_color.0,
    config.visual_feedback.hold_color.1,
    config.visual_feedback.hold_color.2,
    config.visual_feedback.hold_color.3,
);

// スピンに応じたボール色
let ball_color = lerp_color(
    config.visual_feedback.ball_color_slice,
    config.visual_feedback.ball_color_neutral,
    config.visual_feedback.ball_color_topspin,
    spin_value, // -1.0 ~ +1.0
);
```

---

## 次のステップ

1. ✅ データ定義（このドキュメント）
2. ⏳ GameConfig struct の実装（Rust）
3. ⏳ RON ファイルの作成（`.ron`）
4. ⏳ 各 System への `Res<GameConfig>` 依存注入

## 参考資料

- [20000_overview.md](../2_architecture/20000_overview.md) - アーキテクチャ概要
- [20001_layers.md](../2_architecture/20001_layers.md) - Resource Layer
- [.claude/CLAUDE.md](../../.claude/CLAUDE.md) - ハードコーディング禁止原則
