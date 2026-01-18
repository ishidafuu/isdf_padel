# AI Tactics Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2026-01-18

## 概要

AIの戦術選択（攻め/守り）を定義します。攻めた狙いによるミス発生と、守りの安全なプレイのバランスを実現します。

## 設計思想

### 自然なミス発生メカニズム

```
攻め狙い（ライン際）
  → 着地ズレ（REQ-30605-040）
    → コート外着地 → アウト（REQ-30901-001）
```

確率的なミスではなく、「攻めた狙い + 精度によるズレ」で自然にミスが発生。

### 戦術とリスク

| 戦術 | ターゲット | リスク | 効果 |
|------|-----------|--------|------|
| 攻め | ライン際（±マージン小） | 高（アウト可能性） | エースの可能性 |
| 守り | コート中央 | 低（アウト少ない） | ラリー継続 |

---

## Core Requirements (MVP v0.1)

MVPでは戦術選択なし。常に守りモード（コート中央狙い）。

---

## Extended Requirements (v0.4)

### 戦術タイプ定義

#### REQ-30303-001: TacticsType 列挙型
**THE SYSTEM SHALL** 以下の戦術タイプを定義する
```rust
pub enum TacticsType {
    Defensive,  // 守り: コート中央狙い
    Offensive,  // 攻め: ライン際狙い
}
```

### 戦術選択ロジック

#### REQ-30303-010: 戦術選択判断
**WHEN** AIがショットを打つ
**THE SYSTEM SHALL** 以下の条件で戦術を選択する

| 条件 | 選択する戦術 |
|------|-------------|
| 打点距離 ≤ `config.ai.optimal_distance` | Offensive（余裕がある） |
| 打点距離 > `config.ai.optimal_distance` | Defensive（苦しい体勢） |

```rust
fn select_tactics(distance_to_ball: f32, config: &AiConfig) -> TacticsType {
    if distance_to_ball <= config.optimal_distance {
        TacticsType::Offensive
    } else {
        TacticsType::Defensive
    }
}
```

#### REQ-30303-011: 攻め頻度の調整
**WHEN** 戦術を選択する
**THE SYSTEM SHALL** `config.ai.offensive_probability` の確率で Offensive を選択する
- Offensive 条件を満たしても、この確率でのみ攻める
- デフォルト: 0.6（60%）

### ショット戦術

#### REQ-30303-020: 守りショットのターゲット
**WHEN** TacticsType == Defensive
**THE SYSTEM SHALL** 相手コート中央をターゲットとする
- ターゲット位置: (opponent_baseline_x, 0, 0)
- 既存の REQ-30302-003 と同等

#### REQ-30303-021: 攻めショットのターゲット
**WHEN** TacticsType == Offensive
**THE SYSTEM SHALL** ライン際をターゲットとする
- ターゲットX: opponent_baseline_x（深い位置）
- ターゲットZ: ±(`config.court.width / 2.0` - `config.ai.offensive_margin`)
- Z方向はランダムで左右を選択

```rust
fn calculate_offensive_target(config: &Config) -> Vec3 {
    let z_direction = if rand::random::<bool>() { 1.0 } else { -1.0 };
    let target_z = z_direction * (config.court.width / 2.0 - config.ai.offensive_margin);
    Vec3::new(opponent_baseline_x, 0.0, target_z)
}
```

#### REQ-30303-022: 攻めマージンとアウトリスク
**THE SYSTEM SHALL** 攻めマージンとアウトリスクの関係を以下のように定義する

| マージン | ターゲットZ | 着地ズレ0.5mでアウト確率 |
|----------|-----------|-------------------------|
| 0.5m | ±4.5m（サイドラインから0.5m内側） | 約50% |
| 1.0m | ±4.0m（サイドラインから1.0m内側） | 約25% |
| 1.5m | ±3.5m（サイドラインから1.5m内側） | 約10% |

デフォルト: `config.ai.offensive_margin` = 0.8m

### サーブ戦術

#### REQ-30303-030: サーブ戦術選択
**WHEN** AIがサーブを打つ
**THE SYSTEM SHALL** `config.ai.serve_offensive_probability` の確率で攻めサーブを選択する
- デフォルト: 0.5（50%）

#### REQ-30303-031: 守りサーブのターゲット
**WHEN** サーブ戦術 == Defensive
**THE SYSTEM SHALL** サービスエリア中央をターゲットとする
- ターゲットX: サービスライン位置（コート中央）
- ターゲットZ: 対角サービスエリアの中央

