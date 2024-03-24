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
      } @ args: let
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          packageFun = import ./Cargo.nix;
          rustChannel = "1.77.0";
          packageOverrides = pkgs: pkgs.rustBuilder.overrides.all;
        };
      in {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.rust-overlay.overlays.default
            inputs.cargo2nix.overlays.default
          ];
        };
        devshells.default.devshell.packages = with pkgs; [
          (rust-bin.stable.latest.default.override {
            extensions = ["rust-src"];
          })
          inputs.cargo2nix.packages.${system}.cargo2nix
          cargo-deny
          cargo-crev
          nodejs
          yarn
          yarn2nix
          gnumake
          gcc
        ];
        formatter = pkgs.alejandra;
        packages = rec {
          chir-rs-fe = with pkgs;
            mkYarnPackage {
              name = "chir-rs-fe";
              src = ./.;
              packageJSON = ./web/package.json;
              yarnLock = ./web/yarn.lock;
              yarnNix = ./web/yarn.nix;
              nativeBuildInputs = [fixup_yarn_lock jq];
              configurePhase = ''
                cd web
                cp -r $node_modules node_modules
              '';
              buildPhase = "yarn build";
              installPhase = "cp -rv dist $out; cp entrypoints $out";
              distPhase = "true";
            };
          chir-rs = (rustPkgs.workspace.chir-rs {}).overrideAttrs (super: {
            configurePhase =
              super.configurePhase
              + ''
                cp -rv ${chir-rs-fe} web/dist
                cp web/dist/entrypoints web
              '';
          });
        };
      };
      flake = {
        hydraJobs = {
          inherit (inputs.self) devShells packages formatter;
        };
      };
    };
}
