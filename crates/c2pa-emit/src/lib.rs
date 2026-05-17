// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors

//! # c2pa-emit — three-stage manifest chain
//!
//! Stage 1: raw IQ bytes SHA-256 + capture device fingerprint.
//! Stage 2: pattern AST SHA-256 + algebra version.
//! Stage 3: model id + prompt + SynthID + sidecar version. **Required** when an
//!          AI step was applied; omitting it makes a chain non-conformant.
//!
//! This crate intentionally does NOT depend on the full c2pa-rs SDK in v0.1;
//! it emits a manifest-shaped JSON document so the test surface is small. The
//! c2pa-rs integration happens in v0.2.

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use signal_algebra::{EventStream, ProvenanceTag};

/// One stage in the provenance chain.
///
/// `#[non_exhaustive]` so that adding a fourth stage in v0.2 (e.g., a
/// distribution / re-publication stage) is a minor-version change, not
/// a SemVer-major break for every downstream `match`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "stage", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Stage {
    /// L1 — raw bytes from the medium.
    Raw {
        /// SHA-256 of the raw bytes, hex-lowercase.
        sha256_hex: String,
        /// Number of raw bytes hashed.
        bytes_len: usize,
        /// Provenance tag carried from the source `Signal`.
        provenance: ProvenanceTag,
    },
    /// L2/L3 — pattern AST extracted from the signal.
    Pattern {
        /// SHA-256 of the canonical-serialized `EventStream`.
        sha256_hex: String,
        /// `signal-algebra` SPEC_VERSION at the time of emission.
        algebra_version: String,
        /// Number of events in the stream (denormalized for quick audit).
        event_count: usize,
    },
    /// L4 — interpretation by an AI model.
    Interpretation {
        /// Model identifier (e.g., `qwen3-omni:7b`).
        model_id: String,
        /// SHA-256 of the prompt sent to the model.
        prompt_sha256_hex: String,
        /// Whether the model output carries a verifiable SynthID watermark.
        synthid_present: bool,
        /// Sidecar release version.
        sidecar_version: String,
    },
}

/// A three-stage provenance manifest.
///
/// `#[non_exhaustive]` so adding a `signature` / `attestation` field in v0.2
/// is a minor-version bump rather than a SemVer-major break for every
/// downstream pattern-match or struct-literal construction.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub struct Manifest {
    /// Schema identifier, e.g., `kittler-c2pa-shape/0.1.0`.
    pub spec: String,
    /// Ordered chain stages.
    pub stages: Vec<Stage>,
}

impl Manifest {
    /// Build a stage-1 manifest from raw bytes.
    #[must_use]
    pub fn from_raw(bytes: &[u8], provenance: ProvenanceTag) -> Self {
        Self {
            spec: "kittler-c2pa-shape/0.1.0".into(),
            stages: vec![Stage::Raw {
                sha256_hex: hex_sha256(bytes),
                bytes_len: bytes.len(),
                provenance,
            }],
        }
    }

    /// Append the pattern stage.
    ///
    /// Returns the serde error rather than panicking, because the `EventStream`
    /// hash is part of the C2PA chain and a silent fallback to an empty buffer
    /// would write a manifest claiming a stage that cannot be reconstructed.
    pub fn add_pattern(
        &mut self,
        stream: &EventStream,
        algebra_version: impl Into<String>,
    ) -> Result<(), serde_json::Error> {
        let canonical = serde_json::to_vec(stream)?;
        self.stages.push(Stage::Pattern {
            sha256_hex: hex_sha256(&canonical),
            algebra_version: algebra_version.into(),
            event_count: stream.len(),
        });
        Ok(())
    }

    /// Append the (mandatory-if-AI-was-applied) interpretation stage.
    ///
    /// Infallible — the prompt is passed pre-bytes and the model identifier
    /// is `String`-shaped, so no serialization is performed at append time.
    /// Compare with [`Manifest::add_pattern`], which serializes an
    /// `EventStream` and returns the serde error.
    pub fn add_interpretation(
        &mut self,
        model_id: impl Into<String>,
        prompt: &[u8],
        synthid_present: bool,
        sidecar_version: impl Into<String>,
    ) {
        self.stages.push(Stage::Interpretation {
            model_id: model_id.into(),
            prompt_sha256_hex: hex_sha256(prompt),
            synthid_present,
            sidecar_version: sidecar_version.into(),
        });
    }

