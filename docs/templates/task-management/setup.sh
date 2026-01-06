#!/bin/bash
set -e

# Task Management - Setup Script
#
# Usage:
#   1. Copy task-management/ to your project root
#   2. Run either:
#      - From project root: ./task-management/setup.sh
#      - From inside folder: cd task-management && ./setup.sh
#   3. Delete task-management/ folder after setup

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CURRENT_DIR="$(pwd)"

# Determine project root
if [ "$SCRIPT_DIR" = "$CURRENT_DIR" ]; then
    # Running from inside task-management/
    PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
    echo "Running from inside template directory."
    echo "Will setup in parent directory: $PROJECT_ROOT"
else
    # Running from project root
    PROJECT_ROOT="$CURRENT_DIR"
fi

echo ""
echo "Task Management - Setup"
echo "=============================="
echo ""
echo "Source: $SCRIPT_DIR"
echo "Target: $PROJECT_ROOT"
echo ""

# Create directories
echo "[1/4] Creating directories..."
mkdir -p "$PROJECT_ROOT/.claude/skills"
mkdir -p "$PROJECT_ROOT/.claude/agents"
mkdir -p "$PROJECT_ROOT/.claude/commands"
mkdir -p "$PROJECT_ROOT/tasks/1_todo"
mkdir -p "$PROJECT_ROOT/tasks/2_in-progress"
mkdir -p "$PROJECT_ROOT/tasks/3_archive"

# Copy skill files
echo "[2/4] Copying skill files..."
cp "$SCRIPT_DIR/.claude/skills/task-workflow.md" "$PROJECT_ROOT/.claude/skills/"

# Copy agent files
echo "[3/4] Copying agent files..."
cp "$SCRIPT_DIR/.claude/agents/task-agent.md" "$PROJECT_ROOT/.claude/agents/"

# Copy command files
echo "[4/4] Copying command files..."
cp "$SCRIPT_DIR/.claude/commands/id-next.md" "$PROJECT_ROOT/.claude/commands/"
cp "$SCRIPT_DIR/.claude/commands/handover.md" "$PROJECT_ROOT/.claude/commands/"
cp "$SCRIPT_DIR/.claude/commands/resume-handover.md" "$PROJECT_ROOT/.claude/commands/"

# Copy task config
cp "$SCRIPT_DIR/tasks/.taskrc.yaml" "$PROJECT_ROOT/tasks/"
cp "$SCRIPT_DIR/tasks/README.md" "$PROJECT_ROOT/tasks/"

echo ""
echo "Setup complete!"
echo ""
echo "=============================="
echo "Next steps:"
echo ""
echo "1. Add the following to your .claude/CLAUDE.md:"
echo ""
cat "$SCRIPT_DIR/.claude/CLAUDE.md.template"
echo ""
echo "=============================="
echo ""
echo "2. Delete the template folder:"
echo "   rm -rf $PROJECT_ROOT/task-management"
echo ""
echo "3. Start using tasks:"
echo '   "Create a task for implementing login feature"'
echo '   "Start task T001"'
echo '   "Complete task T001"'
echo ""
echo "4. Session handover:"
echo "   /handover         (before ending session)"
echo "   /resume-handover  (when starting new session)"
echo ""
