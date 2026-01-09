---
id: "R30501-002"
title: "コメント内 Player1/Player2 → Left/Right 統一"
type: "refactor"
status: "todo"
priority: "low"
blocked_by: ["R30501-003"]
blocks: []
created_at: "2026-01-09T16:00:00+09:00"
---

# R30501-002: コメント内 Player1/Player2 → Left/Right 統一

## 概要

コメント内の `Player1`, `Player2` 表記を `Left`, `Right` に統一する。

## 背景・目的

- R30501-001 で `CourtSide` enum を `Left/Right` に変更済み
- コメント内に旧表記 (`Player1`, `Player2`) が残存している
- コードとコメントの整合性を取り、可読性を向上させる

## 関連タスク

- R30501-001: CourtSide enum リファクタ（完了）

## 影響範囲

| ファイル | 箇所数 |
|---------|-------|
| `shot_direction.rs` | ~10 |
| `scoring.rs` | ~15 |
| `match_flow.rs` | ~5 |
| `config.rs` | ~10 |
| `ai_movement.rs` | ~2 |
| `trajectory_calculator.rs` | ~8 |
| その他 | ~10 |

## 作業内容

### 1. コメント置換（全ファイル）
- `Player1` → `Left` （コート側を示す場合）
- `Player2` → `Right` （コート側を示す場合）
- `Player1側` → `Left側`
- `Player2側` → `Right側`

### 2. テスト名・関数名の更新
- テスト関数名内の `player1`, `player2` → `left`, `right`

### 注意事項

- **PlayerConfig など、プレイヤー自体を示す場合は変更しない**
- コート側を示す文脈でのみ変更する

## 検証方法

1. `cargo build` - コンパイルエラーなし
2. `cargo test` - 全テストパス
3. grep で残存確認

## ステータス

- [ ] コメント置換
- [ ] テスト名更新
- [ ] 検証
- [ ] コミット
