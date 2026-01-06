# Serve Authority Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2025-12-25

## 概要

審判システムにおけるサーブ権管理（初期化、交代、デュース/アドサイド判定）の仕様を定義します。

## 依存関係

- `307_scoring`: ゲーム終了イベント、ポイント合計
- `306_serve`: サーブ実行

## Core Requirements (MVP v0.1)

### REQ-30903-001: サーブ権初期化
- WHEN ゲームが開始される
- THE SYSTEM SHALL サーブ権を初期プレイヤーに設定する
- WITH 初期サーバー: プレイヤー1（デフォルト）
- WITH 初期サーブサイド: デュースサイド（右側から）
- **テスト**: TST-30904-020
- **データ**: `80101_game_constants.md#serve_config`
- **参考**: [90114_scoring.md](../../9_reference/901_reference_game/mechanics/90114_scoring.md)（★★★★★）

### REQ-30903-002: ゲーム終了時の交代
- WHEN ゲームが終了した（GameEndEvent）
- THE SYSTEM SHALL サーブ権を相手プレイヤーに交代する
- AND 次のゲームをデュースサイドから開始する
- WITH サーバー: 現在のサーバーと逆
- **テスト**: TST-30904-021
- **データ**: `80101_game_constants.md#serve_config`
- **参考**: [90114_scoring.md](../../9_reference/901_reference_game/mechanics/90114_scoring.md)（★★★★★）

### REQ-30903-003: デュースサイド/アドサイド判定
- WHEN ポイントが終了した
- THE SYSTEM SHALL 次のサーブサイドを決定する
- WITH 判定基準: ポイント合計が偶数 → デュースサイド（右）
- WITH 判定基準: ポイント合計が奇数 → アドサイド（左）
- WITH ポイント合計: サーバーのポイント + レシーバーのポイント
- **テスト**: TST-30904-022
- **データ**: `80101_game_constants.md#serve_config`
- **参考**: [90114_scoring.md](../../9_reference/901_reference_game/mechanics/90114_scoring.md)（★★★★★）

---

## Extended Requirements (v0.2)

### REQ-30903-050: GameEndEventによる自動交代
- WHEN GameEndEvent を受信した
- THE SYSTEM SHALL サーブ権を自動的に交代する
- AND ServeAuthorityChangedEvent を発行する
- WITH イベントデータ: 新しいサーバー、サーブサイド
- **テスト**: TST-30904-070
- **データ**: `80101_game_constants.md#serve_config`

### REQ-30903-051: タイブレーク時の管理
- WHEN タイブレークが開始された
- THE SYSTEM SHALL 2ポイントごとにサーブ権を交代する
- AND 最初のサーブは現在のサーバーが担当する
- WITH 交代タイミング: ポイント合計 % 2 == 0
- **テスト**: TST-30904-071

---

## Constraints（Design by Contract）

### Preconditions
- ゲーム開始時にサーブ権が初期化済み
- Scoring System がポイント合計を提供可能

### Postconditions
- ゲーム終了後、サーブ権が正しく交代される
- ポイント終了後、次のサーブサイドが決定される

### Invariants
- サーブ権は常に1人のプレイヤーが保持
- サーブサイドは常にデュースまたはアドのいずれか
