# Match Flow Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

試合全体の状態遷移とフロー管理を定義します。試合開始からサーブ、ラリー、ポイント終了、試合終了までの流れを制御します。

## 状態定義

### MatchState Enum
```csharp
public enum MatchStateType {
    MatchStart,  // 試合開始
    Serve,       // サーブ待機
    Rally,       // ラリー中
    PointEnd,    // ポイント終了
    MatchEnd     // 試合終了
}
```

## Core Requirements (MVP v0.1)

### REQ-30101-001: 試合開始
**WHEN** 試合を開始する
**THE SYSTEM SHALL** プレイヤーを配置する
- Player1: 1Pコート側
- Player2: 2Pコート側
**AND** スコアを初期化する
**AND** サーブ権をPlayer1に設定する
**AND** MatchState を Serve に遷移する
**テスト**: TST-30104-001

### REQ-30101-002: サーブからラリーへ
**WHEN** サーブが打たれる（ShotEvent受信）
**AND** MatchState == Serve
**THE SYSTEM SHALL** MatchState を Rally に遷移する
**テスト**: TST-30104-002

### REQ-30101-003: ラリーからポイント終了へ
**WHEN** 失点条件を満たす
**THE SYSTEM SHALL** MatchState を PointEnd に遷移する
**AND** PointEndEvent を発行する
**テスト**: TST-30104-003

### REQ-30101-004: ポイント終了から次のポイントへ
**WHEN** ポイント終了処理が完了する
**AND** 試合が終了していない
**THE SYSTEM SHALL** プレイヤーを初期位置に戻す
**AND** MatchState を Serve に遷移する
**テスト**: TST-30104-004

### REQ-30101-005: 試合終了
**WHEN** 勝利条件を満たす
**THE SYSTEM SHALL** MatchState を MatchEnd に遷移する
**AND** MatchEndEvent を発行する
**テスト**: TST-30104-005

---

## Extended Requirements (v0.2)

### REQ-30101-050: 複数セット対応
**WHEN** 試合を複数セット（3セット、5セット）で行う
**THE SYSTEM SHALL** セット勝利後も試合を継続する
**AND** セットカウントを管理する
**テスト**: TST-30104-050

### REQ-30101-051: チェンジコート
**WHEN** 奇数ゲーム終了時（1, 3, 5...ゲーム後）
**THE SYSTEM SHALL** プレイヤーのコート位置を入れ替える
**テスト**: TST-30104-051

---

## Future Requirements (v0.3+)

### REQ-30101-100: 試合一時停止
**WHEN** プレイヤーが一時停止を要求する
**THE SYSTEM SHALL** 試合を一時停止状態にする
**AND** 再開時に同じ状態から開始する
**テスト**: TST-30104-100

---

## データ参照
- [80101_game_constants.md](../../8_data/80101_game_constants.md)

## 依存関係
- [30102_serve_spec.md](30102_serve_spec.md)
- [30103_point_end_spec.md](30103_point_end_spec.md)
- [30700_overview.md](../307_scoring/30700_overview.md)
