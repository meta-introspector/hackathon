{
  description = "Combinator Universe - Dioxus WASM Game";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust
            wasm-bindgen-cli
            binaryen
            nodejs
          ];
          
          shellHook = ''
            echo "🌌 Combinator Universe Dev Environment"
            echo "Commands:"
            echo "  cargo build --target wasm32-unknown-unknown --release"
            echo "  wasm-bindgen target/wasm32-unknown-unknown/release/combinator_universe.wasm --out-dir dist --target web"
            echo "  python -m http.server 8000"
          '';
        };
      }
    );
}
