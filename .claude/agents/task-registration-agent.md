---
name: task-registration-agent
type: guideline
description: |
  タスク登録の処理ガイドライン。
  プラン検出、タスクタイプ判定、ID採番、ファイル生成の手順を定義。

  ※ このファイルは「実行者」ではなく「処理ガイドライン」です。
  ※ メイン Claude Code がこのガイドラインを参照しながら直接実行します。
---

# Task Registration Guideline

**タスク登録の処理ガイドライン**

> **重要**: このファイルは Task tool で起動する「実行者」ではありません。
> メイン Claude Code がこのガイドラインを参照し、記載された手順に従ってツールを直接実行します。

## このガイドラインの使い方

```
ユーザー: 「プランからタスクを作成して」
    ↓
メイン Claude Code:
    1. 最新プランファイルを取得（ls -t で更新日時順）
    2. プランを読み込み、概要を表示して確認
    3. タスクタイプを判定
    4. ID採番（Skill: id-next）
    5. タスクファイルを作成（Write）
    6. 完了報告
```

**重要**: 必ず最新のプランファイルを使用し、概要を表示してから作成する。

## 概要

タスクファイルの作成手順を定義します。

**タスクタイプ別の入力:**
- **ゲーム開発（30XXX）**: 仕様書から直接タスク作成（プランモード不要）
- **その他（B30XXX/R30XXX/FXXX/PXXX）**: プランファイルからタスク作成

**処理内容:**
- 入力の判定（仕様書 or プランファイル）
- タスクタイプの判定（framework/game-dev/bugfix/refactor/project-wide）
- ID採番とタスクファイル生成
- 内容の構造化とタスクフォーマットへの変換

## 処理の原則

- **自動検出**: プランファイルを自動発見し、手動指定を最小化
- **明確**: タスクタイプを確実に判定し、曖昧さを排除
- **簡潔**: 登録のみに集中し、余計な操作を行わない
- **トレーサブル**: プランファイルへの参照を保持し、情報源を明確化

## ガイドラインの対象範囲

**このガイドラインで扱う処理**:
- ✅ **ゲーム開発タスク**: 仕様書から直接タスク作成（プランファイル不要）
- ✅ **その他タスク**: プランファイル自動検出（`~/.claude/plans/` から最新を選択）
- ✅ タスクタイプ判定（明示的マーカー → パス解析 → ユーザー確認）
- ✅ タスクID採番（Skill: id-next）
- ✅ タスクファイル生成（status: todo、1_todo/）
- ✅ プラン/仕様書内容の埋め込み（Detailed Implementation Plan セクション）

**このガイドラインで扱わない処理**（task-manager-agent ガイドラインを参照）:
- ❌ タスク状態更新（todo → in-progress → done）
- ❌ worktree管理
- ❌ プラン作成（プランモードはユーザーが実行）
- ❌ 仕様書作成（spec-agent が実行）
- ❌ 依存関係の管理

## ガイドラインの使い分け

| 処理 | 参照するガイドライン |
|------|---------------------|
| タスク作成（プランから） | このガイドライン |
| タスク状態更新 | task-manager-agent.md |
| worktree作成 | task-manager-agent.md |
| 依存関係管理 | task-manager-agent.md |
| タスク検索 | task-manager-agent.md |

**判断基準**: プランファイル → タスクファイル変換はこのガイドライン、それ以外のタスク操作は task-manager-agent.md

---

## 使用するツール

タスク登録処理では以下のツールを使用します。

### 必須: Skill（id-next）

タスクファイルを生成する前に、**必ず Skill ツールで `id-next` を実行**してIDを取得します。

**実行方法**: `Skill(skill="id-next")`

**出力例**:
```
次のID: F005

既存ID:
- F001
- F002
- F003
- F004
```

### 推奨コマンド

#### `/task-status` - タスク状況確認（作成前確認）

タスク作成前に、既存タスクの状況を確認することを推奨します。

**実行方法**: Skill ツールを直接実行する（skill="task-status", args="--type framework"）

---

## 処理フロー

### Phase 0: タスクタイプ別分岐（CRITICAL）

**タスクタイプによって処理フローが異なる。**

