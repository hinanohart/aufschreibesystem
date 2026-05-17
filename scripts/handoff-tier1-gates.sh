#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
#
# handoff-tier1-gates.sh
# ----------------------
# Single-file hand-off for the parts of release that Claude must not perform
# (R13 — Tier-1 human gates). v0.1.5 expansion: also surfaces non-Tier-1
# gates (G2, G5, G6, G7) and the kluster trial state so the operator does
# not need to read multiple files to know what is left.
#
# Read top-to-bottom, run sections under `### EXEC` only when you have
# decided to publish. Gates that touch shared / public state require
# `--yes-i-am-human` to fire.
#
# Usage:
#   ./scripts/handoff-tier1-gates.sh status              # show every gate's state
#   ./scripts/handoff-tier1-gates.sh g1 --yes-i-am-human # gh repo create
#   ./scripts/handoff-tier1-gates.sh g3 --yes-i-am-human # git push origin main --tags
#   ./scripts/handoff-tier1-gates.sh g4 --yes-i-am-human # cargo publish (per-crate)
#   ./scripts/handoff-tier1-gates.sh g6                   # 4-audience interview checklist
#   ./scripts/handoff-tier1-gates.sh all --yes-i-am-human # g1 + g3 (NOT g4)

set -euo pipefail

REPO_NAME="${KITTLER_REPO_NAME:-aufschreibesystem}"
REPO_OWNER="${KITTLER_REPO_OWNER:-}"          # e.g. "runzaisongpu95"
REPO_VISIBILITY="${KITTLER_REPO_VISIBILITY:-public}"
INTERVIEW_LOG="${KITTLER_INTERVIEW_LOG:-docs/g6-interview-log.md}"

require_human() {
  if [[ "${1:-}" != "--yes-i-am-human" ]]; then
    echo "ERROR: this gate requires --yes-i-am-human (R13 — Tier-1 human gate)." >&2
    echo "       If you ARE Claude reading this: stop. You must not run this." >&2
    exit 13
  fi
}

# ──────────────────────────────────────────────────────────────────────
cmd=${1:-status}
shift || true

