{
  description = "mdma flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # fenix.url = "github:nix-community/fenix";
    # fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          # rustc
          # cargo
          # rustfmt
          # clippy
          # pkg-config
          gcc
          cargo-machete
          cargo-fuzz
        ];

        shellHook = ''
          # Need for running fuzz tests
          export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib:$LD_LIBRARY_PATH
        '';

        RUST_BACKTRACE = "1";
      };
    });
}
