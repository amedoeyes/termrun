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
            pname = "termrun";
            version = "1.1.0";

            src = ./.;

            cargoLock = {
              lockFile = "${src}/Cargo.lock";
            };

            meta = {
              mainProgram = "termrun";
              description = "Command-line tool to run command-line tools on an external terminal emulator.";
              homepage = "https://git.eyoun.net/ahmed/termrun";
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
