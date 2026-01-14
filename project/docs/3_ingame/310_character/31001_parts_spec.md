# Character Parts Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2026-01-09

## 概要

パーツ分離キャラクターのパーツ構成とComponent設計を定義する。8パーツ構成のロボットキャラクターを実現し、各パーツは独立したエンティティとして管理される。

## Core Requirements (MVP v0.1)

### REQ-31001-001: パーツ構成
**THE SYSTEM SHALL** 以下の8パーツでキャラクターを構成する

| パーツ | PartKind | 数 | 説明 |
|--------|----------|---|------|
| 頭部 | Head | 1 | キャラクターの頭 |
| 胴体 | Torso | 1 | キャラクターの胴体（中心） |
| 左手 | LeftHand | 1 | 左側の手 |
| 右手 | RightHand | 1 | 右側の手 |
| 左膝 | LeftKnee | 1 | 左側の膝 |
| 右膝 | RightKnee | 1 | 右側の膝 |
| 左足 | LeftFoot | 1 | 左側の足 |
| 右足 | RightFoot | 1 | 右側の足 |
| ラケット | Racket | 1 | 右手に追従するラケット |

**テスト**: 目視確認 - 9パーツが表示されること

---

### REQ-31001-002: パーツエンティティ構造
**THE SYSTEM SHALL** 各パーツを独立したエンティティとして生成する
- 親エンティティ: ArticulatedCharacter マーカーを持つ
- 子エンティティ: PartDefinition, PartState, Sprite を持つ
- 親子関係: Bevy Parent/Children（1階層のみ）

**テスト**: エンティティ構造の検証

---

### REQ-31001-003: パーツ定義（PartDefinition）
**THE SYSTEM SHALL** 各パーツに以下の定義情報を持たせる
- `kind: PartKind` - パーツ種別
- `base_offset: Vec3` - ニュートラル状態での基準位置（親からの相対）
- `size: Vec2` - スプライトサイズ

**データ参照**: `config.Character.Parts[kind]`

---

### REQ-31001-004: パーツ状態（PartState）
**THE SYSTEM SHALL** 各パーツに以下の動的状態を持たせる
- `local_position: Vec3` - 現在のローカル位置（base_offsetからの相対）
- `local_rotation: f32` - 現在の回転角度（度）

**備考**: アニメーションによりPartStateが更新され、Transformに反映される

---

### REQ-31001-005: キャラクター向き（CharacterFacing）
**THE SYSTEM SHALL** キャラクターの向きを管理する
- `Right`: 右向き（デフォルト、mirror = 1.0）
- `Left`: 左向き（mirror = -1.0）

**WHEN** CharacterFacing が Left である
**THE SYSTEM SHALL** パーツのX座標を反転する（ミラーリング）

**テスト**: 左右向き切り替えで表示が反転すること

---

### REQ-31001-006: Transform同期
**WHEN** PartState が更新される
**THE SYSTEM SHALL** パーツのTransformを以下の式で計算する

```
part_transform.translation.x = (base_offset.x + local_position.x) * mirror
part_transform.translation.y = base_offset.y + local_position.y
part_transform.translation.z = (base_offset.z + local_position.z + z_priority) * 0.01
part_transform.rotation = Quat::from_rotation_z(local_rotation.to_radians() * mirror)
```

- `mirror = 1.0` (Right) / `-1.0` (Left)

**テスト**: パーツ位置がPartStateに従って更新されること

---

### REQ-31001-007: 向きを加味したZ優先度
**WHEN** CharacterFacing によってキャラクターが向きを変える
**THE SYSTEM SHALL** 左右対称パーツのZ優先度を動的に調整する

**Z優先度計算ルール**:
```
z_priority =
  if is_symmetric_part:
    if is_right_part: +mirror * Z_PRIORITY_OFFSET
    else:             -mirror * Z_PRIORITY_OFFSET
  else:
    0.0
```

