// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors

//! # pattern — the syntactic surface a Signal can present
//!
//! The `EventStream` is the bridge to L3 (Strudel-style AST). It is
//! intentionally minimal: a sequence of `PatternEvent`s with an attached
//! provenance tag so the chain back to the source signal is never broken.

use crate::provenance::ProvenanceTag;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// One event in a pattern stream.
///
/// `value_atom` is a JSON-serializable opaque atom; the algebra does not
/// interpret it. L3 (pattern-DSL) decides how to render it.
///
/// `#[non_exhaustive]` so v0.2 can add `accent`, `velocity`, etc. without
/// SemVer-breaking every downstream struct-literal.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PatternEvent {
    /// Offset from the start of the current cycle.
    pub when_in_cycle: Duration,
    /// Duration this event occupies.
    pub duration: Duration,
    /// The opaque atom value (e.g., a frequency, a sample id, a chord symbol).
    pub value_atom: PatternAtom,
}

/// Opaque value inside a pattern event. Deliberately non-exhaustive.
///
/// `#[non_exhaustive]` is intentional and load-bearing: L3 (pattern-DSL)
/// will add atom kinds as new media-types land. SemVer-major every time
/// would break every downstream `match`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[non_exhaustive]
pub enum PatternAtom {
    /// A single frequency in Hz (e.g., extracted from an IQ tone).
    Frequency {
        /// Frequency value in hertz.
        hz: f64,
    },
    /// A symbolic sample id (e.g., decoded into a Strudel sample name).
    SampleId {
        /// Sample identifier as named by the L3 pattern-DSL.
        id: String,
    },
    /// A raw byte payload (e.g., one floppy-flux sector).
    Bytes {
        /// Raw payload bytes; interpretation is L3's responsibility.
        payload: Vec<u8>,
    },
}

/// A stream of pattern events produced from a `Signal`.
///
/// The `provenance` field is carried alongside the events so the C2PA chain
/// can re-anchor at the pattern stage without re-fetching the source.
///
/// `#[non_exhaustive]` for the same SemVer reason as [`PatternEvent`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventStream {
    /// Source provenance.
    pub provenance: ProvenanceTag,
    /// Cycle duration (TidalCycles "cycle").
    pub cycle_duration: Duration,
    /// Ordered events within one cycle.
    pub events: Vec<PatternEvent>,
}

impl EventStream {
    /// Construct an empty event stream.
    #[must_use]
    pub fn empty(provenance: ProvenanceTag, cycle_duration: Duration) -> Self {
        Self {
            provenance,
            cycle_duration,
            events: Vec::new(),
        }
    }

    /// Push an event onto the stream.
    pub fn push(&mut self, event: PatternEvent) {
        self.events.push(event);
    }

    /// Number of events in the stream.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// True if the stream has no events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provenance::ProvenanceTag;

    #[test]
    fn empty_stream_is_empty() {
        let s = EventStream::empty(ProvenanceTag::synthetic(48_000), Duration::from_secs(1));
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn pushed_events_round_trip_through_serde() {
        let mut s = EventStream::empty(ProvenanceTag::synthetic(48_000), Duration::from_secs(1));
        s.push(PatternEvent {
            when_in_cycle: Duration::from_millis(250),
            duration: Duration::from_millis(125),
            value_atom: PatternAtom::Frequency { hz: 440.0 },
        });
        let json = serde_json::to_string(&s).unwrap();
        let back: EventStream = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }
}
