{
  description = "rust-template";

  inputs = {
    cargo2nix = {
      url = "github:DarkKirb/cargo2nix/master";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-overlay.follows = "rust-overlay";
    };

    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    flake-utils.url = "github:numtide/flake-utils";

    nixpkgs.url = "github:NixOS/nixpkgs";

    riscv-overlay = {
      url = "github:DarkKirb/riscv-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      cargo2nix,
      riscv-overlay,
      ...
    }@inputs:
    flake-utils.lib.eachSystem
      [
        "x86_64-linux"
        "aarch64-linux"
        "riscv64-linux"
      ]
      (
        system:
        let
          overlays =
            [
              cargo2nix.overlays.default
              (import rust-overlay)
            ]
            ++ (
              if system == "riscv64-linux" then
                [
                  riscv-overlay.overlays.default
                ]
              else
                [ ]
            );
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rustPkgs = pkgs.rustBuilder.makePackageSet {
            packageFun = import ./Cargo.nix;
            rustChannel = "stable";
            rustVersion = "latest";
            packageOverrides = pkgs: pkgs.rustBuilder.overrides.all;
          };
        in
        rec {
          devShells.default =
            with pkgs;
            mkShell {
              buildInputs = [
                cargo2nix.packages.${system}.cargo2nix
                rustfilt
                gdb
                sqlx-cli
                cargo-expand
                sqlite
              ];
            };
          packages = pkgs.lib.mapAttrs (_: v: (v { }).overrideAttrs { dontStrip = true; }) rustPkgs.workspace;
          nixosModules.default = import ./nixos {
            inherit inputs system;
          };
          checks = pkgs.lib.mapAttrs (_: v: pkgs.rustBuilder.runTests v { }) rustPkgs.workspace;
          hydraJobs = {
            inherit packages checks;
          };
          formatter = pkgs.nixfmt-rfc-style;
        }
      );
}
# Trick renovate into working: "github:NixOS/nixpkgs/nixpkgs-unstable"
