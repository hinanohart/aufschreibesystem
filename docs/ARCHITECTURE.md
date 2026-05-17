<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# Architecture

> Source: 3-agent R14 exploration + independent critic round, 2026-05-17.
> Score: **22/25** (ACCEPT-WITH-RESERVATIONS). The concept-memo round
> closed at 23/25; the architecture round paid −1 on the general-purpose
> axis to make the license matrix honest (full story in §7).
> Detailed exploration record:
> `~/.claude/projects/-home-runza/memory/project_kittler-oss-architecture-2026-05-17.md`

## 1. The premise

A signal is not data about a thing. A signal *is* the thing the medium does.
Treating it as a first-class data type in a pattern algebra preserves what
embedding-based pipelines flatten.

This document describes how that premise becomes code.

## 1.1 Recommended stack (one-table quick reference)

| Responsibility    | Choice                                | License                                              | Why                              |
|-------------------|---------------------------------------|------------------------------------------------------|----------------------------------|
| signal-ingest     | GNU Radio 3.10 + Greaseweazle         | GPL-3.0 / Unlicense                                  | Media-archaeology baseline       |
| signal-algebra    | Rust rlib + `trait Signal` (cdylib v0.2) | source MIT / binary GPL-3.0 / spec Apache-2.0 OR MIT | Materiality enforced at type level |
| pattern-DSL       | TS WASM Strudel wrap                  | AGPL-3.0 (web-stage isolation)                       | Algorave inflow                  |
| AI plugin         | Python sidecar + Qwen3 user-pull      | Apache-2.0 (host) / Tongyi (model, user-pulled)      | Commercial-clause trap avoidance |
| provenance        | `c2pa-rs` + SynthID                   | MIT                                                  | Provenance standard              |
| CI                | `make` + GH Actions (Nix in v0.2)     | —                                                    | Bus-factor + reproducibility     |
| ethics audit      | 7 detectors CI + pre-commit (v0.2)    | —                                                    | Litigation fallback              |

(Source: architecture-round R14 memo, 2026-05-17. AUDIT-B in v0.1.3-local
asked for this table to be restored verbatim because the prose alone hid the
license-vs-tool mapping.)

## 2. The five layers

```
┌──────────────────────────────────────────────────────────────────┐
│                       provenance (L5)                            │
│  c2pa-rs + SynthID — three-stage manifest chain (raw / pattern / │
│  interpretation). MIT.                                           │
└──────────────────────────────────────────────────────────────────┘
                              ▲
                              │
┌──────────────────────────────────────────────────────────────────┐
│                       pattern-DSL (L3)                           │
│  Strudel WRAP (not fork) over WASM. AGPL-3.0 process isolation.  │
│  Optional AI plugin (L4) is an OUT-OF-PROCESS sidecar — never    │
│  linked into the algebra.                                        │
└──────────────────────────────────────────────────────────────────┘
                              ▲
                              │
┌──────────────────────────────────────────────────────────────────┐
│                       signal-algebra (L2)                        │
│  Rust rlib in v0.1 (cdylib deferred to v0.2 when the C ABI for   │
│    `kittler-archive` plugins lands; see Cargo.toml comment).     │
│  Source MIT, binary GPL-3.0-or-later (via L1).                   │
│  trait Signal { type Sample; fn sample_rate_hz() -> Option<u64>; │
│                 fn provenance() -> ProvenanceTag; fn next_frame } │
│  trait IntoPatternAtom : Signal { fn to_audio() -> Box<dyn ...>; │
│                                   fn cycle_dur(); to_event_str() }│
│  Spec is published separately as Apache-2.0 OR MIT.              │
└──────────────────────────────────────────────────────────────────┘
                              ▲
                              │
┌──────────────────────────────────────────────────────────────────┐
│                       signal-ingest (L1)                         │
│  GNU Radio 3.10 (GPL-3.0) for RF / SDR.                          │
│  Greaseweazle (Unlicense) for floppy flux.                       │
│  Future: VHS RF, wax cylinder, paper-tape.                       │
└──────────────────────────────────────────────────────────────────┘

L4 (ai-plugin): Python sidecar, separate repo, opt-in.
                Qwen3-Omni weights are USER-PULLED — never bundled.
                IPC over local UNIX socket. Apache-2.0 host wrapper.
```

## 3. Two end-user binaries, one shared spec

| Binary             | Layers used | License             | Audience              |
|--------------------|-------------|---------------------|-----------------------|
| `kittler-archive`  | L1+L2+L5    | GPL-3.0-or-later    | Archivists / DH       |
| `kittler-stage`    | L3+L5       | AGPL-3.0-or-later   | Algorave / live-coder |
| (`kittler-ai`)     | L4          | Apache-2.0 + user-pulled model | AI provenance |

The shared `spec/signal-algebra/` is `Apache-2.0 OR MIT` so that third-party
UIs (e.g., a future Pure-Data binding or a future audio-editor plugin) can
target it without inheriting GPL.

The umbrella distribution always inherits the strongest license that applies
to the code it links — declared in `LICENSE` per directory.

## 4. The Tongyi Qianwen user-pull design (architectural constraint)

