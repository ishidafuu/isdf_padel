---
id: "30058"
title: "ラリー要約・異常フラグ"
type: "game-dev"
status: "done"
priority: "medium"
related_task: null
spec_ids:
  - "REQ-77201-003"
  - "REQ-77201-004"
  - "REQ-77201-005"
  - "REQ-77201-006"
blocked_by:
  - "30057"
blocks:
  - "30059"
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/optimized-strolling-puppy.md"
tags:
  - "telemetry"
  - "llm-qa"
  - "phase2"
parent_task_id: null
created_at: "2026-01-16T16:00:00+09:00"
updated_at: "2026-01-16T18:00:00+09:00"
completed_at: "2026-01-16T19:00:00+09:00"
---

# Task 30058: ラリー要約・異常フラグ

## Summary

Pointイベントでラリーを分割し、各ラリーの統計を計算。異常値を検出してフラグ付け。

## Related Specifications

- `project/docs/7_tools/71_simulation/77201_narrative_spec.md`

## Progress

### Completed

- ✅ `project/src/bin/trace_narrator/analyzer.rs` 新規作成
- ✅ ラリー境界検出ロジック実装（REQ-77201-003）
  - Pointイベントでラリーを区切る
  - 開始/終了フレーム、持続時間を記録
- ✅ ラリー統計計算（REQ-77201-004）
  - ショット数（全体、P1、P2別）
  - 各プレイヤーの平均パワー/精度/スピン
  - バウンス数、壁反射数
- ✅ PhysicsAnomalyハイライト処理（REQ-77201-005）
  - Warning → ⚠️、Error → 🔴 の絵文字表現
  - 異常イベントの収集と表示
- ✅ 統計的異常検出（REQ-77201-006）
  - 平均±閾値×標準偏差を超える値を検出
  - パワーとスピンの外れ値検出
  - デフォルト閾値: 1.5σ
- ✅ main.rsでanalyzerを統合
- ✅ テスト4件追加・全PASS（合計8件）

## Dependencies

- **Blocked By:** 30057 (ナラティブ変換基盤) ✅
- **Blocks:** 30059

## 完了チェックリスト

> このタスクは in-review 経由必須

- [x] ビルド成功（`cargo build`）
- [x] テスト全PASS（`cargo test`） - 8テスト通過
- [x] in-review に移動済み
- [x] レビュー完了

## 実装詳細

### ファイル構成

```
project/src/bin/trace_narrator/
├── analyzer.rs     # ラリー解析・異常検出（新規）
├── parser.rs       # JSONL/JSON配列形式の両対応パーサー
└── types.rs        # serde対応データ構造
```

### 主要構造体

```rust
// ラリー情報
struct Rally {
    number: u32,              // ラリー番号
    start_frame: u64,
    end_frame: u64,
    duration_secs: f32,
    winner: u8,
    end_reason: String,
    shots: Vec<ShotInfo>,
    bounce_count: u32,
    wall_reflect_count: u32,
    anomalies: Vec<Anomaly>,
    stats: RallyStats,
}

// ラリー統計
struct RallyStats {
    shot_count: u32,
    p1_shot_count: u32,
    p2_shot_count: u32,
    p1_avg_power: f32,
    p2_avg_power: f32,
    p1_avg_accuracy: f32,
    p2_avg_accuracy: f32,
    p1_avg_spin: f32,
    p2_avg_spin: f32,
}

// 異常情報
struct Anomaly {
    frame: u64,
    severity: AnomalySeverity,  // Warning / Error
    description: String,
    expected: Option<f32>,
    actual: Option<f32>,
}
```

### CLI出力例

```
=== Rally Analysis ===
Total rallies: 3
Physics anomalies: 1
Statistical anomalies: 2

--- Rally Details ---

Rally 1 (Frame 0-450): P1 wins (DoubleBounce) - 5.50s
  Shots: 4 (P1: 2, P2: 2)
  Bounces: 6, Wall reflects: 2
  P1 avg: power=0.65, accuracy=0.85, spin=0.20
  P2 avg: power=0.72, accuracy=0.78, spin=-0.10
  ⚠️ Frame 280: velocity_spike

--- Statistical Anomalies (threshold: 1.5σ) ---
  ⚠️ Frame 180: Power outlier (P2): 0.95 (mean=0.68, std=0.12)
```

## メモ

- 次タスク30059でマークダウン出力機能を実装予定
