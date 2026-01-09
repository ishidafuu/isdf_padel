# Game Constants

**Version**: 3.4.0
**Last Updated**: 2026-01-09
**Status**: Active

## 概要

ゲーム全体の調整可能なパラメータ定義。**全ての値はハードコーディング禁止**。

> **Note**: v3.0.0でパデルからテニスへのルール変更に伴い、壁・天井関連のパラメータは廃止されました。
> 詳細は Change Log を参照してください。

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
    pub serve: ServeConfig,
    pub ai: AiConfig,
    pub scoring: ScoringConfig,
    pub input: InputConfig,
    pub input_keys: InputKeysConfig,
    pub gamepad_buttons: GamepadButtonsConfig,
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

コートサイズ・範囲（テニス用オープンコート）

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| width | f32 | 12.0 | コート幅（m）- サイドライン境界 |
| depth | f32 | 16.0 | コート奥行き（m）- ベースライン境界（各コート側8.0m） |
| ceiling_height | f32 | 100.0 | 天井高さ（m）- 事実上無効（テニスでは天井なし） |
| max_jump_height | f32 | 5.0 | 最大ジャンプ高さ（m）- ジャンプ制限用 |
| net_height | f32 | 0.88 | ネット高さ（m）- テニス規格 |
| net_z | f32 | 0.0 | ネットのZ座標位置（m） |

> **変更 (v3.0.0)**: `ceiling_height` は壁システム廃止により不要。`max_jump_height` はジャンプ制限の目安として残存。

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

fn default_court_width() -> f32 { 12.0 }
fn default_court_depth() -> f32 { 16.0 }
fn default_ceiling_height() -> f32 { 100.0 }
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
| z_min | f32 | -8.0 | Z軸最小値（m） |
| z_max | f32 | 8.0 | Z軸最大値（m） |

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
fn default_z_min() -> f32 { -8.0 }
fn default_z_max() -> f32 { 8.0 }
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

## Serve Config

サーブパラメータ

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| ball_spawn_offset_y | f32 | 2.0 | オーバーハンドサーブの打点高さオフセット（m） |
| serve_speed | f32 | 10.0 | サーブ速度（m/s） |
| serve_angle | f32 | -15.0 | サーブ角度（度、負の値=下向き） |
| p1_default_direction_x | f32 | 1.0 | Player1のデフォルトサーブ方向X成分 |
| p2_default_direction_x | f32 | -1.0 | Player2のデフォルトサーブ方向X成分 |
| toss_start_offset_y | f32 | 1.0 | トスボール生成高さ（手元位置） |
| toss_velocity_y | f32 | 3.5 | トス上向き初速度（m/s） |
| toss_timeout | f32 | 3.0 | トス失敗までの時間（秒） |
| hit_height_min | f32 | 1.8 | ヒット可能最低高さ（m） |
| hit_height_max | f32 | 2.7 | ヒット可能最高高さ（m） |
| hit_height_optimal | f32 | 2.2 | AI用ヒット最適高さ（m） |
| ai_hit_tolerance | f32 | 0.1 | AI用ヒット許容範囲（m） |
| serve_baseline_x_p1 | f32 | -7.0 | Player1のベースライン位置 |
| serve_baseline_x_p2 | f32 | 7.0 | Player2のベースライン位置 |

