{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix/monthly";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            mold
            rustup
            sccache
          ];

          env = {};

          nativeBuildInputs = [toolchain];

          shellHook = ''
            export RUSTC_WRAPPER=${pkgs.sccache}/bin/sccache
          '';
        };
      }
    );
}
