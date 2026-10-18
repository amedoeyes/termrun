{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.rustPlatform.buildRustPackage rec {
            pname = "fz";
            version = "0.1.0";

            src = ./.;

            cargoLock = {
              lockFile = "${src}/Cargo.lock";
            };

            nativeBuildInputs = with pkgs; [
              nasm
              mold
            ];

            meta = {
              mainProgram = "fz";
              description = "Fuzzy finder";
              homepage = "https://git.eyoun.net/ahmed/fz";
              license = pkgs.lib.licenses.agpl3Only;
            };
          };
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              clippy
              rust-analyzer
              rustc
              rustfmt
            ];
          };
        }
      );
    };
}
