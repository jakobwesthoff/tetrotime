#!/bin/bash
# record.sh - Runs VHS recording for the TetroTime demo
#
# Usage: ./record.sh
#
# Requires tetrotime to be installed and available in PATH.
# Output files (demo.webm, demo.mp4) are moved to docs/pages/assets/.

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

if ! command -v tetrotime &> /dev/null; then
    echo "Error: tetrotime is not installed or not in PATH."
    echo "Please install tetrotime first (e.g. cargo install --path .)"
    exit 1
fi

if ! command -v vhs &> /dev/null; then
    echo "Error: vhs is not installed or not in PATH."
    echo "Please install VHS first: https://github.com/charmbracelet/vhs"
    exit 1
fi

# Record
cd "$SCRIPT_DIR"
vhs demo.tape

# Move recordings to assets folder
ASSETS_DIR="$SCRIPT_DIR/../pages/assets"
mkdir -p "$ASSETS_DIR"
mv demo.webm "$ASSETS_DIR/"
mv demo.mp4 "$ASSETS_DIR/"

echo "Recording complete. Output files in docs/pages/assets/: demo.webm, demo.mp4"
