{
  description = "chir.rs website";

  inputs = {
    cargo2nix = {
      url = "github:cargo2nix/cargo2nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
      inputs.flake-utils.follows = "flake-utils";
    };
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    nixpkgs.url = "github:NixOS/nixpkgs";
    nixtoo.url = "github:DarkKirb/nixtoo";
    nixtoo.flake = false;
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.devshell.flakeModule
      ];
      systems = ["x86_64-linux" "aarch64-linux"];
      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            (import "${inputs.nixtoo}/overlay.nix")
            inputs.rust-overlay.overlays.default
            inputs.cargo2nix.overlays.default
            inputs.self.overlays.default
          ];
        };
        devshells.default.devshell.packages = with pkgs; [
          (rust-bin.nightly.latest.default.override {
            extensions = ["rust-src"];
          })
          inputs.cargo2nix.packages.${system}.cargo2nix
          clang
          cargo-deny
          cargo-crev
          alejandra
          rnix-lsp
          cargo-expand
          llvm
        ];
        formatter = pkgs.alejandra;
        packages = {
          inherit (pkgs) chir-rs-protos;
        };
      };
      flake = {
        hydraJobs = {
          inherit (inputs.self) devShells packages formatter;
        };
        overlays.default = self: super: let
          rustPkgs = self.rustBuilder.makePackageSet {
            packageFun = import ./Cargo.nix;
            rustChannel = "1.75.0";
            packageOverrides = pkgs: pkgs.rustBuilder.overrides.all;
          };
        in {
          chir-rs-protos = rustPkgs.workspace.chir-rs-protos {};
        };
      };
    };
}
