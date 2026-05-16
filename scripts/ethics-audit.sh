#!/usr/bin/env bash
# SPDX-License-Identifier: CC0-1.0
#
# Pre-commit / CI wrapper around the `ethics-audit` Rust binary.
# Usage: scripts/ethics-audit.sh [DIR]   # default: fixtures/
set -euo pipefail

DIR="${1:-fixtures/}"

if [[ ! -d "$DIR" ]]; then
  echo "ethics-audit: directory $DIR does not exist (treating as empty)"
  exit 0
fi

cargo run --quiet -p ethics-audit -- "$DIR"
