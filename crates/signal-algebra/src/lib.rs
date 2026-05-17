// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors

//! # signal-algebra
//!
//! Reference implementation of the `signal-algebra` spec
//! (see `spec/signal-algebra/README.md` for the dual-licensed contract).
//!
//! ## Thought-fidelity axes touched by this crate
//!
//! - **Materiality.** `Signal::next_frame` returns raw samples; `IntoPatternAtom::to_audio`
//!   round-trips them. There is no embedding step inside this crate.
//! - **Subject illusion.** Every `Signal` impl must implement `provenance()`. There is no
//!   default that hides device origin.
//! - **Military origin.** `IqSample::from_iq` documents FFT's nuclear-test-detection origin
//!   at the use site.
//! - **Real-time-ness.** All methods are streaming; no `Vec<f32>` collect-then-process API.
//! - **Cultural-technique priority.** This crate's module docs lead with *what medium*,
//!   not *what analysis*.

#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use sha2::{Digest, Sha256};
use std::time::Duration;

pub mod pattern;
pub mod provenance;
pub mod synthetic;

pub use pattern::{EventStream, PatternAtom, PatternEvent};
pub use provenance::{OriginProtocol, ProvenanceTag};
pub use synthetic::SyntheticIq;

/// Errors returned by `Signal::next_frame`.
///
/// `#[non_exhaustive]` so v0.2 can add `Backpressure`, `Underrun`, or
/// transport-specific variants without forcing every downstream `match`
/// arm to rev a major version.
#[derive(Debug)]
#[non_exhaustive]
pub enum SignalErr {
    /// The signal reached its natural end.
    Eof,
    /// The signal could not be advanced (e.g. underlying I/O failure).
    Advance(String),
}

impl std::fmt::Display for SignalErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eof => write!(f, "end of signal"),
            Self::Advance(s) => write!(f, "signal advance failed: {s}"),
        }
    }
}

impl std::error::Error for SignalErr {}

/// A physical (or symbolic) signal whose samples can be advanced one frame at a time.
///
/// Implementations MUST return a stable `provenance()` for the lifetime of the signal:
/// the medium identity does not change mid-stream.
///
/// # Dyn-compatibility
///
/// This trait MUST remain object-safe (dyn-compatible). `IntoPatternAtom::to_audio`
/// returns a `Box<dyn Signal<Sample = f32, Time = u64>>` for the materiality
/// round-trip; adding a generic method or an associated `const` to `Signal`
/// would break that compile path. Test: `cargo test -p signal-algebra` must
/// continue to type-check `Box::new(SyntheticIq::new(...))` as `Box<dyn Signal>`.
pub trait Signal: Send + 'static {
    /// Element type of the sample buffer (`f32` for audio, `Complex32` for IQ, etc.).
    type Sample: Copy + Send;
    /// Monotonic time index; usually `u64` nanoseconds or sample count.
    type Time: Copy + Ord;

    /// Nominal sample rate. `None` for purely symbolic streams (e.g., MIDI-like events).
    fn sample_rate_hz(&self) -> Option<u64>;

    /// Advance the signal by filling `out` and returning the new monotonic time.
    ///
    /// # Errors
    /// Returns `SignalErr::Eof` at natural end-of-stream, `SignalErr::Advance` on
    /// underlying transport failure.
    fn next_frame(&mut self, out: &mut [Self::Sample]) -> Result<Self::Time, SignalErr>;

    /// Stable provenance tag identifying the medium.
    fn provenance(&self) -> ProvenanceTag;
}

/// A `Signal` that can be reinterpreted as a pattern-algebra atom.
///
/// This is the materiality-enforcing trait: `to_audio` *must* exist, so any
/// pattern can always be re-grounded back into the time-domain signal it came from.
pub trait IntoPatternAtom: Signal {
    /// One "cycle" of the pattern (TidalCycles terminology).
    fn cycle_duration(&self) -> Duration;

    /// Re-ground the pattern to a `Signal<Sample = f32>` audio stream.
    ///
    /// This method may not be implemented as `unimplemented!()`. The compile-time
    /// presence of this trait is the materiality guarantee.
    ///
    /// # Rewind contract (v0.1)
    ///
    /// The returned `Signal` **starts at `t = 0`**. Accumulated mutable state in
    /// `self` (e.g. phase, decoder cursor) is NOT carried into the audio stream;
    /// each call to `to_audio()` produces a fresh, idempotent re-grounding of
    /// the pattern as the source defines it.
    ///
    /// Rationale: in v0.1 all `IntoPatternAtom` impls are deterministic given
    /// their constructor arguments (see `SyntheticIq`), so a rewound replay
    /// matches the original. For non-deterministic L1 captures (RF / floppy
    /// flux) v0.2 will introduce a `to_audio_continuation()` variant; the
    /// rewind-vs-continuation choice will then be explicit in the type system.
    ///
    /// Callers that need two cursors over the same audio MUST call
    /// `to_audio()` twice; do NOT alias the returned `Box<dyn Signal>`.
    fn to_audio(&self) -> Box<dyn Signal<Sample = f32, Time = u64>>;

    /// Yield the pattern as discrete events for the Strudel-style AST.
    fn to_event_stream(&self) -> EventStream;
}

/// Hash a sample buffer to anchor a C2PA raw-stage manifest.
///
/// FFT, used heavily in `signal-ingest`, originated in nuclear-test-detection
/// signal processing. We acknowledge that origin here because it is unavoidable
/// in the dependency chain.
pub fn sha256_of_samples(bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(bytes);
    let out = h.finalize();
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&out);
    arr
}

/// Spec version this implementation conforms to.
pub const SPEC_VERSION: &str = "0.1.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_version_matches_workspace() {
        assert_eq!(SPEC_VERSION, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn sha256_is_deterministic() {
        let a = sha256_of_samples(b"kittler");
        let b = sha256_of_samples(b"kittler");
        assert_eq!(a, b);
        let c = sha256_of_samples(b"Kittler");
        assert_ne!(a, c);
    }
}
