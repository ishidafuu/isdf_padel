# 仕様書執筆ガイド

本ガイドは、仕様書駆動開発フレームワークにおける仕様書の書き方を網羅的に説明します。

---

**対象読者**: 仕様書を作成する開発者
**所要時間**: 40分
**前提知識**: [フレームワーク仕様書](./framework-spec.md)の番号体系・層責務を理解していること
**次に読むべき**: [ツールリファレンス](./tools-reference.md)（エージェント・コマンド一覧）

---

## 仕様書構造

### ECSアーキテクチャとの関係（参考）

本フレームワークはECSを前提としているが、仕様書構成とECS概念は1対1対応ではない。

| 仕様書 | ECSで主に記述する内容 |
|---|---|
| design.md | Component の構造定義 |
| behavior.md | System のロジック |
| spec.md | （ECSとは独立した要件定義） |

Entity は「spec + design の組み合わせで定義されるもの」として扱う。

---

### spec.md（What/Why = 要件）

```markdown
# [機能名] Spec

## Overview
この機能が存在する理由（1-2文）

## Requirements（EARS記法）

### REQ-[ファイル番号]-001: [要件名]
- WHEN [トリガー条件]
- THE SYSTEM SHALL [実行内容]
- AT/WITH [パラメータ/制約]
- **テスト**: TST-[番号]-xxx
- **Issue**: #12

## Constraints（Design by Contract）

### Preconditions
- [事前条件]

### Postconditions
- [事後条件]

### Invariants
- [不変条件]
```

### design.md（Component層 = データ構造）

**原則**: データ構造のみ定義。ロジック（メソッド）は behavior.md に記載。

```markdown
# [機能名] Design

## Components

### 共有Component（参照）
- [PositionComponent](../../2_architecture/209_components/20901_transform.md)

### 機能固有Component

#### [ComponentName]
| Field | Type | Default | Description |
|---|---|---|---|
| fieldName | type | value | 説明 |

## Constants

**CRITICAL**: System で使用する定数は **ハードコーディング禁止**。全て GameConfig 等の外部データとして定義する。

**❌ 禁止**: design.md に具体的な値をハードコーディング

| Name | Value | Description |
|---|---|---|
| JUMP_BUFFER_TIME | 0.1f | 入力先行受付時間 |
| GRAVITY | -9.8f | 重力加速度 |

**✅ 必須**: 設定ファイル参照として記述

| Name | Source | Default | Description |
|---|---|---|---|
| JUMP_BUFFER_TIME | `config.Input.JumpBufferTime` | 0.1f | 入力先行受付時間 |
| GRAVITY | `config.Physics.Gravity` | -9.8f | 重力加速度 |

**データ定義場所**: `8_data/80101_game_constants.md` 等に GameConfig 構造を定義

## Enums

### [EnumName]
| Value | Description |
|---|---|
| ValueA | 説明 |

## Interfaces（Core層）

```csharp
public interface I[Name]
{
    // ...
}
\```
\```

### behavior.md（System層 = ロジック）

**原則**: ロジックのみ定義。データ構造は design.md で定義済みのものを参照。

```markdown
# [機能名] Behavior

## Systems

### [SystemName]
**責務**: [一文で説明]

**フィルタ条件**: [対象となるEntityの条件を自然言語で記述]
**入力**: [読み取るComponent]
**出力**: [更新するComponent]

**ロジック**:
1. [ステップ1]
2. [ステップ2]

## State Transition

\```mermaid
stateDiagram-v2
    [*] --> State1
    State1 --> State2 : Event
\```

## Algorithms

### [アルゴリズム名]
[疑似コードまたは説明]
\```

### test.md（テストシナリオ）

```markdown
# [機能名] Test

## テスト分割の目安

| ケース | 推奨 |
|--------|------|
| 1つの Given/When/Then で表現できる | 1 TST |
| 条件分岐がある | 分岐ごとに TST |
| 正常系 + 境界値 | 2-3 TST |
| 正常系 + 異常系複数 | 個別に TST |

**迷ったら分ける**（細かすぎても害は少ない）

**上限目安**: 1つの test.md に TST が 20 を超えたら機能分割を検討

## TST-[ファイル番号]-001: [テスト名]
**検証対象**: REQ-[番号]-xxx

Given [前提条件]
When [操作]
Then [期待結果]

## TST-[ファイル番号]-002: [テスト名]
**検証対象**: REQ-[番号]-xxx

