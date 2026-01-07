# Shadow System Specification

**Version**: 1.0.0
**Status**: Active
**Last Updated**: 2026-01-07

## 概要

エンティティ（プレイヤー、ボール）の地面への影投影システムを定義します。影はエンティティの地面位置（Y=0）に表示され、高さを視覚的に表現します。

## Core Requirements (MVP v0.1)

### REQ-30801-001: プレイヤー影の生成
**WHEN** Player コンポーネントを持つエンティティが生成される
**AND** そのエンティティに HasShadow マーカーがない
**THE SYSTEM SHALL** プレイヤー用の影エンティティを生成する
- Shadow コンポーネント（owner = プレイヤーエンティティ）を付与
- Sprite を生成（サイズ: `config.shadow.player_size`）
- 透明度: `config.shadow.player_alpha`
- HasShadow マーカーをプレイヤーに付与（重複生成防止）

**実装**: `presentation/mod.rs:spawn_player_shadow_system`

---

### REQ-30801-002: ボール影の生成
**WHEN** Ball コンポーネントを持つエンティティが生成される
**AND** そのエンティティの影がまだ存在しない
**THE SYSTEM SHALL** ボール用の影エンティティを生成する
- Shadow コンポーネント（owner = ボールエンティティ）を付与
- Sprite を生成（サイズ: `config.shadow.ball_size`）
- 透明度: `config.shadow.ball_alpha`

**実装**: `presentation/mod.rs:spawn_ball_shadow_system`

---

### REQ-30801-003: 影位置の同期
**WHILE** 影エンティティが存在する
**AND** 所有者エンティティが存在する
**THE SYSTEM SHALL** 影の表示位置を所有者の地面投影位置に同期する
- 表示X座標 = `owner.logical_pos.z * WORLD_SCALE`
- 表示Y座標 = `owner.logical_pos.x * WORLD_SCALE - y_offset`
  - プレイヤー影: `y_offset = config.shadow.player_y_offset`
  - ボール影: `y_offset = config.shadow.ball_y_offset`
- 表示Z座標 = `config.shadow.z_layer`（最背面）

**実装**: `presentation/mod.rs:sync_shadow_system`

---

### REQ-30801-004: ボール影の削除
**WHEN** ボールエンティティが削除される
**THE SYSTEM SHALL** 対応するボール影エンティティを削除する
- プレイヤー影は削除対象外

**実装**: `presentation/mod.rs:despawn_ball_shadow_system`

---

### REQ-30801-005: 座標変換（論理→表示）
**WHILE** LogicalPosition を持つエンティティが存在する
**THE SYSTEM SHALL** 論理座標を表示座標に変換する
- 表示X座標 = `logical_pos.z * WORLD_SCALE`（奥行き→画面左右）
- 表示Y座標 = `logical_pos.x * WORLD_SCALE + logical_pos.y * WORLD_SCALE`（横移動+高さ→画面上下）
- 表示Z座標 = `1.0 - logical_pos.x * 0.01`（レイヤー深度）

**実装**: `presentation/mod.rs:sync_transform_system`

---

## 制約（Design by Contract）

### 事前条件
- GameConfig が読み込まれている
- shadow セクションの設定が存在する
- エンティティに LogicalPosition コンポーネントが存在する

### 事後条件
- 全てのプレイヤーに対応する影が存在する
- 全ての存在するボールに対応する影が存在する
- 削除されたボールの影は存在しない

### 不変条件
- 影の表示Z座標は常に `config.shadow.z_layer`
- 影は地面位置（論理Y=0）に投影される

---

## データ参照

| パラメータ | データ定義 | 説明 |
|-----------|-----------|------|
| プレイヤー影サイズ | `config.shadow.player_size` | (幅, 高さ) |
| プレイヤー影透明度 | `config.shadow.player_alpha` | 0.0-1.0 |
| プレイヤー影Yオフセット | `config.shadow.player_y_offset` | 足元調整 |
| ボール影サイズ | `config.shadow.ball_size` | (幅, 高さ) |
| ボール影透明度 | `config.shadow.ball_alpha` | 0.0-1.0 |
| ボール影Yオフセット | `config.shadow.ball_y_offset` | 位置調整 |
| 影Zレイヤー | `config.shadow.z_layer` | 描画順序 |

詳細: [80101_game_constants.md](../../8_data/80101_game_constants.md)

---

## 依存関係

### 依存先
- [20001_layers.md](../../2_architecture/20001_layers.md) - レイヤー構造、座標変換
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - shadow パラメータ

### 被依存
- なし

---

## 備考

- 座標変換（sync_transform_system）は影に限らず全エンティティに適用
- 影システムは純粋な表示機能であり、ゲームロジックには影響しない
- WORLD_SCALE = 100.0（1ワールドユニット = 100ピクセル）
