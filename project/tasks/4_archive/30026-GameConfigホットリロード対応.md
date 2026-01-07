# 30026: GameConfig ホットリロード対応

## メタ情報

- **Status**: done
- **Priority**: low
- **Type**: game-dev
- **Created**: 2026-01-07
- **Completed**: 2026-01-07
- **Spec**: 80101_game_constants.md

## 概要

ゲーム実行中に `game_config.ron` を編集・保存すると、自動でパラメータが反映される機能を追加する。

## 現状

- 起動時に `load_game_config()` で1回だけ読み込み
- 調整するたびにゲーム再起動が必要

## 実装方針

Bevyのアセットシステムを使用してホットリロードを実現。

### 変更対象ファイル

- `project/src/resource/config.rs`
- `project/src/main.rs`

### 実装ステップ

1. `GameConfigAsset` と `AssetLoader` を実装
2. `AssetPlugin` で `watch_for_changes` を有効化
3. 設定変更時に `ResMut<GameConfig>` を更新するシステム追加

## 完了条件

- [x] GameConfigAsset と AssetLoader を実装
- [x] AssetPlugin で watch_for_changes を有効化
- [x] 設定変更時に ResMut<GameConfig> を更新するシステム追加
- [x] game_config.ron 編集 → 保存 → 即座に反映を確認

## 備考

- 既存の `Res<GameConfig>` を使っている箇所は変更不要
- AssetEvent::Modified で変更検知し、Resource を更新する方式
- 詳細設計: `~/.claude/plans/floating-wandering-stroustrup.md`
