# flake.nix
{
  description = "Haydov development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        
        local = import ./nix/local/shell.nix { inherit pkgs; };
        k8s = import ./nix/k8s/shell.nix { inherit pkgs; };
        
      in {
        devShells = {
          default = k8s.devShell;
          local = local.devShell;
          k8s = k8s.devShell;
        };
        
        packages = local.packages // k8s.packages;
        apps = local.apps // k8s.apps;

        # Add formatter for `nix fmt`
        formatter = pkgs.nixpkgs-fmt;
      });
}