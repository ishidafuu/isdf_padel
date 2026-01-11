#!/bin/bash
# リプレイファイルを手動でクリアするスクリプト

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
REPLAY_DIR="$PROJECT_DIR/assets/replays"

if [ -d "$REPLAY_DIR" ]; then
    count=$(find "$REPLAY_DIR" -name "*.bin" -type f | wc -l | tr -d ' ')
    rm -f "$REPLAY_DIR"/*.bin
    echo "Cleared $count replay file(s)."
else
    echo "Replay directory does not exist: $REPLAY_DIR"
fi