    /// Validate chain ordering: stages MUST appear in the exact sequence
    /// Raw (1) → Pattern (2) → Interpretation (3), with **no stage skipped**.
    ///
    /// Skipping the Pattern stage (Raw → Interpretation directly) is rejected:
    /// the materiality guarantee depends on every AI interpretation being
    /// re-anchorable to a pattern-stage hash, which in turn re-anchors to
    /// raw bytes. A chain that jumps Raw → Interpretation breaks that
    /// re-anchoring and is therefore non-conformant.
    ///
    /// A `Raw`-only chain (single stage) is well-ordered.
    /// A `Raw → Pattern` chain (two stages) is well-ordered.
    /// A `Raw → Pattern → Interpretation` chain is well-ordered.
    /// Anything else is rejected.
    #[must_use]
    pub fn is_well_ordered(&self) -> bool {
        let mut next_required = 1u8;
        for s in &self.stages {
            let level = match s {
                Stage::Raw { .. } => 1,
                Stage::Pattern { .. } => 2,
                Stage::Interpretation { .. } => 3,
            };
            if level != next_required {
                return false;
            }
            next_required = level + 1;
        }
        // At least one stage is required.
        next_required > 1
    }
}

fn hex_sha256(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    let digest = h.finalize();
    let mut s = String::with_capacity(64);
    for b in digest {
        use std::fmt::Write as _;
        let _ = write!(s, "{b:02x}");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_algebra::{IntoPatternAtom, SyntheticIq};
    use std::time::Duration;

    #[test]
    fn three_stage_chain_is_well_ordered() {
        let mut m = Manifest::from_raw(b"hello", ProvenanceTag::synthetic(48_000));
        let sig = SyntheticIq::new(48_000, 440.0, Duration::from_secs(1));
        m.add_pattern(&sig.to_event_stream(), "0.1.0").unwrap();
        m.add_interpretation("qwen3-omni:7b", b"describe this signal", true, "0.1.0");
        assert!(m.is_well_ordered());
        assert_eq!(m.stages.len(), 3);
    }

    #[test]
    fn out_of_order_chain_is_rejected() {
        let mut m = Manifest::from_raw(b"hello", ProvenanceTag::synthetic(48_000));
        m.add_interpretation("qwen3-omni:7b", b"prompt", false, "0.1.0");
        // Adding pattern after interpretation is illegal.
        let sig = SyntheticIq::new(48_000, 440.0, Duration::from_secs(1));
        m.add_pattern(&sig.to_event_stream(), "0.1.0").unwrap();
        assert!(!m.is_well_ordered());
    }

    #[test]
    fn raw_only_is_well_ordered() {
        let m = Manifest::from_raw(b"hello", ProvenanceTag::synthetic(48_000));
        assert!(m.is_well_ordered());
    }

    #[test]
    fn raw_then_interpretation_skipping_pattern_is_rejected() {
        // Materiality requires every interpretation to be re-anchorable to a
        // pattern hash, which re-anchors to raw bytes. Skipping pattern breaks
        // re-anchoring and must be rejected.
        let mut m = Manifest::from_raw(b"hello", ProvenanceTag::synthetic(48_000));
        m.add_interpretation("qwen3-omni:7b", b"prompt", true, "0.1.0");
        assert!(!m.is_well_ordered());
    }

    #[test]
    fn pattern_without_raw_is_rejected() {
        let mut m = Manifest {
            spec: "kittler-c2pa-shape/0.1.0".into(),
            stages: vec![],
        };
        let sig = SyntheticIq::new(48_000, 440.0, Duration::from_secs(1));
        m.add_pattern(&sig.to_event_stream(), "0.1.0").unwrap();
        assert!(!m.is_well_ordered());
    }
}
