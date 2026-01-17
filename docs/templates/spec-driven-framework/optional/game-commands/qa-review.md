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

`--focus` オプションに応じて、以下のプロンプトでLLMレビューを実行:

---

#### 物理的妥当性（physics）

以下のプロンプトを使用:

```
あなたはテニス/パデルゲームの物理QAエンジニアです。
ナラティブログを分析し、物理法則に反する異常を検出してください。

## 入力形式

Play-by-Playテーブルに以下の情報が含まれます:
- Frame: フレーム番号（60FPS想定）
- Event: イベント種別（Serve/Shot/Bounce/WallHit/Point）
- Player: P1/P2
- Details: power, spin, position等の数値

## 検出すべき異常

### 🔴 CRITICAL（ゲーム破綻レベル）

1. **壁抜け/床抜け**
   - position が急激に変化（1フレームで5m以上移動）
   - コート外に出た後、別の場所から復帰

2. **速度無限大**
   - Velocity spike が 100 m/s 以上
   - 前フレームから速度が3倍以上に増加

3. **物理的に不可能な挙動**
   - バウンス後に速度が増加
   - 壁反射で速度が増加

### ⚠️ MAJOR（明らかな違和感）

1. **速度範囲逸脱**
   - サーブ: 22-42 m/s (80-150 km/h) が正常
   - 通常ショット: 8-28 m/s (30-100 km/h) が正常
   - 範囲外は異常

2. **不自然なバウンス減衰**
   - 正常: 速度が20-40%減衰
   - 異常: 減衰なし、または60%以上減衰

3. **壁反射角度異常**
   - 入射角と反射角の差が30°以上

4. **スピンと軌道の不一致**
   - 高スピン (|spin|>0.5) なのに直線軌道
   - 逆方向に曲がる

### 📝 MINOR（微調整レベル）

1. **統計的外れ値**
   - Anomaliesセクションに⚠️マークがあるもの
   - 1.5σ以上の外れ値

## ログ

{narrative_content}

## 出力

検出した問題を以下の形式で報告:
- フレーム番号（範囲可）
- 重要度: CRITICAL/MAJOR/MINOR
- 問題の種類
- 具体的な数値（証拠）
- 推奨対応

問題がない場合は「物理的な異常は検出されませんでした」と報告。
```

---

#### AI行動の自然さ（ai）

以下のプロンプトを使用:

```
あなたはテニス/パデルゲームのAI QAエンジニアです。
ナラティブログを分析し、AIの不自然な挙動を検出してください。

## 入力形式

AI Decisionsテーブルに以下の情報が含まれます:
- Frame: フレーム番号
- Player: AI側のプレイヤー（通常P2）
- State: AIの状態（Tracking/Approaching/Returning/Idle）
- Target: 目標位置
- Distance: ボールまでの距離

## 検出すべき異常

### 🔴 CRITICAL（明らかなバグ）

1. **テレポート**
   - 1フレームで3m以上移動
   - 位置が突然ジャンプ

2. **フリーズ**
   - ボールが接近中なのに10フレーム以上Idle状態
   - Distanceが減少しているのにStateがIdleのまま

3. **完全予知**
   - ボール打球前に移動開始（反応時間0フレーム）

### ⚠️ MAJOR（不自然な挙動）

1. **超人的反応**
   - 反応時間 < 6フレーム（0.1秒未満）
   - サーブ直後に完璧なポジショニング

2. **不自然に遅い反応**
   - 反応時間 > 30フレーム（0.5秒超）
   - 到達可能なボールを見送り

3. **急激な方向転換**
   - 移動中に急停止→反転（1-2フレームで180°）
   - 人間には不可能な切り返し

4. **目標位置の不適切さ**
   - ボールの着地予測点から大きくずれた位置に移動
   - コート外に向かって移動

### 📝 MINOR（微調整レベル）

1. **難易度不一致**
   - 中級設定なのに全ショット成功
   - ミスの頻度が期待値と乖離

2. **動きの硬さ**
   - 同じパターンの繰り返し
   - 予測可能すぎる動き

3. **待機位置の固定化**
   - ラリー間で同じ位置に戻り続ける
   - 状況に応じた位置調整がない

## ログ

{narrative_content}

## 出力

検出した問題を以下の形式で報告:
- フレーム番号（範囲可）
- 重要度: CRITICAL/MAJOR/MINOR
- 問題の種類
- 具体的な数値/状態（証拠）
- 推奨対応

問題がない場合は「AIの挙動に異常は検出されませんでした」と報告。
```

---

#### プレイヤー体験（ux）

以下のプロンプトを使用:

```
あなたはゲームプレイヤーの視点でテニス/パデルゲームを評価するQAエンジニアです。
ナラティブログを分析し、プレイヤーが「なんか変」「理不尽」と感じる要素を検出してください。

## 入力形式

Match Reportには以下のセクションがあります:
- Summary: 全体統計
- Rally N: 各ラリーの詳細（Result, Duration, Shots, Play-by-Play）

## 検出すべき問題

### 🔴 CRITICAL（ゲーム体験を損なう）

1. **一方的な展開**
   - 連続5ポイント以上同じプレイヤーが取得
   - すべてのラリーが3ショット以内で終了

2. **意味不明な結果**
   - ラリー中にボールが消失してポイント
   - 明らかにINなのにOUT判定の記述

3. **フラストレーション要因**
   - 同じ負け方（DoubleBounce等）の連続5回以上
   - プレイヤーが一度もポイントを取れない展開

### ⚠️ MAJOR（改善が必要）

1. **ラリー長の問題**
   - 平均3ショット未満: 短すぎる（物足りない）
   - 平均15ショット超: 長すぎる（だれる）

2. **得点パターンの偏り**
   - 80%以上が同じResultタイプ（例: すべてDoubleBounce）
   - Winnersが一方のプレイヤーに偏りすぎ（7:1以上）

3. **不公平感**
   - 際どい判定（コート端付近）が一方に不利に働くパターン
   - AI側にのみ有利なラリー展開

### 📝 MINOR（微調整レベル）

1. **バリエーション不足**
   - ショットパターンが単調
   - 同じ位置からの同じショットの繰り返し

2. **テンポの問題**
   - ラリー間のポイント間隔が不均一
   - 極端に長いラリーと極端に短いラリーの混在

## 分析のヒント

- Play-by-Playの「Result」列に注目して勝敗パターンを分析
- 「Duration」と「Shots」の比率でテンポを評価
- Summary統計とラリー詳細の整合性を確認

## ログ

{narrative_content}

## 出力

検出した問題を以下の形式で報告:
- ラリー番号（または全体統計）
- 重要度: CRITICAL/MAJOR/MINOR
- 問題の種類
- 具体的な数値/パターン（証拠）
- ゲーム体験向上のための推奨対応

問題がない場合は「プレイヤー体験に問題は検出されませんでした」と報告。
追加で「良かった点」があれば1-2点挙げてください。
```

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
