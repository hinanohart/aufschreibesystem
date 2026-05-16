<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# Failure modes — 5 named, mitigation status per mode

The architecture-round R14 memo (2026-05-17) enumerated five named failure
modes. v0.1.0 → v0.1.2 closed two of them by code/docs and silently dropped
the other three from the workspace. AUDIT-B in v0.1.3-local caught that
silent drop. This file is the durable record so a future contributor cannot
miss the same risks twice.

| # | Failure mode | v0.1 mitigation status | Where addressed |
|---|---|---|---|
| 1 | **Tongyi Qianwen commercial-clause contamination** | MITIGATED (structural) | `docs/ARCHITECTURE.md` §4 + `plugins/ai/README.md` — user-pull design; the OSS distribution never ships weights, so the consent chain never crosses the project's boundary. |
| 2 | **TidalCycles bus factor (single maintainer)** | DECLARED, partial mitigation | `governance.md` §"Bus-factor declarations" — we `wrap` Strudel (not fork) and pin the Codeberg upstream; the migration on 2025-06-19 is recorded. A second migration forces vendoring. No automated upstream-watch yet (v0.2). |
| 3 | **kluster.ai trial expiration** | DECLARED, degrade path live | `docs/ARCHITECTURE.md` §5 (CI section) + `docs/supervisor-protocol.md` — when kluster verification is unavailable, the autonomous loop degrades to self-review + cross-LLM check + the 3-agent supervisor protocol. The v0.1.x bootstrap actually ran under this degrade mode. |
| 4 | **WASM ⇄ Rust IPC latency > 2 ms blowing the 20 ms end-to-end budget** | DECLARED, fallback documented | `governance.md` §"Kittler thought-fidelity filter" axis 4 (real-time-ness) + `docs/kittler-thought-fidelity.md` — if the WASM bridge exceeds budget, the fallback is L3 Rust-direct, losing Algorave inflow but preserving materiality. The bench (`cargo bench -p signal-algebra -- realtime`) is v0.2 work. |
| 5 | **Ethics audit over-blocking causing archivists to disable CI** | DECLARED, escalation path | `governance.md` §"Ethics audit" + `docs/dropped-designs.md` §"Late additions" — every detector ships with a documented carve-out (e.g., README exemption, consent marker) so the answer to "the audit is too strict" is *grade the carve-out*, not *disable the audit*. Disabling the audit forfeits the project's litigation fallback. |

## Why declaring these matters

Each of these failure modes is a *known unknown* — listed in the architecture
memo, evaluated, and shipped with a mitigation that may or may not hold under
field conditions. Hiding any of them from the artifact tree would convert a
known unknown into an *unknown unknown*, which is the failure shape this OSS
exists to refuse (Kittler thought-fidelity axis 2: surface the apparatus).

## How to use this file

- Adding a new dependency? Verify it does not silently activate any of these
  modes (especially #1: a Qwen3-Omni replacement that bundles weights would
  undo years of license cleanliness).
- Reviewing a PR that touches `web-stage/` or `crates/signal-algebra/`?
  Re-check #4 (WASM IPC latency).
- Reviewing a PR that touches `crates/ethics-audit/`? Re-check #5 and ask
  whether the change *strengthens* or *weakens* the carve-out balance.
