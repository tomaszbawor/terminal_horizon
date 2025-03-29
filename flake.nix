{
  description = "A terminal-based RPG game using Rust and ratatui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [ ];
        };

        nativeBuildInputs = with pkgs; [ rustToolchain pkg-config ];

        buildInputs = with pkgs;
          [
            # Libraries needed for terminal UI
            ncurses
          ] ++ (if pkgs.stdenv.isDarwin then
            with pkgs.darwin.apple_sdk.frameworks; [
              AppKit
              CoreFoundation
              CoreServices
            ]
          else
            [ ]);

      in rec {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          # Set environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          shellHook = ''
            echo "Entering Rust RPG development environment..."
            echo "Rust Version: $(rustc --version)"
            echo "Cargo Version: $(cargo --version)"
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "rust-rpg";
          version = "0.1.0";
          src = ./.;

          inherit nativeBuildInputs buildInputs;

          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = true;
          };

          meta = with pkgs.lib; {
            description = "A terminal-based RPG game using Rust and ratatui";
            homepage = "https://github.com/yourusername/rust-rpg";
            license = licenses.mit;
            maintainers = [ ];
          };
        };

        # For `nix build`
        defaultPackage = packages.default;

        # For `nix run`
        apps.default = flake-utils.lib.mkApp { drv = packages.default; };
      });
}
