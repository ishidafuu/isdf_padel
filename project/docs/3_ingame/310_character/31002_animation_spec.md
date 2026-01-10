# Character Animation Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2026-01-09

## 概要

パーツ分離キャラクターのアニメーションシステムを定義する。パラメトリック補間によりキーフレーム間の位置・回転を滑らかに補間し、Idle・Walk・Jump等のアニメーションを実現する。

## Core Requirements (MVP v0.1)

### REQ-31002-001: アニメーション識別子
**THE SYSTEM SHALL** アニメーションを一意に識別するIDを提供する

| AnimationId | 説明 |
|-------------|------|
| Idle | 待機アニメーション（浮遊感のある微動） |
| Walk | 歩行アニメーション |
| Jump | ジャンプアニメーション |
| Fall | 落下アニメーション |
| Shot | ショットアニメーション |

**テスト**: AnimationIdによりアニメーションデータを取得できること

---

### REQ-31002-002: アニメーション状態（CharacterAnimationState）
**THE SYSTEM SHALL** キャラクターのアニメーション再生状態を管理する
- `current_animation: AnimationId` - 再生中のアニメーション
- `elapsed: f32` - 経過時間（秒）
- `looping: bool` - ループ再生フラグ
- `speed: f32` - 再生速度（1.0 = 通常速度）

**テスト**: CharacterAnimationState の各フィールドが正しく更新されること

---

### REQ-31002-003: キーフレーム定義（Keyframe）
**THE SYSTEM SHALL** 各パーツの時刻ごとの状態をキーフレームで定義する
- `time: f32` - キーフレーム時刻（秒）
- `position: Vec3` - ローカル位置（base_offsetからの相対）
- `rotation: f32` - 回転角度（度）

**備考**: キーフレームはパーツごとに独立して定義

---

### REQ-31002-004: アニメーションデータ構造
**THE SYSTEM SHALL** アニメーションを以下の構造で定義する
- `id: AnimationId` - アニメーション識別子
- `duration: f32` - アニメーション全体の長さ（秒）
- `looping: bool` - デフォルトでループ再生するか
- `keyframes: HashMap<PartKind, Vec<Keyframe>>` - パーツごとのキーフレーム配列

**データ参照**: `assets/animations/character_animations.ron`

---

### REQ-31002-005: キーフレーム補間
**WHEN** アニメーション再生中で経過時間がキーフレーム間にある
**THE SYSTEM SHALL** 線形補間でPartStateを計算する

```
t_normalized = (elapsed - kf_prev.time) / (kf_next.time - kf_prev.time)
position = lerp(kf_prev.position, kf_next.position, t_normalized)
rotation = lerp(kf_prev.rotation, kf_next.rotation, t_normalized)
```

**テスト**: 補間された値がPartStateに反映されること

---

### REQ-31002-006: アニメーション時間進行
**WHEN** ゲームループが更新される
**THE SYSTEM SHALL** アニメーション経過時間を進める

```
elapsed += delta_time * speed
if looping && elapsed >= duration:
    elapsed = elapsed % duration
elif !looping && elapsed >= duration:
    elapsed = duration
```

**テスト**: delta_time に応じて elapsed が増加すること

---

### REQ-31002-007: PartState更新
**WHEN** アニメーション経過時間が更新される
**THE SYSTEM SHALL** 各パーツの PartState を更新する
1. 現在時刻を囲む2つのキーフレームを特定
2. 線形補間で position, rotation を計算
3. PartState に値を設定

**備考**: キーフレームがないパーツは (0, 0, 0), 0 のまま

---

### REQ-31002-008: Idleアニメーション
**THE SYSTEM SHALL** Idle アニメーションで以下の動きを表現する
- 全体的に上下に浮遊するような微動
- 周期: 約2秒
- 振幅: ±2px程度

**データ参照**: `assets/animations/character_animations.ron#Idle`

---

## Extended Requirements (v0.2)

