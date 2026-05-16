// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors

//! # provenance — the device fingerprint stays with the signal
//!
//! Every `Signal` carries a `ProvenanceTag`. There is no way to forget the
//! medium without re-deriving the signal from scratch.

use serde::{Deserialize, Serialize};

/// Stable identifier for the origin of a signal.
///
/// `#[non_exhaustive]` so that v0.2 adapters (paper-tape optical, VHS RF,
/// etc.) can add variants without forcing every downstream `match` arm to
/// rev.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum OriginProtocol {
    /// Software-defined radio capture.
    Rf,
    /// Floppy-disk magnetic-flux capture (e.g., Greaseweazle).
    FloppyFlux,
    /// VHS radio-frequency tap.
    VhsRf,
    /// Wax-cylinder mechanical audio.
    WaxCylinder,
    /// Paper-tape / punched-card optical capture.
    PaperTape,
    /// Synthetic: signal generated from a known seed; *not* a physical capture.
    Synthetic,
}

/// Provenance tag attached to every `Signal`.
///
/// This struct is `Copy` so that propagating it through the pipeline is free.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProvenanceTag {
    /// 32-byte device fingerprint (Ed25519 public key, or all zeros for synthetic).
    pub device_fingerprint: [u8; 32],
    /// UTC capture timestamp in nanoseconds since UNIX epoch, or 0 for synthetic.
    pub capture_ts_utc_ns: i64,
    /// Nominal sample rate of the originating medium. `None` for symbolic streams.
    pub sample_rate_hz: Option<u64>,
    /// Origin protocol.
    pub origin: OriginProtocol,
}

impl ProvenanceTag {
    /// Construct a provenance tag for a synthetic (non-physical) signal.
    ///
    /// Synthetic provenance is encoded explicitly so that downstream consumers
    /// can distinguish a fixture from a real capture without heuristics.
    #[must_use]
    pub const fn synthetic(sample_rate_hz: u64) -> Self {
        Self {
            device_fingerprint: [0u8; 32],
            capture_ts_utc_ns: 0,
            sample_rate_hz: Some(sample_rate_hz),
            origin: OriginProtocol::Synthetic,
        }
    }

    /// True if this tag describes a synthetic (non-physical) signal.
    #[must_use]
    pub const fn is_synthetic(&self) -> bool {
        matches!(self.origin, OriginProtocol::Synthetic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_tag_is_synthetic() {
        let t = ProvenanceTag::synthetic(48_000);
        assert!(t.is_synthetic());
        assert_eq!(t.device_fingerprint, [0u8; 32]);
        assert_eq!(t.capture_ts_utc_ns, 0);
        assert_eq!(t.sample_rate_hz, Some(48_000));
    }
}
