<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# Release status — what `v0.1.5-local` actually claims

This file separates the load-bearing words so a reader does not infer
guarantees the artifact does not carry. Updated each `-local` tag.

## Current tag

`v0.1.5-local` (HEAD as of this commit). Suffix `-local` is governance-defined
(see `governance.md` §G6 clause 3): the build is shippable to **user
inspection** but not to a **public registry**.

## What is verified at this tag

| Claim                                                            | Evidence                                                                                          |
|------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `cargo test --workspace` passes                                  | Local cargo run on maintainer host. Count: 26 tests (5 c2pa-emit + 12 ethics-audit + 9 signal-algebra). |
| `cargo clippy --workspace --all-targets -- -D warnings` clean    | Local cargo run on maintainer host.                                                               |
| `cargo fmt --all -- --check` clean                               | Local cargo run on maintainer host.                                                               |
| MVP example produces valid JSON                                  | `cargo run -p signal-algebra --example iq_to_pattern` end-to-end.                                 |
| Ethics audit detects fixtures for all 7 detectors                | `cargo test -p ethics-audit` covers each `Finding` variant with at least one rejecting fixture (since v0.1.5: scte35, derived-synthetic, smuggled-readme, bare-consent regressions added). |
| SPDX header coverage 35/35 source files                          | `scripts/license-matrix-check.sh fixtures/`.                                                      |
| 3-stage C2PA chain ordering enforced                             | `c2pa-emit::is_well_ordered` + tests for legal/illegal shapes.                                    |
| 5 `#[non_exhaustive]` SemVer guards                              | `Stage`, `Finding`, `OriginProtocol`, `PatternAtom`, `Manifest`, `ProvenanceTag`, `SignalErr`, `EventStream`, `PatternEvent` (since v0.1.5 the omc audit's 5-struct gap was closed). |

## What is NOT verified at this tag (do not claim)

| Claim that would be FALSE today                                  | Reason                                                                                            |
|------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| "CI is green"                                                    | The repo has not been pushed to GitHub. The CI yaml is *configured* and lints clean on a local `act` dry-run is **not** evidence; first push will run it for real. |
| "v0.1 is released"                                               | Governance gate G6 (4-audience interview) is **0/4** complete. Per `governance.md`, no public tag without 4/4 or an explicitly-documented proxy invocation. |
| "All seven detectors use real decoders"                          | They are filename heuristics in v0.1; real decoders are v0.2 work. (`governance.md` §"Ethics audit" already states this, repeated here so a reader of this status file alone is not misled.) |
| "Reproducible via Nix"                                           | `flake.nix` ships without `flake.lock`. v0.2 pins nixpkgs.                                        |
| "Bench-verified < 20 ms end-to-end latency" (axis 4)             | No `benches/` directory exists; `cargo bench` is unimplemented. v0.2 work.                        |
| "kluster.ai code review was applied to every commit"             | Trial expired before the v0.1.x bootstrap; the supervisor protocol's degrade path was used. See `docs/failure-modes.md` #3. |

## What happens at each remaining gate

- **G1** (`gh repo create`) — human action. After: the repo exists publicly. Before this, "the OSS" is a local artifact and that wording is the honest one.
- **G2** (`gh secret set` for CI tokens) — human action. After: CI can use secrets (currently CI needs none, so this is a no-op until v0.2 adds a publish step).
- **G3** (`git push origin main --tags`) — human action. After: CI runs for real, GitHub displays the badges, "CI green" becomes a claim that has evidence.
- **G4** (`cargo publish`) — human action. After: the three crates exist on crates.io. Before this, "library users can `cargo add signal-algebra`" is false.
- **G5** (Qwen3-Omni sidecar default-on vs default-off) — UX decision. The decision is not made; `plugins/ai/README.md` documents both options.
- **G6** (4-audience interview) — human action. After completing or after deliberately invoking proxy mode with the §G6 documentation requirements satisfied, the release blocker lifts.
- **G7** (5-axis filter merge gate) — automated via PR template + CI grep job; lives in `.github/workflows/ci.yml`.

## Why this file exists

To make it impossible to confuse "the maintainer's local cargo passes" with
"the published crate is verified by CI." That confusion is the failure mode
the supervisor protocol's fabrication-resistance axis exists to refuse — and
it can only refuse it if the project's own status language is precise.

The hand-off script `scripts/handoff-tier1-gates.sh` mirrors this table:
running `./scripts/handoff-tier1-gates.sh status` prints the *current* state
of each gate condition that can be checked from the local environment.
