// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors

//! ethics-audit CLI — run over a directory, exit non-zero on any finding.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use ethics_audit::{audit, Finding};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{env, fs, process};

fn main() {
    let mut args = env::args().skip(1);
    let dir = args
        .next()
        .map_or_else(|| PathBuf::from("fixtures/"), PathBuf::from);

    if !dir.exists() {
        // Exit non-zero so a typo in a CI invocation surfaces rather than
        // silently passing. The OSS's own ethics policy (governance.md §5)
        // demands that audit failure modes be loud, not quiet — that
        // applies to the audit itself, not only to its findings.
        eprintln!("ethics-audit: directory does not exist: {}", dir.display());
        process::exit(2);
    }

    let mut total = 0usize;
    let mut failing = 0usize;
    let walker = walk(&dir);

    for entry in walker {
        total += 1;
        let findings = audit(&entry);
        if findings.is_empty() {
            println!("OK     {}", entry.display());
        } else {
            failing += 1;
            for f in &findings {
                println!("REJECT {}: {}", entry.display(), describe(f));
            }
        }
    }

    eprintln!("ethics-audit: {failing}/{total} files rejected");
    if failing > 0 {
        process::exit(1);
    }
}

fn describe(f: &Finding) -> String {
    match f {
        Finding::RdsPresent => "RDS PS/PTY present (broadcast id)".into(),
        Finding::Scte35Present => "SCTE-35 cue markers present".into(),
        Finding::CallsignPresent { matched } => format!("callsign hint: {matched}"),
        Finding::YearMissing => "missing recording-year metadata".into(),
        Finding::ColonialGps => "colonial-era GPS metadata".into(),
        Finding::ManifestMissing { expected_at } => {
            format!("C2PA manifest missing (expected at {expected_at})")
        }
        Finding::LanguageIdMissing => "recording-location language ID missing".into(),
        // `Finding` is `#[non_exhaustive]`; a v0.2 detector variant must be
        // labeled at the CLI even before this match arm is updated.
        _ => "unknown finding (CLI needs updating for new Finding variant)".into(),
    }
}

fn walk(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    // Symlink-loop guard: track canonical directory paths we have already
    // descended. A symlink that points back to its own parent would otherwise
    // spin until OOM.
    let mut visited_dirs: HashSet<PathBuf> = HashSet::new();
    while let Some(p) = stack.pop() {
        let Ok(rd) = fs::read_dir(&p) else { continue };
        for e in rd.flatten() {
            let path = e.path();
            let Ok(file_type) = e.file_type() else { continue };
            if file_type.is_symlink() {
                // Skip symlinks entirely — auditing through them risks both
                // loops and accidentally reaching outside the fixture tree.
                continue;
            }
            if file_type.is_dir() {
                let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
                if visited_dirs.insert(canonical) {
                    stack.push(path);
                }
            } else {
                out.push(path);
            }
        }
    }
    out
}
