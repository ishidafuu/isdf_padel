#!/usr/bin/env bash

set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  ./scripts/qa_cycle.sh [-c debug|stress] [-m MATCH_COUNT] [-o OUTPUT_DIR]

Options:
  -c  Simulation config name (default: debug)
  -m  Override match_count in config
  -o  Output directory (default: qa_reports/YYYY-MM-DD_HHMMSS)
  -h  Show this help
EOF
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

CONFIG_NAME="debug"
MATCH_COUNT=""
OUTPUT_DIR=""

while getopts ":c:m:o:h" opt; do
  case "$opt" in
    c) CONFIG_NAME="$OPTARG" ;;
    m) MATCH_COUNT="$OPTARG" ;;
    o) OUTPUT_DIR="$OPTARG" ;;
    h)
      usage
      exit 0
      ;;
    \?)
      echo "ERROR: Invalid option -$OPTARG" >&2
      usage
      exit 2
      ;;
    :)
      echo "ERROR: Option -$OPTARG requires an argument" >&2
      usage
      exit 2
      ;;
  esac
done

if [[ -n "$MATCH_COUNT" ]] && ! [[ "$MATCH_COUNT" =~ ^[0-9]+$ ]]; then
  echo "ERROR: -m must be integer. got: $MATCH_COUNT" >&2
  exit 2
fi

if [[ -z "$OUTPUT_DIR" ]]; then
  OUTPUT_DIR="${PROJECT_DIR}/qa_reports/$(date '+%Y-%m-%d_%H%M%S')"
