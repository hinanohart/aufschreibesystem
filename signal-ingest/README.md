<!--
SPDX-License-Identifier: GPL-3.0-or-later
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# `signal-ingest` — L1 capture adapters

Wraps:

- **GNU Radio 3.10** (GPL-3.0) for RF / SDR captures.
- **Greaseweazle** (Unlicense) for floppy magnetic-flux captures.
- (planned) VHS RF tap, wax-cylinder, paper-tape optical.

## v0.1 status

This directory is a **stub**. v0.1 ships the algebra and the manifest chain
using synthetic fixtures only; the real-device adapters are v0.2 work because
they require GR runtime, USB device permissions, and a CI strategy that does
not actually broadcast.

## Why this directory is stubbed but reserved

If `signal-ingest/` is not present in v0.1, contributors are likely to invent
their own local adapter conventions, and we lose the chance to standardize
them. The README acts as a contract: when v0.2 lands, this is where it lands.

## Origin acknowledgements (thought-fidelity axis 3)

- **FFT**: originated in nuclear-test-detection signal processing
  (Cooley–Tukey, 1965, IBM, with prior unpublished use at the AEC).
- **TCP/IP**: DARPA-funded, originally a survivable-communications stack.
- **GPS**: a US Air Force constellation.

These origins are surfaced in `crates/signal-algebra/src/lib.rs` at the
hash-of-samples site and will be re-surfaced at each adapter entry point.