```rust
/// サーブ設定
/// @spec 30102_serve_spec.md#req-30102-060
/// @spec 30102_serve_spec.md#req-30102-080
#[derive(Deserialize, Clone, Debug)]
pub struct ServeConfig {
    /// オーバーハンドサーブの打点高さオフセット（m）
    #[serde(default = "default_ball_spawn_offset_y")]
    pub ball_spawn_offset_y: f32,
    /// サーブ速度（m/s）
    #[serde(default = "default_serve_speed")]
    pub serve_speed: f32,
    /// サーブ角度（度）
    #[serde(default = "default_serve_angle")]
    pub serve_angle: f32,
    /// Player1のデフォルトサーブ方向X成分
    #[serde(default = "default_p1_direction_x")]
    pub p1_default_direction_x: f32,
    /// Player2のデフォルトサーブ方向X成分
    #[serde(default = "default_p2_direction_x")]
    pub p2_default_direction_x: f32,
    /// トスボール生成高さ（手元位置）
    #[serde(default = "default_toss_start_offset_y")]
    pub toss_start_offset_y: f32,
    /// トス上向き初速度（m/s）
    #[serde(default = "default_toss_velocity_y")]
    pub toss_velocity_y: f32,
    /// トス失敗までの時間（秒）
    #[serde(default = "default_toss_timeout")]
    pub toss_timeout: f32,
    /// ヒット可能最低高さ（m）
    #[serde(default = "default_hit_height_min")]
    pub hit_height_min: f32,
    /// ヒット可能最高高さ（m）
    #[serde(default = "default_hit_height_max")]
    pub hit_height_max: f32,
    /// AI用ヒット最適高さ（m）
    #[serde(default = "default_hit_height_optimal")]
    pub hit_height_optimal: f32,
    /// AI用ヒット許容範囲（m）
    #[serde(default = "default_ai_hit_tolerance")]
    pub ai_hit_tolerance: f32,
    /// Player1のベースライン位置
    #[serde(default = "default_serve_baseline_x_p1")]
    pub serve_baseline_x_p1: f32,
    /// Player2のベースライン位置
    #[serde(default = "default_serve_baseline_x_p2")]
    pub serve_baseline_x_p2: f32,
}

fn default_ball_spawn_offset_y() -> f32 { 2.0 }
fn default_serve_speed() -> f32 { 10.0 }
fn default_serve_angle() -> f32 { -15.0 }
fn default_p1_direction_x() -> f32 { 1.0 }
fn default_p2_direction_x() -> f32 { -1.0 }
fn default_toss_start_offset_y() -> f32 { 1.0 }
fn default_toss_velocity_y() -> f32 { 3.5 }
fn default_toss_timeout() -> f32 { 3.0 }
fn default_hit_height_min() -> f32 { 1.8 }
fn default_hit_height_max() -> f32 { 2.7 }
fn default_hit_height_optimal() -> f32 { 2.2 }
fn default_ai_hit_tolerance() -> f32 { 0.1 }
fn default_serve_baseline_x_p1() -> f32 { -7.0 }
fn default_serve_baseline_x_p2() -> f32 { 7.0 }
```

**使用例**:
```rust
// オーバーハンドサーブのボール生成位置
let ball_pos = player_pos + Vec3::new(0.0, config.serve.ball_spawn_offset_y, 0.0);

// サーブ速度と角度
let speed = config.serve.serve_speed;
let angle_rad = config.serve.serve_angle.to_radians();
let velocity = Vec3::new(
    direction.x * speed * angle_rad.cos(),
    speed * angle_rad.sin(),
    direction.z * speed * angle_rad.cos(),
);

// トス開始
let toss_pos = player_pos + Vec3::new(0.0, config.serve.toss_start_offset_y, 0.0);
let toss_vel = Vec3::new(0.0, config.serve.toss_velocity_y, 0.0);

// ヒット可能判定
let can_hit = ball_pos.y >= config.serve.hit_height_min
           && ball_pos.y <= config.serve.hit_height_max;
```

---

## AI Config

AI動作パラメータ

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| move_speed | f32 | 4.5 | AI移動速度（m/s） |
| home_position_x | f32 | 7.0 | ホームポジションX座標 |
| shot_cooldown | f32 | 0.5 | ショットクールダウン時間（秒） |
| home_return_stop_distance | f32 | 0.3 | ホーム帰還停止距離（m） |
| serve_delay_min | f32 | 0.5 | AIサーブまでの待機時間下限（秒） |
| serve_delay_max | f32 | 1.5 | AIサーブまでの待機時間上限（秒） |
| serve_direction_variance | f32 | 0.5 | AIサーブ方向バリエーション（Z軸） |

```rust
/// AI設定
/// @spec 30102_serve_spec.md#req-30102-070
/// @spec 30302_ai_shot_spec.md
#[derive(Deserialize, Clone, Debug)]
pub struct AiConfig {
    /// AI移動速度（m/s）
    #[serde(default = "default_ai_move_speed")]
    pub move_speed: f32,
    /// ホームポジションX座標
    #[serde(default = "default_home_position_x")]
    pub home_position_x: f32,
    /// ショットクールダウン時間（秒）
    #[serde(default = "default_ai_shot_cooldown")]
    pub shot_cooldown: f32,
    /// ホーム帰還停止距離（m）
    #[serde(default = "default_home_return_stop_distance")]
    pub home_return_stop_distance: f32,
    /// AIサーブまでの待機時間下限（秒）
    #[serde(default = "default_serve_delay_min")]
    pub serve_delay_min: f32,
    /// AIサーブまでの待機時間上限（秒）
    #[serde(default = "default_serve_delay_max")]
    pub serve_delay_max: f32,
    /// AIサーブ方向バリエーション（Z軸）
    #[serde(default = "default_serve_direction_variance")]
    pub serve_direction_variance: f32,
}

fn default_ai_move_speed() -> f32 { 4.5 }
fn default_home_position_x() -> f32 { 7.0 }
fn default_ai_shot_cooldown() -> f32 { 0.5 }
fn default_home_return_stop_distance() -> f32 { 0.3 }
fn default_serve_delay_min() -> f32 { 0.5 }
fn default_serve_delay_max() -> f32 { 1.5 }
fn default_serve_direction_variance() -> f32 { 0.5 }
```

