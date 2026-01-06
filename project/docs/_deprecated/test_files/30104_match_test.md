# Match Test Cases

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

試合進行（フロー、サーブ、ポイント終了）のテストケースを定義します。

## テストケース

### 試合フローテスト（30101_flow_spec.md）

#### TST-30104-001: 試合開始
**対応**: REQ-30101-001
**Given**: 試合開始前
**When**: 試合を開始する
**Then**:
- Player1が1Pコートに配置される
- Player2が2Pコートに配置される
- スコアが0-0に初期化される
- サーブ権がPlayer1に設定される
- MatchState = Serve

#### TST-30104-002: サーブからラリーへ
**対応**: REQ-30101-002
**Given**: MatchState = Serve
**When**: サーブプレイヤーがショットを実行する
**Then**: MatchState = Rally

#### TST-30104-003: ラリーからポイント終了へ
**対応**: REQ-30101-003
**Given**: MatchState = Rally
**When**: 失点条件を満たす
**Then**:
- MatchState = PointEnd
- PointEndEvent が発行される

#### TST-30104-004: ポイント終了から次のポイントへ
**対応**: REQ-30101-004
**Given**: MatchState = PointEnd、試合継続中
**When**: ポイント終了処理が完了する
**Then**:
- プレイヤーが初期位置に戻る
- MatchState = Serve

#### TST-30104-005: 試合終了
**対応**: REQ-30101-005
**Given**: 勝利条件を満たす
**When**: ポイント終了処理が完了する
**Then**:
- MatchState = MatchEnd
- MatchEndEvent が発行される

---

### サーブテスト（30102_serve_spec.md）

#### TST-30104-006: サーブ権の管理
**対応**: REQ-30102-001
**Given**: ゲーム開始時
**When**: ポイントが開始される
**Then**: サーブ権 = Player1

#### TST-30104-007: サーブ入力
**対応**: REQ-30102-002
**Given**: MatchState = Serve、サーブ権 = Player1
**When**: Player1がBボタンを押す
**Then**:
- ボールが生成される（Player1の足元 + (0, 0.5, 0)）
- ShotEvent が発行される

#### TST-30104-008: サーブの打球
**対応**: REQ-30102-003
**Given**: サーブプレイヤーがショットを実行する
**When**: ショット処理が完了する
**Then**:
- ボールが相手コート方向に発射される
- MatchState = Rally

#### TST-30104-009: サーブ権交代
**対応**: REQ-30102-004
**Given**: ゲームが終了する（Player1が4ポイント獲得）
**When**: 次のゲームが開始される
**Then**: サーブ権 = Player2

---

### ポイント終了テスト（30103_point_end_spec.md）

#### TST-30104-010: ツーバウンド失点
**対応**: REQ-30103-001
**Given**: ボールがPlayer1コート内でバウンドした（1回目）
**When**: 再度Player1コート内でバウンドする（2回目）
**Then**:
- 失点と判定される（Player1失点）
- PointEndEvent が発行される（winner=Player2）

#### TST-30104-011: ネット失点
**対応**: REQ-30103-002
**Given**: Player1がショットを実行した
**When**: ボールがネットに当たって相手コートに届かない
**Then**:
- 失点と判定される（Player1失点）
- PointEndEvent が発行される（winner=Player2）

#### TST-30104-012: 自コート打球失点
**対応**: REQ-30103-003
**Given**: Player1がショットを実行した
**When**: ボールが自コート（Player1コート）に落ちる
**Then**:
- 失点と判定される（Player1失点）
- PointEndEvent が発行される（winner=Player2）

#### TST-30104-013: PointEndEvent の発行
**対応**: REQ-30103-004
**Given**: 失点条件を満たす、EventBus がリスニング中
**When**: 失点判定が実行される
**Then**:
- PointEndEvent が発行される
- イベントデータに Winner, Loser, Reason が含まれる

---

## データ参照
- [80101_game_constants.md](../../8_data/80101_game_constants.md)

## 依存関係
- [30101_flow_spec.md](30101_flow_spec.md)
- [30102_serve_spec.md](30102_serve_spec.md)
- [30103_point_end_spec.md](30103_point_end_spec.md)
