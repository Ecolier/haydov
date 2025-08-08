{
  description = "Haydov development environment with local services";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        nodejs = pkgs.nodejs_20;
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            git
            tilt
            docker
            kubectl
            kind
            kustomize
          ];

          shellHook = ''
            clear
            echo "🌟 Provisioning Kubernetes cluster..."
            kind create cluster --name haydov --image kindest/node:v1.33.2 --wait 5m --quiet || true
            echo "✅ Kubernetes cluster 'haydov' provisioned or already exists."

            echo "🚀 Haydov development environment ready!"
            echo ""
            echo "Available services:"
            echo "  🌍 Maps Importer: http://localhost:5000"
            echo ""
            
            echo "🔧 Quick commands:"
            echo "  nix run .#deploy     # Deploy services"
            echo "  nix run .#clean      # Clean up resources"
          '';
        };

        packages = {
          deploy = pkgs.writeShellScriptBin "deploy" ''
            echo "🔄 Creating or reusing Kubernetes namespace 'haydov'..."
            kubectl create namespace haydov 2>/dev/null || true
            echo "🌐 Deploying services to Kubernetes..."
            tilt up --namespace haydov || true
          '';
          clean = pkgs.writeShellScriptBin "clean" ''
            echo "🧹 Cleaning up Tilt resources..."
            tilt down --namespace haydov || true
            echo "🔄 Deleting Kubernetes namespace 'haydov'..."
            kubectl delete namespace haydov 2>/dev/null || true
          '';
        };

        apps = {
          deploy = {
            type = "app";
            program = "${self.packages.${system}.deploy}/bin/deploy";
          };
          clean = {
            type = "app";
            program = "${self.packages.${system}.clean}/bin/clean";
          };
        };
      });
}