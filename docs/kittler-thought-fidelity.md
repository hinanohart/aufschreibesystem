<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# Kittler thought-fidelity filter (5 axes)

Every pull request must answer these five questions. PRs whose authors
decline to answer are closed with the comment `thought-fidelity gate`.

## Axis 1 — Materiality

> *"Es gibt keine Software."*

**Does this change preserve the signal as signal?**

A change fails this axis if it:

- Converts a `Signal<Sample = f32>` to a `Vec<u8>` for storage without an
  inverse path.
- Adds an "embedding cache" that other layers consume *instead of* the
  underlying samples.
- Removes the `provenance()` method from any `Signal` impl.

A change passes if the medium remains addressable at the byte / sample level
*after* the change, and the C2PA chain can still be re-anchored to L1 bytes.

## Axis 2 — Subject illusion

> *Media constitute subjects; subjects do not constitute media.*

**Does this change reinforce the user's sense of authorial control, or
surface the apparatus producing them?**

A change fails this axis if it:

- Adds UI copy of the form "Your pattern" / "Your composition" / "Your mix"
  without acknowledging the source-medium author.
- Hides the input device fingerprint from the C2PA manifest.
- Defaults to a single-user model (no co-presence indication of the medium).

A change passes if the user can, at any UI surface, see "what medium / what
recording / what protocol is producing this," not just "what I am producing."

## Axis 3 — Military / state origin

> *FFT comes from nuclear test detection. TCP/IP comes from DARPA. GPS is
> a US Air Force constellation. The GPU was a CAD-then-defense pipeline.*

**If this code touches protocols with military or state origins, is the
origin acknowledged in docs at the import site?**

A change fails this axis if it adds a `use gnuradio::fft::*;` without a
comment or doc-link naming the protocol's origin.

A change passes if the origin appears in module docs (`//!`) at the layer
boundary, so a reader of the public API encounters it.

## Axis 4 — Real-time-ness

> *Batch processing is the symbolic order retreating from the real.*

**Does this change keep end-to-end latency under 20 ms (input → AST event),
or does it push the project toward batch processing?**

A change fails this axis if it:

- Introduces an unbounded buffer in the signal path.
- Adds a `tokio::time::sleep` longer than 10 ms in the hot path.
- Replaces a streaming API with a "collect then process" API.

A change passes if `cargo bench -p signal-algebra -- realtime` shows p95 < 20 ms.

## Axis 5 — Cultural-technique priority

> *Aufschreibesysteme are conditions of possibility for analysis, not its
> objects.*

**Is the cultural technique described before the analysis built on it?**

A change fails this axis if it:

- Adds new analysis features without doc-linking the underlying medium /
  protocol they decode.
- Frames `kittler-stage` as "music software" without acknowledging the
  archival source signals it composes.

A change passes if a reader unfamiliar with the project can, from the
public docs alone, learn the cultural-technique context *before* the API.

## Use in code review

This is not a checklist for ticking. It is a frame for arguing. When a PR
clears 5/5, the reviewer should still be able to write one sentence about
why it cleared — that sentence becomes the merge comment.
