---
id: "30061"
title: "QAプロンプトチューニング"
type: "game-dev"
status: "todo"
priority: "medium"
related_task: null
spec_ids:
  - "REQ-77202-004"
  - "REQ-77202-005"
  - "REQ-77202-006"
blocked_by:
  - "30060"
blocks: []
branch_name: null
worktree_path: null
plan_file: "/Users/ishidafuu/.claude/plans/optimized-strolling-puppy.md"
tags:
  - "telemetry"
  - "llm-qa"
  - "phase3"
parent_task_id: null
created_at: "2026-01-16T16:00:00+09:00"
updated_at: "2026-01-16T16:00:00+09:00"
completed_at: null
---

# Task 30061: QAプロンプトチューニング

## Summary

物理・AI・UXの各観点で効果的なプロンプトを設計・チューニング。実際のログでテストして精度を検証。

## Related Specifications

- `project/docs/7_tools/71_simulation/77202_qa_review_spec.md`

## Progress

### Completed

(なし)

## Next Actions

1. 物理チェック用プロンプトを設計
2. AIチェック用プロンプトを設計
3. UXチェック用プロンプトを設計
4. 実際のゲームログでテスト
5. 誤検出/見逃しを分析
6. プロンプトを調整

## Dependencies

- **Blocked By:** 30060 (QAレビュースキル設計)
- **Blocks:** なし

## 完了チェックリスト

- [ ] 物理プロンプト完成
- [ ] AIプロンプト完成
- [ ] UXプロンプト完成
- [ ] 精度検証完了

## メモ

Phase 3 の最終タスク。実際の運用を通じて継続的に改善が必要。

---

## Detailed Implementation Plan

### 物理チェックプロンプト

```markdown
## System

あなたはテニスゲームのQAエンジニアです。物理挙動の妥当性を検証します。

## 期待される物理法則

- ボール速度: サーブ 80-150 km/h、通常ショット 30-100 km/h
- バウンス: 速度20-40%減衰
- 壁反射: 入射角≈反射角（差<30°）
- スピン: 曲がり量がスピン値に比例

## ログ

{narrative_content}

## 指示

上記の物理法則に反する挙動があれば報告してください。

出力形式:
- フレーム番号
- 問題の種類
- 具体的な値
- 重要度（Critical/Major/Minor）
```

### AIチェックプロンプト

```markdown
## System

あなたはテニスゲームのQAエンジニアです。AIの挙動が自然かを検証します。

## AI設定

- 難易度: 中級者
- 反応時間: 0.1-0.5秒
- 人間らしいミスをする
- 動きは滑らか

## ログ

{narrative_content}

## 指示

以下の観点で不自然な点を報告してください:
1. 超人的な反応（反応時間<0.1秒）
2. 不自然に遅い反応（反応時間>0.5秒）
3. 急激な方向転換（1フレームで180度等）
4. 取れるボールを見送る
5. 難易度に合わない完璧なプレイ
```

### UXチェックプロンプト

```markdown
## System

あなたはプレイヤー視点でゲーム体験を評価します。

## 期待されるゲーム体験

- 適度な長さのラリー（3-15ショット）
- 多様な得点パターン
- 納得感のある判定
- フラストレーションの少なさ

## ログ

{narrative_content}

## 指示

プレイヤーが「なんか変」「理不尽」と感じそうな点を報告してください:
1. 極端に短い/長いラリー
2. 同じ負け方の連続
3. 際どい判定が多すぎる
4. 「え？入ってた/入ってない」と思う瞬間
5. AIが急に強く/弱くなる
```
