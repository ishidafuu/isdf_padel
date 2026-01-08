# Input System

**Version**: 4.0.0
**Last Updated**: 2026-01-08
**Status**: Active

## 概要

プラットフォーム非依存の入力抽象化レイヤー。2.5D 移動に対応した入力システム。Bevy の `ButtonInput<KeyCode>` を使用。

**v4.0.0 変更点**: エンティティベースの入力状態管理に移行。プレイヤー固定参照（player1/player2）を廃止。

## 設計方針

### 1. エンティティベースの入力状態（v4.0.0）
- **InputState コンポーネント**: 各プレイヤーエンティティに付与、入力状態を保持
- **制御種別マーカー**: `HumanControlled` / `AiControlled` で人間/AI を区別
- **スケーラビリティ**: ダブルス（4人）、全員AI、全員人間に対応可能

### 2. 抽象化レイヤー
- **Bevy Input → InputState への反映**: 物理キー/ボタンを論理状態に変換
- **入力バッファ**: 先行入力を受け付ける（ジャンプ、コンボ）
- **入力制約**: ふっとばし中は入力を無効化

### 3. 座標系への対応
- **X軸入力**: 方向キー左右 → 左右移動
- **Z軸入力**: 方向キー上下 → 奥行き移動（くにおくん方式）
- **Y軸入力**: Aボタン → ジャンプ（重力と組み合わせ）

---

## 入力アクション定義

| Action | Key/Button | Description | 軸 |
|--------|------------|-------------|---|
| **MoveHorizontal** | Arrow Left/Right, A/D | 左右移動 | X |
| **MoveDepth** | Arrow Up/Down, W/S | 奥行き移動 | Z |
| **Jump** | Space, Gamepad South | ジャンプ | Y |
| **Shot** | E, Gamepad East, Mouse Left | ショット | - |
| **Dash** | Shift（未実装） | ダッシュ | - |

---

## 入力バッファ

先行入力を受け付けることで、操作の快適性を向上。

| Action | Buffer Time | Description |
|--------|-------------|-------------|
| **Jump** | 0.1s | 着地前の先行ジャンプ入力を受け付ける |
| **Shot** | 0.05s | ショット準備中のコンボ入力を受け付ける |

**例**: ジャンプバッファ
```
Player が空中にいる
→ 着地前 0.1s の間に Jump 入力があった
→ 着地と同時に自動的にジャンプ
```

---

## 入力処理フロー

```
1. Bevy ButtonInput<KeyCode> 受信
   ↓
2. human_input_system が HumanControlled エンティティの InputState を更新
   → movement, jump_pressed, shot_pressed 等を設定
   ↓
3. 各処理システムが InputState を参照
   → movement_system: InputState.movement → Velocity
   → jump_system: InputState.jump_pressed → ジャンプ処理
   → shot_input_system: InputState.shot_pressed → ショット処理
   ↓
4. AI の場合は ai_input_system が InputState を更新
   → ゲーム状態から適切な入力を決定
```

### システム構成

| システム | 対象 | 役割 |
|----------|------|------|
| `human_input_system` | `HumanControlled` を持つエンティティ | キーボード入力 → InputState 更新 |
| `ai_input_system` | `AiControlled` を持つエンティティ | AI判断 → InputState 更新 |
| `movement_system` | 全プレイヤー | InputState.movement → Velocity |
| `jump_system` | 全プレイヤー | InputState.jump_pressed → ジャンプ |
| `shot_input_system` | 全プレイヤー | InputState.shot_pressed → ショット |

---

## コンポーネント定義

### InputState（入力状態コンポーネント）

```rust
/// 各プレイヤーエンティティに付与される入力状態
/// @spec 20006_input_system.md
#[derive(Component, Default, Clone)]
pub struct InputState {
    /// 移動入力（-1.0 〜 1.0）
    pub movement: Vec2,
    /// ジャンプボタンが押されたか（今フレーム）
    pub jump_pressed: bool,
    /// ショットボタンが押されたか（今フレーム）
    pub shot_pressed: bool,
    /// ショットボタンを保持中か
    pub holding: bool,
    /// ホールド継続時間（秒）
    pub hold_time: f32,
}
```

### HumanControlled（人間操作マーカー）

```rust
/// 人間が操作するプレイヤーに付与
/// @spec 20006_input_system.md
#[derive(Component)]
pub struct HumanControlled {
    /// 入力デバイスID（キーボード=0, ゲームパッド=1,2,...）
    pub device_id: usize,
}
```

### AiControlled（AI操作マーカー）

```rust
/// AIが操作するプレイヤーに付与
/// @spec 20006_input_system.md
#[derive(Component)]
pub struct AiControlled;
```

---

## 実装例

### human_input_system

