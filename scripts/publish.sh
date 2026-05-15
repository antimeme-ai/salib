#!/usr/bin/env bash
set -euo pipefail

# Publish all workspace crates to crates.io in dependency order.
# Requires: cargo install cargo-workspaces
#
# Usage:
#   ./scripts/publish.sh           # bump patch, publish all
#   ./scripts/publish.sh minor     # bump minor, publish all
#   ./scripts/publish.sh --dry-run # show what would happen

BUMP="${1:-patch}"

if [[ "$BUMP" == "--dry-run" ]]; then
    echo "Publish order:"
    cargo ws plan
    echo ""
    echo "Current versions:"
    cargo ws list
    exit 0
fi

cargo ws publish --from-git --yes "$BUMP"
