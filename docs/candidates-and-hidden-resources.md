<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# Candidate primitives & hidden resources

The concept memo (R14, 2026-05-17) graded ~25 candidate primitives across
three domains and named ~5 "easy to miss" resources. The MVP shipped a subset.
This file records what was named, what status it has at v0.1, and **why** —
so a future contributor can re-evaluate without re-doing the search.

## Media-archaeology + signal primitives

| Primitive | v0.1 status | Reason |
|---|---|---|
| Greaseweazle / FluxEngine | **adopted** (L1 stub) | Floppy-flux imaging is the cleanest non-RF "signal-as-syntax" entry; Unlicense / GPL-2 are tractable. |
| GNU Radio 3.10 | **adopted** (L1 stub) | The RF baseline; GPL-3 inheritance is honest and declared in `LICENSE`. |
| gr-satellites | **deferred to v0.2** | Genre extension of GNU Radio; adds satellite-protocol decoders we do not need for v0.1 synthetic fixtures. Listed so the v0.2 PR has a known entry point. |
| Yosys + Verilator + GHDL | **rejected (A2)** | See `docs/dropped-designs.md` §A2: single-individual HDL+seL4+no-std OSS is bus-factor zero within three years. |
| Amaranth HDL + LiteX | **rejected (A2 cohort)** | Python→FPGA pipeline; same bus-factor argument as Yosys. Recorded here individually because the memo named it separately. |
| Embassy + bare-metal Rust | **rejected (A2 cohort)** | RTOS-class scope creep for a v0.1 archivist toolkit; declared rejected so future "let's add bare-metal" PRs have a prior decision to argue against. |

## Multimodal-AI primitives

| Primitive | v0.1 status | Reason |
|---|---|---|
| Qwen3-Omni | **adopted but user-pulled** | See `docs/ARCHITECTURE.md` §4: weights never ship in the OSS distribution; user opts in to Tongyi Qianwen terms. |
| ColPali / ColQwen2.5 | **deferred to v0.2** | Manuscript-search via late-interaction embeddings; useful but orthogonal to the v0.1 signal-algebra. v0.2 candidate for `plugins/ai/` sibling. |
| LightRAG | **deferred to v0.2** | GraphRAG at ~1/4000 of the cost; promising but a *retrieval* tool, not a *signal-interpretation* tool. v0.2 evaluation. |
| Kraken + eScriptorium | **deferred to v0.2** | HTR for paper-tape / handwritten archival forms. We left L1 paper-tape adapter for v0.2, so the AI side waits with it. |
| Whisper-v3-turbo + Hi-ResLDM | **rejected as a stack (B1)** | See `docs/dropped-designs.md` §B1: Hi-ResLDM license un-auditable at proposal time; Whisper+RAG is a crowded space we would not differentiate in. |

## Cultural-techniques + critical-computing primitives

| Primitive | v0.1 status | Reason |
|---|---|---|
| TidalCycles / Strudel pattern algebra | **adopted** (L3 stub, AGPL-3.0) | Core of the signal-as-syntax framing; Strudel migration from GitHub→Codeberg (2025-06-19) is tracked in `governance.md` §"Bus-factor declarations". |
| C2PA + SynthID | **adopted** (L5) | Three-stage chain enforced in `crates/c2pa-emit/`; SynthID is a presence flag in stage 3. |
| uxn / permacomputing | **rejected (C3 cohort)** | "Collapse Codex" niche already occupied by Hundred Rabbits with conviction we cannot match. |
| Neuronpedia / EleutherAI SAE | **rejected** | Interpretability tooling for *internal* model states — orthogonal to media materiality. We do not interpret model internals; we interpret media. |
| Wenyan + esolang lineage | **rejected** | "Critical language design" is a different project. Including a Chinese-classical-syntax programming language would dilute the cultural-technique claim, not strengthen it. |

## Hidden resources (memo §"見落としやすい")

The concept memo flagged five resources as "easily missed" by anyone working
in this space. None are *adopted as dependencies*, but each is worth a
sentence so the v0.2 round has a known starting list rather than rediscovering.

| Resource | URL | What it offers | Why we noted it |
|---|---|---|---|
| Internet Archive: Kittler 1999 WOS1 lecture | https://archive.org/details/WOS1_170799_1600_KITTLER | Kittler's own talk on "science as an open-source process." | Primary source for the project's intellectual frame; `governance.md` G6 names it as the controversial proxy-interview candidate. |
| UCI Kittler Software Archive | http://hydra.humanities.uci.edu/kittler/software.html | His C / asm code (Markov-Heidegger generator, etc.). | Concrete artifacts in the spirit the project tries to honor; v0.2 may include excerpts as fixtures (PD permission required). |
| Basilisk RISC-V SoC | arXiv:2406.15107 | Yosys + OpenROAD → fabricated 130 nm CMOS; GDSII→Linux-boot is fully OSS. | Existence proof that "open hardware all the way down" is no longer aspirational; relevant to a future L1 hardware-tap chapter we did not write in v0.1. |
| MorphOS / ukubpf (TUM-DSE, CoNEXT 2025) | (search venue) | Unikraft + eBPF real-time injection. | Real-time-ness (thought-fidelity axis 4) inspiration for any future low-latency L1 path. |
| flux-analyze (kristomu) | https://github.com/kristomu/flux-analyze | FluxEngine failure-case completion (alternative floppy-flux toolchain). | Bus-factor mitigation for Greaseweazle: a second floppy-flux project exists if Greaseweazle stalls. |

## How to use this file

If you are about to add a dependency or write a new module: grep this file
first. If the primitive you are reaching for is here as "rejected," your PR
description must argue against the recorded reason — silently re-adopting
a previously-rejected primitive is the failure mode this file exists to
prevent.

If the primitive is "deferred to v0.2," your PR can simply move it from
"deferred" to "adopted" with a one-line license-matrix update.
