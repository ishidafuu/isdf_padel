---
id: "R30000-027"
title: "GameRng リソース実装・乱数呼び出し統一"
type: "refactor"
status: "todo"
priority: "high"
related_task: null
spec_ids: ["REQ-77103-007"]
blocked_by: []
blocks: ["R30000-028"]
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md"
tags: ["rng", "replay", "simulation", "determinism"]
parent_task_id: null
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: null
---

# Task R30000-027: GameRng リソース実装・乱数呼び出し統一

## Summary

シード可能な乱数リソースを実装し、ゲーム全体の乱数呼び出しを統一する。
リプレイの完全再現とヘッドレスシミュレーションの再現性確保のための基盤。

## Related Specifications

- `project/docs/7_tools/71_simulation/77103_replay_spec.md#REQ-77103-007`
- プラン: `/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md`

## Progress

### Completed

(なし)

## Next Actions

1. `project/src/resource/game_rng.rs` を新規作成
2. `project/src/resource/mod.rs` に公開追加
3. `systems/ai/movement.rs` の `rand::rng()` を置換
4. `systems/ai/shot.rs` の `rand::rng()` を置換
5. `systems/ai/serve.rs` の `rand::rng()` を置換
6. `replay/mod.rs` のシード生成を置換
7. `simulation/simulation_runner.rs` でRNG初期化追加

## Dependencies

- **Blocked By:** なし
- **Blocks:** R30000-028

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] 同一シードで2回実行し結果一致を確認
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

乱数使用箇所:
- `systems/ai/movement.rs:117` - AI予測誤差
- `systems/ai/shot.rs:23` - ショット方向ブレ
- `systems/ai/serve.rs:68` - サーブ遅延・方向
- `replay/mod.rs:54-57` - シード生成

---

## Detailed Implementation Plan

> 以下はプランファイル Phase 1 の内容です。

### GameRng リソース作成

**ファイル**: `project/src/resource/game_rng.rs` (新規)

```rust
use bevy::prelude::*;
use rand::{rngs::StdRng, SeedableRng, Rng};

#[derive(Resource)]
pub struct GameRng {
    rng: StdRng,
    seed: u64,
}

impl GameRng {
    pub fn from_seed(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed,
        }
    }

    pub fn from_entropy() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        Self::from_seed(seed)
    }

    pub fn seed(&self) -> u64 { self.seed }

    pub fn random_range<T, R>(&mut self, range: R) -> T
    where T: rand::distributions::uniform::SampleUniform,
          R: rand::distributions::uniform::SampleRange<T> {
        self.rng.gen_range(range)
    }
}
```

### シミュレーターでのRNG初期化

**ファイル**: `simulation/simulation_runner.rs`

```rust
let game_rng = if let Some(seed) = self.config.seed {
    GameRng::from_seed(seed)
} else {
    GameRng::from_entropy()
};
app.insert_resource(game_rng);
```
