# Match - Overview

**Version**: 1.0.0
**Last Updated**: 2025-12-23
**Status**: Draft

---

## 概要

試合進行システムは、試合全体のフロー（サーブ → ラリー → 得点 → 次のポイント）を管理します。

試合の状態遷移、サーブ権管理、ポイント終了判定など、試合進行に関する全ての機能を含みます。

---

## 管理する機能

| ID範囲 | 機能 | 説明 |
|--------|------|------|
| 301xx | 試合進行全般 | 試合開始、サーブ、ラリー、ポイント終了、試合終了 |

### 主要機能

1. **試合開始**
   - プレイヤー配置（左右のコート）
   - スコア初期化
   - サーブ権決定

2. **サーブ**
   - サーブ権管理
   - サーブ動作（アンダーハンド、ワンバウンド後）
   - サーブ失敗判定

3. **ラリー**
   - ボールの打ち合い
   - ターン管理（どちらが打つべきか）

4. **ポイント終了**
   - 得点判定（アウト、ネット、タイムアウト）
   - 次のポイントへ移行
   - サーブ権交代

5. **試合終了**
   - 勝敗判定
   - 結果画面への遷移

---

## MVP v0.1の範囲

### ✅ 含む機能

- **試合開始**: プレイヤー配置、スコア初期化
- **サーブ**: 基本的なサーブ（手動操作）
- **ラリー**: ボールの打ち合い
- **ポイント終了**: 得点判定、次のポイントへ移行
- **試合終了**: 1セットマッチの勝敗判定

### ❌ 含まない機能（v0.2以降）

- **自動サーブ**: サーブを自動で行う
- **サーブ失敗**: フォルト、ダブルフォルト
- **チェンジコート**: セット間のコート交代
- **複数セット**: 3セット、5セットマッチ

---

## 状態遷移

```
MatchStart（試合開始）
    ↓
Serve（サーブ）
    ↓
Rally（ラリー）
    ↓
PointEnd（ポイント終了）
    ↓
[スコア更新]
    ↓
Serve または MatchEnd
```

### 各状態の詳細

| 状態 | 説明 | 遷移条件 |
|------|------|---------|
| **MatchStart** | 試合開始、プレイヤー配置 | 自動で Serve へ |
| **Serve** | サーブ待機、サーブ実行 | サーブが打たれたら Rally へ |
| **Rally** | ボールの打ち合い | ポイント終了で PointEnd へ |
| **PointEnd** | 得点判定、スコア更新 | 次のポイントで Serve へ、または MatchEnd へ |
| **MatchEnd** | 試合終了、結果表示 | - |

---

## コンポーネント設計

### ECS Components

試合は以下のComponentsを持ちます：

| Component | 説明 | 参照 |
|-----------|------|------|
| `MatchState` | 試合状態（Serve, Rally, PointEnd, MatchEnd） | 未定義 |
| `ServeState` | サーブ権、サーブ側プレイヤー | 未定義 |
| `ScoreState` | ポイント、ゲーム、セットのスコア | 未定義 |

### Systems

試合に関連するSystemsは以下の通り：

| System | 責務 | 参照 |
|--------|------|------|
| `MatchFlowSystem` | 状態遷移管理 | 未定義 |
| `ServeSystem` | サーブ処理 | 未定義 |
| `RallySystem` | ラリー監視 | 未定義 |
| `PointEndSystem` | ポイント終了判定 | 未定義 |
| `ScoringSystem` | スコア更新 | 307_scoring参照 |

---

## 試合フロー仕様

### 試合開始

```csharp
// MatchFlowSystem
void StartMatch() {
    // プレイヤー配置
    PlacePlayer(player1Id, config.Court.LeftStartPos);
    PlacePlayer(player2Id, config.Court.RightStartPos);

    // スコア初期化
    var scoreState = GetComponent<ScoreState>(matchEntityId);
    scoreState.Reset();

    // サーブ権決定（ランダム、またはPlayer1固定）
    var serveState = GetComponent<ServeState>(matchEntityId);
    serveState.ServerId = player1Id;  // とりあえずPlayer1からサーブ

    // 状態遷移
    var matchState = GetComponent<MatchState>(matchEntityId);
    matchState.State = MatchStateType.Serve;
}
```

### サーブ

