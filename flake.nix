{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      nixpkgs,
      rust-overlay,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];

      imports = [ inputs.treefmt-nix.flakeModule ];

      perSystem =
        { system, lib, ... }:
        let
          overlays = [ rust-overlay.overlays.default ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          rust = pkgs.rust-bin.stable.latest.default;

          rustPlatform = pkgs.makeRustPlatform {
            cargo = rust;
            rustc = rust;
          };

          packages = { includeDevelopmentTools }: with pkgs;
            [
              (rust.override {
                extensions =
                  if includeDevelopmentTools then
                    [
                      "rust-src"
                      "rust-analyzer"
                    ]
                  else
                    [ ];
              })
              just
            ];
        in
        {
          packages.default = rustPlatform.buildRustPackage {
            pname = "intel-8080";
            version = "0.0.1";

            src =
              let
                fs = lib.fileset;
              in
              fs.toSource {
                root = ./.;
                fileset = (
                  fs.unions [
                    (fs.fileFilter (file: file.hasExt "rs" || file.name == "Cargo.toml") ./.)
                    ./Cargo.lock
                  ]
                );
              };

            cargoLock.lockFile = ./Cargo.lock;
          };

          devShells = {
            default = pkgs.mkShellNoCC {
              packages = packages { includeDevelopmentTools = true; };
            };

            ci = pkgs.mkShellNoCC {
              packages = packages { includeDevelopmentTools = false; };
            };
          };

          treefmt = {
            projectRootFile = "Cargo.lock";
            programs = {
              # Rust files
              rustfmt = {
                enable = true;
                package = rust;
              };
              # Cargo.toml
              taplo.enable = true;
              # This flake
              nixfmt.enable = true;
              # Justfile
              just.enable = true;
              # Markdown
              rumdl-check.enable = true;
              # GitHub Actions
              zizmor.enable = true;
              yamlfmt = {
                enable = true;
                settings.formatter = {
                  retain_line_breaks_single = true;
                  max_line_length = 80;
                };
              };
            };
          };
        };
    };
}