**使用例**:
```rust
// AIサーブタイマー設定
let delay = rng.random_range(config.ai.serve_delay_min..config.ai.serve_delay_max);

// サーブ方向ランダム化
let z_variance = rng.random_range(-config.ai.serve_direction_variance..config.ai.serve_direction_variance);
let direction = Vec2::new(config.serve.p2_default_direction_x, z_variance);
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

## Input Keys Config

入力キーバインド設定

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| move_up | KeyCode | KeyW | 上移動キー |
| move_down | KeyCode | KeyS | 下移動キー |
| move_left | KeyCode | KeyA | 左移動キー |
| move_right | KeyCode | KeyD | 右移動キー |
| move_up_alt | KeyCode | ArrowUp | 上移動キー（代替） |
| move_down_alt | KeyCode | ArrowDown | 下移動キー（代替） |
| move_left_alt | KeyCode | ArrowLeft | 左移動キー（代替） |
| move_right_alt | KeyCode | ArrowRight | 右移動キー（代替） |
| jump | KeyCode | KeyB | ジャンプキー |
| shot | KeyCode | KeyV | ショットキー |

```rust
#[derive(Deserialize, Clone, Debug)]
pub struct InputKeysConfig {
    #[serde(default = "default_key_move_up")]
    pub move_up: KeyCode,
    #[serde(default = "default_key_move_down")]
    pub move_down: KeyCode,
    #[serde(default = "default_key_move_left")]
    pub move_left: KeyCode,
    #[serde(default = "default_key_move_right")]
    pub move_right: KeyCode,
    #[serde(default = "default_key_move_up_alt")]
    pub move_up_alt: KeyCode,
    #[serde(default = "default_key_move_down_alt")]
    pub move_down_alt: KeyCode,
    #[serde(default = "default_key_move_left_alt")]
    pub move_left_alt: KeyCode,
    #[serde(default = "default_key_move_right_alt")]
    pub move_right_alt: KeyCode,
    #[serde(default = "default_key_jump")]
    pub jump: KeyCode,
    #[serde(default = "default_key_shot")]
    pub shot: KeyCode,
}

fn default_key_move_up() -> KeyCode { KeyCode::KeyW }
fn default_key_move_down() -> KeyCode { KeyCode::KeyS }
fn default_key_move_left() -> KeyCode { KeyCode::KeyA }
fn default_key_move_right() -> KeyCode { KeyCode::KeyD }
fn default_key_move_up_alt() -> KeyCode { KeyCode::ArrowUp }
fn default_key_move_down_alt() -> KeyCode { KeyCode::ArrowDown }
fn default_key_move_left_alt() -> KeyCode { KeyCode::ArrowLeft }
fn default_key_move_right_alt() -> KeyCode { KeyCode::ArrowRight }
fn default_key_jump() -> KeyCode { KeyCode::KeyB }
fn default_key_shot() -> KeyCode { KeyCode::KeyV }
```

**使用例**:
```rust
let keys = &config.input_keys;
if keyboard.pressed(keys.move_up) || keyboard.pressed(keys.move_up_alt) {
    movement.x += 1.0;
}
```

**備考**: RON ファイルでの設定はオプション。省略時はデフォルト値が適用される。

---

## Gamepad Buttons Config

ゲームパッドボタン設定（v0.4追加）

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| jump | GamepadButton | South | ジャンプボタン（Xbox: A, PS: ×） |
| shot | GamepadButton | East | ショットボタン（Xbox: B, PS: ○） |
| stick_deadzone | f32 | 0.1 | スティックデッドゾーン |

```rust
/// ゲームパッドボタン設定
/// @spec 20006_input_system.md#req-20006-053
#[derive(Deserialize, Clone, Debug)]
pub struct GamepadButtonsConfig {
    /// ジャンプボタン（デフォルト: South = A on Xbox, × on PlayStation）
    #[serde(default = "default_gamepad_jump")]
    pub jump: GamepadButton,
    /// ショットボタン（デフォルト: East = B on Xbox, ○ on PlayStation）
    #[serde(default = "default_gamepad_shot")]
    pub shot: GamepadButton,
    /// スティックデッドゾーン（入力が無視される範囲）
    #[serde(default = "default_stick_deadzone")]
    pub stick_deadzone: f32,
}

