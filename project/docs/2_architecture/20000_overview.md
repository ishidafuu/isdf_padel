# Architecture Overview

**Version**: 3.0.0
**Last Updated**: 2026-01-06
**Status**: Active

## ゲームコンセプト

パデルを題材とした **2.5Dアクションスポーツゲーム**（横視点、くにおくんドッジボール風）。

- **視点**: 横視点（2.5D）
- **コート**: 壁・天井のある密閉空間（4面の壁 + 天井）
- **移動**: X軸（左右）、Y軸（高さ/ジャンプ）、Z軸（奥行き、無段階）
- **メカニクス**: 壁・天井反射、キャラクター当たり判定、ふっとばし

## アーキテクチャ方針

### 1. Bevy ネイティブ ECS

| 要素 | 担当 | 役割 |
|------|------|------|
| Bevy ECS | ゲームロジック、状態管理 | Entity/Component/System による疎結合設計 |
| Bevy Render | レンダリング | Sprite、Transform、深度ソート |
| Bevy Input | 入力処理 | ButtonInput<KeyCode> によるキー入力 |

**Bevy ECS の役割**:
- Component によるデータ駆動設計（`Position`, `Velocity`, `KnockbackState`）
- System による処理の集約（`movement_system`, `wall_reflection_system`）
- Event による Entity 間通信（`BallHitEvent`, `KnockbackEvent`）

**Bevy Render の役割**:
- Transform による位置・回転・スケール管理
- Sprite2d による 2D 描画
- 深度ソート（Z 座標による描画順序制御）

### 2. 2.5D 座標系

**座標軸の定義（IMPORTANT）**:

| 軸 | 意味 | 範囲 | 備考 |
|----|------|------|------|
| **X** | 左右移動 | -courtWidth/2 ~ +courtWidth/2 | 画面の横方向 |
| **Y** | 高さ（ジャンプ） | 0.0 ~ maxJumpHeight | 重力の影響を受ける |
| **Z** | 奥行き移動 | 0.0 ~ courtDepth | **無段階**、くにおくん方式 |

**レンダリング順序**:
- Z値が大きい（奥にいる）Entity は後ろに描画
- Bevy の `Transform.translation.z` で制御
- `depth_order_system` が毎フレーム更新

**物理演算**:
- カスタム物理システムを使用（Bevy の2D座標系ベース）
- Z軸は論理的な概念（レンダリング順序のみ）
- 当たり判定は X-Y 平面で実行（Z値が近い場合のみ）

### 3. 疎結合設計

**Entity 間の直接参照を禁止**:
```
❌ 禁止: player.rs で Enemy への参照を持つ
✅ 推奨: Event 経由で通信
```

**例**: ボールがキャラクターに当たる
```rust
// ❌ 禁止
if ball_pos == player_pos {
    player.knockback();
}

// ✅ 推奨
events.write(BallHitEvent {
    ball_id: ball_entity,
    target_id: player_entity,
    hit_point: collision_point,
});
```

### 4. データ駆動設計（ハードコーディング禁止）

**CRITICAL**: 調整可能な全てのパラメータ値はハードコーディングを禁止し、外部データファイル化を必須とする。

**❌ 禁止**: ハードコーディング
```rust
velocity.y += -9.8 * time.delta_secs();  // 重力値をハードコーディング
if speed > 5.0 { ... }                    // 速度制限をハードコーディング
```

**✅ 必須**: GameConfig 参照
```rust
velocity.y += config.physics.gravity * time.delta_secs();
if speed > config.player.max_speed { ... }
```

**データ配置**: `project/docs/8_data/80101_game_constants.md` に定義

### 5. 5層構造

| Layer | 責務 | 依存先 |
|-------|------|--------|
| **Core** | 共通トレイト、Event 定義、Utility | なし |
| **Resource** | 設定データ、マスタデータ（`GameConfig`） | Core |
| **Components** | Entity データ構造（`Position`, `KnockbackState`） | Core |
| **Systems** | ゲームロジック（`movement_system`, `wall_reflection_system`） | Core, Resource, Components |
| **Presentation** | Bevy Sprite、Transform、UI | Systems |

