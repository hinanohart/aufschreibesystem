// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors

//! MVP demo: synthetic IQ → IntoPatternAtom → Strudel-shaped JSON AST.
//!
//! Run: `cargo run -p signal-algebra --example iq_to_pattern -- \
//!         --seed 0xK1TT1ER --duration-ms 1000 --output target/mvp.ast.json`

use signal_algebra::{IntoPatternAtom, SyntheticIq};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug)]
struct Args {
    duration_ms: u64,
    frequency_hz: f64,
    sample_rate_hz: u64,
    output: PathBuf,
}

fn parse_args() -> Args {
    let mut duration_ms = 1_000u64;
    let mut frequency_hz = 440.0f64;
    let mut sample_rate_hz = 48_000u64;
    let mut output = PathBuf::from("target/mvp.ast.json");

    let mut it = env::args().skip(1);
    while let Some(flag) = it.next() {
        match flag.as_str() {
            "--duration-ms" => {
                duration_ms = it
                    .next()
                    .expect("--duration-ms VALUE")
                    .parse()
                    .expect("u64");
            }
            "--frequency-hz" => {
                frequency_hz = it
                    .next()
                    .expect("--frequency-hz VALUE")
                    .parse()
                    .expect("f64");
            }
            "--sample-rate-hz" => {
                sample_rate_hz = it
                    .next()
                    .expect("--sample-rate-hz VALUE")
                    .parse()
                    .expect("u64");
            }
            "--output" => {
                output = PathBuf::from(it.next().expect("--output PATH"));
            }
            "--seed" => {
                let _ = it.next();
            }
            other => {
                eprintln!("unknown flag: {other}");
                std::process::exit(2);
            }
        }
    }

    Args {
        duration_ms,
        frequency_hz,
        sample_rate_hz,
        output,
    }
}

fn main() {
    let args = parse_args();

    let sig = SyntheticIq::new(
        args.sample_rate_hz,
        args.frequency_hz,
        Duration::from_millis(args.duration_ms),
    );

    let stream = sig.to_event_stream();
    let json = serde_json::to_string_pretty(&stream).expect("serialize");

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent).expect("create output dir");
    }
    fs::write(&args.output, &json).expect("write output");

    eprintln!(
        "wrote {} events ({} bytes) to {}",
        stream.len(),
        json.len(),
        args.output.display()
    );
}
