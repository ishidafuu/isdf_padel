---
id: "R30501-003"
title: "WallType enum リファクタ（BackWall1P/2P → BackWallLeft/Right）"
type: "refactor"
status: "todo"
priority: "medium"
blocked_by: []
blocks: ["R30501-002"]
created_at: "2026-01-09T16:00:00+09:00"
---

# R30501-003: WallType enum リファクタ（BackWall1P/2P → BackWallLeft/Right）

## 概要

`WallType::BackWall1P/BackWall2P` を `WallType::BackWallLeft/BackWallRight` に変更する。

## 背景・目的

- R30501-001 で `CourtSide` enum を `Left/Right` に変更済み
- `WallType` にも旧表記 (`1P`, `2P`) が残存している
- enum 値の命名規則を統一し、コードベース全体の整合性を向上させる

## 関連タスク

- R30501-001: CourtSide enum リファクタ（完了）
- R30501-002: コメント内統一（予定）

## 影響範囲

| ファイル | 内容 |
|---------|------|
| `core/events.rs` | enum 定義 |
| `core/wall.rs` | 壁反射ロジック、テスト |
| `systems/ball_trajectory.rs` | テストコメント |

## 作業内容

### 1. enum 定義変更
- `core/events.rs`
```rust
pub enum WallType {
    // ...
    BackWallLeft,   // 旧: BackWall1P
    BackWallRight,  // 旧: BackWall2P
}
```

### 2. 一括置換（全ファイル）
- `WallType::BackWall1P` → `WallType::BackWallLeft`
- `WallType::BackWall2P` → `WallType::BackWallRight`
- `BackWall1P` → `BackWallLeft`（パターンマッチ内）
- `BackWall2P` → `BackWallRight`（パターンマッチ内）

### 3. コメント更新
- `1P側`, `2P側` → `Left側`, `Right側`

## 検証方法

1. `cargo build` - コンパイルエラーなし
2. `cargo test` - 全テストパス
3. grep で残存確認

## ステータス

- [ ] enum 定義変更
- [ ] 一括置換
- [ ] コメント更新
- [ ] テスト実行
- [ ] コミット
