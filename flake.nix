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
            sqlx-cli
          ];

          env = {
            PORT = "8080";
            DATABASE_URL = "postgresql://postgres:password@localhost:5432/default";
            READER_DATABASE_URL = "postgresql://postgres:password@localhost:5432/default";
          };

          nativeBuildInputs = [toolchain];

          shellHook = ''
            export RUSTC_WRAPPER=${pkgs.sccache}/bin/sccache
          '';
        };
      }
    );
}
