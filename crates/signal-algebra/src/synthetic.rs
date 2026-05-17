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
    /// Pre-computed phase advance per emitted sample, cached in `new()` so
    /// the hot loop avoids a `2π * f / fs` divide per call.
    step: f64,
    samples_emitted: u64,
    total_samples: u64,
}

impl SyntheticIq {
    /// Construct a synthetic IQ source.
    ///
    /// # Panics
    /// - `sample_rate_hz` is 0
    /// - `duration` is zero
    /// - `duration.as_millis()` does not fit in `u64` (~584 million years;
    ///   practically unreachable but checked rather than silently truncated)
    /// - `sample_rate_hz * duration_ms` would overflow `u64` (saturating mul
    ///   would silently produce a truncated sample count, which we refuse)
    /// - `|frequency_hz|` ≥ Nyquist (`sample_rate_hz / 2`); without this the
    ///   per-sample phase advance can exceed `TAU` and the wrap branch in
    ///   `next_frame` would silently corrupt the tone (v0.1.5 M4 fix from
    ///   the omc code review)
    #[must_use]
    pub fn new(sample_rate_hz: u64, frequency_hz: f64, duration: Duration) -> Self {
        assert!(sample_rate_hz > 0, "sample rate must be > 0");
        assert!(!duration.is_zero(), "duration must be > 0");
        assert!(
            frequency_hz.is_finite(),
            "frequency must be finite (got {frequency_hz})"
        );
        // Nyquist check: cast u64→f64 is lossless for sample_rate_hz < 2^53,
        // which covers all real sample-rates.
        #[allow(clippy::cast_precision_loss)]
        let nyquist_hz = (sample_rate_hz as f64) / 2.0;
        assert!(
            frequency_hz.abs() < nyquist_hz,
            "frequency {frequency_hz} Hz exceeds Nyquist ({nyquist_hz} Hz) for sample rate {sample_rate_hz}"
        );
        let duration_ms_u128 = duration.as_millis();
        let duration_ms = u64::try_from(duration_ms_u128)
            .unwrap_or_else(|_| panic!("duration too large for u64 milliseconds"));
        // Refuse silent saturating-mul truncation: if rate*ms overflows u64,
        // the sample count would be wrong and the C2PA stage-1 byte_len would
        // be a lie. v0.1.4 used saturating_mul; v0.1.5 makes overflow loud.
        let total = sample_rate_hz
            .checked_mul(duration_ms)
            .unwrap_or_else(|| panic!("sample_rate_hz * duration_ms overflows u64"))
            / 1_000;
        #[allow(clippy::cast_precision_loss)]
        let step = 2.0 * std::f64::consts::PI * frequency_hz / (sample_rate_hz as f64);
        Self {
            sample_rate_hz,
            frequency_hz,
            duration,
            phase: 0.0,
            step,
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
        // u64→usize truncation: legal because we cap at out.len() (usize) below
        // and a buffer larger than usize::MAX cannot exist. On 32-bit targets a
        // remaining count above u32::MAX truncates to a smaller chunk, but the
        // next call resumes from where this one stopped — correctness, not
        // throughput, is the invariant.
        #[allow(clippy::cast_possible_truncation)]
        let to_emit = out.len().min(self.remaining() as usize);
        for slot in out.iter_mut().take(to_emit) {
            // f64→f32 narrowing: deliberate. Signal::Sample = f32 by contract;
            // the f64 phase is precision insurance against drift over long runs.
            #[allow(clippy::cast_possible_truncation)]
            {
                *slot = self.phase.sin() as f32;
            }
            self.phase += self.step;
        }
        // v0.1.5 M4 fix: collapse accumulated phase via rem_euclid so a step
        // approaching TAU (only possible if Nyquist guard is bypassed in tests)
        // is normalized in one operation rather than a single-shot subtract.
        self.phase = self.phase.rem_euclid(std::f64::consts::TAU);
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
        //
        // CONTRACT: the returned signal starts at t=0 — accumulated phase
        // from `self` is NOT carried over. This is intentional for v0.1
        // because the synthetic source is deterministic given (rate, freq,
        // duration). For non-deterministic L1 captures (RF / floppy flux),
        // v0.2 will need a continuation-vs-rewind decision in the spec.
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
    use proptest::prelude::*;

    proptest! {
        /// Property: total samples emitted equal `sample_rate_hz * duration_ms / 1000`
        /// for any reasonable (rate, duration) combination, regardless of the chosen
        /// per-call buffer size.
        #[test]
        fn property_total_samples_match_rate_times_duration(
            sample_rate_hz in 8_000u64..=192_000u64,
            duration_ms in 1u64..=2_000u64,
            buf_size in 1usize..=4_096usize,
        ) {
            let mut sig = SyntheticIq::new(sample_rate_hz, 440.0, Duration::from_millis(duration_ms));
            let expected = sample_rate_hz * duration_ms / 1_000;
            let mut buf = vec![0.0f32; buf_size];
            let mut total: u64 = 0;
            loop {
                let before = sig.remaining();
                match sig.next_frame(&mut buf) {
                    Ok(_) => total += before - sig.remaining(),
                    Err(SignalErr::Eof) => break,
                    Err(_) => prop_assert!(false, "unexpected advance error"),
                }
                if sig.remaining() == 0 {
                    break;
                }
            }
            prop_assert_eq!(total, expected);
        }
    }

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