```
ユーザーの依頼内容を確認
  ↓
┌──────────────────────────────────────────────┐
│ 「仕様書から実装タスクを作成して」              │
│ 「ジャンプ機能の実装タスクを作成して」          │
│  → ゲーム開発フロー（Phase A）                │
├──────────────────────────────────────────────┤
│ 「プランからタスクを作成して」                  │
│  → プランファイル経由フロー（Phase 1〜6）      │
└──────────────────────────────────────────────┘
```

---

## ゲーム開発フロー（Phase A）: 仕様書から直接タスク作成

**プランモード不要。仕様書が既に「計画」の役割を果たしている。**

### Phase A-1: 仕様書の確認

```
ユーザー: 「ジャンプ機能の実装タスクを作成して」
  ↓
仕様書を確認（Read）:
  - project/docs/3_ingame/30X_*/spec.md
  ↓
仕様書の内容を表示:
  - タイトル
  - 概要（Summaryセクション）
  - 主要な要件（REQ-ID）
```

### Phase A-2: ID採番

```
Skill(skill="id-next", args="30XXX")
  ↓
次のID: 30101
```

### Phase A-3: タスクファイル生成

```
Write: project/tasks/1_todo/30101-ジャンプ機能実装.md

---
id: "30101"
title: "ジャンプ機能実装"
type: "game-dev"
status: "todo"
priority: "medium"
spec_ids: ["30101"]  # 対応する仕様書ID
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: null  # ゲーム開発はプランファイル不要
tags: []
created_at: "..."
updated_at: "..."
completed_at: null
---

# Task 30101: ジャンプ機能実装

## 説明

[仕様書の概要から抽出]

## 関連仕様書

- project/docs/3_ingame/301_player/30101_jump_spec.md

## 実装内容

[仕様書の要件から実装チェックリストを生成]
- [ ] REQ-30101-001: ...
- [ ] REQ-30101-002: ...

## 完了チェックリスト

> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] in-review に移動済み
- [ ] レビュー完了

## メモ

[必要に応じて追記]
```

### Phase A-4: 完了報告

```
✅ タスク登録完了

Task ID: 30101
Type: game-dev
Status: todo
File: project/tasks/1_todo/30101-ジャンプ機能実装.md
Spec: project/docs/3_ingame/301_player/30101_jump_spec.md

次のステップ:
- タスクを開始: 「30101を開始」と依頼してください
```

---

## プランファイル経由フロー（Phase 1〜6）: バグ修正・リファクタ・フレームワーク・プロジェクト横断

### Phase 1: 最新プランファイルの取得（CRITICAL）

**必ず最新のプランファイルを取得する。時間窓ではなく、更新日時の最新を優先。**

**検出ロジック**:
```bash
# ~/.claude/plans/ から最新のプランファイルを取得（更新日時順）
ls -t "$HOME/.claude/plans"/*.md 2>/dev/null | head -1
```

**検出基準**:
- 場所: `~/.claude/plans/*.md`
- 優先順位: **更新日時が最新のファイルを選択**（時間窓による制限なし）

**エラーハンドリング**:
| シナリオ | 動作 |
|---------|------|
| プランファイルなし | 「プランファイルが見つかりません」と報告 |
| ディレクトリなし | 「~/.claude/plans/ が存在しません」と報告 |

### Phase 1.5: プラン概要の確認表示（CRITICAL）

**タスク作成前に、必ずプランの概要をユーザーに表示して確認する。**

**表示フォーマット**:
```
📋 プラン確認

ファイル: ~/.claude/plans/xxx.md
更新日時: 2025-12-30 21:45

## タイトル
[プランの見出し]

## 概要
[Summary または 概要 セクションの内容]

## タスクタイプ
[framework / game-dev / project-wide]

---
このプランからタスクを作成します。
```

**実行手順**:
1. Read でプランファイルを読み込む
2. タイトル（# 見出し）を抽出
3. Summary または 概要 セクションを抽出
4. Task Type セクションを確認
5. 上記フォーマットで表示

**構造検証**:
- 必須セクション: `## Summary` または `## 概要`
- 無効な構造の場合はエラー報告、プラン修正依頼

### Phase 2: タスクタイプ判定

**優先順位**:
1. **Level 1: 明示的マーカー** - プランファイル内の `## Task Type` セクション
   ```markdown
   ## Task Type

   game-dev
   ```

2. **Level 1b: バグ修正/リファクタキーワード検出** - プラン内のキーワードから判定
   - "バグ修正", "bug fix", "不具合", "修正" → bugfix
   - "リファクタ", "refactor", "最適化", "改善", "リファクタリング" → refactor
   - 元タスクID（30XXX）の記載を検出 → `related_task` に設定

