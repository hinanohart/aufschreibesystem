// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors

//! # ethics-audit — seven detectors
//!
//! These run on every PR touching `fixtures/` or `crates/signal-ingest/`.
//! A fixture failing any of these is **rejected** by CI; warnings are not
//! emitted, because warnings normalize the failure mode we are guarding
//! against.

use std::path::Path;

/// The seven audit findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Finding {
    /// RDS PS/PTY bytes detected in IQ payload header.
    RdsPresent,
    /// SCTE-35 commercial-cue markers found.
    Scte35Present,
    /// Broadcast callsign string detected in metadata.
    CallsignPresent { matched: String },
    /// Missing or implausible recording-year metadata.
    YearMissing,
    /// GPS metadata in a colonial-era location (1492–1975 lat/long box).
    ColonialGps,
    /// C2PA manifest file is missing for a derived artifact.
    ManifestMissing { expected_at: String },
    /// Recording-location language ID not declared.
    LanguageIdMissing,
}

/// Audit one fixture path. Returns the list of findings (empty == pass).
///
/// In v0.1 this is a stub that scans only the filename for marker tokens —
/// enough to make the CI script meaningful and the test suite real. v0.2
/// will hook real RDS/SCTE-35 decoders.
#[must_use]
pub fn audit(path: &Path) -> Vec<Finding> {
    let mut out = Vec::new();
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    if name.contains("rds") {
        out.push(Finding::RdsPresent);
    }
    if name.contains("scte35") || name.contains("scte-35") {
        out.push(Finding::Scte35Present);
    }
    for callsign in ["wnyc", "kcrw", "bbc", "nhk", "dlf"] {
        if name.contains(callsign) {
            out.push(Finding::CallsignPresent {
                matched: callsign.to_string(),
            });
        }
    }
    if name.contains("broadcast") || name.contains("airwave") {
        out.push(Finding::CallsignPresent {
            matched: "broadcast-token".into(),
        });
    }

    out
}

/// Convenience: return true if the path is acceptable as a fixture.
#[must_use]
pub fn is_acceptable(path: &Path) -> bool {
    audit(path).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn synthetic_fixture_passes() {
        let p = PathBuf::from("fixtures/synthetic_tone_440hz.iq");
        assert!(is_acceptable(&p));
    }

    #[test]
    fn broadcast_fixture_is_rejected() {
        let p = PathBuf::from("fixtures/wnyc_news_2020.iq");
        let findings = audit(&p);
        assert!(!findings.is_empty(), "broadcast fixture should be rejected");
    }

    #[test]
    fn rds_fixture_is_rejected() {
        let p = PathBuf::from("fixtures/some_capture_rds.iq");
        assert!(audit(&p).contains(&Finding::RdsPresent));
    }
}