**依存ルール**:
- 上位レイヤーは下位レイヤーのみ参照可能
- System は GameConfig を `Res<GameConfig>` で受け取る
- 同一レイヤー内の System 間相互参照は禁止（Event 経由）

## 主要システム

### 1. 移動システム（movement_system）

| 入力 | 処理 | 出力 |
|------|------|------|
| 方向キー左右 | X軸速度を設定 | `Velocity.x` |
| 方向キー上下 | Z軸速度を設定 | `Velocity.z` |
| Aボタン | ジャンプ、Y軸速度を設定 | `Velocity.y` |

**重力処理**:
```
毎フレーム: velocity.y += config.physics.gravity * time.delta_secs()
地面接触時: velocity.y = 0, is_grounded = true
```
（`config.physics.gravity` のデフォルト: -9.8 m/s²）

### 2. 壁・天井反射システム（wall_reflection_system）

| 壁 | 反射軸 | 条件 |
|----|--------|------|
| 左壁 | X軸反転 | position.x < -court_width/2 |
| 右壁 | X軸反転 | position.x > +court_width/2 |
| 前壁 | Z軸反転 | position.z < 0 |
| 後壁 | Z軸反転 | position.z > court_depth |
| 天井 | Y軸反転 | position.y > ceiling_height |

**反射ルール**:
- 入射角 = 反射角（基本）
- バウンド係数 = `config.ball.bounce_factor`（デフォルト: 0.8、減衰はあるがラリー継続のため強め）
- 天井反射は **アウトではない**（パデルルール）

### 3. キャラクター当たり判定システム（character_collision_system）

**衝突判定**:
```
ボールとキャラクターの距離 < (config.ball.radius + config.character.radius)
&& |Z値の差| < config.character.z_tolerance
→ BallHitEvent 発行
```
（デフォルト: character.radius=0.5, ball.radius=0.2, z_tolerance=0.3）

**ふっとばし処理**:
```rust
// knockback_system が BallHitEvent を購読
fn knockback_system(
    mut events: EventReader<BallHitEvent>,
    mut commands: Commands,
    mut query: Query<&mut Velocity>,
) {
    for event in events.read() {
        // 1. KnockbackState コンポーネントを追加
        // 2. ふっとばし方向・速度を計算
        // 3. 操作不能フラグをセット
        // 4. 無敵時間タイマーを開始
    }
}
```

### 4. 深度レンダリングシステム（depth_order_system）

**毎フレーム実行**:
```rust
fn depth_order_system(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut query {
        transform.translation.z = pos.z * 0.01;  // Bevy の Z は小さい値が手前
    }
}
```

- Z値が大きいほど奥に描画
- くにおくんドッジボール風の奥行き表現

## MVP v0.1 の範囲

| 機能 | 含む | 含まない |
|------|------|---------|
| 移動 | X/Y/Z 3軸移動、ジャンプ | ダッシュ |
| 反射 | 壁・天井反射 | 特殊な反射（スピン等） |
| 当たり判定 | キャラクター当たり、ふっとばし | ボール間の衝突 |
| レンダリング | 深度ソート | アニメーション |
| ゲームモード | シングルス（1vs1） | ダブルス、AI |

## 次のステップ

1. ✅ アーキテクチャ設計（このドキュメント）
2. ⏳ ECS コンポーネント詳細設計（`20004_ecs_overview.md`）
3. ⏳ イベントシステム設計（`20005_event_system.md`）
4. ⏳ 入力システム設計（`20006_input_system.md`）
5. ⏳ 共有コンポーネント定義（`209_components/`）

## 参考資料

- [10001_concept.md](../1_project/10001_concept.md) - ゲームコンセプト（v2.0.0）
- [20001_layers.md](20001_layers.md) - 5層構造詳細
- [20004_ecs_overview.md](20004_ecs_overview.md) - ECS 設計詳細
