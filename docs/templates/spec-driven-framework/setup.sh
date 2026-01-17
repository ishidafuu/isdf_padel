#!/bin/bash
# Spec-Driven Framework Setup Script
# Usage: ./setup.sh [target_directory]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Spec-Driven Framework - Setup Script                  ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

# Get target directory
if [ -n "$1" ]; then
    TARGET_DIR="$1"
else
    echo -e "${YELLOW}Target directory not specified.${NC}"
    read -p "Enter target directory path: " TARGET_DIR
fi

# Expand ~ to home directory
TARGET_DIR="${TARGET_DIR/#\~/$HOME}"

# Convert to absolute path
TARGET_DIR="$(cd "$(dirname "$TARGET_DIR")" 2>/dev/null && pwd)/$(basename "$TARGET_DIR")" || TARGET_DIR="$TARGET_DIR"

echo ""
echo -e "${BLUE}Target directory: ${NC}$TARGET_DIR"

# Check if directory exists
if [ -d "$TARGET_DIR" ]; then
    echo -e "${YELLOW}Warning: Directory already exists.${NC}"
    read -p "Continue and merge? (y/n): " CONFIRM
    if [ "$CONFIRM" != "y" ] && [ "$CONFIRM" != "Y" ]; then
        echo "Aborted."
        exit 1
    fi
fi

# Create target directory
mkdir -p "$TARGET_DIR"

echo ""
echo -e "${BLUE}Step 1: Creating directory structure...${NC}"

# Create directory structure
mkdir -p "$TARGET_DIR"/{.claude/{agents,commands,skills,hooks,handover/archive},docs,tasks/{0_backlog,1_todo,2_in-progress,3_in-review,4_archive},project/{docs/{1_project,2_architecture,3_ingame,4_outgame,7_tools,8_data,9_reference,_deprecated},src,tests,tasks/{0_backlog,1_todo,2_in-progress,3_in-review,4_archive},plans},plans}

echo -e "${GREEN}  ✓ Directory structure created${NC}"

echo ""
echo -e "${BLUE}Step 2: Copying framework files...${NC}"

