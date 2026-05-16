# SPDX-License-Identifier: CC0-1.0
#
# This Nix flake exists for 5-year reproducibility (Kittler thought-fidelity
# axis #1 — materiality). It is OPTIONAL: the Makefile + devcontainer.json
# already give you a working build environment.
#
# If you have Nix installed:   nix develop
# If you don't:                 make dev   (or open in devcontainer)
{
  description = "kittler-aufschreibesystem — signal-as-syntax toolkit";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc cargo clippy rustfmt rust-analyzer
            nodejs_22
            python312
            git gh
            gnuradio
            reuse
          ];
          shellHook = ''
            echo "kittler — Aufschreibesystem Synthesizer dev shell"
            echo "Run 'make dev' to probe the toolchain."
          '';
        };
      });
}