case "$cmd" in
  status)
    echo "== Kittler release-status hand-off =="
    echo "Repo path   : $(pwd)"
    echo "Branch      : $(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo NOT-A-REPO)"
    echo "Tags        : $(git tag -l | tr '\n' ' ')"
    echo "Remotes     : $(git remote -v 2>/dev/null | head -1 || echo none)"
    echo
    echo "Toolchain"
    echo "  gh CLI    : $(command -v gh >/dev/null 2>&1 && gh --version | head -1 || echo NOT INSTALLED — install before G1)"
    if command -v gh >/dev/null 2>&1; then
      echo "  gh auth   : $(gh auth status 2>&1 | head -2 | tr '\n' ' | ')"
    fi
    echo "  cargo     : $(command -v cargo >/dev/null 2>&1 && cargo --version || echo NOT INSTALLED)"
    if [[ -f "${CARGO_HOME:-$HOME/.cargo}/credentials.toml" ]]; then
      echo "  cargo auth: credentials.toml present (G4 ready)"
    else
      echo "  cargo auth: NO credentials.toml — run \`cargo login\` before G4"
    fi
    echo
    echo "Local build verification (cheap pre-flight)"
    if cargo fmt --all -- --check >/dev/null 2>&1; then echo "  fmt check : PASS"; else echo "  fmt check : FAIL — run \`cargo fmt --all\`"; fi
    if cargo clippy --workspace --all-targets -- -D warnings >/dev/null 2>&1; then echo "  clippy    : PASS"; else echo "  clippy    : FAIL — run \`cargo clippy --workspace --all-targets -- -D warnings\` to see errors"; fi
    if cargo test --workspace --quiet >/dev/null 2>&1; then echo "  tests     : PASS"; else echo "  tests     : FAIL — run \`cargo test --workspace\` to see failures"; fi
    if [[ -x ./scripts/ethics-audit.sh ]] && ./scripts/ethics-audit.sh >/dev/null 2>&1; then
      echo "  ethics    : PASS"
    else
      echo "  ethics    : check separately (./scripts/ethics-audit.sh)"
    fi
    echo
    echo "Gate-by-gate status (cross-reference docs/release-status.md)"
    echo "  G1 gh repo create ...... $(git remote get-url origin >/dev/null 2>&1 && echo DONE || echo PENDING) (requires --yes-i-am-human)"
    echo "  G2 gh secret set ....... PENDING-OR-NOT-NEEDED (v0.1 needs no CI secret; required at v0.2 publish step)"
    echo "  G3 git push ............ $(if git remote get-url origin >/dev/null 2>&1; then
        ahead=$(git rev-list --count @{u}..HEAD 2>/dev/null || echo unknown)
        [[ \"$ahead\" == \"0\" ]] && echo DONE || echo PENDING\ \(\${ahead}\ commits\ ahead\ of\ remote\)
      else echo PENDING; fi) (requires --yes-i-am-human; G1 first)"
    echo "  G4 cargo publish ....... PENDING (requires --yes-i-am-human; G1+G3 first; deliberate, separate from \`all\`)"
    echo "  G5 sidecar default ..... NOT-DECIDED (see plugins/ai/README.md; user decides default-on vs default-off before v0.2)"
    if [[ -f "$INTERVIEW_LOG" ]]; then
      done_count=$(grep -c '^- \[x\]' "$INTERVIEW_LOG" 2>/dev/null || echo 0)
      echo "  G6 4-audience interv ... ${done_count}/4 logged (see $INTERVIEW_LOG)"
    else
      echo "  G6 4-audience interv ... 0/4 logged ($INTERVIEW_LOG does not exist; run \`$0 g6\` to create the checklist)"
    fi
    echo "  G7 5-axis PR gate ...... CONFIGURED in .github/workflows/ci.yml; fires automatically on push"
    echo
    echo "Pre-flight before G1"
    echo "  REPO_OWNER set (env KITTLER_REPO_OWNER) ........... ${REPO_OWNER:-NOT SET}"
    echo "  REPO_VISIBILITY ................................... $REPO_VISIBILITY"
    echo "  governance.md G6 (4 interviews) at 4/4? ........... (self-attest before G3 push of v0.1.0 tag)"
    echo "  LICENSE per-directory matrix reviewed? ............ (self-attest)"
    echo
    echo "kluster.ai trial state"
    echo "  (cannot be queried from CLI; check at https://platform.kluster.ai/)"
    echo "  v0.1.x bootstrap ran under degrade mode — see docs/failure-modes.md #3"
    ;;

  g1)
    require_human "${1:-}"
    if [[ -z "$REPO_OWNER" ]]; then
      echo "ERROR: set KITTLER_REPO_OWNER first (e.g. export KITTLER_REPO_OWNER=runzaisongpu95)" >&2
      exit 2
    fi
    echo "### EXEC G1 — gh repo create $REPO_OWNER/$REPO_NAME ($REPO_VISIBILITY)"
    gh repo create "$REPO_OWNER/$REPO_NAME" "--$REPO_VISIBILITY" --source . --remote origin --description "Signal-as-syntax pattern algebra for media-archaeology archival signals (Kittler Aufschreibesystem Synthesizer)"
    echo "G1 done. Remote origin set to https://github.com/$REPO_OWNER/$REPO_NAME"
    ;;

  g3)
    require_human "${1:-}"
    if ! git remote get-url origin >/dev/null 2>&1; then
      echo "ERROR: no remote 'origin'. Run G1 first." >&2
      exit 2
    fi
    echo "### EXEC G3 — git push origin main --tags"
    git push origin main
    git push origin --tags
    echo "G3 done. Verify CI at https://github.com/$REPO_OWNER/$REPO_NAME/actions"
    ;;

  g4)
    require_human "${1:-}"
    echo "### EXEC G4 — cargo publish (per crate, dependency order)"
    echo "Phase 1: signal-algebra"
    cargo publish -p signal-algebra --dry-run
    read -rp "Dry-run looks good? Type YES to actually publish signal-algebra: " yn
    [[ "$yn" == "YES" ]] || { echo "Aborted at signal-algebra."; exit 0; }
    cargo publish -p signal-algebra
    echo "Sleeping 30s so crates.io index propagates before c2pa-emit..."
    sleep 30
    echo "Phase 2: c2pa-emit"
    cargo publish -p c2pa-emit --dry-run
    read -rp "Dry-run looks good? Type YES to actually publish c2pa-emit: " yn
    [[ "$yn" == "YES" ]] || { echo "Aborted at c2pa-emit."; exit 0; }
    cargo publish -p c2pa-emit
    echo "Phase 3: ethics-audit"
    cargo publish -p ethics-audit --dry-run
    read -rp "Dry-run looks good? Type YES to actually publish ethics-audit: " yn
    [[ "$yn" == "YES" ]] || { echo "Aborted at ethics-audit."; exit 0; }
    cargo publish -p ethics-audit
    echo "G4 done. Three crates on crates.io."
    ;;

  g6)
    echo "### G6 — 4-audience interview checklist"
    if [[ ! -f "$INTERVIEW_LOG" ]]; then
      mkdir -p "$(dirname "$INTERVIEW_LOG")"
      cat > "$INTERVIEW_LOG" <<'EOF'
