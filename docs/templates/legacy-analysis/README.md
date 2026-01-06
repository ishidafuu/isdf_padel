# レガシーコード解析テンプレート - 使用ガイド

**最終更新**: 2025-12-17

## 概要

このディレクトリには、レガシーコード解析（Phase 1〜3）で使用するテンプレートが含まれています。

legacy-analyzer-agentと制作者が協力してレガシーコードの仕様を記録する際に、これらのテンプレートを活用してください。

---

## テンプレート一覧

| フェーズ | テンプレート | 目的 |
|---------|-------------|------|
| Phase 1 | phase1-overview-template.md | ゲーム全体の概要 |
| Phase 1 | phase1-directories-template.md | ディレクトリ構造の把握 |
| Phase 1 | phase1-classes-template.md | 主要クラスのリストアップ |
| Phase 2 | phase2-subsystem-template.md | サブシステムの詳細解析 |
| Phase 2 | phase2-data-structure-template.md | データ構造の詳細解析 |
| Phase 3 | phase3-algorithm-template.md | 重要アルゴリズムの詳細記録 |

---

## 使用方法

### Phase 1: 粗視化解析

#### 1. overview.md の作成

**テンプレート**: `phase1-overview-template.md`

**配置先**: `project/docs/9_reference/901_reference_game/overview.md`

**作業の流れ**:
```
1. テンプレートをコピー
   cp docs/templates/legacy-analysis/phase1-overview-template.md \
      project/docs/9_reference/901_reference_game/overview.md

2. legacy-analyzer-agentに解析を依頼
   「エントリーポイントを特定して、overview.mdを埋めて」

3. 生成された内容を確認
   - 推測部分を確認
   - 制作者の記憶で補完

4. 制作者が「制作者による補完」セクションを記入
```

#### 2. directories.md の作成

**テンプレート**: `phase1-directories-template.md`

**配置先**: `project/docs/9_reference/901_reference_game/architecture/directories.md`

**作業の流れ**:
```
1. ディレクトリを作成
   mkdir -p project/docs/9_reference/901_reference_game/architecture

2. テンプレートをコピー
   cp docs/templates/legacy-analysis/phase1-directories-template.md \
      project/docs/9_reference/901_reference_game/architecture/directories.md

3. legacy-analyzer-agentに解析を依頼
   「ディレクトリ構造を解析して、directories.mdを埋めて」

4. 制作者が補完
```

#### 3. classes.md の作成

**テンプレート**: `phase1-classes-template.md`

**配置先**: `project/docs/9_reference/901_reference_game/data/classes.md`

**作業の流れ**:
```
1. ディレクトリを作成
   mkdir -p project/docs/9_reference/901_reference_game/data

2. テンプレートをコピー
   cp docs/templates/legacy-analysis/phase1-classes-template.md \
      project/docs/9_reference/901_reference_game/data/classes.md

3. legacy-analyzer-agentに解析を依頼
   「Serena MCPを使って主要クラスをリストアップし、classes.mdを埋めて」

4. 制作者が補完
```

---

### Phase 2: 詳細解析

#### 1. subsystem.md の作成（サブシステムごと）

**テンプレート**: `phase2-subsystem-template.md`

**配置先**: `project/docs/9_reference/901_reference_game/mechanics/[サブシステム名].md`

**作業の流れ**:
```
1. ディレクトリを作成
   mkdir -p project/docs/9_reference/901_reference_game/mechanics

2. テンプレートをコピー
   cp docs/templates/legacy-analysis/phase2-subsystem-template.md \
      project/docs/9_reference/901_reference_game/mechanics/game_logic.md

3. legacy-analyzer-agentに解析を依頼
   「ゲームロジックサブシステムを詳細に解析して、game_logic.mdを埋めて」

4. 制作者が補完
   - 処理の意図を説明
   - 設計判断を記録
```

**主要なサブシステム**:
- game_logic.md（ゲームロジック）
- rendering.md（レンダリング）
- input.md（入力処理）
- ui.md（UI）

#### 2. data-structure.md の作成（データ構造ごと）

**テンプレート**: `phase2-data-structure-template.md`

**配置先**: `project/docs/9_reference/901_reference_game/data/[データ構造名].md`

**作業の流れ**:
```
1. テンプレートをコピー
   cp docs/templates/legacy-analysis/phase2-data-structure-template.md \
      project/docs/9_reference/901_reference_game/data/player.md

2. legacy-analyzer-agentに解析を依頼
   「Playerデータ構造を詳細に解析して、player.mdを埋めて」

3. 制作者が補完
   - メンバー変数の意味を説明
   - データ形式の選択理由を記録
```

**主要なデータ構造**:
- player.md（プレイヤー）
- enemy.md（敵）
- item.md（アイテム）
- stage.md（ステージ）

---

### Phase 3: アルゴリズム抽出

#### algorithm.md の作成（アルゴリズムごと）

**テンプレート**: `phase3-algorithm-template.md`

**配置先**: `project/docs/9_reference/901_reference_game/mechanics/[アルゴリズム名].md`

