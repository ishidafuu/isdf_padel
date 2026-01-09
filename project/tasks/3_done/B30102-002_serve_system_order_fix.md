# B30102-002: サーブシステム実行順序・トス表示バグ修正

## 概要

サーブ機能が正常に動作しない問題の修正。

## 問題

1. **Vキーを押してもトスが発生しない** → ✅ 修正済み
2. **トスボールが表示されない（即座に消える）** → ✅ 修正済み
3. **サーブ後のボールがあらぬ方向に飛んでいく** → 🟡 原因特定済（別タスクで対応）

## 原因分析

### 問題1: 入力が反映されない

**原因**: システム実行順序が保証されていなかった
- `human_input_system` と `serve_toss_input_system` が別々のプラグインで登録
- Bevyでは異なるプラグインのUpdateシステムは順序保証なし
- `serve_toss_input_system` が先に実行されると `input_state.shot_pressed` は常に `false`

**修正**: `GameSystemSet` 導入
- `Input` セット: 入力読み取りシステム
- `GameLogic` セット: ゲームロジックシステム
- `Input.before(GameLogic)` で順序保証

### 問題2: トスボールが即座に消える

**原因**: `serve_toss_timeout_system` の判定ロジック不備
- トスボール初期高さ: 1.0m
- `hit_height_min`: 1.80m
- ボールがスポーン直後に「低すぎる」判定されてフォルト

**修正**: 落下中のみフォルト判定
```rust
let is_falling = velocity.value.y < 0.0;
let is_too_low = is_falling && ball_height < config.serve.hit_height_min;
```

## 変更ファイル

| ファイル | 変更内容 |
|---------|---------|
| `src/systems/mod.rs` | `GameSystemSet` enum 追加 |
| `src/main.rs` | SystemSet順序設定、Input/GameLogicセット分離 |
| `src/systems/match_flow.rs` | サーブシステムを `GameLogic` セットに追加 |
| `src/systems/ai_serve.rs` | AIサーブシステムを `GameLogic` セットに追加 |
| `src/systems/serve.rs` | `serve_toss_timeout_system` に Velocity クエリ追加、落下判定修正 |

## Progress

### Completed

- [x] 問題1の原因特定と修正（SystemSet導入）
- [x] 問題2の原因特定と修正（落下中のみフォルト判定）
- [x] ビルド成功確認
- [x] テスト148件パス確認
- [x] トスボール表示確認
- [x] 問題3の原因特定（サーブ方向異常）

## Next Actions

なし（本タスク完了、問題3の修正は別タスクとして切り出し）

## メモ

### 問題3の原因（調査完了）

- **根本原因**: サーブ後に `ShotEvent` を発行 → `shot_direction_system` がラリー用の弾道計算でボール速度を上書き
- **ログ証拠**:
  ```
  Serve hit success: velocity=Vec3(9.66, -2.59, 0.0)    ← サーブ直後（正しい）
  shot_direction: velocity=Vec3(7.81, 4.71, 3.90)       ← 上書き後（異常）
  ```

### 設計課題（別タスクへ）

- サーブは独自の弾道計算（高打点・下向き角度）を持つ
- ジャンプショット（スマッシュ）も同様に高打点から打つが、現在は未実装
- **本来あるべき姿**: `trajectory_calculator` が打点高さを考慮し、サーブ・ジャンプショット・通常ショットを統一的に処理
- **後続タスク**: サーブ/スマッシュ弾道統合の設計・実装

## 関連

- 仕様: `project/docs/3_ingame/301_match/30102_serve_spec.md`
- 関連タスク: B30102-001（過去のサーブ角度修正）
