<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# `bins/` — end-user binaries (v0.2 work, intentionally absent in v0.1)

The architecture (`docs/ARCHITECTURE.md` §3) names two end-user binaries:

| Binary             | Layers used | License            | Audience              |
|--------------------|-------------|--------------------|-----------------------|
| `kittler-archive`  | L1+L2+L5    | GPL-3.0-or-later   | Archivists / DH       |
| `kittler-stage`    | L3+L5       | AGPL-3.0-or-later  | Algorave / live-coder |

**Neither is present in v0.1.** v0.1 ships the algebra (`crates/signal-algebra`),
the manifest chain (`crates/c2pa-emit`) and the audit (`crates/ethics-audit`)
as a library skeleton plus a runnable example
(`cargo run -p signal-algebra --example iq_to_pattern`). The two binaries above
become real entry points in v0.2 when L1 (`signal-ingest`) and L3 (`web-stage`)
land. Until then, the architecture's "two end-user deliverables" framing is
*intent*, not *artifact* — this README exists so that intent and artifact do
not silently diverge.

## Why this stub README exists

A previous audit (MONITOR-2 in `docs/supervisor-protocol.md`) caught that
`LICENSE` referenced `bins/kittler-archive/` without a matching directory or
README. That class of drift — documentation describing artifacts that do not
exist — is exactly what `governance.md` §"Kittler thought-fidelity filter" axis 1
(materiality) is meant to refuse. Adding this stub closes the gap honestly:
the architecture *names* these binaries, the workspace *acknowledges* that they
are not built yet, and `LICENSE` *describes* the license each will inherit when
they are built.

## When to delete this README

Delete this file when both `bins/kittler-archive/` and `bins/kittler-stage/`
are real crates with passing tests and at least one end-to-end smoke test
demonstrating an audible / inspectable artifact emitted by each.
