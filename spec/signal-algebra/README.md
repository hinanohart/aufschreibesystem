<!--
SPDX-License-Identifier: Apache-2.0 OR MIT
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# `signal-algebra` — specification

This directory contains the **dual-licensed** specification of the
signal-algebra trait surface. Third-party UIs (a Pure Data binding, a future
audio-editor plugin, a non-WASM live-coding host) may target this spec
without inheriting GPL.

The Rust reference implementation in `crates/signal-algebra/` is licensed
MIT and is functionally equivalent to this spec at the same version.

## Core types (informative summary)

```
type ProvenanceTag
  device_fingerprint : 32 bytes (Ed25519 device pubkey, or zero for synthetic)
  capture_ts_utc     : i64 nanoseconds since UNIX epoch (or 0 for synthetic)
  sample_rate_hz     : u64 (Some(...) for time-domain, None for symbolic)
  origin_protocol    : enum { RF, FloppyFlux, VhsRf, WaxCylinder, Synthetic, … }

trait Signal
  type Sample : Copy + Send
  type Time   : Copy + Ord
  fn sample_rate_hz() -> Option<u64>       // method, not associated const, so the
                                            // trait stays dyn-compatible (a Box<dyn Signal>
                                            // is the materiality round-trip target)
  fn next_frame(out: &mut [Sample]) -> Result<Time, SignalErr>
  fn provenance() -> ProvenanceTag

trait IntoPatternAtom : Signal
  fn cycle_duration()  -> Duration
  fn to_audio()        -> Box<dyn Signal<Sample = f32>>       // materiality enforced
  fn to_event_stream() -> EventStream                          // for L3 pattern AST
```

## Versioning

The spec uses semver. Breaking changes require a major bump and a
written rationale in `docs/spec-changes/<version>.md`.

## Why dual-license?

- **Apache-2.0** for third-party closed-source integrations.
- **MIT** for permissively-licensed adapters.
- The reference implementation is **MIT-only** because the spec is the
  contract; the implementation is one valid contract bearer among many.
