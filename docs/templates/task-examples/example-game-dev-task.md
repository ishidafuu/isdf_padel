---
id: "30101"
title: "プレイヤージャンプ機能実装"
type: "game-dev"
status: "todo"
priority: "high"
spec_ids: ["30201", "30202"]
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["player", "physics", "animation"]
created_at: "2025-12-29T10:00:00.000000"
updated_at: "2025-12-29T10:00:00.000000"
completed_at: null
parent_task_id: null
---

# プレイヤージャンプ機能実装

## 概要

プレイヤーキャラクターのジャンプ機能を実装する。
仕様書 `30201_player_spec.md` および `30202_player_design.md` に基づいて実装する。

## 対象仕様書

- `project/docs/3_ingame/302_player/30201_player_spec.md`
  - REQ-30201-010: ジャンプ入力
  - REQ-30201-011: ジャンプ物理演算
  - REQ-30201-012: 二段ジャンプ
  - REQ-30201-013: 着地判定

- `project/docs/3_ingame/302_player/30202_player_design.md`
  - DES-30202-005: JumpComponent
  - DES-30202-006: PhysicsComponent

- `project/docs/3_ingame/302_player/30203_player_behavior.md`
  - BHV-30203-003: PlayerJumpSystem
  - BHV-30203-004: 状態遷移（Ground → Air → Ground）

## 実装計画

### Phase 1: データ構造実装（1-2h）

- [ ] `JumpComponent` クラス実装
  - [ ] `MaxJumpHeight` プロパティ
  - [ ] `JumpForce` プロパティ
  - [ ] `CanDoubleJump` プロパティ
  - [ ] `IsJumping` プロパティ

- [ ] `PhysicsComponent` 拡張
  - [ ] `Velocity.Y` 管理
  - [ ] `IsGrounded` フラグ

### Phase 2: システム実装（2-3h）

- [ ] `PlayerJumpSystem` クラス実装
  - [ ] ジャンプ入力処理（`Input.IsActionJustPressed("jump")`）
  - [ ] ジャンプ物理演算（`velocity.Y = JumpForce`）
  - [ ] 二段ジャンプロジック
  - [ ] 着地判定（`RayCast` による地面検出）

- [ ] 状態遷移実装
  - [ ] Ground → Air（ジャンプ開始）
  - [ ] Air → Ground（着地）

### Phase 3: テスト実装（1h）

- [ ] ユニットテスト
  - [ ] ジャンプ入力時にvelocity.Yが正の値になる
  - [ ] 二段ジャンプが1回のみ可能
  - [ ] 着地時にIsGroundedがtrueになる

- [ ] 統合テスト
  - [ ] ジャンプ→着地の一連の流れ
  - [ ] ジャンプ中に攻撃可能

## データ依存

### パラメータファイル

- `project/docs/8_data/801_physics/30801_physics_parameters.md`
  - `Gravity`: -9.8 m/s²
  - `JumpForce`: 5.0 m/s（標準ジャンプ）
  - `MaxJumpHeight`: 2.5 m

### アニメーションデータ

- `project/docs/8_data/802_animation/30802_player_animations.md`
  - `jump_start`: ジャンプ開始アニメーション
  - `jump_loop`: 空中アニメーション
  - `jump_land`: 着地アニメーション

## 実装上の注意点

### 1. ハードコーディング禁止

❌ **絶対に禁止:**
```csharp
velocity.Y = 5.0f;  // ハードコーディング
```

✅ **必須:**
```csharp
velocity.Y = physicsParams.JumpForce;  // データから取得
```

### 2. 実装コメント付与

すべての実装に `@spec`, `@test`, `@data` コメントを付与する：

```csharp
// @spec REQ-30201-010
// @data 30801_physics_parameters.md:JumpForce
public void HandleJumpInput()
{
    if (Input.IsActionJustPressed("jump") && IsGrounded)
    {
        velocity.Y = physicsParams.JumpForce;
    }
}
```

### 3. MVP範囲の確認

**MVP v0.1 に含まれる:**
- ✅ 基本ジャンプ
- ✅ 着地判定

**MVP 範囲外（v0.2以降）:**
- ❌ 二段ジャンプ（v0.2で実装）
- ❌ 壁ジャンプ（v0.3で実装）

MVP範囲は `project/docs/1_project/10009_mvp_scope.md` を参照。

## レビュー基準

### コード品質

- [ ] 全ての実装に `@spec` コメントが付与されている
- [ ] ハードコーディングされた値がない（全てデータファイルから取得）
- [ ] 命名規則に従っている（PascalCase for classes, camelCase for methods）
- [ ] 適切なエラーハンドリングが実装されている

### テストカバレッジ

- [ ] 全ての要件（REQ-*）に対応するテストが存在する
- [ ] エッジケース（空中でジャンプ入力、着地直前にジャンプ入力など）がカバーされている

### 整合性

- [ ] 仕様書と実装が一致している
- [ ] 依存する仕様書（PhysicsComponent等）との整合性が取れている

## 完了条件

- [ ] 全ての実装計画のチェックリストが完了
- [ ] テストが全て通過
- [ ] コードレビューが完了
- [ ] PRがマージされた
- [ ] worktreeとブランチがクリーンアップされた

## メモ

### 技術的な課題

- RayCastによる着地判定の精度を確認する必要がある
- アニメーションとの同期タイミングを調整

### 参考資料

- [参照ゲームのジャンプ仕様](../../project/project/docs/9_reference/901_reference_game/90101_movement.md)
- Godot公式ドキュメント: CharacterBody2D

### 関連タスク

- 30102: 敵キャラクター実装（ジャンプ機能を参考にする）
- 30103: プレイヤー攻撃機能実装（ジャンプ中攻撃と連携）

---

**このテンプレートは `game-dev` タスクの例です。**
**実際のタスクでは、プロジェクトの仕様書構造に合わせて調整してください。**
