# task-planning

## 概要

**タスク計画スキル** - タスクタイプ別の計画フロー

### 参照先

- task-registration-agent.md（タスク登録時の詳細）
- docs/concepts/tasks.md（人間向け詳細ガイド）

## タスクタイプ別ワークフロー（IMPORTANT）

**タスクタイプによって計画フローが異なる。**

| タスクタイプ | 計画フロー | 理由 |
|-------------|-----------|------|
| **ゲーム開発（30XXX）** | 仕様書作成 → タスク作成 | 仕様書が計画の役割を果たす |
| **バグ修正（B30XXX）** | プランモード → タスク作成 | 既存コードの修正計画 |
| **リファクタ（R30XXX）** | プランモード → タスク作成 | 既存コードの改善計画 |
| **フレームワーク（FXXX）** | プランモード → タスク作成 | 設計変更の計画 |
| **プロジェクト横断（PXXX）** | プランモード → タスク作成 | インフラ等の計画 |

---

## ゲーム開発（30XXX）: 仕様書駆動フロー

**プランモードは不要。仕様書作成から直接タスク作成へ。**

```
ユーザー: 「ジャンプ機能を作りたい」
↓
仕様書作成（spec-agent 参照）
├─ 要件定義（EARS記法）
├─ データ定義（8_data/）
└─ 仕様書完成・コミット
↓
タスク作成
├─ 仕様書から実装タスクを抽出
└─ 30XXX-機能名.md を作成
↓
実装（impl-agent 参照）
```

**ポイント**: 仕様書自体が「計画」の役割を果たすため、プランモードは冗長

---

## バグ修正・リファクタ・フレームワーク・プロジェクト横断: プランモード経由

**プランモードで計画を立ててからタスク作成。**

### Phase 1: プラン作成

```
ユーザー：プランモードに入る
↓
【プランモード】
├─ 関連調査（Explore agent）
├─ 仕様書/コード確認
├─ ユーザー確認（必要に応じて）
├─ プランファイル作成（~/.claude/plans/xxx.md）
└─ ExitPlanMode
↓
プランファイル保存完了
```

### Phase 2: タスク登録

```
ユーザー：「プランからタスクを作成して」
↓
メイン Claude Code が task-registration-agent.md を参照して直接実行
↓
タスク登録完了
```

**詳細は `.claude/agents/task-registration-agent.md` を参照**

## プランモード実行手順

### Step 1: 関連調査（Explore agent）

**必須**: Explore agentで関連調査を行う（直接Glob/Grepは使わない）

```markdown
Task(
  subagent_type="Explore",
  thoroughness="medium",
  prompt="[タスク名]に関連する既存実装、仕様書、依存関係を調査"
)
```

**thoroughness**:
- `quick`: 簡単なタスク
- `medium`: 通常（推奨）
- `very thorough`: 複雑なタスク

### Step 2: 仕様書確認

Explore agentの結果を基に関連仕様書を読み込む。

### Step 3: ユーザー確認

不明点があれば AskUserQuestion で確認する。

### Step 4: プランファイル作成

**パス**: `~/.claude/plans/[plan-name].md`（Claude Codeが自動生成）

**必須セクション**:

```markdown
## Task Type
framework / game-dev / project-wide

## Summary
[1-2行の要約]

## Implementation Plan
[実装手順]

## Progress Steps
- [ ] [ステップ1]
- [ ] [ステップ2]
...

## Verification Checklist
- [ ] [検証項目1]
- [ ] [検証項目2]
...
```

**オプションセクション**（複雑なタスクの場合）:
- Related Files
- Dependencies
- Technical Notes
- Next Actions (Initial)

### Step 5: ExitPlanMode

プランファイル作成後、ExitPlanMode を呼ぶ。

## プランモードの目的

1. **実装前の全体把握** - 依存関係・影響範囲の特定
2. **ユーザーとの認識合わせ** - 不明点の早期解消
3. **高品質な計画書** - 実装時の迷い削減

## プランファイル → タスクファイル

タスク登録時、プランの全内容は「Detailed Implementation Plan」セクションに埋め込まれる。

**埋め込み後のタスクファイル構造**:

```markdown
---
[Frontmatter]
---

# Task [ID]: [タスク名]

## Summary
## Related Specifications
## Progress
## Next Actions
## Dependencies

---

## Detailed Implementation Plan

> プランファイルの全内容がここに埋め込まれる
```

## 注意事項

- **ゲーム開発（30XXX）はプランモード不要** - 仕様書が計画の役割を果たす
- **その他のタスクはプランモード経由** - バグ修正、リファクタ、フレームワーク、プロジェクト横断
- **プランモード中は Explore agent を使用** - 直接 Glob/Grep を使用しない
- **プランファイルは削除しない** - タスクファイル生成後も保持
- **タスクタイプで配置先が決まる**:
  - game-dev / bugfix / refactor / project-wide → `project/tasks/1_todo/`
  - framework → `tasks/1_todo/`
