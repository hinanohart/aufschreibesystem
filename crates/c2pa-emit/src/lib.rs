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

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use signal_algebra::{EventStream, ProvenanceTag};

/// One stage in the provenance chain.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "stage", rename_all = "snake_case")]
pub enum Stage {
    /// L1 — raw bytes from the medium.
    Raw {
        sha256_hex: String,
        bytes_len: usize,
        provenance: ProvenanceTag,
    },
    /// L2/L3 — pattern AST extracted from the signal.
    Pattern {
        sha256_hex: String,
        algebra_version: String,
        event_count: usize,
    },
    /// L4 — interpretation by an AI model.
    Interpretation {
        model_id: String,
        prompt_sha256_hex: String,
        synthid_present: bool,
        sidecar_version: String,
    },
}

/// A three-stage provenance manifest.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Manifest {
    pub spec: String,
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
    pub fn add_pattern(&mut self, stream: &EventStream, algebra_version: impl Into<String>) {
        let canonical = serde_json::to_vec(stream).expect("event stream serializable");
        self.stages.push(Stage::Pattern {
            sha256_hex: hex_sha256(&canonical),
            algebra_version: algebra_version.into(),
            event_count: stream.len(),
        });
    }

    /// Append the (mandatory-if-AI-was-applied) interpretation stage.
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

    /// Validate basic chain ordering: Raw must precede Pattern must precede Interpretation.
    #[must_use]
    pub fn is_well_ordered(&self) -> bool {
        let mut state = 0u8;
        for s in &self.stages {
            let level = match s {
                Stage::Raw { .. } => 1,
                Stage::Pattern { .. } => 2,
                Stage::Interpretation { .. } => 3,
            };
            if level <= state {
                return false;
            }
            state = level;
        }
        state >= 1
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
        m.add_pattern(&sig.to_event_stream(), "0.1.0");
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
        m.add_pattern(&sig.to_event_stream(), "0.1.0");
        assert!(!m.is_well_ordered());
    }

    #[test]
    fn raw_only_is_well_ordered() {
        let m = Manifest::from_raw(b"hello", ProvenanceTag::synthetic(48_000));
        assert!(m.is_well_ordered());
    }
}