fn default_gamepad_jump() -> GamepadButton { GamepadButton::South }
fn default_gamepad_shot() -> GamepadButton { GamepadButton::East }
fn default_stick_deadzone() -> f32 { 0.1 }
```

**使用例**:
```rust
// ゲームパッド入力システム
if gamepad.just_pressed(config.gamepad_buttons.jump) {
    input_state.jump_pressed = true;
}

// デッドゾーン適用
let stick = gamepad.left_stick();
if stick.length() < config.gamepad_buttons.stick_deadzone {
    movement = Vec2::ZERO;
}
```

**RONファイル追加**:
```ron
gamepad_buttons: GamepadButtonsConfig(
    jump: South,
    shot: East,
    stick_deadzone: 0.1,
),
```

**関連仕様**:
- [20006_input_system.md](../2_architecture/20006_input_system.md#req-20006-053) - ボタンマッピング設定

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
        width: 12.0,
        depth: 16.0,
        ceiling_height: 100.0,
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
        z_min: -8.0,
        z_max: 8.0,
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
    serve: ServeConfig(
        ball_spawn_offset_y: 2.0,
        serve_speed: 10.0,
        serve_angle: -15.0,
        p1_default_direction_x: 1.0,
        p2_default_direction_x: -1.0,
        toss_start_offset_y: 1.0,
        toss_velocity_y: 3.5,
        toss_timeout: 3.0,
        hit_height_min: 1.8,
        hit_height_max: 2.7,
        hit_height_optimal: 2.2,
        ai_hit_tolerance: 0.1,
        serve_baseline_x_p1: -7.0,
        serve_baseline_x_p2: 7.0,
    ),
    ai: AiConfig(
        move_speed: 4.5,
        home_position_x: 7.0,
        shot_cooldown: 0.5,
        home_return_stop_distance: 0.3,
        serve_delay_min: 0.5,
        serve_delay_max: 1.5,
        serve_direction_variance: 0.5,
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

## Spin Physics Config

スピン物理パラメータ（v0.3追加）

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| gravity_spin_factor | f32 | 0.3 | 重力変動割合（±30%） |
| bounce_spin_horizontal_factor | f32 | 0.3 | 水平バウンド変動割合（±30%） |
| bounce_spin_vertical_factor | f32 | 0.2 | 垂直バウンド変動割合（±20%） |
| base_air_drag | f32 | 0.0 | ベース空気抵抗 |
| spin_drag_factor | f32 | 0.3 | スピンによる追加空気抵抗係数 |
| spin_decay_rate | f32 | 0.5 | スピン時間減衰率（1秒あたり） |

```rust
/// スピン物理パラメータ
/// @data REQ-30401-100, REQ-30401-101, REQ-30401-102, REQ-30402-100
#[derive(Deserialize, Clone, Debug)]
pub struct SpinPhysicsConfig {
    /// 重力に対するスピンの影響度（±30%時 = 0.3）
    #[serde(default = "default_gravity_spin_factor")]
    pub gravity_spin_factor: f32,

    /// バウンド時の水平方向へのスピンの影響度
    #[serde(default = "default_bounce_spin_horizontal_factor")]
    pub bounce_spin_horizontal_factor: f32,

    /// バウンド時の垂直方向へのスピンの影響度
    #[serde(default = "default_bounce_spin_vertical_factor")]
    pub bounce_spin_vertical_factor: f32,

    /// ベース空気抵抗（スピンなしでも適用）
    #[serde(default = "default_base_air_drag")]
    pub base_air_drag: f32,

    /// スピンによる追加空気抵抗係数
    #[serde(default = "default_spin_drag_factor")]
    pub spin_drag_factor: f32,

    /// スピン時間減衰率（1秒あたり）
    #[serde(default = "default_spin_decay_rate")]
    pub spin_decay_rate: f32,
}