**作業の流れ**:
```
1. テンプレートをコピー
   cp docs/templates/legacy-analysis/phase3-algorithm-template.md \
      project/docs/9_reference/901_reference_game/mechanics/damage_calculation.md

2. legacy-analyzer-agentに解析を依頼
   「ダメージ計算アルゴリズムを詳細に解析して、damage_calculation.mdを埋めて」

3. 制作者が補完
   - 数式の根拠を説明
   - バランス調整の経緯を記録
   - 設計判断を記録
```

**重要なアルゴリズム例**:
- damage_calculation.md（ダメージ計算）
- ai_behavior.md（AI処理）
- collision_detection.md（衝突判定）
- pathfinding.md（経路探索）

---

## テンプレートの構成要素

### 共通要素

全てのテンプレートに含まれる要素：

1. **解析日・解析者**: いつ、誰が解析したか
2. **参照**: コードの場所（ファイル名、行番号）
3. **確認事項**: 制作者に確認したいこと（チェックボックス）
4. **制作者による補完**: 制作者が記憶を元に記入
5. **メモ**: 気づいた点、補足事項

### 推測と確定の区別

- **推測**: 「（推測）」と明記
- **確定**: 制作者が確認して「確認済み」とマーク
- **不明**: 「不明」と明記し、新規仕様で改善

---

## テンプレートのカスタマイズ

### 必要に応じてセクションを追加・削除

テンプレートは**ガイドライン**です。プロジェクトに合わせてカスタマイズしてください：

- 不要なセクションは削除
- 必要なセクションは追加
- 項目の順序は変更可能

### 例: シンプル版

小規模なプロジェクトでは、セクションを減らしてもOK：

```markdown
# [アルゴリズム名]

## 概要
[説明]

## コード
```cpp
// コード
```

## 制作者による補完
- [記憶に基づく説明]
```

---

## 実践的なヒント

### 1. 完璧を求めない

- テンプレートの全項目を埋める必要はありません
- **観察できた範囲で記述**すればOK
- 不明点は「不明」としてマーク

### 2. 段階的に充実させる

- 最初は大まかに記入
- 後から詳細を追加
- 制作者の記憶が蘇ったら随時更新

### 3. チェックボックスを活用

- [ ] 未確認の項目
- [x] 確認済みの項目

チェックボックスで進捗を管理してください。

### 4. 制作者のメモを積極的に

「制作者による補完」「制作者のメモ」セクションは非常に重要です：

- 当時の設計判断
- 試行錯誤の経緯
- 反省点
- 改善案

これらは**コードからは読み取れない貴重な情報**です。

---

## フォルダ構造の例

テンプレートを使って解析を進めると、以下のような構造になります：

```
project/docs/9_reference/901_reference_game/
├── overview.md                      # Phase 1
├── architecture/
│   ├── directories.md               # Phase 1
│   └── dependencies.md              # Phase 2（任意）
├── data/
│   ├── classes.md                   # Phase 1
│   ├── player.md                    # Phase 2
│   ├── enemy.md                     # Phase 2
│   └── item.md                      # Phase 2
├── screens/
│   ├── title.md                     # Phase 2
│   └── game.md                      # Phase 2
└── mechanics/
    ├── game_logic.md                # Phase 2
    ├── rendering.md                 # Phase 2
    ├── damage_calculation.md        # Phase 3
    └── ai_behavior.md               # Phase 3
```

---

## よくある質問

### Q1: テンプレートを使わないといけませんか？

**A**: いいえ。テンプレートはガイドラインです。

- 自由な形式で記述してもOK
- ただし、テンプレートを使うと**統一感**が出ます
- legacy-analyzer-agentもテンプレートに沿った方が作業しやすい

### Q2: 全ての項目を埋めないといけませんか？

**A**: いいえ。**観察できた範囲**で記述すればOKです。

- 不明な項目は「不明」とマーク
- 後から追加・更新も可能

### Q3: Phase 1→2→3の順番は厳密ですか？

**A**: 推奨順序ですが、柔軟に対応してください。

- Phase 2 の途中で Phase 3 の解析が必要になることもあります
- 重要な部分から優先的に解析してもOK

### Q4: テンプレートを改善したい

**A**: 改善は歓迎です。

- プロジェクトに合わせてカスタマイズ
- 改善した内容は `docs/templates/` に反映
- 他のプロジェクトでも活用可能に

---

## 関連ドキュメント

- [レガシーコード解析ガイド](../../guides/legacy-code-analysis.md)
- [制作者向け手順書](../../guides/legacy-code-workflow-for-creator.md)
- [legacy-analyzer-agent](../../../agents/legacy-analyzer-agent.md)
- [9_reference Overview](../../9_reference/90000_overview.md)

---

## まとめ

これらのテンプレートは、レガシーコード解析を**効率的かつ体系的**に進めるためのツールです。

- **柔軟に使う**: 完璧を求めない
- **段階的に**: Phase 1 → 2 → 3
- **協力して**: legacy-analyzer-agent + 制作者

着実に進めていきましょう。
