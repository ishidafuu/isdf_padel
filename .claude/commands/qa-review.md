---
description: ゲームプレイログをLLMでレビューし違和感を検出
argument-hint: <file.md> [--trace <file.jsonl>] [--focus physics|ai|ux] [--threshold critical|major|minor]
---

# /qa-review コマンド

ゲームプレイログをLLMでレビューし、物理的妥当性・AI挙動・プレイヤー体験の観点から違和感を検出します。

**引数**: $ARGUMENTS

## 使用者

**🤖 エージェント専用コマンド** - 開発者のQA支援として使用

### 使用タイミング

- ヘッドレスシミュレーション後のログ分析
- ゲームプレイの品質チェック
- 「なんとなく変」な挙動の検出

## 使用方法

```bash
# ナラティブファイルをレビュー
/qa-review report.md

# 直接JSONLをレビュー（内部でナラティブ変換）
/qa-review --trace trace.jsonl

# 特定観点のみ
/qa-review report.md --focus physics
/qa-review report.md --focus ai
/qa-review report.md --focus ux

# 重要度でフィルタ
/qa-review report.md --threshold critical
```

## 引数

| 引数 | 説明 |
|------|------|
| `<file.md>` | ナラティブマークダウンファイル |
| `--trace <file>` | JSONLファイルを直接指定（ナラティブ変換を自動実行） |
| `--focus <type>` | レビュー観点を絞る（physics/ai/ux/all、デフォルト: all） |
| `--threshold <level>` | レポートする問題の最低重要度（critical/major/minor、デフォルト: minor） |
| `--output <file>` | レポートをファイルに保存 |

## 関連仕様

- `project/docs/7_tools/71_simulation/77202_qa_review_spec.md`
- `project/docs/7_tools/71_simulation/77201_narrative_spec.md`

## 指示

### Step 1: 引数パース

$ARGUMENTS を解析し、以下を特定:
1. 入力ファイル（.md または --trace で指定された .jsonl）
2. `--focus` オプション（未指定時は all）
3. `--threshold` オプション（未指定時は minor）
4. `--output` オプション（未指定時は stdout）

### Step 2: 入力処理

**ナラティブファイル（.md）の場合:**
1. ファイルを読み込む
2. 内容をプロンプトに組み込む

**JSONLファイル（--trace）の場合:**
1. `trace_narrator` CLIを呼び出してナラティブに変換
2. 変換結果をプロンプトに組み込む

```bash
# JSONL → ナラティブ変換
cd project && cargo run --bin trace_narrator -- <file.jsonl>
```

### Step 3: レビュー実行

`--focus` オプションに応じて、以下の観点でレビューを実行:

#### 物理的妥当性（physics）

以下のテニス/パデルの物理ログを確認してください。

**期待される物理法則:**
- サーブ速度: 80-150 km/h の範囲
- 通常ショット速度: 30-100 km/h の範囲
- バウンス時は速度が20-40%減衰
- 壁反射は入射角≈反射角（差が30°以内）
- スピンは曲がり量に影響

**確認観点:**
- 速度が範囲外の場合は異常
- バウンス減衰が期待値から外れている場合は異常
- 反射角度が不自然な場合は異常
- スピンと軌道の不一致

#### AI行動の自然さ（ai）

以下のゲームログでAIの挙動を確認してください。

**期待されるAI行動:**
- 反応時間: 0.1-0.5秒の範囲
- 移動パターン: 滑らかで人間らしい
- 目標位置: ボール軌道に対して適切
- 難易度一貫性: 設定に応じた強さ

**確認観点:**
- 超人的な反応（0.1秒未満）
- 不自然に遅い反応
- 急激な方向転換（テレポート的）
- 明らかに最適でない選択
- 難易度設定と実際の強さの不一致

#### プレイヤー体験（ux）

以下のゲームログをプレイヤー視点で評価してください。

**確認観点:**
1. ラリーの長さは適切か（3-15ショットが理想）
2. ポイントの取られ方に納得感があるか
3. 「え？なんで？」と思う瞬間はないか
4. 不公平に感じる判定はないか
5. ゲームとして面白そうか

### Step 4: レポート出力

以下の形式でマークダウンレポートを生成:

```markdown
# QA Review Report

**File**: [入力ファイル名]
**Reviewed**: [日時]
**Focus**: [レビュー観点]

## Summary

| Severity | Count |
|----------|-------|
| Critical | N |
| Major | N |
| Minor | N |

## Issues

### [CRITICAL/MAJOR/MINOR] [問題タイトル]

- **Frame**: [フレーム番号]
- **Category**: [physics/ai/ux]
- **Description**: [問題の説明]
- **Evidence**: [証拠となるデータ]
- **Recommendation**: [推奨対応]

## Overall Assessment

[全体評価のまとめ]
```

### Step 5: 出力

- `--output` 指定時: ファイルに保存
- 未指定時: 標準出力に表示

`--threshold` でフィルタ:
- `critical`: CRITICAL のみ
- `major`: CRITICAL と MAJOR
- `minor`: すべて（デフォルト）

## 出力例

```markdown
# QA Review Report

**File**: match_20260116_1530.md
**Reviewed**: 2026-01-16 15:45:00
**Focus**: all

## Summary

| Severity | Count |
|----------|-------|
| Critical | 1 |
| Major | 2 |
| Minor | 3 |

## Issues

### [CRITICAL] 壁抜けバグ

- **Frame**: 1234
- **Category**: physics
- **Description**: ボールが壁を貫通している
- **Evidence**: position: (12.5, 0.5, -8.2) → (-0.3, 0.5, -8.5) (1フレームで12.8m移動)
- **Recommendation**: 壁判定のレイキャストを確認

### [MAJOR] AI反応が超人的

- **Frame**: 2100-2150
- **Category**: ai
- **Description**: ボール到達まで0.05秒で反応開始
- **Evidence**: reaction_timer=0.05, 人間の反応限界は0.1秒
- **Recommendation**: 難易度設定の反応時間下限を確認

## Overall Assessment

物理演算は概ね正常だが、壁抜けバグが1件検出された。
AI反応速度に調整が必要。ゲーム体験としてはラリーが
短めだが、許容範囲内。
```

## エラーハンドリング

### E001: ファイルが見つからない

```
ERROR: File not found: [ファイルパス]
```

### E002: JSONLの変換失敗

```
ERROR: Failed to convert JSONL to narrative
[trace_narrator のエラー出力]
```

### E003: 不正なオプション

```
ERROR: Invalid option: --focus [値]
Valid options: physics, ai, ux, all
```

## 注意事項

- ナラティブファイルはUTF-8エンコーディングを想定
- JSONLファイルは `trace_narrator` の入力形式に準拠
- LLMの判定は目安であり、最終判断は開発者が行う
- レポートは開発時のデバッグ支援を目的とする

## 関連コマンド

- `/run-game` - ゲーム起動
- `/impl-validate` - 実装と仕様書の整合性検証
- `/docs-validate` - 仕様書全体の整合性チェック
