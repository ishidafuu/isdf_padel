# Point End Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-23

## 概要

ポイント終了条件（失点判定）を定義します。パデルテニスの失点条件は3つです。

## Core Requirements (MVP v0.1)

### REQ-30103-001: ツーバウンド失点
**WHEN** ボールが自コート内で2回バウンドした
**THE SYSTEM SHALL** 失点と判定する
**AND** PointEndEvent を発行する（winner=相手プレイヤー）
**テスト**: TST-30104-010

### REQ-30103-002: ネット失点
**WHEN** 打ったボールがネットに当たって相手コートに届かなかった
**THE SYSTEM SHALL** 失点と判定する
**AND** PointEndEvent を発行する（winner=相手プレイヤー）
**テスト**: TST-30104-011

### REQ-30103-003: 自コート打球失点
**WHEN** 打った打球が自コートに落ちた（相手コートに届かなかった）
**THE SYSTEM SHALL** 失点と判定する
**AND** PointEndEvent を発行する（winner=相手プレイヤー）
**テスト**: TST-30104-012

### REQ-30103-004: PointEndEvent の発行
**WHEN** 失点条件を満たす
**THE SYSTEM SHALL** PointEndEvent をイベントバスに発行する
- Winner: 得点したプレイヤーID
- Loser: 失点したプレイヤーID
- Reason: 失点理由（TwoBounce, NetHit, OwnCourtHit）
**テスト**: TST-30104-013

---

## Extended Requirements (v0.2)

### REQ-30103-050: リプレイ表示
**WHEN** ポイントが終了する
**THE SYSTEM SHALL** 得点シーンのスローモーションリプレイを表示する
**テスト**: TST-30104-050

### REQ-30103-051: 統計情報表示
**WHEN** ポイントが終了する
**THE SYSTEM SHALL** ラリー時間、打球数などの統計情報を表示する
**テスト**: TST-30104-051

---

## Future Requirements (v0.3+)

### REQ-30103-100: チャレンジシステム
**WHEN** プレイヤーが判定に異議を唱える
**THE SYSTEM SHALL** ビデオ判定を実行する
**テスト**: TST-30104-100

---

## データ参照
なし（ルールベース）

## 依存関係
- [30700_overview.md](../307_scoring/30700_overview.md) - スコアリング
- [30503_boundary_behavior.md](../305_court/30503_boundary_behavior.md) - ネット接触判定

## 備考
- アウトは存在しない（壁で囲まれている）
- 壁反射は失点にならない（プレイ続行）
- 相手コート側の壁に直接当たってもプレイ続行（ゲームバランス上の調整）
