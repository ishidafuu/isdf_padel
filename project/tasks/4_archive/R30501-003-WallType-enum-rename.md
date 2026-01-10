# R30501-003: WallType enum リファクタ（BackWall1P/2P → BackWallLeft/Right）

## 概要

`WallType::BackWall1P/BackWall2P` を `WallType::BackWallLeft/BackWallRight` に変更し、R30501-001 の CourtSide リファクタと命名を統一する。

## 背景・目的

- R30501-001 で `CourtSide::Player1/Player2` → `CourtSide::Left/Right` に変更した
- `BackWall1P/2P` も同様に「物理的位置」を表す名前に統一
- 座標系との対応: BackWallLeft = X < 0（Left側）, BackWallRight = X > 0（Right側）

## 関連仕様

- REQ-30501-006: コート区分（Left/Right）
- R30501-001: CourtSide enum リファクタ（完了）

## 影響範囲

| 項目 | 数 |
|------|-----|
| 変更ファイル | 2 |
| 変更箇所 | ~17 |
| 単純置換 | ~15 |
| ロジック検証 | ~2 |

## 作業内容

### 1. 仕様書更新
- `project/docs/2_architecture/20005_event_system.md`（不要: 既に Left/Right/Front/Back 表記）

### 2. enum 定義変更
- `project/src/core/events.rs`
```rust
pub enum WallType {
    LeftWall,       // 左壁（Z = -Court.Width/2）
    RightWall,      // 右壁（Z = +Court.Width/2）
    BackWallLeft,   // 後壁（Left側、X = -Court.Depth/2）← BackWall1P
    BackWallRight,  // 後壁（Right側、X = +Court.Depth/2）← BackWall2P
    Ceiling,        // 天井（Y = Court.CeilingHeight）
}
```

### 3. 一括置換（全ファイル）
- `WallType::BackWall1P` → `WallType::BackWallLeft`
- `WallType::BackWall2P` → `WallType::BackWallRight`
- コメント: `1P側` → `Left側`, `2P側` → `Right側`

### 4. ロジック検証（重点確認）

| ファイル | 確認項目 |
|---------|---------|
| `events.rs` | `normal()`: BackWallLeft=+X, BackWallRight=-X 維持 |
| `wall.rs` | 境界判定位置が変わらないこと |

## 重要ファイル

1. `project/src/core/events.rs` - enum定義（起点）
2. `project/src/core/wall.rs` - 使用箇所+テスト

## 検証方法

1. `cargo build` - コンパイルエラーなし
2. `cargo test` - 全テストパス

## リスク

| リスク | 軽減策 |
|-------|-------|
| 置換漏れ | Rust コンパイラが検出 |
| ロジック誤り | 既存テストで検出 |

## ステータス

- [x] enum 定義変更
- [x] 一括置換
- [x] コメント更新
- [x] テスト実行
- [x] コミット

## 完了結果

- **変更ファイル数**: 2
- **テスト結果**: 148 passed
