# R30501-001: CourtSide enum リファクタ（Player1/Player2 → Left/Right）

## 概要

`CourtSide::Player1/Player2` を `CourtSide::Left/Right` に変更し、「物理的位置」を表す名前にする。

## 背景・目的

- 現在の `Player1/Player2` は「プレイヤーID」と「物理的位置」を混同している
- チェンジサイド実装時に Player1 が Right 側に移動すると意味が破綻する
- 座標系との対応: X < 0 = Left（現Player1）, X >= 0 = Right（現Player2）

## 関連仕様

- REQ-30501-006: コート区分（1P/2P）

## 影響範囲

| 項目 | 数 |
|------|-----|
| 変更ファイル | 20 |
| 変更箇所 | ~200 |
| 単純置換 | ~130 |
| ロジック検証 | ~57 |
| テスト修正 | ~30 |

## 作業内容

### 1. 仕様書更新
- `project/docs/3_ingame/305_court/30501_court_spec.md`
- REQ-30501-006: 「1Pコート」→「Left コート」、「2Pコート」→「Right コート」

### 2. enum 定義変更
- `project/src/core/court.rs`
```rust
pub enum CourtSide {
    #[default]
    Left,   // 左コート側（X < net_x）
    Right,  // 右コート側（X >= net_x）
}
```

### 3. 一括置換（全ファイル）
- `CourtSide::Player1` → `CourtSide::Left`
- `CourtSide::Player2` → `CourtSide::Right`

### 4. ロジック検証（重点確認）

| ファイル | 確認項目 |
|---------|---------|
| `scoring.rs` | `Left as usize = 0`, `Right as usize = 1` 維持 |
| `trajectory_calculator.rs` | `sign()`: Left=+1.0, Right=-1.0 |
| `ai_movement.rs` | Left → `x < 0`, Right → `x > 0` |
| `boundary.rs` | ネット通過制限の方向 |

## 重要ファイル

1. `project/src/core/court.rs` - enum定義（起点）
2. `project/src/resource/scoring.rs` - 配列インデックス制約
3. `project/src/systems/trajectory_calculator.rs` - sign() Trait
4. `project/docs/3_ingame/305_court/30501_court_spec.md` - 仕様書

## 検証方法

1. `cargo build` - コンパイルエラーなし
2. `cargo test` - 全テストパス
3. ゲーム起動して動作確認

## リスク

| リスク | 軽減策 |
|-------|-------|
| 置換漏れ | Rust コンパイラが検出 |
| ロジック誤り | 既存テストで検出 |
| 仕様書不整合 | 仕様書を先に更新 |

## ステータス

- [x] 仕様書更新
- [x] enum 定義変更
- [x] 一括置換
- [x] ロジック検証
- [x] テスト実行
- [x] コミット

## 完了結果

- **変更ファイル数**: 24
- **テスト結果**: 148 passed
- **コミット**: 983ceb8