Given [前提条件]
When [操作]
Then [期待結果]
\```

**テストIDと要件の関係:**
- 1つの要件（REQ）に対して、複数のテスト（TST）を作成してよい
- テストIDは test.md のファイル番号を使用する（例: TST-30105-001）
- `**検証対象**` フィールドで対応する要件を明示する
- **異なるフォルダの REQ を検証する場合も、テストIDは test.md のファイル番号を使用する**（番号の混在を許容）

### design.md と behavior.md の境界

| 内容 | 配置先 |
|------|--------|
| データフィールド定義 | design.md |
| 定数（Systemで使用する固定値） | design.md |
| Enum 定義 | design.md |
| インターフェース定義 | design.md |
| 状態遷移ロジック | behavior.md |
| アルゴリズム・計算式 | behavior.md |
| System の責務定義 | behavior.md |

**迷ったら behavior.md** に書く。

### behavior.md 記述ガイドライン

behavior.md は「疑似コード」よりも **「ルールと制約」** に重きを置く。AIにとっては、具体的なコードよりも「見落としがちなエッジケースの挙動」が言語化されている方が、バグの少ないコードを生成できる。

**推奨する記述**:
- 「HPが0以下になったフレームの**最後**に死亡ステートへ遷移する（即時ではない）」
- 「ダメージ計算は防御力を引いた後、最低1ダメージを保証する」
- 「空中でジャンプ入力があった場合、着地後にジャンプを実行する（入力バッファ: 0.1秒）」

**避けるべき記述**:
- `if (hp <= 0) { state = Dead; }` （コードを見ればわかる）
- 単純な条件分岐の列挙（状態遷移図で表現すべき）

**記述の観点**:
- タイミング（即時？フレーム末？遅延あり？）
- 境界値（0以下？未満？最小値保証？）
- 例外・エッジケース（入力バッファ、キャンセル可否）
- 優先順位（複数条件が同時に成立した場合）

**ECS原則**:
- **System はステートレス**: タイマー、フラグ、カウンターなどの「状態」が必要になった場合、System のメンバ変数ではなく、design.md に戻って Component のフィールドとして定義する
- **フィードバックループ**: behavior.md 記述中にデータ不足に気づいたら、即座に design.md を更新する

---

## テストコードとの対応（任意）

test.md は「テスト仕様書」として機能し、自動テスト・手動テストの両方をサポートする。

### 運用方式の選択

| 方式 | test.md の役割 | 推奨ケース |
|---|---|---|
| 自動テスト | テスト設計書 | コア機能（衝突判定、ダメージ計算等） |
| 手動テスト | チェックリスト | UI、演出、操作感の確認 |
| 段階的 | 両方を併用 | 初期は手動、重要機能のみ自動化 |

### 自動テストの場合

**ファイル配置**
```
Tests/
└── 301_Player/
    └── PlayerJumpTests.cs  # TST-30105-xxx に対応
```

**テストメソッド命名**
```csharp
// @test TST-30105-001
[Test]
public void Jump_OnGround_ShouldApplyUpwardVelocity()
{
    // Given: プレイヤーが地上にいる
    // When: ジャンプボタンを押す
    // Then: プレイヤーが上方向に移動する
}
```

### 手動テストの場合

test.md をチェックリストとして使用:

```markdown
## TST-30105-001: 地上ジャンプ
**検証対象**: REQ-30101-001

Given プレイヤーが地上にいる
When ジャンプボタンを押す
Then プレイヤーが上方向に移動する
```

実施記録は省略可。問題発見時は Issue を作成する。

---

## トレーサビリティID体系

### ID形式

```
[種別]-[ファイル番号]-[連番3桁]
```

### 種別コード

| コード | 種別 | 使用場所 | 例 |
|---|---|---|---|
| REQ | 要件 | spec | REQ-30101-001 |
| EXT | 拡張性要件 | spec | EXT-30101-001 |
| DES | 設計要素 | design | DES-30102-001 |
| BHV | 振る舞い | behavior | BHV-30103-001 |
| MOD | モジュール設計 | module | MOD-30104-001 |
| TST | テストケース | test | TST-30105-001 |

---

## 実装コメント規約

仕様書と実装コードの対応関係を明示するため、以下のコメント規約を使用する。

### タグ一覧

| タグ | 用途 | 例 |
|---|---|---|
| `@spec` | 要件との対応 | `// @spec REQ-30101-001` |
| `@test` | テストとの対応 | `// @test TST-30105-001` |
| `@data` | データ定義との対応 | `// @data 80101_enemy_params.md#enemy_slime` |

### ルール

- 要件を実装するメソッドやクラスに `@spec` コメントを付与
- テストメソッドに `@test` コメントを付与
- データ定義を参照する箇所に `@data` コメントを付与
- 1つの実装が複数の要件に対応する場合は複数行で記載
- コメントは実装の直前に配置

### 例

```csharp
// @spec REQ-30101-001
// @spec REQ-30101-002
public class PlayerJumpSystem : ISystem
{
    public void Execute()
    {
        // ジャンプ処理
    }
}

// @data 80101_enemy_params.md#enemy_slime
public static readonly EnemyParam Slime = new(hp: 10, attack: 2, speed: 1.0f);
```