```rust
/// 人間操作プレイヤーの InputState を更新
pub fn human_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&HumanControlled, &mut InputState)>,
) {
    for (human, mut input_state) in &mut query {
        // device_id=0 のみキーボード対応（将来的にゲームパッド対応）
        if human.device_id != 0 {
            continue;
        }

        // 移動入力
        let mut move_x = 0.0;
        let mut move_z = 0.0;
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            move_x -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            move_x += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
            move_z += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
            move_z -= 1.0;
        }
        input_state.movement = Vec2::new(move_x, move_z);

        // ジャンプ入力
        input_state.jump_pressed = keyboard.just_pressed(KeyCode::Space);

        // ショット入力
        input_state.shot_pressed = keyboard.just_pressed(KeyCode::KeyV);
        input_state.holding = keyboard.pressed(KeyCode::KeyV);
    }
}
```

### 入力バッファ（Resource）

```rust
#[derive(Resource, Default)]
pub struct InputBuffer {
    jump_buffer: Option<f32>,  // 残り時間
    shot_buffer: Option<f32>,
}

impl InputBuffer {
    pub fn buffer_jump(&mut self, buffer_time: f32) {
        self.jump_buffer = Some(buffer_time);
    }

    pub fn try_consume_jump(&mut self) -> bool {
        if self.jump_buffer.is_some() {
            self.jump_buffer = None;
            true
        } else {
            false
        }
    }

    pub fn tick(&mut self, delta: f32) {
        if let Some(ref mut time) = self.jump_buffer {
            *time -= delta;
            if *time <= 0.0 {
                self.jump_buffer = None;
            }
        }
        if let Some(ref mut time) = self.shot_buffer {
            *time -= delta;
            if *time <= 0.0 {
                self.shot_buffer = None;
            }
        }
    }
}
```

### 入力バッファ付き input_system

```rust
pub fn input_system_with_buffer(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    config: Res<GameConfig>,
    mut input_buffer: ResMut<InputBuffer>,
    mut query: Query<(&mut Velocity, &Controllable, &GroundState), With<Player>>,
) {
    // バッファ時間を更新
    input_buffer.tick(time.delta_secs());

    for (mut velocity, controllable, ground_state) in &mut query {
        if !controllable.is_controllable {
            continue;
        }

        // ジャンプ入力をバッファに登録
        if keyboard.just_pressed(KeyCode::Space) {
            input_buffer.buffer_jump(config.input.jump_buffer_time);
        }

        // 着地時にバッファされたジャンプを消費
        if ground_state.is_grounded && input_buffer.try_consume_jump() {
            velocity.y = config.player.jump_force;
        }

        // ... 他の入力処理
    }
}
```

---

## 設計原則

### 1. 入力とロジックの分離
```rust
// ✅ 良い例（入力 → Velocity → 移動処理）
// input_system: キー入力 → Velocity を設定
// movement_system: Velocity → Position を更新

// ❌ 悪い例（入力とロジックが密結合）
// 同じ System 内で Position を直接変更
```

### 2. プラットフォーム非依存
```rust
// ✅ 良い例（抽象的な InputAction）
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputAction {
    Jump,
    Shot,
    Move,
}

// ❌ 悪い例（プラットフォーム固有の入力をロジックに埋め込み）
// ロジック内で直接 KeyCode::Space をチェック
```

### 3. 入力バッファで操作感向上
```rust
// ✅ 良い例（先行入力を受け付ける）
if ground_state.is_grounded && input_buffer.try_consume_jump() {
    velocity.y = config.player.jump_force;
}

// ❌ 悪い例（着地と同時の入力のみ受け付け）
if ground_state.is_grounded && keyboard.just_pressed(KeyCode::Space) {
    velocity.y = config.player.jump_force;
}
```

---

## Bevy Input API

### ButtonInput<KeyCode>

| メソッド | 説明 |
|---------|------|
| `pressed(key)` | キーが押されているか |
| `just_pressed(key)` | キーが今フレーム押されたか |
| `just_released(key)` | キーが今フレーム離されたか |

### Gamepad 対応

```rust
pub fn gamepad_input_system(
    gamepads: Query<&Gamepad>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for gamepad in &gamepads {
        for mut velocity in &mut query {
            // 左スティック
            let left_stick = gamepad.left_stick();
            velocity.x = left_stick.x * config.player.move_speed;
            velocity.z = left_stick.y * config.player.move_speed;

            // ジャンプボタン（South = A on Xbox, X on PlayStation）
            if gamepad.just_pressed(GamepadButton::South) {
                // ジャンプ処理
            }
        }
    }
}
```

---

## 次のステップ

1. ✅ 入力システム設計（このドキュメント）
2. ✅ エンティティベースの入力状態管理（v4.0.0）
3. ✅ HumanControlled / InputState コンポーネント実装
4. ✅ human_input_system 実装
5. ✅ movement_system, jump_system, shot_input_system との統合
6. ⏳ 入力バッファの実装（先行入力対応）
7. ⏳ ゲームパッド対応
8. ⏳ 旧リソース（MovementInput, JumpInput, ShotInput, ShotButtonState）の削除

## 参考資料

- [20000_overview.md](20000_overview.md) - アーキテクチャ概要
- [20004_ecs_overview.md](20004_ecs_overview.md) - ECS 詳細
- [20005_event_system.md](20005_event_system.md) - イベントシステム