3. **Level 2: パス解析** - プラン内に記載されたファイルパスから判定
   - `project/docs/3_ingame/`, `project/src/` → game-dev
   - `.claude/agents/`, `.claude/commands/`, `.claude/skills/` → framework
   - `CI/CD`, `infrastructure`, `.github/workflows/` → project-wide

4. **Level 3: ユーザー確認** - 判定できない場合は AskUserQuestion で確認
   ```
   タスクタイプを判定できませんでした。
   このタスクはどのタイプですか？

   - framework: フレームワーク開発（.claude/, docs/）
   - game-dev: ゲーム開発（project/docs/, project/src/）
   - bugfix: バグ修正（既存機能の不具合修正）
   - refactor: リファクタリング（既存機能の改善・最適化）
   - project-wide: プロジェクト横断（CI/CD、インフラ）
   ```

5. **Level 3b: ユーザー確認（bugfix/refactorの場合）** - 元タスクIDの確認
   ```
   バグ修正/リファクタタスクです。
   元の機能タスクID（30XXX）を指定してください。

   例: 30101（ジャンプ機能）、30201（敵キャラクター）
   ```

### Phase 3: タスクID採番

Skill ツールで `id-next` を実行してIDを取得する。

**実行方法**:
```
# タスクタイプに応じて引数を指定
Skill(skill="id-next", args="FXXX")    # framework
Skill(skill="id-next", args="30XXX")   # game-dev
Skill(skill="id-next", args="B30101")  # bugfix（元タスクID指定）
Skill(skill="id-next", args="R30101")  # refactor（元タスクID指定）
Skill(skill="id-next", args="PXXX")    # project-wide
```

**出力例**:
```
次のID: F012

既存タスクID:
- F001
- F002
- ...
- F011
```

**ID形式**:
- framework: `FXXX`（例: F012）
- game-dev: `30XXX`（例: 30101）
- bugfix: `B30XXX-NNN`（例: B30101-001）
- refactor: `R30XXX-NNN`（例: R30101-001）
- project-wide: `PXXX`（例: P003）

### Phase 4: 配置先決定

```
if task_type == "framework":
    配置先 = "tasks/1_todo/"
    設定ファイル = "tasks/.taskrc.yaml"

elif task_type in ["game-dev", "bugfix", "refactor"]:
    配置先 = "project/tasks/1_todo/"
    設定ファイル = "project/tasks/.taskrc.yaml"

elif task_type == "project-wide":
    配置先 = "project/tasks/1_todo/"
    設定ファイル = "project/tasks/.taskrc.yaml"
```

### Phase 5: タスクファイル生成

**フロントマター**:
```yaml
---
id: "F005"
title: "[プランのタイトルから抽出]"
type: "framework"  # or "game-dev", "bugfix", "refactor", "project-wide"
status: "todo"  # 初期ステータスは todo
priority: "medium"  # プランから抽出、デフォルトは medium
related_task: null  # 元機能タスクID（bugfix/refactorのみ必須、例: "30101"）
spec_ids: []
blocked_by: []
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/username/.claude/plans/xxx.md"  # プランファイル参照
tags: []
created_at: "2025-12-30T20:00:00+09:00"
updated_at: "2025-12-30T20:00:00+09:00"
completed_at: null
---
```

**本文構造**:
```markdown
# Task F005: [タイトル]

## 説明

[プランの概要から抽出]

## 背景

### 現状
[この改修前の状況・課題をプランから抽出]

### 改修理由
[なぜこの改修を行うかをプランから抽出]

## 実装内容

[プランの実装内容をチェックリスト形式で記載]

## 完了チェックリスト

[タスクタイプに応じて以下から選択]

### game-dev/bugfix/refactor の場合:
> このタスクは in-review 経由必須

- [ ] ビルド成功（`cargo build`）
- [ ] テスト全PASS（`cargo test`）
- [ ] in-review に移動済み
- [ ] レビュー完了

### framework/project-wide の場合:
- [ ] 変更内容の検証完了
- [ ] ドキュメント整合性確認（frameworkのみ）

## メモ

[必要に応じて追記]

## 依存関係

- **ブロック**: なし
- **ブロックされる**: なし
- **関連ドキュメント**: [プランから抽出]

---

## Detailed Implementation Plan

以下は、プランファイル `~/.claude/plans/xxx.md` の全内容です。

[プランファイルの全内容をここにコピー]
```

