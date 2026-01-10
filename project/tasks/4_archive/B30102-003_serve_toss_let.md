# B30102-003: サーブトス打たず→打ち直し（let）に修正

## 概要
トスをして打たなかった場合（タイムアウト/低すぎ）がフォルト扱いになっている。
実際のパデル/テニスでは打ち直し（let）が正しい。

## 問題
- 現状: `serve_toss_timeout_system` で `record_fault()` を呼び、フォルトカウント増加
- 正しい挙動: fault_count を変更せず、単に打ち直しにする

## 修正内容

### 1. 仕様書更新
**ファイル**: `project/docs/3_ingame/301_match/30102_serve_spec.md`
- REQ-30102-084: 「Fault と判定する」→「打ち直し（let）にする」
- fault_count は変更しない旨を明記

### 2. ServeState にメソッド追加
**ファイル**: `project/src/resource/scoring.rs`
```rust
/// 打ち直し（let）時のリセット
/// @spec 30102_serve_spec.md#req-30102-084
pub fn reset_for_retry(&mut self) {
    self.phase = ServeSubPhase::Waiting;
    self.toss_time = 0.0;
    self.toss_origin = None;
    // fault_count は変更しない
}
```

### 3. システム修正
**ファイル**: `project/src/systems/match_control/serve.rs`
- `serve_toss_timeout_system` で `record_fault()` → `reset_for_retry()` に変更
- ログメッセージを「Serve fault」→「Serve let」に変更

## 検証方法
1. ゲームを起動
2. サーブ時にトスして打たずに待つ
3. フォルトにならず、打ち直しになることを確認
4. 2回連続でトスを打たなくてもダブルフォルトにならないことを確認

## 関連
- 仕様: REQ-30102-084
