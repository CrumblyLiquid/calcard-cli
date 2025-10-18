{
  description = "calcard-cli";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, ...}: 
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem = {self', pkgs, system, ...}:
        let
          rustVersion = "1.90.0";
          pkgs = import nixpkgs {
            inherit system;
            overlays = [inputs.cargo2nix.overlays.default (import inputs.rust-overlay)];
          };
          rustPkgs = pkgs.rustBuilder.makePackageSet {
            inherit rustVersion;
            packageFun = import ./Cargo.nix;
          };
        in {
          packages = rec {
            iced_nix = (rustPkgs.workspace.iced_nix {}).bin;
            default = iced_nix;
          };
          devShells.default = pkgs.mkShell rec {
            buildInputs = with pkgs; [
              pkg-config
              wayland
              libxkbcommon
              libGL

              rust-analyzer-unwrapped
              (rust-bin.stable.${rustVersion}.default.override { extensions = [ "rust-src" ]; })
            ];
            LD_LIBRARY_PATH = "${nixpkgs.lib.makeLibraryPath buildInputs}";
          };
        };
    };
}
