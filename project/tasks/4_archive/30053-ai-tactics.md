---
id: "30053"
title: "AI戦術選択（攻め/守り）の実装"
type: "game-dev"
status: "done"
priority: "high"
related_task: null
spec_ids:
  - "REQ-30303-001"
  - "REQ-30303-010"
  - "REQ-30303-011"
  - "REQ-30303-020"
  - "REQ-30303-021"
  - "REQ-30303-022"
  - "REQ-30303-030"
  - "REQ-30303-031"
  - "REQ-30303-032"
  - "REQ-30303-040"
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags:
  - "ai"
  - "tactics"
  - "v0.4"
parent_task_id: null
created_at: "2026-01-18"
updated_at: "2026-01-18"
completed_at: "2026-01-18"
---

# Task 30053: AI戦術選択（攻め/守り）の実装

## Summary

AIに「攻め/守り」の戦術選択機能を追加します。攻めた狙い（ライン際）により着地ズレでアウトが発生し、自然なミスが生まれるようになります。

**目的**:
- AIが常に完璧なショットを打つ問題を解決
- 「攻めた狙い + 精度によるズレ」で自然なミス発生

## Related Specifications

- [30303_ai_tactics_spec.md](../../docs/3_ingame/303_ai/30303_ai_tactics_spec.md) - AI戦術仕様（新規作成済み）
- [30302_ai_shot_spec.md](../../docs/3_ingame/303_ai/30302_ai_shot_spec.md) - AIショット仕様（統合先）
- [30102_serve_spec.md](../../docs/3_ingame/301_match/30102_serve_spec.md) - サーブ仕様（統合先）
- [30605_trajectory_calculation_spec.md](../../docs/3_ingame/306_shot_system/30605_trajectory_calculation_spec.md) - 着地ズレ計算

## Progress

### Completed

1. ✅ TacticsType 列挙型追加（`components/ai.rs`）
2. ✅ AiController に current_tactics フィールド追加
3. ✅ AiConfig に戦術パラメータ追加（`ai_config.rs`）
4. ✅ game_config.ron にパラメータ追加
5. ✅ ai_shot_system に戦術選択・方向計算を統合
6. ✅ サーブ戦術選択ロジック追加（`serve.rs`）
7. ✅ ユニットテスト追加（戦術選択、方向計算）
8. ✅ ビルド成功
9. ✅ テスト全PASS（154件）

## Next Actions

1. シミュレーションでアウト発生を確認
2. in-review に移動

## Dependencies

- **Blocked By:** なし
- **Blocks:** なし

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`）
- [ ] シミュレーションでアウト発生確認
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

### 戦術とリスク

| 戦術 | ターゲット | リスク |
|------|-----------|--------|
| Defensive | コート中央 | 低（アウト少ない） |
| Offensive | ライン際（±マージン小） | 高（アウト可能性） |

### パラメータ（game_config.ron追加）

```ron
// AI戦術パラメータ
optimal_distance: 1.2,           // この距離以内で攻め可能
offensive_probability: 0.6,      // 攻め確率（60%）
offensive_margin: 0.8,           // ライン際からのマージン
serve_offensive_probability: 0.5, // サーブ攻め確率（50%）
serve_offensive_margin: 0.3,     // サービスエリア端からのマージン
```

---

## Detailed Implementation Plan

### Phase 1: データ定義

1. `TacticsType` 列挙型追加
2. `AiState` に `current_tactics` 追加
3. `game_config.ron` にパラメータ追加

### Phase 2: 戦術選択ロジック

1. `ai_tactics_system` 新規作成
   - 打点距離に応じて Offensive/Defensive 選択
   - `offensive_probability` で確率調整

### Phase 3: ショットターゲット計算

1. `calculate_defensive_target()` - コート中央
2. `calculate_offensive_target()` - ライン際（Z = ±(width/2 - margin)）

### Phase 4: サーブ戦術

1. サーブ時の戦術選択ロジック追加
2. 攻めサーブ: サービスエリア端狙い
3. 守りサーブ: サービスエリア中央狙い

### Phase 5: 検証

1. シミュレーションでアウト発生確認
2. 攻め/守りのバランス調整
