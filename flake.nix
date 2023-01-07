{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, rust-overlay, nixpkgs, naersk }:
    let
      program_name = "lox";
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit overlays system;
      };

      rust-bin = pkgs.rust-bin.selectLatestNightlyWith
        (toolchain: toolchain.default.override {
          extensions = [ "rust-src" ];
        });
      naersk-lib = naersk.lib.${system}.override {
        cargo = rust-bin;
        rustc = rust-bin;
      };

      rust-dev-deps = with pkgs; [
        rust-analyzer
        rustfmt
        lldb
        cargo-geiger
        renderdoc
      ];
      build-deps = with pkgs; [
        pkgconfig
        mold
        clang
        makeWrapper
      ];
      all_deps = build-deps ++ rust-dev-deps ++ [ rust-bin ];
    in
    {
      devShell.${system} =
        pkgs.mkShell {
          buildInputs = all_deps;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath all_deps;
          PROGRAM_NAME = program_name;
          shellHook = ''
            export CARGO_MANIFEST_DIR=$(pwd)
          '';
        };
    };
  }
