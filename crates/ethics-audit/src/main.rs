// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors

//! ethics-audit CLI — run over a directory, exit non-zero on any finding.

use ethics_audit::{audit, Finding};
use std::path::PathBuf;
use std::{env, fs, process};

fn main() {
    let mut args = env::args().skip(1);
    let dir = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("fixtures/"));

    if !dir.exists() {
        eprintln!("ethics-audit: directory does not exist: {}", dir.display());
        process::exit(0);
    }

    let mut total = 0usize;
    let mut failing = 0usize;
    let walker = walk(&dir);

    for entry in walker {
        total += 1;
        let findings = audit(&entry);
        if !findings.is_empty() {
            failing += 1;
            for f in &findings {
                println!("REJECT {}: {}", entry.display(), describe(f));
            }
        } else {
            println!("OK     {}", entry.display());
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
    }
}

fn walk(root: &PathBuf) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.clone()];
    while let Some(p) = stack.pop() {
        let Ok(rd) = fs::read_dir(&p) else { continue };
        for e in rd.flatten() {
            let path = e.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                out.push(path);
            }
        }
    }
    out
}