### REQ-31002-050: イージング補間
**WHERE** キーフレームにイージング関数が指定されている
**THE SYSTEM SHALL** 指定されたイージング関数で補間する
- Linear（デフォルト）
- EaseIn / EaseOut / EaseInOut
- SineIn / SineOut / SineInOut

---

### REQ-31002-051: アニメーション遷移
**WHEN** アニメーションが切り替わる
**THE SYSTEM SHALL** 現在の状態から新しいアニメーションへスムーズに遷移する
- ブレンド時間: 0.1〜0.2秒程度
- クロスフェード方式

---

### REQ-31002-052: アニメーションホットリロード
**WHERE** 開発モードである
**WHEN** アニメーション .ron ファイルが変更される
**THE SYSTEM SHALL** リアルタイムでアニメーションを再読み込みする

---

## 制約（Design by Contract）

### 事前条件
- CharacterPlugin がアプリに登録されている
- アニメーションデータ（.ron）が読み込まれている
- ArticulatedCharacter エンティティが存在する

### 事後条件
- elapsed が duration を超えない（looping=false の場合）
- PartState が補間された値で更新される

### 不変条件
- キーフレーム配列は time で昇順ソート済み
- duration > 0

---

## Component設計

### AnimationId（アニメーション識別子）
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationId {
    Idle,
    Walk,
    Jump,
    Fall,
    Shot,
}
```

### CharacterAnimationState（アニメーション状態Component）
```rust
#[derive(Component)]
pub struct CharacterAnimationState {
    pub current_animation: AnimationId,
    pub elapsed: f32,
    pub looping: bool,
    pub speed: f32,
}
```

### Keyframe（キーフレームデータ）
```rust
pub struct Keyframe {
    pub time: f32,
    pub position: Vec3,
    pub rotation: f32,
}
```

### CharacterAnimationData（アニメーションデータ）
```rust
pub struct CharacterAnimationData {
    pub id: AnimationId,
    pub duration: f32,
    pub looping: bool,
    pub keyframes: HashMap<PartKind, Vec<Keyframe>>,
}
```

### CharacterAnimations（全アニメーションResource）
```rust
#[derive(Resource)]
pub struct CharacterAnimations {
    pub animations: HashMap<AnimationId, CharacterAnimationData>,
}
```

---

## System設計

### advance_animation_system
- **トリガー**: Update
- **入力**: Time, Query<&mut CharacterAnimationState>
- **処理**: elapsed を delta_time * speed だけ進める
- **System Set**: アニメーション更新

### update_part_states_system
- **トリガー**: Update（advance_animation_system の後）
- **入力**: CharacterAnimations, Query<(&CharacterAnimationState, &Children)>, Query<(&PartDefinition, &mut PartState)>
- **処理**: キーフレーム補間で PartState を更新
- **System Set**: アニメーション更新

---

## データファイル

### assets/animations/character_animations.ron

```ron
CharacterAnimationsData(
    animations: {
        Idle: (
            duration: 2.0,
            looping: true,
            keyframes: {
                Head: [
                    (time: 0.0, position: (0.0, 0.0, 0.0), rotation: 0.0),
                    (time: 1.0, position: (0.0, 2.0, 0.0), rotation: 0.0),
                    (time: 2.0, position: (0.0, 0.0, 0.0), rotation: 0.0),
                ],
                Torso: [
                    (time: 0.0, position: (0.0, 0.0, 0.0), rotation: 0.0),
                    (time: 1.0, position: (0.0, 1.5, 0.0), rotation: 0.0),
                    (time: 2.0, position: (0.0, 0.0, 0.0), rotation: 0.0),
                ],
                // ... 他パーツ
            }
        ),
    }
)
```

---

## 依存関係

### 依存先
- [31001_parts_spec.md](31001_parts_spec.md) - パーツ構成・Component
- [20004_ecs_overview.md](../../2_architecture/20004_ecs_overview.md) - ECS設計原則

### 被依存
- プレイヤーシステム（アニメーション遷移トリガー）

---

## Change Log

### 2026-01-09 - v1.0.0（初版）
- 初版作成
