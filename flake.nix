{
  description = "TradeWars 3033 - Y Combinator Game for Cloudflare Workers";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          name = "tradewars-ycombinator";
          src = ./.;
          
          buildInputs = with pkgs; [
            nodejs
            wrangler
          ];
          
          buildPhase = ''
            echo "Building TradeWars for Cloudflare Workers"
          '';
          
          installPhase = ''
            mkdir -p $out
            cp -r game $out/
            cp -r tycoon $out/
            cp tradewars_ycombinator.rs $out/
          '';
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            nodejs
            nodePackages.wrangler
            rustc
            cargo
          ];
          
          shellHook = ''
            echo "🚀 TradeWars 3033 Development Environment"
            echo "Commands:"
            echo "  wrangler dev    - Run locally"
            echo "  wrangler deploy - Deploy to Cloudflare"
          '';
        };
      }
    );
}