The Qwen3-Omni model is one of the few open-weight multimodal models suitable
for our work. Its license (Tongyi Qianwen) imposes commercial-use restrictions
that we deliberately do not want to inherit into a permissively-licensed OSS.

The architectural answer is **strict bundling negation**:

1. The `kittler-ai` sidecar **never** ships model weights.
2. On first run the user is shown the Tongyi Qianwen license excerpt verbatim
   and asked to confirm. Only on confirmation does the sidecar download into
   `~/.kittler/models/`.
3. The download URL points to the **official Alibaba release channel**, not
   a mirror this project controls. We never re-host weights.
4. If the user declines, the sidecar enters a "no-AI" mode where L1–L3 still
   function fully; only L4 stages of the C2PA chain become unavailable.

This isolates the license matrix from Tongyi's commercial clause and removes
the project's distributors from the consent chain.

## 5. CI and reproducibility

- Primary build: `make` + system Rust/Node/Python (low barrier to entry).
- Reproducibility build: `nix develop` via `flake.nix` — **v0.1 ships
  without `flake.lock`**, so the 5-year reproducibility claim is currently
  *intended*, not *verified*. Generating the lockfile and pinning nixpkgs is
  a v0.2 task and a precondition for any honest reproducibility marketing.
- CI runner: GitHub Actions, **make-based only in v0.1**. The original
  architecture round called for "Nix-based steps in CI"; we did not implement
  Nix-in-CI in v0.1 because the bootstrap host had no Nix installed and
  building a CI on a flake we cannot ourselves run locally would have been
  the literal "tooling we describe but cannot operate" failure mode this
  project exists to refuse. Nix-in-CI moves to v0.2 alongside the lockfile.

Bus-factor (governance.md §"Bus-factor declarations") is the dominant risk;
reproducibility is the secondary risk. Both are *declared* rather than
*hidden*. The above paragraph is itself an honesty audit: it documents what
the architecture round promised and what v0.1 actually delivered.

## 6. The autonomous loop (Claude-implemented)

For internal automation (this codebase was bootstrapped autonomously):

```
edit  →  kluster_code_review_auto  →  cargo test  →  commit
   ↓
   on 3rd consecutive failure
   ↓
   R8: move WIP to experiments/_wip/<stage>/
   ↓
   R14: spawn 3 sub-agents to widen the search
```

Tier-1 actions (`gh repo create`, `git push`, `cargo publish`) are *never*
inside the loop. They are gates G1, G3, G4 and require human action.

## 7. Score deductions (honest version)

### The five axes (each scored out of 5, max total 25)

A reader has the right to know what `n/25` means before judging `22/25`. The
R14 critic round graded along five axes, defined as:

1. **General-purpose** — how broadly the design serves users beyond its primary
   audience (an archivist tool that *also* helps live-coders scores higher than
   one that needs separate binaries per audience).
2. **Practicality** — how cheap it is to install, run, and reproduce on a
   real-world machine (fewer host languages, simpler build, lower bus-factor).
3. **Durability** — how long the design can survive the legal / upstream /
   organizational drift around it (license shifts, maintainer departures,
   protocol obsolescence).
4. **Thought-fidelity** — how faithfully the artifact preserves Kittler's
   `Aufschreibesystem` perspective: medium-first, materiality-preserving,
   apparatus-surfacing (the 5-axis PR filter is its merge-time enforcement).
5. **Fabrication-resistance** — how hard it is to ship a coherent lie inside
   the artifact (CI gates, ethics audit, C2PA chain ordering, declared failure
   modes). The supervisor protocol is its bootstrap-time enforcement.

The concept-memo round closed at **23/25** (the original 4-agent R14 score).
The architecture round closed at **22/25** — a −1 drift driven by the
2-binary split (`kittler-archive` vs `kittler-stage`) traded for license
cleanliness. We record the drift explicitly: the architecture round did not
simply "execute" the concept round, it *cost* a point on the general-purpose
axis to make the license matrix honest. A reader who only sees the 22/25
figure should also know that 23/25 was the moving-from point.

### The architecture-round −3 break down as:

- −1 **General-purpose** axis. The 2-binary split (archive vs stage) trades
  some cross-pollination for license cleanliness.
- −1 **Practicality** axis. Three host languages (Rust + TypeScript + Python)
  raise the bus-factor; we declare it rather than hide it.
- −1 **Durability** axis. The Tongyi Qianwen commercial clause is mitigated
  by the user-pull model but not eliminated; future model-license changes
  could force L4 to switch base models.

Thought-fidelity and Fabrication-resistance retained full 5/5 each in v0.1.5
(after the v0.1.5 closure pass: filename-exemption hardening, SemVer enums,
SCTE-35 test, governance count regression fix all attach to fabrication).

We chose to ship with the deductions visible. That is the Kittlerian move.

## 8. What this is *not* (anti-architecture)

- Not a "single source of truth" — there is no master embedding store.
- Not a vector database — `IntoPatternAtom` is *syntactic*, not *semantic*.
- Not a moderation layer — ethics audit is a CI **fail**, not a model output
  to be soft-filtered.
- Not a hosted service — `kittler-stage` runs in *your* browser, against
  *your* signals. We provide no SaaS endpoint.
