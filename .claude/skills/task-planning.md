# task-planning

## 概要

**タスク計画スキル** - プランモード → タスク登録の標準手順

### 参照先

- task-registration-agent.md（タスク登録時の詳細）
- docs/concepts/tasks.md（人間向け詳細ガイド）

## 2段階フロー

タスク作成は2段階に分離:

| フェーズ | 担当 | 責務 |
|---------|------|------|
| プラン作成 | ユーザー（プランモード） | 調査、設計、プランファイル作成 |
| タスク登録 | task-registration-agent | プランファイル → タスクファイル変換、ID採番 |

### Phase 1: プラン作成

```
ユーザー：プランモードに入る
↓
【プランモード】
├─ 関連調査（Explore agent）
├─ 仕様書確認
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

- **プランモードは全タスクで必須** - 簡単なタスクでも調査で予期しない依存が見つかる
- **Explore agent は必須** - プランモード中は直接 Glob/Grep を使用しない
- **プランファイルは削除しない** - タスクファイル生成後も保持
- **タスクタイプで配置先が決まる**:
  - game-dev / project-wide → `project/tasks/1_todo/`
  - framework → `tasks/1_todo/`
