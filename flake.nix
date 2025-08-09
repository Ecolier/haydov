# flake.nix
{
  description = "Haydov development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        
        localDev = import ./nix/local-dev.nix { inherit pkgs; };
        k8sDev = import ./nix/k8s-dev.nix { inherit pkgs; };
        
      in {
        devShells = {
          default = k8sDev.devShell;
          local = localDev.devShell;
          k8s = k8sDev.devShell;
        };
        
        packages = localDev.packages // k8sDev.packages;
        apps = localDev.apps // k8sDev.apps;

        # Add formatter for `nix fmt`
        formatter = pkgs.nixpkgs-fmt;
      });
}