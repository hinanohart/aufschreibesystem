<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# Dropped designs

Designs considered during the R14 exploration round and rejected, with
reasons. Kept because rejection memory is the project memory most likely
to be forgotten.

## A2 — FPGA-RTOS continuum

**Idea.** seL4 + HDL (Yosys/Verilator/GHDL) + Rust `no_std` as a single
bare-metal stack for capture devices.

**Rejected because.** A single individual cannot maintain seL4 verification,
HDL toolchain integration, and no-std Rust at the level the project requires.
Bus-factor would be 0 within three years.

## A3 — Diskurs-Netzwerk replica

**Idea.** Reproduce the `arXiv:2305.16862`-style discourse-network analysis
and add IRENE-style optical audio capture.

**Rejected because.** IRENE depends on Library-of-Congress-specific hardware
and is not OSS-able. The arXiv reproducibility is unclear. The project would
need years of hardware development before any release.

## B1 — GFT 2.0 (general-purpose foundation transcription)

**Idea.** A Qwen3-Omni + Hi-ResLDM stack for general historical-recording
restoration.

**Rejected because.** License chain (Qwen3 + Hi-ResLDM) was not auditable at
proposal time. The "Whisper + RAG" core of the proposal is already a crowded
space; we would compete on margins, not on shelf-position.

## B2 — Diskurs archaeology engine

**Idea.** A literature-network tool (ResearchRabbit / Inciteful / Litmaps
genre) specialized for Kittler scholars.

**Rejected because.** Differentiation versus the existing three tools was
too thin. Building a fourth tool with the same UX shape is not "a new shelf."

## B3 — voice restoration & attribution (the rejected-on-principle one)

**Idea.** Use F5-TTS to "fill in lost syllables" of damaged wax-cylinder
recordings, then attribute the reconstruction.

**Rejected because.** This is the symbolic order erasing the real, in
Kittler's strict sense. The damaged surface of the wax *is* the recording;
re-synthesizing what "should" be there replaces the medium with its model.

We keep this entry because the temptation will recur. The proxy-interview
clause in `governance.md` is structurally adjacent and required separate
justification not to commit the same error.

## C2 — Kulturtechniken tracer

**Idea.** Combine stylo + sparse-autoencoders + ancient-script decoding
into one tool.

**Rejected because.** Three good tools wedged together does not constitute
a coherent fourth tool. The unifying motivation was thin.

## C3 — Collapse Codex

**Idea.** A "permacomputing" OSS for a post-collapse computing scenario.

**Rejected because.** Hundred Rabbits and uxn already occupy this niche
with conviction we cannot match. A second OSS here would be derivative.

## Late additions (rejected during the architecture round)

### "signal-as-effect" (Koka / OCaml 5 algebraic effects)

**Idea.** Express `Signal` as an algebraic-effect handler so that
`to_audio()` and `to_event_stream()` become effect-row handled functions.

**Status.** Recorded for v2. Not used in v0.1 because Rust does not have
first-class effects, and switching to Koka/OCaml5 loses Algorave fluency.

### Reverse mapping (Strudel pattern → synthesized IQ)

**Idea.** Allow users to author Strudel patterns and synthesize them into
"fake" IQ samples.

**Status.** Recorded for v0.2 backlog. The reverse direction is allowed
later, but not in v0.1 — the v0.1 invariant is that signals originate
from the real, not from the user. Reversing the arrow at MVP would betray
axis 1 of the thought-fidelity filter.

### MIT-licensed monorepo

**Idea.** Use MIT for the entire tree to maximize integration.

**Status.** Rejected. GNU Radio linkage forces GPL-3.0 inheritance on
`kittler-archive`. Hiding that with a top-level MIT label would be an
honest-licensing failure. The LICENSE matrix is granular by directory.

### Single binary

**Idea.** One executable, all five layers, all licenses bundled.

**Status.** Rejected because GPL-3.0 + AGPL-3.0 cannot coexist cleanly in
one process without triggering the strongest license over the whole. The
2-binary split is the −1 on the general-purpose axis; we accepted it.
