# 77201: Narrative Converter Specification

**Version**: 1.0.0
**Status**: Draft
**Last Updated**: 2026-01-16

## 概要

JSONLテレメトリログを人間/LLMが読みやすいマークダウン形式に変換するCLIツール。ラリー単位での要約と異常フラグ付けを行う。

## 目的

- テレメトリログの可読性向上
- LLMによる分析の前処理として機能
- 異常値の自動検出とハイライト

## 関連仕様

- 77200: テレメトリ拡張仕様
- 77103: リプレイシステム

## CLIインターフェース

### 基本使用法

```bash
# 基本変換
cargo run --bin trace_narrator -- input.jsonl -o output.md

# 詳細オプション
cargo run --bin trace_narrator -- input.jsonl \
    --output output.md \
    --detail-level full \
    --anomaly-threshold 1.5
```

### コマンドラインオプション

| オプション | 短縮形 | 説明 | デフォルト |
|-----------|--------|------|-----------|
| `--output` | `-o` | 出力ファイルパス | stdout |
| `--detail-level` | `-d` | 詳細度 (summary/normal/full) | normal |
| `--anomaly-threshold` | `-a` | 異常検出の閾値倍率 | 1.5 |
| `--include-physics` | | 物理詳細を含める | false |
| `--rally-only` | | ラリー要約のみ出力 | false |

## 出力形式

### ヘッダー

```markdown
# Match Report

- **Recorded**: 2026-01-16 10:30:00
- **Duration**: 5m 32s
- **Total Rallies**: 12
- **Total Points**: P1: 4, P2: 3

## Summary

| Metric | P1 | P2 |
|--------|----|----|
| Avg Power | 0.72 | 0.68 |
| Avg Accuracy | 0.85 | 0.78 |
| Winners | 3 | 2 |
| Errors | 1 | 2 |
```

### ラリー詳細

```markdown
## Rally 1 (Frame 120-450)

**Result**: P1 wins (DoubleBounce)
**Duration**: 5.5s
**Shots**: 4

### Play-by-Play

| Frame | Event | Player | Details |
|-------|-------|--------|---------|
| 120 | Serve | P1 | power=0.65, spin=0.2 |
| 125 | Bounce | - | IN (4.2, 0, 1.5) |
| 180 | Shot | P2 | power=0.72, spin=-0.3 |
| 250 | Bounce | - | IN (-2.1, 0, 0.5) |
| 320 | Shot | P1 | power=0.85, spin=0.4 |
| 380 | Bounce | - | IN (5.5, 0, -1.2) |
| 450 | Point | P1 | DoubleBounce |

### Anomalies

- ⚠️ **Frame 280**: Velocity spike (45 → 52 m/s)

### AI Decisions (if enabled)

| Frame | Player | State | Target | Distance |
|-------|--------|-------|--------|----------|
| 185 | P1 | Tracking | (-3.5, 0, 0.8) | 1.2m |
| 255 | P2 | Idle | (0, 0, -3.0) | - |
```

## Core Requirements (MVP)

### 入力処理

#### REQ-77201-001: JSONL読み込み
**WHEN** JSONLファイルが指定される
**THE SYSTEM SHALL** 1行ずつJSONをパースする
- 不正な行はスキップしてログ出力
- 大容量ファイルでもメモリ効率的に処理
**テスト**: 10万行のJSONLファイルを処理できることを確認

#### REQ-77201-002: フレームトレース構造解析
**WHEN** FrameTraceオブジェクトを読み込む
**THE SYSTEM SHALL** イベントを時系列で整理する
- frame, timestamp でソート
- イベントタイプ別に分類
**テスト**: 順序が入れ替わった入力でも正しくソートされることを確認

### ラリー検出

#### REQ-77201-003: ラリー境界検出
**WHEN** Pointイベントが検出される
**THE SYSTEM SHALL** 1つのラリーとして区切る
- 開始: 前のPointイベント直後またはファイル先頭
- 終了: 現在のPointイベント
**テスト**: 複数ラリーが正しく分割されることを確認

#### REQ-77201-004: ラリー統計計算
**FOR EACH** ラリー
**THE SYSTEM SHALL** 以下の統計を計算する
- ショット数
- 各プレイヤーの平均パワー/スピン/精度
- ラリー時間
**テスト**: ラリー統計が正しく計算されることを確認

### 異常検出

#### REQ-77201-005: PhysicsAnomalyのハイライト
**WHEN** PhysicsAnomalyイベントが存在する
**THE SYSTEM SHALL** 警告マーカー付きで出力する
- severity: "Error" → 🔴
- severity: "Warning" → ⚠️
**テスト**: 異常イベントがハイライトされることを確認

#### REQ-77201-006: 統計的異常検出
**WHEN** 数値が平均から閾値×標準偏差を超える
**THE SYSTEM SHALL** 異常としてマーキングする
- 対象: power, speed, spin
- 閾値: デフォルト1.5σ
**テスト**: 外れ値が検出されることを確認

### 出力生成

#### REQ-77201-007: マークダウン出力
**THE SYSTEM SHALL** 有効なマークダウン形式で出力する
- テーブル記法を使用
- 見出しレベルを適切に設定
**テスト**: 出力がマークダウンパーサーでエラーなく解析できることを確認

#### REQ-77201-008: 詳細度制御
**WHEN** `--detail-level` オプションが指定される
**THE SYSTEM SHALL** 出力量を調整する
- summary: ラリー要約のみ
- normal: ラリー要約 + 主要イベント
- full: 全イベント + AI決定詳細
**テスト**: 各詳細度で適切な出力が生成されることを確認

## Extended Requirements

### REQ-77201-101: HTML出力
**WHEN** `--format html` が指定される
**THE SYSTEM SHALL** インタラクティブなHTMLを出力する
- 折りたたみ可能なラリー詳細
- フレームへのジャンプリンク

### REQ-77201-102: フィルタリング
**WHEN** `--player` オプションが指定される
**THE SYSTEM SHALL** 特定プレイヤーのイベントのみ出力する

## 実装対象ファイル

| ファイル | 内容 |
|---------|------|
| `bin/trace_narrator.rs` | CLIエントリポイント |
| `bin/trace_narrator/parser.rs` | JSONL解析 |
| `bin/trace_narrator/analyzer.rs` | ラリー検出・統計計算 |
| `bin/trace_narrator/formatter.rs` | マークダウン生成 |
