# SPDX-License-Identifier: CC0-1.0
# Authoritative reproducible-build entry point.
# Nix flake is also provided (flake.nix) but is OPTIONAL.

.PHONY: dev test mvp build fmt lint clean ci

CARGO ?= cargo
NODE ?= node
PYTHON ?= python3

dev:
	@echo "[dev] toolchain probe"
	@$(CARGO) --version
	@$(NODE) --version
	@$(PYTHON) --version
	@command -v git
	@echo "[dev] (optional) nix --version:"; nix --version 2>/dev/null || echo "  not installed (devcontainer.json provides Docker fallback)"

test:
	$(CARGO) test --workspace --all-features

mvp:
	$(CARGO) run -p signal-algebra --example iq_to_pattern -- \
		--seed 0xK1TT1ER --duration-ms 1000 --output target/mvp.ast.json
	@echo "[mvp] AST written to target/mvp.ast.json"

build:
	$(CARGO) build --workspace --release

fmt:
	$(CARGO) fmt --all

lint:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

ci: fmt lint test
	@scripts/ethics-audit.sh fixtures/
	@scripts/license-matrix-check.sh

clean:
	$(CARGO) clean
	rm -rf target dist node_modules