---

## 共有Componentの運用ルール

### 基本方針

- 初期は機能固有Componentとして作成
- 2つ目の機能で同じComponentが必要になった時点で共有化

### 共有化の手順

1. `/component-share [Component名] [元ファイル番号]` を実行
2. `209_components/` に移動される
3. 元の機能から参照リンクに置き換え
4. `20002_dependencies.md` を更新

### 共有化の判断基準

| 状況 | 判断 |
|------|------|
| 1機能でのみ使用 | 機能固有のまま |
| 2機能以上で使用 | 共有化 |
| 将来的に共有される可能性が高い | 機能固有のまま（早すぎる抽象化を避ける） |

---

## 機能の廃止

### 廃止マーカー（移動前の一時状態）

即座に移動できない場合、overview.md に廃止予定を明記:

```markdown
# [機能名] Overview

> ⚠️ **DEPRECATED**: v0.5 で廃止予定。代替: [新機能](../3xx_new/)
```

### 廃止チェックリスト

- [ ] overview.md に廃止マーカーを追加（任意）
- [ ] `/deps-check` を実行し、参照元を特定
- [ ] 参照元のリンクを更新（0件になるまで）
- [ ] 該当フォルダを `docs/_deprecated/` に移動
- [ ] `20002_dependencies.md` から参照を削除
- [ ] 関連 Issue をクローズ
- [ ] commit: `[番号] refactor: [機能名] を廃止`

### ディレクトリ構成

```
docs/
├── 3_ingame/
│   └── 301_player/
└── _deprecated/
    └── 399_old_feature/  # 廃止された機能
```

### 完全削除

- マイルストーン完了後、`_deprecated/` 内のフォルダを削除してよい
- 履歴は Git に残る

### 廃止時の注意事項

- **ファイル番号は変更しない**（Git 履歴で追跡可能にするため）
- **コマンドの除外**: `/id-list`, `/docs-validate` 等のコマンドはデフォルトで `_deprecated/` を除外する。`--include-deprecated` オプションで含めることも可能

---

## タスク管理

タスクはMarkdownファイルベースのシステムで管理します。詳細は `.claude/skills/task-workflow.md` を参照してください。

---

## 開発フロー

### 仕様書の完成度と開発フェーズ

| フェーズ | 作成する仕様書 | 実装可否 | 備考 |
|----------|---------------|----------|------|
| アイデア検討 | （任意）spec ドラフト | ✗ | Issue で十分な場合も |
| 要件固め | spec.md | ✗ | 必須 |
| 設計 | design.md | △ | プロトタイプ可 |
| 実装準備完了 | behavior.md | ◯ | 推奨（実装中に書いても可） |
| TDD開始 | test.md | ◎ | 任意 |

**原則**: 最低限 spec + design が揃ってから実装を開始する

### 新機能追加

```
1. 仕様書作成
   ├─ spec.md: 要件定義
   ├─ design.md: データ構造
   └─ behavior.md: ロジック

2. タスク作成
   ├─ タスク名: "Player ジャンプ機能実装"
   └─ 説明欄に関連 REQ-ID を記載

3. Claude Code が実装

4. PR 作成

5. レビュー・マージ
```

### 新規機能フォルダの追加手順

1. 親フォルダ内で次の連番を確認（例: 303の次は304）
2. `3XX_[機能名]/` フォルダを作成
3. spec, design, behavior, test の4ファイルを作成（overview は任意）
4. `20002_dependencies.md` に依存関係を追記
5. 親フォルダの overview.md に目次を追加

### 仕様変更フロー

```
変更が必要になった
    │
    ▼
タスク作成（task-manager-agent）
    │
    ▼
仕様書を更新（spec/design/behavior）
    │
    ▼
Git commit（仕様変更）
    │
    ▼
実装
    │
    ▼
Git commit（実装）
    │
    ▼
PR → マージ
    │
    ▼
タスク完了（status: done）
```

**重要:** タスクIDはコミットメッセージに含めます：
```
[30101] spec: プレイヤージャンプ機能追加
[30101] impl: プレイヤージャンプ機能実装
```

### Git commit 規約

```
[ファイル番号] [種別]: 変更内容

種別（仕様書）:
- spec: 要件変更
- design: データ構造変更
- behavior: ロジック変更
- test: テスト追加・変更

種別（実装）:
- feat: 新機能実装
- fix: バグ修正
- refactor: リファクタリング
- balance: 8_data/ の値調整（例: 敵HP 50→55）。仕様書更新不要
```

**Issue 省略可の特例**:

