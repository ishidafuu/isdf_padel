---
id: "R30000-031"
title: "DebugConfig・ログカテゴリ制御"
type: "refactor"
status: "todo"
priority: "medium"
related_task: null
spec_ids: []
blocked_by: []
blocks: ["R30000-032"]
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md"
tags: ["debug", "logging", "config"]
parent_task_id: null
created_at: "2026-01-11"
updated_at: "2026-01-11"
completed_at: null
---

# Task R30000-031: DebugConfig・ログカテゴリ制御

## Summary

カテゴリ別のログレベル制御を実装する。
「AIだけ詳細」「物理は要らない」といった使い分けを可能にする。

## Related Specifications

- プラン: `/Users/ishidafuu/.claude/plans/expressive-seeking-gizmo.md`

## Progress

### Completed

(なし)

## Next Actions

1. `simulation/config.rs` に DebugConfig 構造体追加
2. SimulationFileConfig に debug フィールド追加
3. `simulation/debug_logger.rs` 新規作成
4. `simulation_debug.ron` に debug セクション追加
5. `simulation_runner.rs` のデバッグシステム更新

## Dependencies

- **Blocked By:** なし
- **Blocks:** R30000-032

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] log_ai: true 時に AI ログのみ出力されることを確認
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

現在は verbose: true/false の二択のみ。

---

## Detailed Implementation Plan

### DebugConfig 構造体

**ファイル**: `project/src/simulation/config.rs`

```rust
#[derive(Clone, Debug, Deserialize, Default)]
pub struct DebugConfig {
    pub log_ai: bool,           // AI行動ログ
    pub log_physics: bool,      // 物理イベントログ
    pub log_scoring: bool,      // 得点イベントログ
    pub log_state: bool,        // 状態遷移ログ
    pub log_interval_secs: f32, // 定期ログ間隔（0で無効）
    pub log_file: Option<String>, // ログファイル出力パス
}
```

### 設定ファイル

```ron
debug: (
    log_ai: true,
    log_physics: false,
    log_scoring: true,
    log_state: true,
    log_interval_secs: 1.0,
    log_file: Some("debug_log.txt"),
),
```

### DebugLogger リソース

```rust
#[derive(Resource)]
pub struct DebugLogger {
    config: DebugConfig,
    file_handle: Option<File>,
}

impl DebugLogger {
    pub fn log_ai(&mut self, message: &str);
    pub fn log_physics(&mut self, message: &str);
    pub fn log_scoring(&mut self, message: &str);
    pub fn log_state(&mut self, message: &str);
}
```
