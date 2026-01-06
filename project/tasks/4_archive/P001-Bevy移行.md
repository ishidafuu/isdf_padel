---
id: "P001"
title: "Godot + C# から Bevy 0.17 (Rust) への移行"
type: "project-wide"
status: "done"
priority: "high"
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/s13219/.claude/plans/radiant-hopping-rain.md"
tags: ["bevy", "rust", "migration", "engine"]
created_at: "2026-01-06T09:18:32+09:00"
updated_at: "2026-01-06T11:00:00+09:00"
completed_at: "2026-01-06T12:00:00+09:00"
---

# Task P001: Godot + C# から Bevy 0.17 (Rust) への移行

## 説明

仕様書駆動開発フレームワークのゲームエンジンを **Godot + C#** から **Bevy 0.17 (Rust)** に変更する。

実装コード（src/, tests/）は空のため、**仕様書の書き換えのみ**で完了する。

## 背景

### 現状

- ゲームエンジン: Godot 4.x
- 言語: C# / GDScript
- ECS: Godot + ECS ハイブリッド
- データ形式: .tres (Godot Resource)

### 改修理由

1. **Claude Code との相性**: 全てコードベースの Bevy は Claude Code での開発に適している
2. **学習目的**: Rust と ECS アーキテクチャの習得
3. **Web対応**: Bevy も WASM エクスポート対応
4. **パフォーマンス**: Rust による高パフォーマンス

## 実装内容

### Phase 1: 基盤ドキュメント（最優先）

- [x] `project/docs/2_architecture/20000_overview.md` - Godot+ECS → Bevy ECS
- [x] `project/docs/2_architecture/20001_layers.md` - C# interface/class → Rust trait/struct
- [x] `project/docs/2_architecture/20004_ecs_overview.md` - C# Component → Rust #[derive(Component)]

### Phase 2: システム設計

- [x] `project/docs/2_architecture/20005_event_system.md` - Godot Signal → Bevy Event
- [x] `project/docs/2_architecture/20006_input_system.md` - Godot Input API → Bevy ButtonInput

### Phase 3: コンポーネント定義

- [x] `project/docs/2_architecture/209_components/20901_position.md` - C# → Rust
- [x] `project/docs/2_architecture/209_components/20902_velocity.md` - C# → Rust

### Phase 4: データ定義

- [x] `project/docs/8_data/80101_game_constants.md` - C# class + .tres → Rust struct + .ron

### Phase 5: フレームワーク定義

- [x] `.claude/CLAUDE.md` - C# コード例 → Rust
- [x] `.claude/agents/design-agent.md` - Godot固有型 → Bevy固有型の指針
- [x] `.claude/skills/impl-comments.md` - GDScript例 → Rust例
- [x] `README.md` - Godot対応 → Bevy対応

## メモ

- Bevy バージョン: 0.17（2026年1月時点の最新）
- データ形式: RON (Rust Object Notation)
- ECS: Bevy ネイティブ ECS に完全移行

## 依存関係

- **ブロック**: なし
- **ブロックされる**: なし
- **関連ドキュメント**:
  - プランファイル: `~/.claude/plans/radiant-hopping-rain.md`

---

## Detailed Implementation Plan

以下は、プランファイル `~/.claude/plans/radiant-hopping-rain.md` の全内容です。

# Godot + C# → Bevy 0.17 (Rust) 移行プラン

## 概要

仕様書駆動開発フレームワークのゲームエンジンを **Godot + C#** から **Bevy 0.17 (Rust)** に変更する。

### 確定事項

| 項目 | 変更前 | 変更後 |
|------|--------|--------|
| エンジン | Godot 4.x | Bevy 0.17 |
| 言語 | C# / GDScript | Rust |
| ECS | Godot + ECS ハイブリッド | Bevy ネイティブ ECS |
| データ形式 | .tres (Godot Resource) | .ron (RON) |
| Web対応 | Godot WASM | Bevy WASM |

### 実装コード状況