以下のケースでは Issue 作成を省略し、コミットメッセージのみで処理してよい:
- 明白なバグ修正（Hotfix）
- ドキュメントの誤字修正
- 議論の余地がない軽微な変更

```bash
# Issue なしの軽微な修正
git commit -m "fix: ダメージ計算の0除算を修正"
git commit -m "docs: README の誤字修正"
```

**PR連携の例**:
```bash
# 仕様書更新
git commit -m "[30101] spec: ジャンプ機能の要件定義"

# 実装（Vibe Task #12 に紐付け）
git commit -m "[30101] feat: ジャンプ機能実装

Vibe Task: #12"
```

---

## overview テンプレート

### overview テンプレート（層）

```markdown
# [層名] Overview

## 目的
この層の責務（1-2文）

## 機能一覧

| フォルダ | 説明 | ステータス |
|----------|------|-----------|
| 301_player | プレイヤー制御 | 実装中 |
| 302_enemy | 敵AI | 未着手 |

## 関連
- [上位層へのリンク]
- [依存関係図](../2_architecture/20002_dependencies.md)
```

### overview テンプレート（機能）

```markdown
# [機能名] Overview

## 目的
この機能が存在する理由（1-2文）

## 構成
| ファイル | 説明 |
|---|---|
| [spec](./xxxxx_xxx_spec.md) | 要件定義 |
| [design](./xxxxx_xxx_design.md) | データ構造 |
| [behavior](./xxxxx_xxx_behavior.md) | 振る舞い |
| [test](./xxxxx_xxx_test.md) | テスト |

## 関連タスク
- Vibe Task #12: ジャンプ機能実装
- Vibe Task #13: 移動機能実装

## 関連機能
- [Enemy](../302_enemy/)
```

---

## 仕様書執筆のベストプラクティス

### 1. ハードコーディング禁止原則（CRITICAL）

**NEVER hardcode parameter values. ALL adjustable values MUST be externalized to data files.**

#### 対象となる値

以下の値は**必ず**外部データ化してください：

| 分類 | 例 |
|------|---|
| **物理パラメータ** | 重力、摩擦係数、バウンド係数 |
| **移動パラメータ** | 最大速度、加速度、ジャンプ力 |
| **サイズ・範囲** | コートサイズ、当たり判定半径、視野範囲 |
| **時間** | 無敵時間、ふっとばし時間、入力バッファ時間 |
| **ゲームバランス** | ダメージ、得点、経験値 |

#### 仕様書での記載ルール

具体的な値は「参考値」として示すが、実装時は必ず設定ファイル参照であることを明記：

```markdown
✅ 良い例:
- 重力: `config.Physics.Gravity` (デフォルト: -9.8 m/s²)
- 最大速度: `config.Player.MaxSpeed` (デフォルト: 5.0 m/s)

❌ 悪い例:
- 重力: -9.8 m/s²
- 最大速度: 5.0 m/s
```

#### 例外（ハードコーディング可能な値）

以下のみハードコーディング可能：

- **数学定数**: `Math.PI`, `Math.E`
- **ビルトイン定数**: `Vector3.Zero`, `Color.White`
- **実装ロジックの定数**: 配列サイズ、ループ回数（ゲームバランスに無関係な値）

**原則**: 「後で調整する可能性がある値」は全て外部化

#### データ配置場所

- **定義**: `8_data/80101_game_constants.md` 等に GameConfig 構造を定義
- **実装**: Godot Resource（`.tres`）または JSON/YAML
- **アクセス**: System に依存注入

詳細は [80101_game_constants.md](../../project/docs/8_data/80101_game_constants.md)（プロジェクト側）を参照。

### 2. EARS記法の活用

- 要件は EARS記法（Event-driven, State-driven, Unwanted behavior等）で構造化
- 詳細は [EARS記法スキル](../../skills/ears.md) を参照

### 3. Design by Contract の明示

- Preconditions（事前条件）
- Postconditions（事後条件）
- Invariants（不変条件）

これらを spec.md の Constraints セクションに明記する。

### 4. エッジケースの言語化

- 「何が起きるか」だけでなく「いつ起きるか」を明記
- 境界値、タイミング、優先順位を明確にする

### 5. データとロジックの分離

- design.md: データ構造のみ
- behavior.md: ロジックのみ
- 迷ったら behavior.md に書く

### 6. テストと要件の双方向トレーサビリティ

- spec.md: `**テスト**: TST-30105-001`
- test.md: `**検証対象**: REQ-30101-001`

---

## 次に読むべきドキュメント

- [ツールリファレンス](./tools-reference.md) - エージェント・コマンド・Skills一覧
- [設計判断集](./design-decisions.md) - フレームワークの設計理由
- [フレームワーク仕様書](./framework-spec.md) - コア仕様全体
