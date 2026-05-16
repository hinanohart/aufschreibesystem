<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# `make` targets and produced artifacts

What each target does and what (if anything) lands in `target/`.

| Target | What it does | Produced artifacts (paths under repo root) |
|--------|-------------|---------------------------------------------|
| `make dev` | Probes Rust / Node / Python toolchains; reports Nix presence. | (none) |
| `make test` | `cargo test --workspace --all-features`. | (none) |
| `make mvp` | Runs the synthetic-IQ → Strudel-AST example. | `target/mvp.ast.json` |
| `make build` | `cargo build --workspace --release`. | `target/release/ethics-audit` (CLI auditor). **`kittler-archive` and `kittler-stage` are absent in v0.1** — see `bins/README.md`. |
| `make fmt` | `cargo fmt --all`. | (none) |
| `make lint` | `cargo clippy --workspace --all-targets -- -D warnings`. | (none) |
| `make ci` | fmt + lint + test + ethics-audit + SPDX coverage. | (none) |
| `make clean` | `cargo clean` + remove `target/`, `dist/`, `node_modules/`. | (deletions) |

## What `make build` does NOT produce in v0.1

- `target/release/kittler-archive` — the GPL-3.0 archivist CLI. v0.2 work.
- `target/release/kittler-stage` — the AGPL-3.0 web/WASM bundle. v0.2 work.
- Any AI sidecar binary. By design (see `plugins/ai/README.md`), the sidecar
  ships from a *separate repository* and pulls model weights from Alibaba
  on first run.

If you `make build` expecting a runnable archivist binary, you get
`target/release/ethics-audit` and nothing else. That is honest; v0.1 is the
algebra plus the manifest chain plus the audit, not the user-facing wrappers.

## `make ci` exit codes

- `0` — all checks pass.
- `1` — `cargo test`, `cargo clippy`, ethics-audit, or SPDX coverage failed.
  Read the last 40 lines of output and fix the first failure; downstream
  steps may be cascading from it.