# Copy agents
cp "$SCRIPT_DIR"/.claude/agents/*.md "$TARGET_DIR"/.claude/agents/
echo -e "${GREEN}  ✓ Agents copied ($(ls "$SCRIPT_DIR"/.claude/agents/*.md | wc -l | tr -d ' ') files)${NC}"

# Copy skills
cp "$SCRIPT_DIR"/.claude/skills/*.md "$TARGET_DIR"/.claude/skills/
echo -e "${GREEN}  ✓ Skills copied ($(ls "$SCRIPT_DIR"/.claude/skills/*.md | wc -l | tr -d ' ') files)${NC}"

# Copy commands
cp "$SCRIPT_DIR"/.claude/commands/*.md "$TARGET_DIR"/.claude/commands/
echo -e "${GREEN}  ✓ Commands copied ($(ls "$SCRIPT_DIR"/.claude/commands/*.md | wc -l | tr -d ' ') files)${NC}"

# Copy hooks
cp "$SCRIPT_DIR"/.claude/hooks/*.py "$TARGET_DIR"/.claude/hooks/
echo -e "${GREEN}  ✓ Hooks copied ($(ls "$SCRIPT_DIR"/.claude/hooks/*.py | wc -l | tr -d ' ') files)${NC}"

# Copy docs
cp -r "$SCRIPT_DIR"/docs/* "$TARGET_DIR"/docs/
echo -e "${GREEN}  ✓ Documentation copied${NC}"

# Copy task config
cp "$SCRIPT_DIR"/tasks/.taskrc.yaml "$TARGET_DIR"/tasks/
cp "$SCRIPT_DIR"/project/tasks/.taskrc.yaml "$TARGET_DIR"/project/tasks/
echo -e "${GREEN}  ✓ Task configuration copied${NC}"

echo ""
echo -e "${BLUE}Step 3: Setting up templates...${NC}"

# Copy template files (will be processed later)
cp "$SCRIPT_DIR"/.claude/CLAUDE.md.template "$TARGET_DIR"/.claude/
cp "$SCRIPT_DIR"/.claude/settings.json.template "$TARGET_DIR"/.claude/
cp "$SCRIPT_DIR"/.claudeignore.template "$TARGET_DIR"/
cp "$SCRIPT_DIR"/project/docs/1_project/10001_concept.md.template "$TARGET_DIR"/project/docs/1_project/
cp "$SCRIPT_DIR"/project/docs/2_architecture/20000_overview.md.template "$TARGET_DIR"/project/docs/2_architecture/

echo -e "${GREEN}  ✓ Templates copied${NC}"

echo ""
echo -e "${BLUE}Step 4: Processing templates...${NC}"

# Get project info
echo ""
read -p "Project name: " PROJECT_NAME
read -p "Project description: " PROJECT_DESCRIPTION

PROJECT_ROOT=$(basename "$TARGET_DIR")

# Process CLAUDE.md.template
sed -e "s/{{PROJECT_NAME}}/$PROJECT_NAME/g" \
    -e "s/{{PROJECT_DESCRIPTION}}/$PROJECT_DESCRIPTION/g" \
    -e "s/{{PROJECT_ROOT}}/$PROJECT_ROOT/g" \
    "$TARGET_DIR"/.claude/CLAUDE.md.template > "$TARGET_DIR"/.claude/CLAUDE.md
rm "$TARGET_DIR"/.claude/CLAUDE.md.template
echo -e "${GREEN}  ✓ CLAUDE.md generated${NC}"

# Process settings.json.template
cp "$TARGET_DIR"/.claude/settings.json.template "$TARGET_DIR"/.claude/settings.json
rm "$TARGET_DIR"/.claude/settings.json.template
echo -e "${GREEN}  ✓ settings.json generated${NC}"

# Process .claudeignore.template
cp "$TARGET_DIR"/.claudeignore.template "$TARGET_DIR"/.claudeignore
rm "$TARGET_DIR"/.claudeignore.template
echo -e "${GREEN}  ✓ .claudeignore generated${NC}"

# Process project templates
sed -e "s/{{PROJECT_NAME}}/$PROJECT_NAME/g" \
    -e "s/{{PROJECT_DESCRIPTION}}/$PROJECT_DESCRIPTION/g" \
    -e "s/{{TARGET_PLATFORM}}/TBD/g" \
    -e "s/{{LANGUAGE}}/TBD/g" \
    "$TARGET_DIR"/project/docs/1_project/10001_concept.md.template > "$TARGET_DIR"/project/docs/1_project/10001_concept.md
rm "$TARGET_DIR"/project/docs/1_project/10001_concept.md.template

sed -e "s/{{LANGUAGE}}/TBD/g" \
    "$TARGET_DIR"/project/docs/2_architecture/20000_overview.md.template > "$TARGET_DIR"/project/docs/2_architecture/20000_overview.md
rm "$TARGET_DIR"/project/docs/2_architecture/20000_overview.md.template

echo -e "${GREEN}  ✓ Project documents generated${NC}"

echo ""
echo -e "${BLUE}Step 5: Optional features...${NC}"

# Ask about Rust hook
echo ""
read -p "Enable Rust compilation check hook? (y/n): " ENABLE_RUST
if [ "$ENABLE_RUST" = "y" ] || [ "$ENABLE_RUST" = "Y" ]; then
    # Add rust-check hook to settings.json
    # Using Python for JSON manipulation to avoid jq dependency
    python3 << EOF
import json
import os

settings_path = os.path.join("$TARGET_DIR", ".claude", "settings.json")
with open(settings_path, 'r') as f:
    settings = json.load(f)

# Find or create the Edit|Write matcher
hooks = settings.get('hooks', {}).get('PostToolUse', [])
rust_hook_added = False

for hook in hooks:
    if hook.get('matcher') == 'Edit|Write':
        hook['hooks'].insert(0, {
            "type": "command",
            "command": 'python3 "\$CLAUDE_PROJECT_DIR/.claude/hooks/rust-check.py"'
        })
        rust_hook_added = True
        break

if not rust_hook_added:
    hooks.insert(0, {
        "matcher": "Edit|Write",
        "hooks": [{
            "type": "command",
            "command": 'python3 "\$CLAUDE_PROJECT_DIR/.claude/hooks/rust-check.py"'
        }]
    })
    settings['hooks']['PostToolUse'] = hooks

with open(settings_path, 'w') as f:
    json.dump(settings, f, indent=2)
EOF
    echo -e "${GREEN}  ✓ Rust hook enabled${NC}"
else
    echo -e "${YELLOW}  - Rust hook skipped${NC}"
fi

# Ask about game-specific commands
echo ""
read -p "Include game-specific commands? (run-game, qa-cycle, qa-review, delete-replays) (y/n): " ENABLE_GAME
if [ "$ENABLE_GAME" = "y" ] || [ "$ENABLE_GAME" = "Y" ]; then
    if [ -d "$SCRIPT_DIR/optional/game-commands" ]; then
        cp "$SCRIPT_DIR"/optional/game-commands/*.md "$TARGET_DIR"/.claude/commands/
        echo -e "${GREEN}  ✓ Game commands installed${NC}"
    else
        echo -e "${YELLOW}  - Game commands not found in template${NC}"
    fi
else
    echo -e "${YELLOW}  - Game commands skipped${NC}"
fi

echo ""
echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Setup Complete!                                       ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Project created at: ${NC}$TARGET_DIR"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. cd $TARGET_DIR"
echo "  2. git init  (if not already a git repo)"
echo "  3. Edit .claude/CLAUDE.md to customize project rules"
echo "  4. Edit project/docs/1_project/10001_concept.md"
echo "  5. Edit project/docs/2_architecture/20000_overview.md"
echo "  6. Run 'claude' to start Claude Code"
echo ""
echo -e "${BLUE}Documentation:${NC}"
echo "  - Framework docs: $TARGET_DIR/docs/index.md"
echo "  - Quick start: $TARGET_DIR/docs/tutorials/quickstart.md"
echo ""
