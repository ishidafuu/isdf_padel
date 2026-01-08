# P003-005: PlayerBundle 色決定の外部化

## 概要

PlayerBundle 内でハードコーディングされているプレイヤー色を外部データ化する。

## 現状の問題

```rust
// components/mod.rs:370-375
// Player1: 青、Player2: 赤
let color = if id == 1 {
    Color::srgb(0.2, 0.4, 0.8)  // ❌ ハードコーディング
} else {
    Color::srgb(0.8, 0.2, 0.2)  // ❌ ハードコーディング
};
```

**違反**: CLAUDE.md ルール3「ハードコーディング禁止原則」

## 対応方針

### 1. データ定義（RON ファイル）

```ron
// project/assets/config/player_visual.ron
PlayerVisualConfig(
    colors: {
        Player1: (r: 0.2, g: 0.4, b: 0.8),
        Player2: (r: 0.8, g: 0.2, b: 0.2),
    },
    size: (width: 40.0, height: 60.0),
)
```

### 2. コード修正

```rust
impl PlayerBundle {
    pub fn new(id: u8, position: Vec3, config: &PlayerVisualConfig) -> Self {
        let court_side = CourtSide::from_id(id);
        let color = config.get_color(court_side);
        // ...
    }
}
```

## 影響範囲

- `project/src/components/mod.rs` - PlayerBundle
- `project/src/main.rs` - プレイヤー生成箇所
- `project/assets/config/` - 新規データファイル

## 完了条件

- [ ] PlayerVisualConfig データ定義作成
- [ ] RON ファイル作成
- [ ] PlayerBundle から色のハードコーディング削除
- [ ] テスト通過

## メタデータ

- **優先度**: 中
- **種別**: リファクタリング
- **依存**: なし
- **並列実行**: 可
