# Scoring Test Cases

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

スコアリングシステム（ポイント進行、ゲーム管理、セット管理）のテストケースを定義します。

## テストケース

### ポイント進行テスト（30701_point_spec.md）

#### TST-30704-001: ポイント加算
**対応**: REQ-30701-001
**Given**: 試合中、Player1が得点する
**When**: PointEndEvent を受信する（winner=Player1）
**Then**: Player1のポイントカウントが1増える

---

#### TST-30704-002: スコア表示の更新
**対応**: REQ-30701-002
**Given**: Player1のポイントカウントが2
**When**: ポイントカウントが更新される
**Then**: 表示用スコアが "30" に変換される

---

#### TST-30704-003: ゲーム獲得判定
**対応**: REQ-30701-003
**Given**: Player1のポイントカウントが3（40ポイント）
**When**: Player1が得点する（ポイントカウント = 4）
**Then**:
- ゲーム獲得と判定される
- GameWonEvent が発行される（winner=Player1）

---

#### TST-30704-004: ポイントリセット
**対応**: REQ-30701-004
**Given**: Player1がゲームを獲得した（ポイントカウント = 4）
**When**: GameWonEvent が発行される
**Then**:
- Player1のポイントカウントが0にリセットされる
- Player2のポイントカウントが0にリセットされる

---

#### TST-30704-005: PointScoreEvent の発行
**対応**: REQ-30701-005
**Given**: 試合中、Player1が得点する、EventBus がリスニング中
**When**: ポイントが加算される
**Then**:
- PointScoreEvent が発行される
- イベントデータに Winner, CurrentScore が含まれる

---

### ゲーム管理テスト（30702_game_spec.md）

#### TST-30704-006: ゲームカウント加算
**対応**: REQ-30702-001
**Given**: Player1がゲームを獲得した（ポイントカウント = 4）
**When**: GameWonEvent を受信する（winner=Player1）
**Then**:
- Player1のゲームカウントが1増える
- GameScoreEvent が発行される

---

#### TST-30704-007: セット獲得判定
**対応**: REQ-30702-002
**Given**: Player1のゲームカウントが5
**When**: Player1がゲームを獲得する（ゲームカウント = 6）
**Then**:
- セット獲得と判定される
- SetWonEvent が発行される（winner=Player1）

---

#### TST-30704-008: ゲームカウントリセット
**対応**: REQ-30702-003
**Given**: Player1がセットを獲得した（ゲームカウント = 6）
**When**: SetWonEvent が発行される
**Then**:
- Player1のゲームカウントが0にリセットされる
- Player2のゲームカウントが0にリセットされる

---

#### TST-30704-009: サーブ権交代
**対応**: REQ-30702-004
**Given**: 現在のサーバーが Player1
**When**: ゲームが終了する（GameWonEvent を受信）
**Then**:
- サーブ権が Player2 に交代する
- ServeChangeEvent が発行される（nextServer=Player2）

---

### セット管理テスト（30703_set_spec.md）

#### TST-30704-010: セットカウント加算
**対応**: REQ-30703-001
**Given**: Player1がセットを獲得した（ゲームカウント = 6）
**When**: SetWonEvent を受信する（winner=Player1）
**Then**:
- Player1のセットカウントが1増える
- SetScoreEvent が発行される

---

#### TST-30704-011: マッチ終了判定
**対応**: REQ-30703-002
**Given**: Player1のセットカウントが0（MVP v0.1: 1セットマッチ）
**When**: Player1がセットを獲得する（セットカウント = 1）
**Then**:
- マッチ終了と判定される
- MatchEndEvent が発行される（winner=Player1, loser=Player2）
- MatchState が MatchEnd に遷移する

---

#### TST-30704-012: 試合勝者の決定
**対応**: REQ-30703-003
**Given**: Player1のセットカウントが1、Player2のセットカウントが0
**When**: マッチ終了判定が発生する
**Then**:
- 勝者: Player1
- 敗者: Player2

---

#### TST-30704-013: 試合終了後の状態
**対応**: REQ-30703-004
**Given**: MatchEndEvent が発行された
**When**: 試合終了処理が完了する
**Then**:
- スコアが保持される（ポイント、ゲーム、セット）
- プレイヤーが操作不能になる
- ボールが停止する

---

## エッジケース

### EC-30704-001: ゲーム獲得境界値
**Given**: Player1のポイントカウントが3（40ポイント）
**When**: Player1が得点する（ポイントカウント = 4）
**Then**: ゲーム獲得と判定される（境界値を含む）

---

### EC-30704-002: セット獲得境界値
**Given**: Player1のゲームカウントが5
**When**: Player1がゲームを獲得する（ゲームカウント = 6）
**Then**: セット獲得と判定される（境界値を含む）

---

### EC-30704-003: マッチ終了境界値
**Given**: Player1のセットカウントが0（MVP v0.1: 1セットマッチ）
**When**: Player1がセットを獲得する（セットカウント = 1）
**Then**: マッチ終了と判定される（境界値を含む）

---

### EC-30704-004: 一方的な試合
**Given**: Player1がすべてのポイントを獲得
**When**: Player1が6ゲーム連続で獲得する
**Then**:
- スコア: Player1 6-0 Player2
- セット獲得: Player1
- マッチ終了

---

### EC-30704-005: 接戦
**Given**: Player1とPlayer2が交互にゲームを獲得
**When**: Player1が6ゲーム目を獲得する（スコア 6-5）
**Then**:
- セット獲得: Player1
- マッチ終了（MVP v0.1: タイブレークなし）

---

## データ参照

全テストケースは以下のデータを参照します：
- [80101_game_constants.md](../../8_data/80101_game_constants.md) - Scoring パラメータ

---

## 依存関係

### 依存先
- [30701_point_spec.md](30701_point_spec.md) - ポイント進行仕様
- [30702_game_spec.md](30702_game_spec.md) - ゲーム管理仕様
- [30703_set_spec.md](30703_set_spec.md) - セット管理仕様
