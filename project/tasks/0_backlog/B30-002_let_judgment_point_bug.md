# B30-002: レット判定時のポイント加算バグ修正

## 概要

サーブがネットに触れた後にサービスボックスに入った場合（レット）に、ポイントが加算されてしまうバグを修正する。

## 問題詳細

- **タイプ**: バグ修正
- **優先度**: Critical
- **原因特定済み**: Yes
- **リプレイ**: `replay_20260119_072528.replay` (Rally 4, 7)
- **QAレポート**: `project/qa_reports/2026-01-19_072511/`

## 症状

- NetFault頻発（25%）
- サーブレット時にサーバーにポイントが加算される

## 根本原因

`src/systems/point_judgment/net_judgment.rs:71` でレット判定時に`RallyEndEvent`を発行し、不正にポイント加算している。

```rust
// 現在のコード（バグ）
// レット時は再サーブすべきだが、ポイント加算イベントを発行している
rally_events.write(RallyEndEvent {
    winner: server_side,  // サーバーがポイント獲得（誤り）
    reason: RallyEndReason::NetFault,
});
```

**問題**: レットは再サーブであり、ポイント加算すべきではない。

## 修正内容

レット判定時は`RallyEndEvent`ではなく、`LetEvent`（再サーブトリガー）を発行する。

```rust
// 修正後
// レット時は再サーブイベントを発行
let_events.write(LetEvent {
    server_side,
    serve_number: current_serve_number,
});
// ポイント加算は行わない
```

## 修正対象ファイル

- `project/src/systems/point_judgment/net_judgment.rs`

## 検証方法

1. `/qa-cycle` でQAサイクル再実行
2. リプレイビューアーでレット時の挙動を確認
3. レット時にポイントが加算されないことを確認
4. レット時に再サーブが行われることを確認

## 関連

- 関連イベント: LetEvent, RallyEndEvent, NetFault
- 関連コンポーネント: NetJudgment, ServeState
