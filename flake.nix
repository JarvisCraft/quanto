{
  description = "An emmbeddable Quantum language backed by OpenQASM";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion =
          (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
      in { devShell = pkgs.mkShell { buildInputs = [ rustVersion ]; }; });
}