**重要な注意点**:
- **プランファイルは削除しない** - `plan_file` フィールドで参照を保持
- **初期ステータスは todo** - プラン作成は完了しているため、planning ではなく todo
- **プラン全文を埋め込む** - 情報欠損を防ぐため、プランの全内容を Detailed Implementation Plan セクションに含める
- **完了チェックリストはタスクタイプに応じて選択** - game-dev/bugfix/refactorはin-review必須版、framework/project-wideは簡易版を使用

### Phase 6: 完了報告

```
✅ タスク登録完了

Task ID: F005
Type: framework
Status: todo
File: tasks/1_todo/F005-xxx.md
Plan: ~/.claude/plans/xxx.md（保持）

次のステップ:
- タスクを開始: 「F005を開始」と依頼してください
- ファイル確認: Read("tasks/1_todo/F005-xxx.md")
```

---

## タスクタイプ別処理

### framework タスク

```markdown
配置先: tasks/1_todo/FXXX-*.md
対象: .claude/agents/, docs/, .claude/commands/, .claude/skills/
worktree: 無効（task-manager-agent が判定）
```

**例**:
- エージェント定義の更新
- ドキュメントの追加
- コマンドの実装

### game-dev タスク

```markdown
配置先: project/tasks/1_todo/30XXX-*.md
対象: project/docs/, project/src/, project/tests/
worktree: 有効（task-manager-agent が作成）
```

**例**:
- ジャンプ機能実装
- 敵キャラクター追加
- UI要素の実装

### bugfix タスク

```markdown
配置先: project/tasks/1_todo/B30XXX-NNN-*.md
対象: project/docs/, project/src/, project/tests/
worktree: 有効（task-manager-agent が作成）
related_task: 必須（元の機能タスクID）
```

**例**:
- B30101-001: 着地判定のバグ修正
- B30201-001: 敵の当たり判定バグ修正
- B30301-002: UI表示のバグ修正

### refactor タスク

```markdown
配置先: project/tasks/1_todo/R30XXX-NNN-*.md
対象: project/docs/, project/src/, project/tests/
worktree: 有効（task-manager-agent が作成）
related_task: 必須（元の機能タスクID）
```

**例**:
- R30101-001: ジャンプ処理の最適化
- R30201-001: 敵AIロジックのリファクタリング
- R30301-001: UIコンポーネントの共通化

### project-wide タスク

```markdown
配置先: project/tasks/1_todo/PXXX-*.md
対象: リポジトリ全体（CI/CD、インフラ、.github/workflows/）
worktree: 無効（task-manager-agent が判定）
```

**例**:
- CI/CD構築
- テストインフラ整備
- デプロイ自動化

---

## エラーハンドリング

### プランファイルが存在しない

```
❌ エラー: プランファイルが見つかりません

~/.claude/plans/ にプランファイルが存在しません。

対処方法:
1. プランモードでプランを作成してください
2. プランファイルのパスを直接指定してください
```

### ID採番失敗

```
❌ エラー: ID採番に失敗しました

/id-next コマンドが正常に動作していることを確認してください。

対処方法:
1. /id-next コマンドの動作確認
2. タスク設定ファイル (.taskrc.yaml) の確認
```

### タスクタイプ不明

```
⚠️ 警告: タスクタイプを自動判定できません

プランファイルに明示的なタスクタイプ情報がありませんでした。
以下のいずれかの方法でタスクタイプを指定してください:

方法1: プランファイルにマーカーを追加
---
## Task Type

framework  # or game-dev, bugfix, refactor, project-wide
---

方法2: ユーザー確認（AskUserQuestion）
```

### プランファイル構造エラー

```
❌ エラー: 無効なプラン構造

プランファイルに必須セクションが不足しています。

必須セクション:
- ## Summary（または ## 概要）
- 実装内容の記載

対処方法:
プランを再作成するか、必須セクションを追加してください。
```

---

## 実行例

### 例1: framework タスクの登録

**ユーザー**: 「プランからタスクを作成して」

**task-registration-agent の行動**:

1. **Phase 1**: Bash ツールで `ls -t "$HOME/.claude/plans"/*.md | head -1` を実行
   → 最新のプランファイルのパスを取得

