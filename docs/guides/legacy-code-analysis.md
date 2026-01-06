# レガシーコード解析ガイド

**最終更新**: 2025-12-17

## 概要

このガイドは、過去に自分が作成した大規模レガシーコード（10万行規模）を仕様書駆動フレームワークで再構築するための詳細な手順を説明します。

## 前提条件

### 対象となるレガシーコード

- **規模**: 数万〜数十万行
- **言語**: C++（他の言語でも応用可能）
- **状態**: ドキュメントが失われている、または実行不可能
- **知識**: 制作者本人だが、記憶が曖昧（10年以上前など）

### 必要なツール

- **legacy-analyzer-agent**: レガシーコード解析専用エージェント
- **Serena MCP**: コード解析ツール（トークン効率化のため必須）
- **制作者の記憶**: コードから読み取れない意図を補完

## 全体フロー

```
【準備】レガシーコードの配置
  ↓
【Phase 0】解析環境のセットアップ
  ↓
【Phase 1】粗視化解析（1-2週間）
  ↓
【Phase 2】詳細解析（2-4週間）
  ↓
【Phase 3】アルゴリズム抽出（1-2週間）
  ↓
【Phase 4】新規仕様の作成（1-2週間）
  ↓
【Phase 5】実装（実装時期に実施）
```

---

## Phase 0: 解析環境のセットアップ

### Step 0.1: ディレクトリ構造の準備

```bash
# 901_reference_game/ のサブディレクトリを作成
mkdir -p project/docs/9_reference/901_reference_game/{architecture,screens,mechanics,data}

# .gitkeep を配置（必要に応じて）
touch project/docs/9_reference/901_reference_game/architecture/.gitkeep
touch project/docs/9_reference/901_reference_game/screens/.gitkeep
touch project/docs/9_reference/901_reference_game/mechanics/.gitkeep
touch project/docs/9_reference/901_reference_game/data/.gitkeep
```

### Step 0.2: レガシーコードの配置（実際の解析時に実施）

```bash
# legacy/ ディレクトリを作成
mkdir -p legacy/

# レガシーコードを配置
# （例: cp -r ~/old-game-src/* legacy/）
```

**注意**: レガシーコードは読み取り専用として扱います。

---

## Phase 1: 粗視化解析（全体構造の把握）

**目的**: ゲーム全体のアーキテクチャを理解する

**期間**: 1-2週間

### Step 1.1: エントリーポイントの特定

**作業**:
```
ユーザー: 「legacy-analyzer-agentを使って、エントリーポイントを特定して」

legacy-analyzer-agent:
1. legacy/ 配下のファイル一覧を取得
2. main関数を検索
3. ゲームループを特定
4. 901_reference_game/overview.md に記録
```

**生成される文書**: `901_reference_game/overview.md`

### Step 1.2: ディレクトリ構造の把握

**作業**:
```
ユーザー: 「ディレクトリ構造を解析して、各ディレクトリの役割を推測して」

legacy-analyzer-agent:
1. legacy/src/ 配下のディレクトリを列挙
2. ファイル数、命名から役割を推測
3. 901_reference_game/architecture/directories.md に記録
```

**生成される文書**: `901_reference_game/architecture/directories.md`

### Step 1.3: 主要クラス/構造体の列挙

**作業**:
```
ユーザー: 「Serena MCPを使って主要クラスをリストアップして」

legacy-analyzer-agent:
1. Serena MCPでクラス一覧を取得
2. 主要そうなクラスをピックアップ
3. 901_reference_game/data/classes.md に記録
```

**生成される文書**: `901_reference_game/data/classes.md`

### Step 1.4: 制作者の記憶で補完

**作業**:
```
制作者（ユーザー）:
- overview.md の推測部分を確認
- 記憶に基づいて補完・修正
- 不明点は「不明」としてマーク
```

### Phase 1 の成果物

- ✅ `901_reference_game/overview.md`
- ✅ `901_reference_game/architecture/directories.md`
- ✅ `901_reference_game/data/classes.md`

---

## Phase 2: 詳細解析（サブシステムごと）

**目的**: 各サブシステムの詳細仕様を記録

**期間**: 2-4週間

### 解析の優先順位

```
1. ゲームロジック（最重要）
2. データ構造
3. UI/画面構成
4. レンダリング
5. 入力処理
6. その他
```

### Step 2.1: ゲームロジックの解析

**作業**:
```
ユーザー: 「ゲームロジックサブシステムを詳細に解析して」

legacy-analyzer-agent:
1. src/game/ ディレクトリを対象
2. Serena MCPで主要クラス/関数を取得
3. 処理フローを追跡
4. 901_reference_game/mechanics/ に記録
```

**生成される文書**:
- `901_reference_game/mechanics/game_state.md`
- `901_reference_game/mechanics/event_system.md`
- その他

### Step 2.2: データ構造の解析

**作業**:
```
ユーザー: 「プレイヤーデータの構造を解析して」

legacy-analyzer-agent:
1. Player クラスを Serena MCP で取得
2. メンバー変数、メソッドを列挙
3. 901_reference_game/data/player.md に記録
```

**生成される文書**:
- `901_reference_game/data/player.md`
- `901_reference_game/data/enemy.md`
- `901_reference_game/data/item.md`

### Step 2.3: UI/画面構成の解析

**作業**:
```
ユーザー: 「画面構成を解析して」

legacy-analyzer-agent:
1. UI関連のクラスを特定
2. 画面遷移フローを推測
3. 901_reference_game/screens/ に記録
```

**生成される文書**:
- `901_reference_game/screens/title.md`
- `901_reference_game/screens/game.md`
- `901_reference_game/screens/menu.md`

