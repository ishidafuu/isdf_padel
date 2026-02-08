# QA Cycle Report

- Generated: 2026-02-08 12:18:13
- Config: debug
- Simulation Status: COMPLETE
- Narrative Status: COMPLETE

## KPI

| KPI | Value | Notes |
|-----|-------|-------|
| Completion Rate | 0.000 | completed_matches / total_matches |
| Avg Rally Count | 6.0 | from result.json |
| Avg Duration (s) | 60.015923 | from result.json |
| Total Anomalies | 0 | from result.json |
| Fault DoubleFault | 1 | from trace point reason |
| Fault NetFault | 1 | from trace point reason |
| Fault Out | 4 | from trace point reason |
| Shot Stats Rate | 1.000 | 1.0 if ShotAttributes events detected |

## Artifacts

- Result: `/Users/ishidafuu/Documents/repository/isdf_padel/project/qa_reports/baseline_phase3/result.json`
- Trace: `/Users/ishidafuu/Documents/repository/isdf_padel/project/qa_reports/baseline_phase3/trace.jsonl`
- Narrative: `/Users/ishidafuu/Documents/repository/isdf_padel/project/qa_reports/baseline_phase3/narrative.md`

## Config Roles

- debug:
  - 1 match short-run observation
  - trace enabled for detailed diagnosis
- stress:
  - multi-match stability check
  - trace disabled for throughput