- **src/**: 空（コードなし）
- **tests/**: 空（コードなし）
- **仕様書のみ修正が必要**（実装コードの移行作業は不要）

---

## 修正対象ファイル一覧

### Phase 1: 基盤ドキュメント（最優先）

| ファイル | 主な変更内容 |
|----------|-------------|
| `project/docs/2_architecture/20000_overview.md` | Godot+ECS → Bevy ECS、C#コード例 → Rust |
| `project/docs/2_architecture/20001_layers.md` | C# interface/class → Rust trait/struct |
| `project/docs/2_architecture/20004_ecs_overview.md` | C# Component → Rust #[derive(Component)] |

### Phase 2: システム設計

| ファイル | 主な変更内容 |
|----------|-------------|
| `project/docs/2_architecture/20005_event_system.md` | Godot Signal → Bevy Event |
| `project/docs/2_architecture/20006_input_system.md` | Godot Input API → Bevy ButtonInput |

### Phase 3: コンポーネント定義

| ファイル | 主な変更内容 |
|----------|-------------|
| `project/docs/2_architecture/209_components/20901_position.md` | C# → Rust |
| `project/docs/2_architecture/209_components/20902_velocity.md` | C# → Rust |

### Phase 4: データ定義

| ファイル | 主な変更内容 |
|----------|-------------|
| `project/docs/8_data/80101_game_constants.md` | C# class + .tres → Rust struct + .ron |

### Phase 5: フレームワーク定義

| ファイル | 主な変更内容 |
|----------|-------------|
| `.claude/CLAUDE.md` | C# コード例 → Rust |
| `.claude/agents/design-agent.md` | Godot固有型 → Bevy固有型の指針 |
| `.claude/skills/impl-comments.md` | GDScript例 → Rust例 |
| `README.md` | Godot対応 → Bevy対応 |

---

## 変更パターン（共通）

### 1. Component 定義

```csharp
// Before (C#)
public class Position {
    public float X { get; set; }
    public float Y { get; set; }
    public float Z { get; set; }
}
```

```rust
// After (Rust)
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
```

### 2. Event 定義

```csharp
// Before (C#)
public class BallHitEvent {
    public Entity BallId;
    public Entity TargetId;
}
```

```rust
// After (Rust)
#[derive(Event)]
pub struct BallHitEvent {
    pub ball_id: Entity,
    pub target_id: Entity,
}
```

### 3. System 定義

```csharp
// Before (C#)
public void Execute(float deltaTime) {
    velocity.Y += config.Physics.Gravity * deltaTime;
}
```

```rust
// After (Rust)
fn gravity_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<&mut Velocity>,
) {
    for mut velocity in &mut query {
        velocity.y += config.physics.gravity * time.delta_secs();
    }
}
```

### 4. データファイル

```gdscript
# Before (.tres)
[resource]
gravity = -9.8
max_fall_speed = -20.0
```

```ron
// After (.ron)
GameConfig(
    physics: PhysicsConfig(
        gravity: -9.8,
        max_fall_speed: -20.0,
    ),
)
```

---

## 実行手順

### Step 1: Phase 1 基盤ドキュメント

1. `20000_overview.md` を開き、以下を変更:
   - 「Godot + ECS ハイブリッド」→「Bevy ネイティブ ECS」
   - 表の Godot 列を Bevy に置換
   - C# コード例をすべて Rust に変換

2. `20001_layers.md` を変更:
   - C# interface → Rust trait
   - C# class → Rust struct
   - 全コード例を Rust に変換

3. `20004_ecs_overview.md` を変更:
   - Component 定義を Rust derive マクロ形式に
   - System 定義を Bevy System 形式に

### Step 2: Phase 2 システム設計

4. `20005_event_system.md` を変更:
   - Godot Signal の説明を削除
   - Bevy Event (#[derive(Event)]) に置換
   - EventWriter/EventReader パターンを記述

5. `20006_input_system.md` を変更:
   - Godot Input API → Bevy ButtonInput<KeyCode>
   - Input.IsActionPressed → keyboard.pressed()

### Step 3: Phase 3 コンポーネント定義

6. `20901_position.md` を変更:
   - C# Position クラス → Rust Position struct
   - Vector2/Vector3 → Vec2/Vec3

7. `20902_velocity.md` を変更:
   - 同様に Rust 形式へ変換

### Step 4: Phase 4 データ定義

8. `80101_game_constants.md` を変更:
   - C# GameConfig クラス → Rust struct + serde
   - .tres 形式 → .ron 形式
   - ResourceLoader → Bevy Asset System

### Step 5: Phase 5 フレームワーク定義

9. `.claude/CLAUDE.md` を変更:
   - ハードコーディング禁止例の C# → Rust
   - .tres → .ron

10. `.claude/agents/design-agent.md` を変更:
    - 「Unity/Godot 固有の型」→「Bevy 固有の型」

11. `.claude/skills/impl-comments.md` を変更:
    - GDScript 例を Rust 例に置換

12. `README.md` を変更:
    - 「エンジン非依存（Unity/Godot対応）」→「Bevy 0.17 (Rust)」

---

## Bevy 0.17 API 注意点

| 項目 | Bevy 0.17 での書き方 |
|------|---------------------|
| Event 定義 | `#[derive(Event)]` |
| Component 定義 | `#[derive(Component)]` |
| Resource 定義 | `#[derive(Resource)]` |
| System 引数 | `Query<&T>`, `Res<T>`, `ResMut<T>` |
| Event 発行 | `EventWriter<T>::write()` |
| Event 購読 | `EventReader<T>::read()` |
| 時間取得 | `time.delta_secs()` |
| キー入力 | `ButtonInput<KeyCode>` |
| System 順序 | `.chain()` または `before()`/`after()` |

---

## 完了条件

- [x] 全 12 ファイルの修正完了
- [x] C#/GDScript のコード例がゼロ
- [x] Godot 固有 API への参照がゼロ
- [x] .tres への参照がゼロ（.ron に置換）
- [x] 各ファイルのバージョン/日付を更新
