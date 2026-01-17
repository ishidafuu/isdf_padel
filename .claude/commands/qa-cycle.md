---
description: QAワークフロー統合（シミュレーション → ナラティブ → LLMレビュー）
argument-hint: [-m <matches>] [-f physics|ai|ux] [-c stress] [-t critical|major]
---

# /qa-cycle コマンド

ヘッドレスシミュレーション → ナラティブ変換 → LLMレビューを1コマンドで実行します。

**引数**: $ARGUMENTS

## 使用者

**🤖 エージェント専用コマンド** - 開発者のQA支援として使用

### 使用タイミング

- 実装後の品質チェック
- リリース前の自動QA
- 問題検出の自動化

## 使用方法

```bash
/qa-cycle                           # デフォルト: 1試合、全観点
/qa-cycle -m 10                     # 10試合実行
/qa-cycle -f physics                # 物理観点のみ
/qa-cycle -c stress -t critical     # ストレステスト、重大問題のみ
```

## 引数

| 引数 | 説明 | デフォルト |
|------|------|-----------|
| `-m <count>` | 実行試合数 | 1 |
| `-f <focus>` | レビュー観点（physics/ai/ux/all） | all |
| `-c <config>` | シミュレーション設定（debug/stress） | debug |
| `-t <threshold>` | レポート重要度（critical/major/minor） | minor |
| `-o <dir>` | 出力ディレクトリ（指定しない場合は自動生成） | qa_reports/YYYY-MM-DD_HHMMSS/ |

## 関連仕様

- `project/docs/7_tools/71_simulation/77100_headless_sim.md`
- `project/docs/7_tools/71_simulation/77201_narrative_spec.md`
- `project/docs/7_tools/71_simulation/77202_qa_review_spec.md`

## 指示

### Step 1: 引数パース

$ARGUMENTS を解析し、以下を特定:
1. `-m` オプション（試合数、デフォルト: 1）
2. `-f` オプション（レビュー観点、デフォルト: all）
3. `-c` オプション（設定、デフォルト: debug）
4. `-t` オプション（重要度閾値、デフォルト: minor）
5. `-o` オプション（出力先、未指定時は自動生成）

### Step 2: 出力ディレクトリ作成

```bash
# タイムスタンプ付きディレクトリを作成
OUTPUT_DIR="project/qa_reports/$(date +%Y-%m-%d_%H%M%S)"
mkdir -p "$OUTPUT_DIR"
```

`-o` が指定されている場合はそのディレクトリを使用。

### Step 3: ヘッドレスシミュレーション実行

設定ファイルを一時的に作成（試合数を反映）:

```bash
# 設定に応じてシミュレーション実行
cd project

# -c debug の場合
cargo run --bin headless_sim -- -c debug

# -c stress の場合
cargo run --bin headless_sim -- -c stress
```

**注意**: 試合数（-m）は設定ファイル内の `match_count` を上書きする必要がある場合、一時設定ファイルを作成するか、CLIオプションで対応（現在はCLI未対応のため設定ファイル編集が必要）。

**出力ファイル**:
- `debug_trace.jsonl` （または `stress_trace.jsonl`）

トレースファイルを出力ディレクトリにコピー:

```bash
cp debug_trace.jsonl "$OUTPUT_DIR/trace.jsonl"
```

### Step 4: ナラティブ変換

```bash
cd project
cargo run --bin trace_narrator -- "$OUTPUT_DIR/trace.jsonl" > "$OUTPUT_DIR/narrative.md"
```

### Step 5: LLMレビュー実行

`/qa-review` の処理を内部で実行:

1. `$OUTPUT_DIR/narrative.md` を読み込む
2. `-f` オプションに応じたプロンプトでLLMレビュー
3. 結果を `$OUTPUT_DIR/qa_review.md` に保存

**プロンプト選択**:
- `physics`: 物理的妥当性チェック
- `ai`: AI行動の自然さチェック
- `ux`: プレイヤー体験チェック
- `all`: 上記すべて実行

### Step 6: 統合レポート生成

以下の形式で `$OUTPUT_DIR/qa_cycle_report.md` を生成:

```markdown
# QA Cycle Report

**Generated**: [日時]
**Config**: [設定名]
**Matches**: [試合数]

## Execution Summary

| Phase | Status | Output |
|-------|--------|--------|
| Simulation | ✅ Complete | trace.jsonl |
| Narrative | ✅ Complete | narrative.md |
| LLM Review | ✅ Complete | qa_review.md |

## Files

- [trace.jsonl](./trace.jsonl) - シミュレーショントレース
- [narrative.md](./narrative.md) - ナラティブレポート
- [qa_review.md](./qa_review.md) - LLMレビュー結果

## Quick Summary

[qa_review.md の Summary セクションを転記]

## Next Steps

[検出された問題に基づく推奨アクション]
```

### Step 7: 結果報告

実行完了後、以下を報告:

```
QA Cycle 完了

📁 出力ディレクトリ: [OUTPUT_DIR]
📊 実行結果:
  - シミュレーション: ✅ [N]試合完了
  - ナラティブ変換: ✅ 完了
  - LLMレビュー: ✅ 完了

🔍 検出された問題:
  - Critical: [N]件
  - Major: [N]件
  - Minor: [N]件

詳細は qa_cycle_report.md を参照してください。
```

## エラーハンドリング

### E001: シミュレーション失敗

```
ERROR: Simulation failed
[headless_sim のエラー出力]

対処: cargo build --bin headless_sim でビルドを確認
```

### E002: ナラティブ変換失敗

```
ERROR: Narrative conversion failed
[trace_narrator のエラー出力]

対処: trace.jsonl の形式を確認
```

### E003: 無効なオプション

```
ERROR: Invalid option: -f [値]
Valid options: physics, ai, ux, all
```

## 注意事項

- 初回実行時はビルドが必要
- 設定ファイルは `project/assets/config/` に配置
- レポートは UTF-8 エンコーディング
- LLM判定は目安であり、最終判断は開発者が行う

## 関連コマンド

- `/qa-review` - LLMレビュー単体実行
- `/run-game` - ゲーム起動
- `/impl-validate` - 実装と仕様書の整合性検証
