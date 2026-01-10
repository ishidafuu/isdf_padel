---
id: "B30102-003"
title: "AIサーブlet条件後のスタック修正"
type: "bugfix"
status: "todo"
priority: "high"
related_task: "30102"
spec_ids: ["REQ-30102-084"]
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null
tags: ["ai", "serve", "bugfix"]
parent_task_id: null
---

# B30102-003: AIサーブlet条件後のスタック修正

## 概要

AIサーブがlet条件（トスタイムアウト/高さ不足）発生後にスタックする問題を修正。

## 根本原因

1. **`hit_executed`フラグのリセット漏れ** - AI連続サーブ時にフラグがtrueのまま
2. **let条件後のタイマー再初期化漏れ** - `AiServeTimer`が再初期化されない
3. **タイマー初期化の競合チェック** - 既存タイマーがあると新規初期化スキップ

## 修正内容

### 1. serve_toss_timeout_system にAIタイマーリセット追加

**ファイル**: `project/src/systems/match_control/serve.rs`

```rust
// serve_toss_timeout_system 内
if is_timeout || is_too_low {
    commands.entity(toss_entity).despawn();
    serve_state.reset_for_retry();

    // 追加: AIタイマーリセット
    ai_serve_timer.toss_timer = None;
    ai_serve_timer.hit_executed = false;
}
```

### 2. ai_serve_timer_init_system の確認

**ファイル**: `project/src/systems/ai/serve.rs`

- `hit_executed = false` が初期化時に設定されていることを確認
- let条件後に再初期化されることを確認

## 対象仕様

- REQ-30102-084: トス打ち直し（let）

## 検証

```bash
# ヘッドレスシミュレーション 10回
for i in {1..10}; do cargo run --release --features headless; done
```

## 関連ファイル

- `project/src/systems/ai/serve.rs`
- `project/src/systems/match_control/serve.rs`
- `project/docs/3_ingame/301_match/30102_serve_spec.md`
