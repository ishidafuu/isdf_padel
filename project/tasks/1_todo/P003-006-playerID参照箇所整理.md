# P003-006: player.id 参照箇所の整理

## 概要

コードベース内の `player.id` 参照を整理し、CourtSide ベースまたはマーカーコンポーネントベースに統一する。

## 現状

`player.id` を使った分岐が複数箇所に存在：

```rust
// systems/movement.rs:89
let (z_min, z_max) = get_player_z_bounds(player.id, &config);

// systems/shot_input.rs:50
info!("Player {} shot ignored: knockback active", player.id);

// systems/serve.rs:34-35
CourtSide::Player1 => 1,
CourtSide::Player2 => 2,
```

## 分類

### A. ログ出力用（許容）
```rust
info!("Player {} shot ignored", player.id);  // デバッグ用、影響なし
```

### B. ロジック分岐用（要検討）
```rust
let (z_min, z_max) = get_player_z_bounds(player.id, &config);
// → CourtSide ベースに変更可能
```

### C. ID ↔ CourtSide 変換（整理対象）
```rust
match player.id { 1 => CourtSide::Player1, _ => CourtSide::Player2 }
// → Player コンポーネントに court_side があるので直接使用可能
```

## 対応方針

1. **Player.id の用途を明確化**
   - ログ表示用の識別子として維持
   - ロジック分岐は `Player.court_side` を使用

2. **ヘルパー関数の統一**
   ```rust
   // Before: get_player_z_bounds(player.id, &config)
   // After:  get_player_z_bounds(player.court_side, &config)
   ```

3. **不要な変換を削除**
   ```rust
   // Before
   let side = match player.id { 1 => CourtSide::Player1, ... };

   // After
   let side = player.court_side;  // 既に持っている
   ```

## 影響範囲

- `project/src/systems/movement.rs`
- `project/src/systems/knockback.rs`
- `project/src/systems/ai_movement.rs`
- `project/src/systems/serve.rs`
- `project/src/systems/match_flow.rs`

## 完了条件

- [ ] player.id による分岐を player.court_side に置換
- [ ] ヘルパー関数のシグネチャを CourtSide ベースに変更
- [ ] 不要な id ↔ CourtSide 変換を削除
- [ ] テスト通過

## メタデータ

- **優先度**: 低
- **種別**: リファクタリング
- **依存**: P003-004（スコアリングECS化）完了後が望ましい
- **並列実行**: 可（ただし P003-004 との競合に注意）