fn default_gravity_spin_factor() -> f32 { 0.3 }
fn default_bounce_spin_horizontal_factor() -> f32 { 0.3 }
fn default_bounce_spin_vertical_factor() -> f32 { 0.2 }
fn default_base_air_drag() -> f32 { 0.0 }
fn default_spin_drag_factor() -> f32 { 0.3 }
fn default_spin_decay_rate() -> f32 { 0.5 }
```

**使用例**:
```rust
// 重力変動（REQ-30401-100）
let effective_gravity = config.physics.gravity
    * (1.0 + ball_spin.value * config.spin_physics.gravity_spin_factor);

// スピン時間減衰（REQ-30401-101）
ball_spin.value *= (1.0 - config.spin_physics.spin_decay_rate * delta).max(0.0);

// 空気抵抗（REQ-30401-102）
let drag = config.spin_physics.base_air_drag
    + ball_spin.value.abs() * config.spin_physics.spin_drag_factor;
velocity *= (1.0 - drag * delta).max(0.9);

// バウンド挙動（REQ-30402-100）
let h_factor = config.spin_physics.bounce_spin_horizontal_factor;
let v_factor = config.spin_physics.bounce_spin_vertical_factor;
let h_bounce = config.ball.bounce_factor * (1.0 + ball_spin.value * h_factor);
let v_bounce = config.ball.bounce_factor * (1.0 - ball_spin.value * v_factor);
```

**RONファイル追加**:
```ron
spin_physics: SpinPhysicsConfig(
    gravity_spin_factor: 0.3,
    bounce_spin_horizontal_factor: 0.3,
    bounce_spin_vertical_factor: 0.2,
    base_air_drag: 0.0,
    spin_drag_factor: 0.3,
    spin_decay_rate: 0.5,
),
```

**関連仕様**:
- [30401_trajectory_spec.md](../3_ingame/304_ball/30401_trajectory_spec.md#req-30401-100) - スピンによる重力変動
- [30401_trajectory_spec.md](../3_ingame/304_ball/30401_trajectory_spec.md#req-30401-101) - スピン時間減衰
- [30401_trajectory_spec.md](../3_ingame/304_ball/30401_trajectory_spec.md#req-30401-102) - スピンによる空気抵抗
- [30402_reflection_spec.md](../3_ingame/304_ball/30402_reflection_spec.md#req-30402-100) - スピンによるバウンド挙動変化

---

## 次のステップ

1. ✅ データ定義（このドキュメント）
2. ⏳ GameConfig struct の実装（Rust）
3. ⏳ RON ファイルの作成（`.ron`）
4. ⏳ 各 System への `Res<GameConfig>` 依存注入

---

## Change Log

### 2026-01-09 - v3.4.0

- ServeConfigにトス→ヒット方式パラメータ追加
  - toss_start_offset_y, toss_velocity_y, toss_timeout
  - hit_height_min, hit_height_max, hit_height_optimal, ai_hit_tolerance
  - serve_baseline_x_p1, serve_baseline_x_p2

### 2026-01-09 - v3.3.0

- GamepadButtonsConfig追加（ゲームパッド対応）
- GameConfig構造体にgamepad_buttonsフィールド追加

### 2026-01-09 - v3.2.0

- ServeConfig追加（オーバーハンドサーブ対応）
- AiConfig追加（AIサーブ対応）
- GameConfig構造体にserve, aiフィールド追加

### 2026-01-08 - v3.1.0（コートサイズ調整）

- **Court Config**: テニス用ゲームサイズに調整
  - width: 10.0 → 12.0m
  - depth: 6.0 → 16.0m（各コート側8.0m）
  - ceiling_height: 5.0 → 100.0m（事実上無効）
- **Player Config**: コートサイズに合わせて移動範囲を調整
  - z_min: -3.0 → -8.0m
  - z_max: 3.0 → 8.0m

### 2026-01-08 - v3.0.0（テニスへ変更）

- **Court Config**: 天井高さ（ceiling_height）を廃止マーク
- **概要**: テニス化による壁・天井削除の注記を追加
- **net_height**: 0.88m（テニス規格）に調整
- **Court境界**: サイドライン・ベースラインとしての役割を明記

### 2026-01-06 - v2.0.0

- Visual Feedback Config, Spin Physics Config追加
- Input Keys Config追加

### 2025-12-23 - v1.0.0（初版）

- 初版作成（パデルベース）

---

## 参考資料

- [20000_overview.md](../2_architecture/20000_overview.md) - アーキテクチャ概要
- [20001_layers.md](../2_architecture/20001_layers.md) - Resource Layer
- [.claude/CLAUDE.md](../../../.claude/CLAUDE.md) - ハードコーディング禁止原則
