# nix/k8s-dev.nix
{ pkgs }:
let
  shared = import ./shared.nix { inherit pkgs; };

  packages = {
    deploy = pkgs.writeShellScriptBin "deploy" ''
      echo "🔄 Creating or reusing Kubernetes namespace 'haydov'..."
      kubectl create namespace haydov 2>/dev/null || true
      echo "🌐 Deploying services to Kubernetes..."
      tilt up --namespace haydov
    '';
    
    clean = pkgs.writeShellScriptBin "clean" ''
      echo "🧹 Cleaning up Tilt resources..."
      tilt down --namespace haydov || true
      echo "🔄 Deleting Kubernetes namespace 'haydov'..."
      kubectl delete namespace haydov 2>/dev/null || true
      echo "🗑️  Cleaning up kind cluster..."
      kind delete cluster --name haydov || true
    '';

    reset-cluster = pkgs.writeShellScriptBin "reset-cluster" ''
      echo "🔄 Resetting kind cluster..."
      kind delete cluster --name haydov || true
      kind create cluster --name haydov --image kindest/node:v1.33.2 --wait 5m
      echo "✅ Fresh cluster ready"
    '';
  };

in {
  devShell = pkgs.mkShell {
    buildInputs = with pkgs; [
      rustToolchain
      docker docker-compose
      kubectl kind kustomize tilt helm
      git
    ];
    shellHook = ''
      clear
      echo "🌟 Provisioning Kubernetes cluster..."
      kind create cluster --name haydov --image kindest/node:v1.33.2 --wait 5m --quiet || true
      echo "✅ Kubernetes cluster 'haydov' provisioned or already exists."
      echo ""
      echo "🚀 Haydov K8s development environment ready!"
      echo ""
      echo "Commands:"
      echo "  deploy          # Deploy to K8s with Tilt"
      echo "  clean           # Clean up K8s resources"
      echo "  reset-cluster   # Reset kind cluster"
    '';
  };

  inherit packages;

  apps = {
    deploy = {
      type = "app";
      program = "${packages.deploy}/bin/deploy";
    };
    clean = {
      type = "app";
      program = "${packages.clean}/bin/clean";
    };
    reset-cluster = {
      type = "app";
      program = "${packages.reset-cluster}/bin/reset-cluster";
    };
  };
}