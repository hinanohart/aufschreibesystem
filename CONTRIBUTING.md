# Contributing to aufschreibesystem

## Prerequisites

- Rust stable (latest) via [rustup](https://rustup.rs/)

## Clone and build

```bash
git clone https://github.com/hinanohart/aufschreibesystem.git
cd aufschreibesystem
cargo build
```

## Run tests

```bash
cargo test
```

## Lint

```bash
cargo clippy -- -D warnings
cargo fmt --check
```

## Branch convention

Use `feat/<topic>`, `fix/<topic>`, or `chore/<topic>` branches off `main`.

## Pull request convention

- One logical change per PR.
- Include tests for new behaviour.
- Ensure `cargo test` and `cargo clippy` pass locally before opening the PR.