2. **Phase 2**: Read ツールでプランファイルを読み込む
   → `## Task Type: framework` を発見、タイプ確定

3. **Phase 3**: Skill ツールで `id-next` を実行
   → 次のID（例: F005）を取得

4. **Phase 4**: 配置先を決定
   → framework なので `tasks/1_todo/`

5. **Phase 5**: Write ツールでタスクファイルを作成
   → `tasks/1_todo/F005-xxx.md` を実際に書き込む

6. **Phase 6**: 完了報告をテキストで出力

**重要**: 各ステップでツールを**直接実行**する。テキストとして出力しない。

### 例2: game-dev タスクの登録

**ユーザー**: 「ジャンプ機能のプランからタスクを作成して」

**task-registration-agent の行動**:

1. Bash ツールでプランファイルを検索
2. Read ツールでプランを読み込み → パス解析で `game-dev` と判定
3. Skill ツールで ID 採番 → 30101
4. 配置先: `project/tasks/1_todo/`
5. Write ツールで `project/tasks/1_todo/30101-ジャンプ機能実装.md` を作成
6. 完了報告

### 例3: タスクタイプ判定失敗（ユーザー確認）

**ユーザー**: 「新機能のプランからタスクを作成して」

**task-registration-agent の行動**:

1. プランファイル読み込み
2. タスクタイプ判定 → 明示的マーカーなし、パス解析でも判定不可
3. AskUserQuestion ツールでユーザーに確認
4. ユーザーが "game-dev" を選択
5. 以降の処理を続行

---

## 禁止事項

- ❌ タスク状態の変更（todo → in-progress → done）
- ❌ worktreeの作成・管理
- ❌ プランファイルの削除（保持する）
- ❌ タスク依存関係の設定（blocked_by, blocks）
- ❌ タスクの検索・フィルタリング

**これらはすべて task-manager-agent の責務です。**

---

## 設計判断

### なぜタスク登録を分離するか

1. **責務の単純化**
   - task-manager-agent の複雑度を削減（738行 → 500行）
   - タスク登録ロジックが一箇所に集約

2. **メンテナンス性向上**
   - タスク作成フローの変更が容易
   - 影響範囲が限定的

3. **再利用性向上**
   - 他のエージェントからもタスク登録を呼び出し可能
   - プランファイル → タスクファイル変換の標準化

### なぜ status: "todo" で作成するか

1. **プラン作成は完了済み**
   - planning 状態は「プラン作成中」を意味する
   - プランが完了しているため、todo が適切

2. **即座に開始可能**
   - todo 状態のタスクは task-manager-agent が即座に開始できる
   - 余計な状態遷移を削減

3. **ワークフロー簡素化**
   ```
   ❌ 複雑: planning → todo → in-progress
   ✅ シンプル: todo → in-progress
   ```

### なぜプランファイルを保持するか

1. **情報源の保持**
   - プラン作成時の調査・設計内容を保持
   - タスクファイルには埋め込むが、原本も保持

2. **トレーサビリティ**
   - `plan_file` フィールドで参照可能
   - 後から確認・参照できる

3. **再利用可能性**
   - 同じプランから複数タスクを生成する可能性
   - プランを削除すると再利用不可

---

## 関連ドキュメント

- `.claude/agents/task-manager-agent.md` - タスク管理専門エージェント
- `.claude/skills/task-planning.md` - タスク計画スキル
- `.claude/skills/task-workflow.md` - タスクライフサイクル管理
- `.claude/CLAUDE.md` - プロジェクトルール
- `docs/reference/task-management-faq.md` - タスク管理FAQ

---

## タスク作成から実装までのフロー

```
1. ユーザーがプランモードでプラン作成
   ↓
   プランファイル保存（~/.claude/plans/xxx.md）

2. task-registration-agent がタスク登録
   ↓
   タスクファイル生成（tasks/1_todo/ または project/tasks/1_todo/）
   status: "todo"

3. task-manager-agent がタスク開始
   ↓
   status: "todo" → "in-progress"
   worktree作成（game-devタスクの場合）

4. impl-agent が実装
   ↓
   仕様書作成・実装・テスト

5. task-manager-agent がタスク完了
   ↓
   status: "in-progress" → "done"
   ファイル移動（→ 4_archive/）
```

詳細は `.claude/skills/task-workflow.md` を参照してください。
