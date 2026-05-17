<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# Kittler — Aufschreibesystem Synthesizer

> *"There is no software."* — Friedrich Kittler

A **signal-as-syntax** toolkit: SDR bursts, floppy flux, VHS RF and wax-cylinder
audio become first-class data types in a TidalCycles/Strudel-style pattern
algebra. The medium is not flattened into an embedding; it stays a signal until
the moment a human chooses to interpret it.

## Three-line use case

1. *Archivist:* point `kittler-archive` at a captured RF burst → get a Strudel-shaped pattern AST whose `provenance` field carries the device fingerprint and an unbroken C2PA chain back to the raw IQ bytes.
2. *Algorave performer:* drop that AST into `kittler-stage` and improvise live against the medium itself — the audience hears the wax cylinder and the live pattern together, not one in place of the other.
3. *AI-provenance practitioner:* if (and only if) you opt in to the user-pulled Qwen3-Omni sidecar, the C2PA chain grows a third stage — model id + prompt hash + SynthID — so an AI interpretation is *visibly* an AI interpretation.

## Why this exists (positioning)

The EU AI Act (Regulation 2024/1689) carves a research-and-cultural-heritage
clause that presumes pipelines treating archival media as analyzable signals
rather than as text-equivalents. The "OCR → embedding → RAG" stack widely
deployed for "AI for cultural heritage" *flattens* media into embeddings at
the first step, which forecloses the signal-level provenance that cultural-
heritage exemptions are written to enable. This OSS is a technical answer to
that gap: keep the medium addressable, keep the C2PA chain rooted in raw
bytes, keep AI interpretation as an optional, isolated, opt-in stage.

## Status

- **v0.1.5-local** — IQ file → `Signal` trait → `IntoPatternAtom` →
  Strudel-style AST. Synthetic-only fixtures. No broadcast recordings.
- **Local toolchain status:** `cargo test --workspace` and
  `cargo clippy -- -D warnings` are green on the maintainer's host.
- **CI status:** `.github/workflows/ci.yml` is *configured* but has never
  executed — the repository has not been pushed to GitHub (G1/G3 are
  Tier-1 human gates per `governance.md`). "CI green" will only be true
  after the first push. See `docs/release-status.md`.
- **Not yet published** — see `governance.md` for the seven gates (G1–G7)
  every release must clear, and why some of them are *deliberately* manual.
  A one-file hand-off script for the human-gated steps lives at
  `scripts/handoff-tier1-gates.sh`.

## What this is for

Four audiences (intentional cross-pollination):

1. **Archivists** at media-archaeology institutions (BnF, DNB, NDL) who need a
   pipeline that does not destroy signal-level provenance.
2. **Algorave / live-coding** practitioners who want patterns rooted in real
   physical recordings, not synthetic toy oscillators.
3. **Digital humanities** researchers studying *Aufschreibesysteme* 1800 / 1900
   / 2000 with executable artifacts.
4. **AI provenance** practitioners who need C2PA chains that begin at the raw
   IQ sample, not at the decoded text.

## What this is *not*

- Not "AI for cultural heritage" — there is no OCR → embedding → RAG pipeline
  here. That pipeline destroys the very thing media archaeology cares about.
- Not a voice / image restoration tool. Reconstructing "lost syllables" with
  generative models is, in Kittler's terms, the symbolic order washing over
  the real. See [B3 in `docs/dropped-designs.md`](docs/dropped-designs.md).
- Not a single-binary monolith. See `LICENSE` for the three-license boundary.

## Architecture

Five layers, three license zones, two end-user binaries:

```
L1 signal-ingest  (GPL-3.0)  ─┐
L2 signal-algebra (MIT/GPL)   ├─→  kittler-archive  (CLI / Docker)
L5 provenance     (MIT)      ─┘
L3 pattern-DSL    (AGPL-3.0) ───→  kittler-stage    (WASM, browser)
L4 ai-plugin      (Apache)   ───→  sidecar (separate repo, opt-in, user-pulled model)
```

See `docs/ARCHITECTURE.md` for the long form and `spec/signal-algebra/` for the
Apache-2.0 OR MIT spec that third-party UIs may target.

## Quick start (developers)

```bash
make dev          # devcontainer or local toolchain probe
make test         # cargo test --workspace
make mvp          # generate synthetic IQ → emit Strudel AST → print
```

A Nix flake (`flake.nix`) is provided for 5-year reproducibility but is **not
required**. `make` plus a system Rust + Node + Python 3 toolchain is enough.

## Contributing

Read `governance.md` first. The seven gates exist because OSS is not the
same thing as making something publicly downloadable.

## License

See `LICENSE` for the per-tree SPDX matrix.

## Provenance

- `docs/kittler-thought-fidelity.md` — the 5-axis filter every PR must clear.
- `docs/dropped-designs.md` — designs we considered and rejected, with reasons.
- `docs/no-prior-search-2026-05-17.md` — R18 evidence that this design space
  was not already occupied at project start.
