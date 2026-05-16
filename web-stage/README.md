<!--
SPDX-License-Identifier: AGPL-3.0-or-later
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# `kittler-stage` — pattern-DSL (L3)

WASM-hosted, browser-side **wrap** (not fork) of Strudel.

This layer is intentionally a thin skin over Strudel, plus an FFI surface that
accepts `EventStream` JSON emitted by `signal-algebra`. AGPL-3.0 is the
correct license for a venue-facing live-coding environment: if anyone hosts
a public `kittler-stage`, they must publish their modifications.

## v0.1 status

This directory is a **stub**. The MVP pipeline in v0.1 emits an `EventStream`
JSON that *would* be consumed by Strudel; the browser integration is
deliberately deferred to a small follow-up PR so that the v0.1 tag is the
algebra plus the chain plus the audit, all green, before browser UX work.

## v0.2 plan

- Vite + esbuild scaffold.
- Strudel wrapped (not forked) via the Codeberg upstream
  (`https://codeberg.org/uzu/strudel`).
- `EventStream` JSON → Strudel pattern surface.
- "Replay the wax-cylinder signal *while* the pattern plays" — the live-coding
  surface and the source signal must be audible together; muting the source is
  a thought-fidelity violation (axis 1, materiality).

## Bus-factor note

Strudel moved from GitHub to Codeberg on 2025-06-19. Our wrap pins the
Codeberg commit; a second migration would force us to vendor.