- `Z_PRIORITY_OFFSET = 0.5`（右パーツが前、左パーツが後ろを基本とした差分）

**具体例**:
| パーツ | 右向き(mirror=1) | 左向き(mirror=-1) |
|--------|------------------|-------------------|
| Head | 0.0（中央） | 0.0（中央） |
| Torso | 0.0（中央） | 0.0（中央） |
| RightHand | +0.5（前面） | -0.5（背面） |
| LeftHand | -0.5（背面） | +0.5（前面） |
| RightKnee | +0.5（前面） | -0.5（背面） |
| LeftKnee | -0.5（背面） | +0.5（前面） |
| RightFoot | +0.5（前面） | -0.5（背面） |
| LeftFoot | -0.5（背面） | +0.5（前面） |

**テスト**: 向き反転時に前後関係が正しく入れ替わること

---

### REQ-31001-008: ラケットパーツ
**THE SYSTEM SHALL** ラケットを右手に追従する独立パーツとして表示する
- ラケットは常に表示される（全アニメーション状態で）
- ラケットのアニメーションは右手と同様のキーフレームを使用し、追加の回転を適用
- Z優先度は右手と同様に向きに応じて前後が入れ替わる
- サイズ: 6x16（縦長）
- 色: 黄色系

**テスト**: ショット時にラケットがスイングすること

---

## Extended Requirements (v0.2)

### REQ-31001-050: パーツカラー
**WHERE** パーツにカスタムカラーが設定されている
**THE SYSTEM SHALL** スプライトの色を変更する

---

### REQ-31001-051: パーツスケール
**WHERE** パーツにカスタムスケールが設定されている
**THE SYSTEM SHALL** スプライトのスケールを変更する

---

## 制約（Design by Contract）

### 事前条件
- GameConfig が読み込まれている
- パーツ定義データが存在する

### 事後条件
- 8パーツのエンティティが生成される
- 全パーツが親エンティティの子として登録される
- PartState の変更が Transform に反映される

### 不変条件
- パーツ数は常に9
- 親子関係は1階層のみ

---

## Component設計

### PartKind（パーツ種別列挙型）
```rust
pub enum PartKind {
    Head,
    Torso,
    LeftHand,
    RightHand,
    LeftKnee,
    RightKnee,
    LeftFoot,
    RightFoot,
    Racket,
}
```

### PartDefinition（パーツ定義Component）
```rust
pub struct PartDefinition {
    pub kind: PartKind,
    pub base_offset: Vec3,
    pub size: Vec2,
}
```

### PartState（パーツ状態Component）
```rust
pub struct PartState {
    pub local_position: Vec3,
    pub local_rotation: f32,
}
```

### ArticulatedCharacter（マーカーComponent）
```rust
pub struct ArticulatedCharacter;
```

### CharacterFacing（向きComponent）
```rust
pub enum CharacterFacing {
    Right,
    Left,
}
```

---

## データ参照

| パラメータ | データ定義 | デフォルト値 |
|-----------|-----------|------------|
| Head base_offset | config.Character.Parts.Head.BaseOffset | (0, 24, 0) |
| Torso base_offset | config.Character.Parts.Torso.BaseOffset | (0, 12, 0) |
| Hand base_offset | config.Character.Parts.Hand.BaseOffset | (12, 16, 0) |
| Knee base_offset | config.Character.Parts.Knee.BaseOffset | (4, 4, 0) |
| Foot base_offset | config.Character.Parts.Foot.BaseOffset | (6, -4, 0) |
| Part size | config.Character.Parts.*.Size | 各パーツ固有 |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先
- [20004_ecs_overview.md](../../2_architecture/20004_ecs_overview.md) - ECS設計原則
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - パーツパラメータ

### 被依存
- [31002_animation_spec.md](31002_animation_spec.md) - アニメーションシステム

---

## Change Log

### 2026-01-09 - v1.0.0（初版）
- 初版作成
