{
  description = "Mega CLI - A tool for downloading file from Mega.nz";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        mega-cli = pkgs.rustPlatform.buildRustPackage {
          pname = "mega-cli";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          buildInputs = with pkgs; [
            openssl
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          meta = with pkgs.lib; {
            description = "Mega CLI";
            license = licenses.mit;
          };
        };
      in
      {
        packages = {
          default = mega-cli;
          inherit mega-cli;
        };
      }
    );
}