```csharp
// ServeSystem
void OnServe(playerId) {
    // サーブ側プレイヤーか確認
    var serveState = GetComponent<ServeState>(matchEntityId);
    if (serveState.ServerId != playerId) {
        return;  // サーブ側ではない
    }

    // ボール生成（プレイヤーの足元）
    var playerPos = GetComponent<Position>(playerId);
    CreateBall(playerPos + Vector3.Up * 0.5f);

    // サーブ完了、ラリーへ
    var matchState = GetComponent<MatchState>(matchEntityId);
    matchState.State = MatchStateType.Rally;
}
```

**MVP v0.1**: サーブは手動操作（プレイヤーがBボタンを押してサーブ）

### ラリー

```csharp
// RallySystem
void UpdateRally() {
    // ボールの状態を監視
    // ポイント終了条件（プレイヤーの失点）：
    // 1. 自コート内でツーバウンド（打ち返せなかった）
    // 2. 打ったボールがネットに当たって相手コートに届かなかった
    // 3. 打った打球が自コートに落ちた（相手コートに届かなかった）
    // 注: 壁に当たってもプレイ続行（相手側の壁も許容）

    // ポイント終了イベントを購読
    eventBus.Subscribe<PointEndEvent>(OnPointEnd);
}
```

### ポイント終了

```csharp
// PointEndSystem
void OnPointEnd(PointEndEvent e) {
    // スコア更新
    var scoreState = GetComponent<ScoreState>(matchEntityId);
    scoreState.AddPoint(e.winner);

    // 試合終了判定
    if (scoreState.IsMatchEnd()) {
        var matchState = GetComponent<MatchState>(matchEntityId);
        matchState.State = MatchStateType.MatchEnd;
        return;
    }

    // サーブ権交代（ゲーム終了時）
    if (scoreState.IsGameEnd()) {
        var serveState = GetComponent<ServeState>(matchEntityId);
        serveState.ServerId = GetOpponent(serveState.ServerId);
    }

    // 次のポイントへ
    ResetPositions();
    var matchState = GetComponent<MatchState>(matchEntityId);
    matchState.State = MatchStateType.Serve;
}
```

---

## サーブ仕様（パデルルール）

### アンダーハンドサーブ

- **ルール**: アンダーハンド（下から打つ）
- **ワンバウンド**: ボールを地面にワンバウンドさせてから打つ

**MVP v0.1**: 簡易実装
- プレイヤーがBボタンを押すとサーブ
- ボールは自動的に相手コートに飛ぶ（簡易な軌道）

**v0.2以降**: 詳細実装
- ワンバウンド動作の実装
- サーブ失敗（フォルト）の実装

---

## データ定義

試合のパラメータは `8_data/80101_game_constants.md` に定義されます。

| パラメータ | 設定ファイル参照 | デフォルト値（参考） |
|-----------|----------------|------------------|
| 左側開始位置 | `config.Court.LeftStartPos` | (-5.0, 0, 5.0) |
| 右側開始位置 | `config.Court.RightStartPos` | (5.0, 0, 5.0) |
| サーブ初速度 | `config.Serve.DefaultSpeed` | 12.0 m/s |
| サーブ角度 | `config.Serve.DefaultAngle` | 30度 |

**重要**: 具体的な値は参考値です。実装時は必ず `config.*` を参照してください。

---

## 参考資料

### ナムコテニス

- [Match Screen](../../9_reference/901_reference_game/screens/90103_match.md)
  - 確度: ★★★☆☆
  - 参照推奨項目: 試合フロー、画面レイアウト

### アーキテクチャ

- [20000_overview.md](../../2_architecture/20000_overview.md) - アーキテクチャ概要
- [20005_event_system.md](../../2_architecture/20005_event_system.md) - イベントシステム

### Padel Actionの特徴

- **パデルルール**: アンダーハンドサーブ、ワンバウンド後に打つ
- **1セットマッチ**: MVP v0.1では1セットのみ

---

## 次のステップ

1. ✅ Match全体の構造定義（このドキュメント）
2. ⏳ 詳細仕様の策定
   - 30101_flow_spec.md: 試合フローの詳細
   - 30102_serve_spec.md: サーブの詳細
   - 30103_point_end_spec.md: ポイント終了判定の詳細
3. ⏳ データ定義の更新（8_data/80101_game_constants.md）
4. ⏳ 実装開始

---

## Change Log

### 2025-12-23 - v1.0.0（初版）

- Match機能の全体構造定義
- MVP v0.1の範囲設定
- 状態遷移の明確化
- 試合フロー仕様の定義