elif [[ "$OUTPUT_DIR" != /* ]]; then
  OUTPUT_DIR="${PROJECT_DIR}/${OUTPUT_DIR}"
fi

mkdir -p "$OUTPUT_DIR"

BASE_CONFIG="${PROJECT_DIR}/assets/config/simulation_${CONFIG_NAME}.ron"
if [[ ! -f "$BASE_CONFIG" ]]; then
  echo "ERROR: Config not found: $BASE_CONFIG" >&2
  exit 2
fi

RUN_CONFIG_NAME="qa_${CONFIG_NAME}_$(date '+%s')_$$"
RUN_CONFIG="${PROJECT_DIR}/assets/config/simulation_${RUN_CONFIG_NAME}.ron"

RESULT_FILE="${OUTPUT_DIR}/result.json"
TRACE_FILE="${OUTPUT_DIR}/trace.jsonl"
LOG_FILE="${OUTPUT_DIR}/debug_log.txt"
NARRATIVE_FILE="${OUTPUT_DIR}/narrative.md"
REPORT_FILE="${OUTPUT_DIR}/qa_cycle_report.md"

cleanup() {
  rm -f "$RUN_CONFIG"
}
trap cleanup EXIT

CONFIG_CONTENT="$(cat "$BASE_CONFIG")"

if [[ -n "$MATCH_COUNT" ]]; then
  CONFIG_CONTENT="$(printf '%s' "$CONFIG_CONTENT" | sed -E "s/match_count:[[:space:]]*[0-9]+/match_count: ${MATCH_COUNT}/")"
fi

CONFIG_CONTENT="$(printf '%s' "$CONFIG_CONTENT" | sed -E "s@result_file:[[:space:]]*(Some\\(\"[^\"]*\"\\)|None)@result_file: Some(\"${RESULT_FILE}\")@")"

TRACE_ENABLED=0
if printf '%s' "$CONFIG_CONTENT" | rg -q 'trace_file:[[:space:]]*Some\('; then
  TRACE_ENABLED=1
  CONFIG_CONTENT="$(printf '%s' "$CONFIG_CONTENT" | sed -E "s@trace_file:[[:space:]]*(Some\\(\"[^\"]*\"\\)|None)@trace_file: Some(\"${TRACE_FILE}\")@")"
else
  CONFIG_CONTENT="$(printf '%s' "$CONFIG_CONTENT" | sed -E "s@trace_file:[[:space:]]*(Some\\(\"[^\"]*\"\\)|None)@trace_file: None@")"
fi

if printf '%s' "$CONFIG_CONTENT" | rg -q 'log_file:'; then
  CONFIG_CONTENT="$(printf '%s' "$CONFIG_CONTENT" | sed -E "s@log_file:[[:space:]]*(Some\\(\"[^\"]*\"\\)|None)@log_file: Some(\"${LOG_FILE}\")@")"
fi

printf '%s\n' "$CONFIG_CONTENT" > "$RUN_CONFIG"

echo "=== QA Cycle ==="
echo "Config: ${CONFIG_NAME}"
if [[ -n "$MATCH_COUNT" ]]; then
  echo "Match Count Override: ${MATCH_COUNT}"
fi
echo "Output: ${OUTPUT_DIR}"
echo

pushd "$PROJECT_DIR" >/dev/null

SIM_EXIT=0
set +e
cargo run --bin headless_sim -- -c "$RUN_CONFIG_NAME"
SIM_EXIT=$?
set -e

if [[ ! -f "$RESULT_FILE" ]]; then
  echo "ERROR: result file not found: $RESULT_FILE" >&2
  exit "${SIM_EXIT:-1}"
fi

NARRATIVE_STATUS="SKIPPED"
if [[ "$TRACE_ENABLED" -eq 1 && -f "$TRACE_FILE" ]]; then
  set +e
  cargo run --bin trace_narrator -- "$TRACE_FILE" -d normal > "$NARRATIVE_FILE"
  NARRATIVE_EXIT=$?
  set -e
  if [[ "$NARRATIVE_EXIT" -eq 0 ]]; then
    NARRATIVE_STATUS="COMPLETE"
  else
    NARRATIVE_STATUS="FAILED"
  fi
fi

popd >/dev/null

extract_json_number() {
  local key="$1"
  local file="$2"
  rg -o "\"${key}\"[[:space:]]*:[[:space:]]*[0-9]+(\\.[0-9]+)?" "$file" \
    | head -n 1 \
    | awk -F: '{gsub(/[[:space:]]/, "", $2); print $2}' || true
}

count_occurrences() {
  local pattern="$1"
  local file="$2"
  (rg -o "$pattern" "$file" || true) | wc -l | tr -d ' '
}

TOTAL_MATCHES="$(extract_json_number "total_matches" "$RESULT_FILE")"
COMPLETED_MATCHES="$(extract_json_number "completed_matches" "$RESULT_FILE")"
AVG_RALLY_COUNT="$(extract_json_number "avg_rally_count" "$RESULT_FILE")"
TOTAL_ANOMALIES="$(extract_json_number "total_anomalies" "$RESULT_FILE")"
AVG_DURATION_SECS="$(extract_json_number "avg_duration_secs" "$RESULT_FILE")"

if [[ -z "$TOTAL_MATCHES" ]]; then TOTAL_MATCHES=0; fi
if [[ -z "$COMPLETED_MATCHES" ]]; then COMPLETED_MATCHES=0; fi
if [[ -z "$AVG_RALLY_COUNT" ]]; then AVG_RALLY_COUNT=0; fi
if [[ -z "$TOTAL_ANOMALIES" ]]; then TOTAL_ANOMALIES=0; fi
if [[ -z "$AVG_DURATION_SECS" ]]; then AVG_DURATION_SECS=0; fi

COMPLETION_RATE="$(awk -v c="$COMPLETED_MATCHES" -v t="$TOTAL_MATCHES" 'BEGIN{ if (t == 0) { printf "0.000" } else { printf "%.3f", c / t } }')"

DOUBLE_FAULT_COUNT=0
NET_FAULT_COUNT=0
OUT_COUNT=0
SHOT_EVENTS=0
SHOT_STATS_RATE="N/A"

if [[ "$TRACE_ENABLED" -eq 1 && -f "$TRACE_FILE" ]]; then
  DOUBLE_FAULT_COUNT="$(count_occurrences '"reason": "DoubleFault"' "$TRACE_FILE")"
  NET_FAULT_COUNT="$(count_occurrences '"reason": "NetFault"' "$TRACE_FILE")"
  OUT_COUNT="$(count_occurrences '"reason": "Out"' "$TRACE_FILE")"
  SHOT_EVENTS="$(count_occurrences '"type": "ShotAttributesCalculated"' "$TRACE_FILE")"
  SHOT_STATS_RATE="$(awk -v s="$SHOT_EVENTS" 'BEGIN{ if (s > 0) { printf "1.000" } else { printf "0.000" } }')"
fi

SIM_STATUS="COMPLETE"
if [[ "$SIM_EXIT" -ne 0 ]]; then
  SIM_STATUS="FAILED(${SIM_EXIT})"
fi

cat > "$REPORT_FILE" <<EOF
# QA Cycle Report

- Generated: $(date '+%Y-%m-%d %H:%M:%S')
- Config: ${CONFIG_NAME}
- Simulation Status: ${SIM_STATUS}
- Narrative Status: ${NARRATIVE_STATUS}

## KPI

| KPI | Value | Notes |
|-----|-------|-------|
| Completion Rate | ${COMPLETION_RATE} | completed_matches / total_matches |
| Avg Rally Count | ${AVG_RALLY_COUNT} | from result.json |
| Avg Duration (s) | ${AVG_DURATION_SECS} | from result.json |
| Total Anomalies | ${TOTAL_ANOMALIES} | from result.json |
| Fault DoubleFault | ${DOUBLE_FAULT_COUNT} | from trace point reason |
| Fault NetFault | ${NET_FAULT_COUNT} | from trace point reason |
| Fault Out | ${OUT_COUNT} | from trace point reason |
| Shot Stats Rate | ${SHOT_STATS_RATE} | 1.0 if ShotAttributes events detected |

## Artifacts

- Result: \`${RESULT_FILE}\`
EOF

if [[ -f "$TRACE_FILE" ]]; then
  cat >> "$REPORT_FILE" <<EOF
- Trace: \`${TRACE_FILE}\`
EOF
fi

if [[ -f "$NARRATIVE_FILE" ]]; then
  cat >> "$REPORT_FILE" <<EOF
- Narrative: \`${NARRATIVE_FILE}\`
EOF
fi

cat >> "$REPORT_FILE" <<'EOF'

## Config Roles

- debug:
  - 1 match short-run observation
  - trace enabled for detailed diagnosis
- stress:
  - multi-match stability check
  - trace disabled for throughput
EOF

echo
echo "QA Cycle 完了"
echo "  Report: ${REPORT_FILE}"
echo "  Result: ${RESULT_FILE}"
if [[ -f "$TRACE_FILE" ]]; then
  echo "  Trace:  ${TRACE_FILE}"
fi
if [[ -f "$NARRATIVE_FILE" ]]; then
  echo "  Narrative: ${NARRATIVE_FILE}"
fi

exit "$SIM_EXIT"
