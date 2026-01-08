# Scoring - Overview

**Version**: 1.1.0
**Last Updated**: 2026-01-09
**Status**: Draft

---

## 概要

スコアリングシステムは、ポイント・ゲーム・セットの管理を行います。

テニス式のスコアリング（0→15→30→40→Game）、ゲーム/セットの階層構造、勝敗判定など、スコアに関する全ての機能を含みます。

---

## 管理する機能

| ID範囲 | 機能 | 説明 |
|--------|------|------|
| 307xx | スコアリング全般 | ポイント進行、ゲーム/セット管理、勝敗判定 |

### 主要機能

1. **ポイント進行**
   - 0→15→30→40→Game
   - ポイント加算
   - デュース処理（v0.2以降）

2. **ゲーム管理**
   - ゲーム獲得条件（4点先取、2点差以上）
   - ゲームカウント更新
   - サーブ権交代

3. **セット管理**
   - セット獲得条件（6ゲーム先取）
   - セットカウント更新
   - タイブレーク（v0.2以降）

4. **勝敗判定**
   - マッチ終了条件
   - 勝者決定

---

## MVP v0.1の範囲

### ✅ 含む機能

- **ポイント進行**: 0→15→30→40→Game
- **ゲーム獲得**: 4点先取（デュースなし）
- **セット獲得**: 6ゲーム先取（1セットマッチ）
- **勝敗判定**: 1セット獲得で勝利

### ❌ 含まない機能（v0.4以降）

- **デュース処理**: 40-40時の処理
- **タイブレーク**: 6-6時の処理
- **複数セット**: 3セット、5セットマッチ

---

## スコア構造

```
Match（試合）
  ├─ Set（セット）
  │    ├─ Game（ゲーム）
  │    │    └─ Point（ポイント）: 0, 15, 30, 40, Game
  │    └─ Game
  └─ Set
```

### スコア例

```
Player1: 2セット, 4ゲーム, 30ポイント
Player2: 1セット, 3ゲーム, 15ポイント

現在のゲーム: Player1 30 - 15 Player2
現在のセット: Player1 4 - 3 Player2
マッチ: Player1 2 - 1 Player2
```

---

## コンポーネント設計

### ECS Components

スコアは以下のComponentsを持ちます：

| Component | 説明 | 参照 |
|-----------|------|------|
| `ScoreState` | 各プレイヤーのポイント、ゲーム、セットスコア | 未定義 |
| `MatchState` | 試合状態（進行中、終了） | 301_match参照 |

### Systems

スコアに関連するSystemsは以下の通り：

| System | 責務 | 参照 |
|--------|------|------|
| `ScoringSystem` | スコア更新、判定 | 未定義 |
| `ScoreDisplaySystem` | スコア表示（UI） | 未定義 |

---

## スコアリング仕様

### ポイント進行

```csharp
// ScoringSystem
void AddPoint(playerId) {
    var scoreState = GetComponent<ScoreState>(matchEntityId);

    // ポイント加算
    scoreState.Point[playerId]++;

    // ポイント → スコア変換
    int[] scoreTable = { 0, 15, 30, 40 };
    if (scoreState.Point[playerId] < 4) {
        scoreState.DisplayScore[playerId] = scoreTable[scoreState.Point[playerId]];
    }

    // ゲーム獲得判定
    if (IsGameWon(playerId)) {
        AddGame(playerId);
        ResetPoints();
    }
}
```

### ゲーム獲得条件

**MVP v0.1**: シンプルな4点先取

```csharp
// ゲーム獲得判定（デュースなし）
bool IsGameWon(playerId) {
    var scoreState = GetComponent<ScoreState>(matchEntityId);

    // 4点以上で勝利
    return scoreState.Point[playerId] >= 4;
}
```

**v0.2以降**: デュース対応

```csharp
// ゲーム獲得判定（デュースあり）
bool IsGameWon(playerId) {
    var scoreState = GetComponent<ScoreState>(matchEntityId);
    int myPoint = scoreState.Point[playerId];
    int opponentPoint = scoreState.Point[GetOpponent(playerId)];

    // 4点以上 かつ 2点差以上で勝利
    return myPoint >= 4 && (myPoint - opponentPoint) >= 2;
}
```

### セット獲得条件

**MVP v0.1**: 6ゲーム先取（タイブレークなし）

```csharp
// セット獲得判定
bool IsSetWon(playerId) {
    var scoreState = GetComponent<ScoreState>(matchEntityId);

    // 6ゲーム以上で勝利
    return scoreState.Game[playerId] >= 6;
}
```

**v0.2以降**: タイブレーク対応

