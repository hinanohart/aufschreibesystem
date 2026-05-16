#!/usr/bin/env bash
# SPDX-License-Identifier: CC0-1.0
#
# Verifies that every file in the tree carries an SPDX-License-Identifier
# header. We do not run `reuse lint` here because `reuse` is an optional
# Nix-provided tool; this script is a lightweight equivalent that runs in
# any plain CI environment.
set -euo pipefail

cd "$(dirname "$0")/.."

EXCLUDE_REGEX='(^\.git/|^target/|^node_modules/|/__pycache__/|^Cargo\.lock$|/Cargo\.lock$|^\.gitignore$|^LICENSE$|/package-lock\.json$|/\.cargo-ok$|\.ast\.json$|/MEMORY\.md$|/MANIFEST\.md$|/devcontainer\.json$|\.json$)'

missing=0
total=0

while IFS= read -r f; do
  total=$((total+1))
  if ! grep -q -E 'SPDX-License-Identifier' "$f"; then
    echo "MISSING SPDX: $f"
    missing=$((missing+1))
  fi
done < <(git ls-files | grep -E -v "$EXCLUDE_REGEX")

echo "license-matrix-check: $missing/$total files missing SPDX header"
if [[ "$missing" -gt 0 ]]; then
  exit 1
fi