#### REQ-30303-032: 攻めサーブのターゲット
**WHEN** サーブ戦術 == Offensive
**THE SYSTEM SHALL** サービスエリア端をターゲットとする
- ターゲットX: サービスライン位置
- ターゲットZ: サービスエリア端（±margin）
- margin: `config.ai.serve_offensive_margin`（デフォルト: 0.3m）

```rust
fn calculate_serve_target(tactics: TacticsType, config: &Config) -> Vec3 {
    let service_x = config.court.service_line_x;
    let service_z = match tactics {
        TacticsType::Defensive => config.court.service_area_center_z,
        TacticsType::Offensive => {
            let edge_z = config.court.service_area_width / 2.0 - config.ai.serve_offensive_margin;
            if rand::random::<bool>() { edge_z } else { -edge_z }
        }
    };
    Vec3::new(service_x, 0.0, service_z)
}
```

### フォルト発生メカニズム

#### REQ-30303-040: サーブフォルトの発生
**WHEN** 攻めサーブを打つ
**AND** 着地ズレ（REQ-30605-040）が適用される
**IF** 着地地点がサービスエリア外
**THE SYSTEM SHALL** フォルトと判定する

**依存**: REQ-30901-002（サーブのサービスエリア判定）

---

## データ定義

### AI戦術パラメータ

| パラメータ | データパス | デフォルト値 | 説明 |
|-----------|-----------|-------------|------|
| 最適距離 | `config.ai.optimal_distance` | 1.2 | この距離以内なら攻め可能 |
| 攻め確率 | `config.ai.offensive_probability` | 0.6 | 条件を満たした時の攻め確率 |
| 攻めマージン | `config.ai.offensive_margin` | 0.8 | ライン際からの内側マージン(m) |
| サーブ攻め確率 | `config.ai.serve_offensive_probability` | 0.5 | 攻めサーブの確率 |
| サーブ攻めマージン | `config.ai.serve_offensive_margin` | 0.3 | サービスエリア端からのマージン(m) |

### コートパラメータ（参照）

| パラメータ | 値 | 説明 |
|-----------|---|------|
| コート幅 | 10.0m | サイドラインZ = ±5.0m |
| サービスエリア幅 | 5.0m | サービスエリアZ = ±2.5m |

**参照**: `80101_game_constants.md#court_config`

---

## 依存関係

### 依存先
- [30605_trajectory_calculation_spec.md](../306_shot_system/30605_trajectory_calculation_spec.md) - 着地ズレ計算
- [30901_point_judgment_spec.md](../309_referee/30901_point_judgment_spec.md) - アウト判定

### 依存元
- [30302_ai_shot_spec.md](30302_ai_shot_spec.md) - AIショット（戦術を使用）
- [30102_serve_spec.md](../301_match/30102_serve_spec.md) - AIサーブ（戦術を使用）

---

## 実装ガイド

### 既存仕様との統合

1. **30302_ai_shot_spec.md との統合**
   - REQ-30302-003（コート中央狙い）→ REQ-30303-020（守り）に置き換え
   - REQ-30302-054（戦略的配球 v0.2）→ REQ-30303-021（攻め）で実現

2. **30102_serve_spec.md との統合**
   - REQ-30102-071（サーブ方向ランダム化）→ REQ-30303-030〜032 に置き換え

### System設計

```rust
pub fn ai_tactics_system(
    mut query: Query<(&AiController, &Position, &mut AiState)>,
    ball_query: Query<&Position, With<Ball>>,
    config: Res<GameConfig>,
) {
    for (ai, ai_pos, mut ai_state) in query.iter_mut() {
        let ball_pos = ball_query.single();
        let distance = ai_pos.0.distance(ball_pos.0);
        
        // 戦術選択
        ai_state.current_tactics = select_tactics(distance, &config.ai);
    }
}
```

---

## シミュレーション例

### 攻めショットによるアウト

```
状況: AIが optimal_distance 以内で打つ
  → TacticsType::Offensive 選択
  → ターゲット: サイドライン際（Z = 4.2m、サイドライン5.0m）
  → 着地ズレ: accuracy 0.7 → deviation 0.6m
  → 実際の着地: Z = 4.8m or 5.2m（50%でアウト）
```

### 守りショットの安全性

```
状況: AIが遠い位置から打つ
  → TacticsType::Defensive 選択
  → ターゲット: コート中央（Z = 0m）
  → 着地ズレ: accuracy 0.5 → deviation 1.0m
  → 実際の着地: Z = ±1.0m（コート内に収まる）
```

---

## Change Log

### 2026-01-18 - v1.0.0
- 初版作成
- 戦術選択（攻め/守り）の定義
- ショット戦術（REQ-30303-020〜022）
- サーブ戦術（REQ-30303-030〜040）
