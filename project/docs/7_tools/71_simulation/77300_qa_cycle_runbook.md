# 77300: QA Cycle Runbook

## 目的

`headless_sim` と `trace_narrator` の実行手順を一本化し、  
比較可能なKPIを毎回同じ形式で出力する。

## 実行コマンド

```bash
cd project
./scripts/qa_cycle.sh -c debug
```

任意オプション:

```bash
# 試合数上書き
./scripts/qa_cycle.sh -c debug -m 3

# 出力先指定
./scripts/qa_cycle.sh -c stress -o qa_reports/stress_latest
```

## 出力物

`qa_reports/<timestamp>/` 配下に以下を出力する。

- `result.json`: `headless_sim` の集計結果
- `trace.jsonl`: トレース（trace有効設定時のみ）
- `narrative.md`: `trace_narrator` 出力（trace有効時のみ）
- `qa_cycle_report.md`: KPIサマリ

## 固定KPI

毎回以下を `qa_cycle_report.md` に出力する。

1. Completion Rate (`completed_matches / total_matches`)
2. Avg Rally Count
3. Avg Duration (s)
4. Total Anomalies
5. Fault内訳（`DoubleFault` / `NetFault` / `Out`）
6. Shot Stats Rate（`ShotAttributesCalculated` 検出有無）

## debug / stress の役割

`debug`:
- 1試合の短時間観測
- trace有効で原因分析向き

`stress`:
- 複数試合の安定性確認
- trace無効でスループット重視

## 運用ルール

1. 実装修正後は `debug` で1回実行してKPI確認
2. 手触り調整の節目で `stress` を実行して回帰確認
3. KPI悪化時は `qa_cycle_report.md` を基準に差分比較する
