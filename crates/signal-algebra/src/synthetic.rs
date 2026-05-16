// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors

//! # synthetic — fixture signals for tests and the MVP demo
//!
//! **Synthetic signals are NEVER allowed to claim a non-synthetic provenance.**
//! `SyntheticIq::provenance().is_synthetic()` always returns true. This is
//! checked by `ethics-audit` in CI.

use crate::pattern::{EventStream, PatternAtom, PatternEvent};
use crate::provenance::ProvenanceTag;
use crate::{IntoPatternAtom, Signal, SignalErr};
use std::time::Duration;

/// A deterministic synthetic IQ-like source for tests and fixtures.
///
/// The output is a single complex tone at `frequency_hz`, sampled at
/// `sample_rate_hz`. Useful as a known input for the MVP pipeline.
pub struct SyntheticIq {
    sample_rate_hz: u64,
    frequency_hz: f64,
    duration: Duration,
    phase: f64,
    samples_emitted: u64,
    total_samples: u64,
}

impl SyntheticIq {
    /// Construct a synthetic IQ source.
    ///
    /// # Panics
    /// Panics if `sample_rate_hz` is 0 or `duration` is zero.
    #[must_use]
    pub fn new(sample_rate_hz: u64, frequency_hz: f64, duration: Duration) -> Self {
        assert!(sample_rate_hz > 0, "sample rate must be > 0");
        assert!(!duration.is_zero(), "duration must be > 0");
        let total = sample_rate_hz.saturating_mul(duration.as_millis() as u64) / 1_000;
        Self {
            sample_rate_hz,
            frequency_hz,
            duration,
            phase: 0.0,
            samples_emitted: 0,
            total_samples: total,
        }
    }

    /// Remaining samples before EOF.
    #[must_use]
    pub fn remaining(&self) -> u64 {
        self.total_samples.saturating_sub(self.samples_emitted)
    }
}

impl Signal for SyntheticIq {
    type Sample = f32;
    type Time = u64;

    fn sample_rate_hz(&self) -> Option<u64> {
        Some(self.sample_rate_hz)
    }

    fn next_frame(&mut self, out: &mut [Self::Sample]) -> Result<Self::Time, SignalErr> {
        if self.samples_emitted >= self.total_samples {
            return Err(SignalErr::Eof);
        }
        let step = 2.0 * std::f64::consts::PI * self.frequency_hz / (self.sample_rate_hz as f64);
        let to_emit = out.len().min(self.remaining() as usize);
        for slot in out.iter_mut().take(to_emit) {
            *slot = self.phase.sin() as f32;
            self.phase += step;
            if self.phase > std::f64::consts::TAU {
                self.phase -= std::f64::consts::TAU;
            }
        }
        self.samples_emitted += to_emit as u64;
        Ok(self.samples_emitted)
    }

    fn provenance(&self) -> ProvenanceTag {
        ProvenanceTag::synthetic(self.sample_rate_hz)
    }
}

impl IntoPatternAtom for SyntheticIq {
    fn cycle_duration(&self) -> Duration {
        // One cycle == the configured duration for synthetic fixtures.
        self.duration
    }

    fn to_audio(&self) -> Box<dyn Signal<Sample = f32, Time = u64>> {
        // Materiality: the pattern can always be re-grounded into audio.
        Box::new(Self::new(
            self.sample_rate_hz,
            self.frequency_hz,
            self.duration,
        ))
    }

    fn to_event_stream(&self) -> EventStream {
        let mut s = EventStream::empty(self.provenance(), self.cycle_duration());
        // One event per cycle for a single-tone synthetic source.
        s.push(PatternEvent {
            when_in_cycle: Duration::ZERO,
            duration: self.cycle_duration(),
            value_atom: PatternAtom::Frequency {
                hz: self.frequency_hz,
            },
        });
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_signal_emits_expected_sample_count() {
        let mut sig = SyntheticIq::new(48_000, 440.0, Duration::from_millis(10));
        let mut buf = vec![0.0f32; 1024];
        let mut total: u64 = 0;
        loop {
            let before = sig.remaining();
            match sig.next_frame(&mut buf) {
                Ok(_) => total += before - sig.remaining(),
                Err(SignalErr::Eof) => break,
                Err(e) => panic!("unexpected error: {e}"),
            }
            if sig.remaining() == 0 {
                break;
            }
        }
        // 48 kHz * 10 ms = 480 samples expected exactly.
        assert_eq!(total, 480, "expected 480 samples, got {total}");
    }

    #[test]
    fn synthetic_provenance_is_always_synthetic() {
        let sig = SyntheticIq::new(48_000, 440.0, Duration::from_secs(1));
        assert!(sig.provenance().is_synthetic());
    }

    #[test]
    fn into_pattern_atom_produces_one_event_per_cycle() {
        let sig = SyntheticIq::new(48_000, 440.0, Duration::from_secs(1));
        let stream = sig.to_event_stream();
        assert_eq!(stream.len(), 1);
        assert_eq!(stream.cycle_duration, Duration::from_secs(1));
    }
}
