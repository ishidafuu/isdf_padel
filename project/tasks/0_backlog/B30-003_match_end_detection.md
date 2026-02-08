# B30-003: マッチ終了判定バグ調査・修正

## 概要

プレイヤーが勝利条件（5ポイント先取の場合4ポイント）を満たしてもマッチが終了しない問題を調査・修正する。

## 問題詳細

- **タイプ**: バグ修正
- **優先度**: Critical
- **原因特定済み**: No（要調査）
- **リプレイ**: `replay_20260119_072528.replay`
- **QAレポート**: `project/qa_reports/2026-01-19_072511/`

## 症状

- P2が5ポイント取得してもゲーム終了しない
- シミュレーションがタイムアウトで終了
- `MatchFlowState::MatchEnd` に遷移しない

## 調査対象ファイル

- `project/src/systems/match_control/scoring/rally.rs` - ラリー終了時のスコア更新
- `project/src/systems/match_control/scoring/game_set.rs` - ゲーム/セット終了判定
- `project/src/systems/match_control/flow.rs` - マッチフロー状態遷移
- `project/src/simulation/simulation_runner.rs` - シミュレーション実行制御

## 調査ポイント

1. **スコア更新の確認**
   - `RallyEndEvent` 受信後にスコアが正しく更新されているか
   - スコアリソースの値が期待通りか

2. **終了条件判定の確認**
   - ゲーム終了条件（4ポイント先取）の判定ロジック
   - セット終了条件の判定ロジック
   - マッチ終了条件の判定ロジック

3. **状態遷移の確認**
   - `MatchFlowState` の遷移が正しく行われているか
   - イベント処理順序に問題がないか

4. **シミュレーション終了の確認**
   - `simulation_runner` がマッチ終了を検知できているか

## 予想される原因

- 状態遷移の条件分岐の誤り
- イベント処理順序の問題（スコア更新前に終了判定）
- 終了条件の閾値設定ミス

## 修正方針

調査結果に基づき修正内容を決定する。

## 検証方法

1. `/qa-cycle` でQAサイクル再実行
2. リプレイビューアーでマッチ終了を確認
3. 勝利条件到達時にマッチが正常終了することを確認

## 関連

- 関連イベント: RallyEndEvent, GameEndEvent, SetEndEvent, MatchEndEvent
- 関連コンポーネント: MatchFlowState, Score
