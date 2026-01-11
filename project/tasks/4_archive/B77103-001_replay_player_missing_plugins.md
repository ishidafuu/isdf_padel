# B77103-001: リプレイプレイヤーのプラグイン設定漏れ修正

## 状態: 完了

## 問題

リプレイプレイヤー（`replay_player`）でリプレイ再生時にパニックが発生する。

```
The `StateTransition` schedule is missing. Did you forget to add StatesPlugin or DefaultPlugins before calling init_state?
```

## 原因

`MinimalPlugins`のみを使用していたが、以下のプラグイン/リソースが不足していた：

1. `StatesPlugin` - State管理用（MinimalPluginsに含まれない）
2. `AssetPlugin` - CharacterPluginがアセットローダーを使用
3. `ScheduleRunnerPlugin` - 60FPS固定タイムステップ
4. `FixedDeltaTime` - 物理計算用リソース
5. `AiServePlugin` - AiServeTimerリソースを提供

## 修正内容

`src/bin/replay_player.rs` を修正：

- `StatesPlugin`を追加
- `AssetPlugin`を追加
- `ScheduleRunnerPlugin`で60FPS固定タイムステップを設定
- `FixedDeltaTime`リソースを初期化
- `AiServePlugin`を追加

`SimulationRunner`（headless_sim）の実装を参考に、ヘッドレス実行に必要なプラグイン群を揃えた。

## 対象ファイル

- `src/bin/replay_player.rs`

## 関連仕様

- @spec 77103_replay_spec.md
