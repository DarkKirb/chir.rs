{
  description = "rust-template";

  inputs = {
    cargo2nix = {
      url = "github:DarkKirb/cargo2nix/respect-config-toml";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-overlay.follows = "rust-overlay";
    };

    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    flake-utils.url = "github:numtide/flake-utils";

    nixpkgs.url = "github:NixOS/nixpkgs";

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
      ...
    }@inputs:
    flake-utils.lib.eachSystem
      [
        "x86_64-linux"
        "aarch64-linux"
      ]
      (
        system:
        let
          overlays = [
            cargo2nix.overlays.default
            (import rust-overlay)
          ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          pkgs-wasm32 = import nixpkgs {
            inherit system overlays;
            crossSystem = {
              system = "wasm32-wasi";
              useLLVM = true;
            };
          };
          rustPkgs = pkgs.rustBuilder.makePackageSet {
            packageFun = import ./Cargo.nix;
            rustChannel = "nightly";
            rustVersion = "latest";
            packageOverrides = pkgs: pkgs.rustBuilder.overrides.all;
            cargoConfig.unstable.bindeps = true;
          };
          rustPkgs-wasm32 = pkgs-wasm32.rustBuilder.makePackageSet {
            packageFun =
              attrs:
              import ./Cargo.nix (
                attrs
                // {
                  hostPlatform = attrs.hostPlatform // {
                    parsed = attrs.hostPlatform.parsed // {
                      kernel.name = "unknown";
                    };
                  };
                }
              );
            rustChannel = "nightly";
            rustVersion = "latest";
            packageOverrides = pkgs: pkgs.rustBuilder.overrides.all;
            target = "wasm32-unknown-unknown";
            cargoConfig.unstable.bindeps = true;
          };
        in
        rec {
          devShells.default =
            with pkgs;
            mkShell {
              buildInputs = [
                (rust-bin.selectLatestNightlyWith (
                  toolchain:
                  toolchain.default.override {
                    extensions = [ "rust-src" ];
                    targets = [ "wasm32-unknown-unknown" ];
                  }
                ))
                cargo2nix.packages.${system}.cargo2nix
                gdb
                sqlx-cli
                cargo-expand
                sqlite
                treefmt
                nixfmt-rfc-style
                wabt
                trunk
                (rustPkgs."registry+https://github.com/rust-lang/crates.io-index".wasm-bindgen-cli."0.2.100" { })
                binaryen
              ];
            };
          packages =
            (pkgs.lib.mapAttrs (_: v: (v { }).overrideAttrs { dontStrip = true; }) rustPkgs.workspace)
            // {
              chir-rs-fe =
                let
                  chir-rs-fe = rustPkgs-wasm32.workspace.chir-rs-fe { };
                  wasm-bindgen-cli =
                    rustPkgs."registry+https://github.com/rust-lang/crates.io-index".wasm-bindgen-cli."0.2.100"
                      { };
                in
                pkgs.stdenvNoCC.mkDerivation {
                  inherit (chir-rs-fe) name version;
                  src = chir-rs-fe.out;
                  dontUnpack = true;
                  dontBuild = true;
                  nativeBuildInputs = [
                    wasm-bindgen-cli
                    pkgs.binaryen
                  ];
                  installPhase = ''
                    mkdir $out
                    wasm-opt $src/lib/chir_rs_fe.wasm -o chir_rs_fe.wasm
                    wasm-bindgen chir_rs_fe.wasm --out-dir $out --target web
                    cp ${./chir-rs-fe/index.html} $out/index.html
                  '';
                };
            };
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