### Step 2.4: 制作者の記憶で補完

各サブシステムの解析後、制作者が記憶を元に補完します。

### Phase 2 の成果物

- ✅ `901_reference_game/mechanics/` 配下の詳細文書
- ✅ `901_reference_game/data/` 配下のデータ構造文書
- ✅ `901_reference_game/screens/` 配下の画面仕様文書

---

## Phase 3: アルゴリズム抽出（重要処理の詳細化）

**目的**: 重要なアルゴリズムを詳細に記録

**期間**: 1-2週間

### Step 3.1: 重要アルゴリズムの特定

**制作者が記憶を元に特定**:
- 戦闘システム
- AI
- 物理演算
- スコア計算
- セーブ/ロード
- その他

### Step 3.2: 各アルゴリズムの詳細解析

**作業例（戦闘システム）**:
```
ユーザー: 「戦闘システムのダメージ計算を詳細に解析して」

legacy-analyzer-agent:
1. BattleSystem::calculateDamage() を Serena MCP で取得
2. 処理の流れを追跡
3. 数式やロジックを抽出
4. 901_reference_game/mechanics/battle.md に詳細記録
```

**生成される文書**:
- `901_reference_game/mechanics/battle.md`（詳細版）
- `901_reference_game/mechanics/ai.md`
- `901_reference_game/mechanics/physics.md`

### Phase 3 の成果物

- ✅ 各重要アルゴリズムの詳細文書

---

## Phase 4: 新規仕様の作成

**目的**: 9_reference を元に、新規ゲームの仕様を作成

**期間**: 1-2週間

### Step 4.1: 改善点の洗い出し

**作業**:
```
制作者（ユーザー）:
1. 9_reference/901_reference_game/ の内容を確認
2. 改善したい点をリストアップ
   - 「レンダリングをOpenGLからWebGLに」
   - 「データ形式をバイナリからJSONに」
   - 「複雑な戦闘システムをシンプル化」
```

### Step 4.2: 要件定義の作成

**作業**:
```
ユーザー: 「requirements-agentを使って、改善版の要件定義を作成して」

requirements-agent:
1. 9_reference を参照
2. 改善点を反映した要件を作成
3. 3_ingame, 4_outgame に配置
4. WITH句で参照元を明記
```

**生成される文書**:
- `3_ingame/30100_requirements.md`（改善版）

### Step 4.3: 詳細仕様の作成

**作業**:
```
ユーザー: 「spec-agentを使って、詳細仕様を作成して」

spec-agent:
1. requirements を元に詳細仕様を作成
2. レガシーとの差分を明記
3. 新技術スタックでの実現方法を記述
```

**生成される文書**:
- `3_ingame/30200_spec.md`（詳細仕様）

### Phase 4 の成果物

- ✅ `3_ingame/` 配下の要件・仕様文書
- ✅ `4_outgame/` 配下の要件・仕様文書（必要に応じて）

---

## Phase 5: 実装（実装時期に実施）

**注意**: 実装は仕様書駆動システムが完成してから実施します。

**作業**:
```
ユーザー: 「impl-agentを使って、仕様に基づいて実装して」

impl-agent:
1. 3_ingame, 4_outgame の仕様を確認
2. 新技術スタックで実装
3. src/ に配置
```

---

## 重要な原則

### 1. Serena MCPの積極活用

**必須**: 大規模コードベースでは、Readツールで直接ファイルを読むとトークンが爆発します。

- ✅ **推奨**: Serena MCPで関数/クラス単位で取得
- ❌ **禁止**: 数千行のファイルを直接Read

### 2. 段階的解析

一度に全体を解析せず、段階的に進めます。

### 3. 制作者の記憶との協働

コードだけでは意図が読み取れない部分は、制作者の記憶で補完します。

### 4. 不完全でOK

10万行を完璧に解析することは不可能です。観察できた範囲で記述し、不明点は明記します。

---

## トラブルシューティング

### Q1: コードが複雑すぎて理解できない

**対処法**:
1. より小さな単位に分割
2. 制作者に説明を依頼
3. 該当部分を「不明」としてマーク

### Q2: Serena MCPが使えない/動作しない

**対処法**:
1. MCP設定を確認
2. 小さなファイルはReadツールで対応
3. 可能な限り関数単位で分割して読む

### Q3: 記憶が曖昧で補完できない

**対処法**:
1. 「推測」として記述
2. 複数の可能性を列挙
3. 新規仕様で改善方向を決定

---

## チェックリスト

### Phase 1 完了時

- [ ] overview.md が作成されている
- [ ] ディレクトリ構造が記録されている
- [ ] 主要クラスがリストアップされている
- [ ] 制作者の記憶で補完済み

### Phase 2 完了時

- [ ] ゲームロジックが詳細に記録されている
- [ ] データ構造が明確になっている
- [ ] UI/画面構成が記録されている
- [ ] 各サブシステムが整理されている

### Phase 3 完了時

- [ ] 重要アルゴリズムが詳細に記録されている
- [ ] 数式やロジックが抽出されている
- [ ] コード参照が明記されている

### Phase 4 完了時

- [ ] 改善点がリストアップされている
- [ ] 要件定義が作成されている
- [ ] 詳細仕様が作成されている
- [ ] WITH句で参照元が明記されている

---

## 関連ドキュメント

- [legacy-analyzer-agent](../../agents/legacy-analyzer-agent.md)
- [9_reference Overview](../9_reference/90000_overview.md)
- [制作者向け手順書](./legacy-code-workflow-for-creator.md)