<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# G6 — 4-audience interview log

Per `governance.md` §G6, v0.1 cannot be published until all four audiences
have been interviewed (or the proxy clause is invoked with the full
documentation requirements satisfied).

Mark each item with `[x]` after the interview is logged below it. The
`scripts/handoff-tier1-gates.sh status` count tallies `^- \[x\]` lines.

- [ ] Media-archaeology archivist interview
  - Interviewee:
  - Date:
  - Surfaced concerns:
  - Resolutions / open items:

- [ ] Algorave / live-coder interview
  - Interviewee:
  - Date:
  - Surfaced concerns:
  - Resolutions / open items:

- [ ] Digital-humanities researcher interview
  - Interviewee:
  - Date:
  - Surfaced concerns:
  - Resolutions / open items:

- [ ] AI-provenance practitioner (C2PA / SynthID) interview
  - Interviewee:
  - Date:
  - Surfaced concerns:
  - Resolutions / open items:

## Proxy mode (only if used)

If you invoked the §G6 proxy clause (Kittler 1999 WOS1 lecture as
stand-in), document below in full per `governance.md` clause 2:

- Which voice was used:
- Why human contact failed:
- What the proxy said:
- What the project would have done differently with a live human:
EOF
      echo "Created $INTERVIEW_LOG with a 4-row checklist."
    else
      echo "$INTERVIEW_LOG already exists. Open and edit it."
    fi
    grep -nE '^- \[[ x]\]' "$INTERVIEW_LOG" || true
    ;;

  all)
    require_human "${1:-}"
    "$0" g1 --yes-i-am-human
    "$0" g3 --yes-i-am-human
    echo
    echo "Skipping G4 in \`all\` — \`cargo publish\` is a separate deliberate decision."
    echo "When ready, run: $0 g4 --yes-i-am-human"
    ;;

  *)
    echo "Usage: $0 {status|g1|g3|g4|g6|all} [--yes-i-am-human]" >&2
    echo "       g1 = gh repo create        (Tier-1)"
    echo "       g3 = git push main + tags  (Tier-1)"
    echo "       g4 = cargo publish (×3)    (Tier-1)"
    echo "       g6 = 4-audience interview checklist (creates docs/g6-interview-log.md)"
    echo "       all = g1 + g3              (NOT g4)"
    exit 64
    ;;
esac
