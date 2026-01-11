# R30000-033: 試合途中終了時のリプレイ保存

## 概要
ゲーム終了時（途中終了含む）に、それまでの入力データをリプレイとして保存する。

## 背景
現在、リプレイは `MatchFlowState::MatchEnd` 時のみ保存される。
ゲームを途中で終了（×ボタン、Escapeキー）すると保存されず、AIデバッグに支障がある。

## 実装内容

### 変更ファイル
- `src/replay/mod.rs` - 終了時保存システムを追加

### 実装詳細

1. `AppExit`メッセージをリッスンする新システムを追加
2. `Last`スケジュールで実行し、終了前に保存を完了

```rust
/// ゲーム終了時に記録中のリプレイを保存
fn save_replay_on_exit(
    mut exit_reader: MessageReader<AppExit>,
    mut recorder: ResMut<ReplayRecorder>,
    manager: Res<ReplayManager>,
) {
    for _ in exit_reader.read() {
        if recorder.is_recording() {
            recorder.stop_recording();
            if let Some(data) = recorder.take_data() {
                match manager.save_replay(&data) {
                    Ok(path) => info!("Replay saved on exit: {:?}", path),
                    Err(e) => error!("Failed to save replay on exit: {}", e),
                }
            }
        }
    }
}
```

## 検証方法
1. `cargo run --bin padel_game` でゲーム起動
2. 試合を途中まで進める
3. ×ボタンまたはEscapeキーで終了
4. `ls project/assets/replays/` でリプレイファイルが作成されていることを確認

## 制限事項
- Ctrl+C（SIGINT）での終了はBevyイベントを経由しないため保存されない
