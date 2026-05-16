// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors

//! # ethics-audit — seven detectors
//!
//! These run on every PR touching `fixtures/` or `crates/signal-ingest/`.
//! A fixture failing any of these is **rejected** by CI; warnings are not
//! emitted, because warnings normalize the failure mode we are guarding
//! against.

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::path::Path;

/// The seven audit findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Finding {
    /// RDS PS/PTY bytes detected in IQ payload header.
    RdsPresent,
    /// SCTE-35 commercial-cue markers found.
    Scte35Present,
    /// Broadcast callsign string detected in metadata.
    CallsignPresent {
        /// The callsign token matched (e.g., `"wnyc"`, `"broadcast-token"`).
        matched: String,
    },
    /// Missing or implausible recording-year metadata.
    YearMissing,
    /// GPS metadata in a colonial-era location (1492–1975 lat/long box).
    ColonialGps,
    /// C2PA manifest file is missing for a derived artifact.
    ManifestMissing {
        /// Filesystem path where the manifest was expected to live.
        expected_at: String,
    },
    /// Recording-location language ID not declared.
    LanguageIdMissing,
}

/// Audit one fixture path. Returns the list of findings (empty == pass).
///
/// **v0.1 is a filename-scan stub for all seven detectors.** Detectors 1–3
/// (RDS / SCTE-35 / callsign) use real CI-blocking heuristics; detectors 4–7
/// (year / colonial GPS / manifest / language) use minimum filename heuristics
/// so that the *shape* of the audit can actually reject a fixture — making the
/// thought-fidelity guarantee a CI fact rather than a doc promise. Real decoders
/// land in v0.2 (see `governance.md` §"Ethics audit").
#[must_use]
pub fn audit(path: &Path) -> Vec<Finding> {
    let mut out = Vec::new();
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let path_str = path.to_string_lossy().to_ascii_lowercase();

    // Detector 1 — RDS broadcast identification
    if name.contains("rds") {
        out.push(Finding::RdsPresent);
    }
    // Detector 2 — SCTE-35 commercial-cue
    if name.contains("scte35") || name.contains("scte-35") {
        out.push(Finding::Scte35Present);
    }
    // Detector 3 — broadcast callsign
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

    // Detector 4 — recording-year must appear as an ISO-shaped 4-digit token
    // somewhere in the filename. Synthetic fixtures should always include
    // `synthetic` in their name and are exempted.
    let is_synthetic_name = name.contains("synthetic") || name.contains("readme");
    if !is_synthetic_name && !has_iso_year_token(&name) {
        out.push(Finding::YearMissing);
    }

    // Detector 5 — colonial-context heuristic: filename contains a token from
    // a small curated list of colonial-era place names without a "consent"
    // marker. v0.2 will replace this with GPS metadata extraction.
    let colonial_tokens = [
        "brazzaville",
        "leopoldville",
        "saigon",
        "batavia",
        "rhodesia",
        "tanganyika",
        "indochine",
        "indochina",
    ];
    let has_colonial = colonial_tokens.iter().any(|t| name.contains(t));
    let has_consent_marker = name.contains("consent")
        || path_str.contains("/community-approved/")
        || name.contains("community-approved");
    if has_colonial && !has_consent_marker {
        out.push(Finding::ColonialGps);
    }

    // Detector 6 — C2PA manifest must accompany any derived artifact
    // (filenames ending in .ast.json / .derived.json / .interp.json).
    let derived_exts = [".ast.json", ".derived.json", ".interp.json"];
    if let Some(matched_ext) = derived_exts.iter().find(|ext| name.ends_with(*ext)) {
        // Strip the full compound extension before appending .c2pa so the
        // manifest sits beside the artifact as "<stem>.c2pa", not as
        // "<stem>.ast.c2pa" (which `Path::with_extension` would produce).
        let path_str_full = path.to_string_lossy();
        let stem_len = path_str_full.len() - matched_ext.len();
        let manifest_path_str = format!("{}.c2pa", &path_str_full[..stem_len]);
        let manifest_path = Path::new(&manifest_path_str);
        if !manifest_path.exists() {
            out.push(Finding::ManifestMissing {
                expected_at: manifest_path_str.clone(),
            });
        }
    }

    // Detector 7 — recording-location language ID must be in the filename for
    // non-synthetic captures, as an ISO 639-3 three-letter token after `lang-`.
    if !is_synthetic_name && !name.contains("lang-") {
        out.push(Finding::LanguageIdMissing);
    }

    out
}

/// Returns true if the filename contains a plausible 4-digit ISO-year token
/// (1800–2099) bounded by non-digit characters.
fn has_iso_year_token(name: &str) -> bool {
    let bytes = name.as_bytes();
    for i in 0..bytes.len().saturating_sub(3) {
        let win = &bytes[i..i + 4];
        if win.iter().all(u8::is_ascii_digit) {
            let before_ok = i == 0 || !bytes[i - 1].is_ascii_digit();
            let after_ok = i + 4 == bytes.len() || !bytes[i + 4].is_ascii_digit();
            if before_ok && after_ok {
                let year: u32 = std::str::from_utf8(win)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                if (1800..=2099).contains(&year) {
                    return true;
                }
            }
        }
    }
    false
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

    #[test]
    fn colonial_recording_without_consent_is_rejected() {
        // The exact failure mode MONITOR-1 flagged as the audit's thought-fidelity
        // test case: a recording from a colonial-era place name with no consent marker.
        let p = PathBuf::from("fixtures/brazzaville_field_recording_1923_lang-fra.iq");
        let findings = audit(&p);
        assert!(
            findings.contains(&Finding::ColonialGps),
            "colonial-context fixture should be rejected without consent marker; got {findings:?}"
        );
    }

    #[test]
    fn colonial_recording_with_consent_marker_passes_colonial_detector() {
        let p = PathBuf::from(
            "fixtures/community-approved/brazzaville_field_recording_1923_lang-fra.iq",
        );
        let findings = audit(&p);
        assert!(
            !findings.contains(&Finding::ColonialGps),
            "consent-marked fixture should clear the colonial detector; got {findings:?}"
        );
    }

    #[test]
    fn missing_year_token_is_rejected() {
        let p = PathBuf::from("fixtures/some_capture_unknown_date_lang-eng.iq");
        let findings = audit(&p);
        assert!(
            findings.contains(&Finding::YearMissing),
            "fixture without ISO-year token should be rejected; got {findings:?}"
        );
    }

    #[test]
    fn missing_language_id_is_rejected() {
        let p = PathBuf::from("fixtures/some_capture_1923.iq");
        let findings = audit(&p);
        assert!(
            findings.contains(&Finding::LanguageIdMissing),
            "fixture without lang- token should be rejected; got {findings:?}"
        );
    }

    #[test]
    fn derived_artifact_without_manifest_is_rejected() {
        // We point at a non-existing .ast.json file: the manifest cannot exist either,
        // so the detector must fire. (Synthetic-name exemption is intentionally NOT
        // applied to derived artifacts — derivations from synthetic sources still
        // need a manifest stage 2.)
        let p = PathBuf::from("/tmp/this-derived-file-does-not-exist.ast.json");
        let findings = audit(&p);
        assert!(
            findings.contains(&Finding::ManifestMissing {
                expected_at: "/tmp/this-derived-file-does-not-exist.c2pa".to_string()
            }),
            "derived artifact without manifest should be rejected; got {findings:?}"
        );
    }
}