```csharp
// セット獲得判定（タイブレークあり）
bool IsSetWon(playerId) {
    var scoreState = GetComponent<ScoreState>(matchEntityId);
    int myGame = scoreState.Game[playerId];
    int opponentGame = scoreState.Game[GetOpponent(playerId)];

    // 6ゲーム以上 かつ 2ゲーム差以上で勝利
    if (myGame >= 6 && (myGame - opponentGame) >= 2) {
        return true;
    }

    // タイブレーク（6-6の場合）
    if (myGame == 7 && opponentGame == 6) {
        return true;
    }

    return false;
}
```

### マッチ終了判定

**MVP v0.1**: 1セットマッチ

```csharp
// マッチ終了判定
bool IsMatchEnd() {
    var scoreState = GetComponent<ScoreState>(matchEntityId);

    // どちらかが1セット獲得で終了
    return scoreState.Set[player1Id] >= 1
        || scoreState.Set[player2Id] >= 1;
}
```

**v0.2以降**: 複数セットマッチ

```csharp
// マッチ終了判定（3セットマッチ）
bool IsMatchEnd() {
    var scoreState = GetComponent<ScoreState>(matchEntityId);

    // どちらかが2セット獲得で終了
    return scoreState.Set[player1Id] >= 2
        || scoreState.Set[player2Id] >= 2;
}
```

---

## ポイント獲得条件（失点条件）

プレイヤーが**失点**となる条件（相手にポイントが入る）：

| 条件 | 説明 | 失点するプレイヤー | 得点するプレイヤー |
|------|------|------------------|------------------|
| **ツーバウンド** | 自コート内でボールが2回バウンドした | ボールを打ち返せなかった側 | 相手 |
| **ネット** | 打ったボールがネットに当たって相手コートに届かなかった | ボールを打った側 | 相手 |
| **自コート打球** | 自分が打った打球が自コートに落ちた（相手コートに届かなかった） | ボールを打った側 | 相手 |
| **キャラクター当たり** | 飛んできたボールがキャラクターに当たった（打ち返せなかった） | ボールが当たった側 | 相手（仮） |

### 重要な補足

- **壁の扱い**:
  - **自コート側の壁**: 跳ね返ってプレイ続行（失点にならない）
  - **相手コート側の壁**: 本来のパデルではアウトだが、このゲームでは許容（失点にしない）
  - ゲームバランス上の調整として、壁に当たってもプレイ続行

- **ワンバウンドは有効**: 自コート内で1回バウンド → 打ち返せばOK

- **ツーバウンドは失点**: 自コート内で2回バウンド → 失点

- **自コート打球は失点**: 打った打球が相手コートに届かず自コートに落ちた → 失点

### キャラクター当たり判定の扱い

- **MVP v0.1**: 打ち返せなかった側（当たった側）の失点（仮）
- **v0.2以降**: ゲームバランスを見て調整
  - パデルルールでは「ボレーで当たった」場合は打った側の得点
  - 「打ち返せずに当たった」場合は当たった側の失点

---

## データ定義

スコアのパラメータは `8_data/80101_game_constants.md` に定義されます。

| パラメータ | 設定ファイル参照 | デフォルト値（参考） |
|-----------|----------------|------------------|
| ゲーム獲得ポイント数 | `config.Scoring.GamePoint` | 4点 |
| セット獲得ゲーム数 | `config.Scoring.SetGame` | 6ゲーム |
| マッチセット数 | `config.Scoring.MatchSet` | 1セット（MVP v0.1） |

**重要**: 具体的な値は参考値です。実装時は必ず `config.Scoring.*` を参照してください。

---

## 参考資料

### ナムコテニス

- [Scoring System](../../9_reference/901_reference_game/mechanics/90114_scoring.md)
  - 確度: ★★★★☆
  - 参照推奨項目: ポイント制、ゲーム/セットの階層構造

### テニスルール

- ポイント進行: 0→15→30→40→Game
- デュース: 40-40時の処理（v0.2以降）
- タイブレーク: 6-6時の処理（v0.2以降）

---

## 次のステップ

1. ✅ Scoring全体の構造定義（このドキュメント）
2. ⏳ 詳細仕様の策定
   - 30701_point_spec.md: ポイント進行の詳細
   - 30702_game_spec.md: ゲーム管理の詳細
   - 30703_set_spec.md: セット管理の詳細
3. ⏳ データ定義の更新（8_data/80101_game_constants.md）
4. ⏳ 実装開始

---

## Change Log

### 2026-01-09 - v1.1.0

- 含まない機能をv0.4以降に変更

### 2025-12-23 - v1.0.0（初版）

- Scoring機能の全体構造定義
- MVP v0.1の範囲設定（デュース、タイブレークなし）
- スコア構造の明確化（ポイント/ゲーム/セットの階層）
- スコアリング仕様の定義
