#!/usr/bin/env python3
"""
Rust ファイル編集時に cargo check を自動実行する hook スクリプト

stdin から JSON を受け取り、.rs ファイルの場合のみ cargo check を実行。
"""
import json
import sys
import subprocess
import os

def main():
    try:
        input_data = json.load(sys.stdin)
    except json.JSONDecodeError:
        sys.exit(0)

    tool_name = input_data.get("tool_name", "")
    tool_input = input_data.get("tool_input", {})
    file_path = tool_input.get("file_path", "")

    # Rust ファイル以外は無視
    if tool_name not in ("Write", "Edit") or not file_path.endswith(".rs"):
        sys.exit(0)

    # project ディレクトリを特定
    project_dir = os.environ.get("CLAUDE_PROJECT_DIR", ".")
    cargo_dir = os.path.join(project_dir, "project")

    # Cargo.toml が存在するか確認
    if not os.path.exists(os.path.join(cargo_dir, "Cargo.toml")):
        cargo_dir = project_dir  # fallback

    try:
        result = subprocess.run(
            ["cargo", "check", "--message-format=short"],
            cwd=cargo_dir,
            capture_output=True,
            text=True,
            timeout=60,
            env={**os.environ, "PATH": f"{os.environ.get('HOME')}/.cargo/bin:{os.environ.get('PATH', '')}"}
        )

        if result.returncode == 0:
            print(f"cargo check passed")
        else:
            # エラー出力を整形（最初の20行のみ）
            stderr_lines = result.stderr.strip().split('\n')[:20]
            error_msg = '\n'.join(stderr_lines)
            print(f"cargo check failed:\n{error_msg}")

    except subprocess.TimeoutExpired:
        print("cargo check timeout (60s)")
    except FileNotFoundError:
        print("cargo not found - skipping check")
    except Exception as e:
        print(f"cargo check error: {e}")

    sys.exit(0)

if __name__ == "__main__":
    main()
