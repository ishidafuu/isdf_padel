# Racket Contact Shot Specification

**Version**: 1.0.0  
**Status**: Draft  
**Last Updated**: 2026-02-08

## 概要

ショット成立条件を「入力時の距離判定」から「ラケット軌道とボールの接触」に変更する。  
入力はスイング意図を生成するのみとし、実際の打球速度更新は接触フレームでのみ実行する。

## Core Requirements (v0.7)

### REQ-30606-001: スイング意図イベント
**WHEN** プレイヤーがショット入力条件を満たす  
**THE SYSTEM SHALL** `SwingIntentEvent` を発行する  
- 入力方向（Vec2）
- プレイヤーID
- プレイヤーのコートサイド
- ホールド時間

---

### REQ-30606-002: 予測打点の計画
**WHEN** `SwingIntentEvent` を受信する  
**THE SYSTEM SHALL** 近未来のボール位置を予測し、打点時刻 `t_hit` と打点位置 `p_hit` を決定する
- 重力を考慮した予測
- 到達可能距離・高さで最適候補を選択

---

### REQ-30606-003: ラケット軌道の時間進行
**WHEN** スイングが開始された  
**THE SYSTEM SHALL** 事前動作→インパクト→フォロースルーの軌道を時間で更新する
- `elapsed` を固定デルタで進める
- `contact_time` 近傍で接触判定を有効化する

---

### REQ-30606-004: 接触時のみショット成立
**WHEN** ラケットのスイープ軌道がボールと接触した  
**THE SYSTEM SHALL** そのフレームで `ShotEvent` を発行する
- 非接触のスイングでは `ShotEvent` を発行しない
- 接触イベント `RacketContactEvent` を発行する

---

### REQ-30606-005: 既存弾道計算との互換
**WHEN** 接触で `ShotEvent` が発行された  
**THE SYSTEM SHALL** 既存の弾道計算システムを利用して打球速度を算出する
- `shot_direction_system` を再利用
- ショット属性計算（30604）は継続利用

## 設計メモ

- 目的は「当たったように見せる」ではなく「当たった瞬間だけ打球が変わる」を保証すること
- 見た目は抽象化してもよく、接触の因果関係を優先する
