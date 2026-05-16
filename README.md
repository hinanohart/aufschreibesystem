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

## Status

- **v0.1 MVP (local)** — IQ file → `Signal` trait → `IntoPatternAtom` →
  Strudel-style AST. Synthetic-only fixtures. No broadcast recordings.
- **Not yet published** — see `governance.md` for the seven gates (G1–G7)
  every release must clear, and why some of them are *deliberately* manual.

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
