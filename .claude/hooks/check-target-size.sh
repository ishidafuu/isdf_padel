#!/bin/bash

# target/ ディレクトリの容量チェック
# セッション開始時に実行され、閾値超過時に警告を表示

TARGET_DIR="$CLAUDE_PROJECT_DIR/project/target"
THRESHOLD_GB=20  # 20GB を超えたら警告

if [ -d "$TARGET_DIR" ]; then
  SIZE_KB=$(du -sk "$TARGET_DIR" 2>/dev/null | cut -f1)
  SIZE_GB=$((SIZE_KB / 1024 / 1024))

  if [ "$SIZE_GB" -ge "$THRESHOLD_GB" ]; then
    echo "⚠️ project/target/ が ${SIZE_GB}GB あります（閾値: ${THRESHOLD_GB}GB）"
    echo "   削除コマンド: cd project && cargo clean"
  fi
fi

exit 0
