{ pkgs, env }:
let
  start-maps-storage = pkgs.writeShellScriptBin "start-maps-storage" ''
    echo "📦 Starting maps storage..."

    ${env}
    
    mkdir -p .dev/minio
    exec ${pkgs.bash}/bin/bash ${./scripts/start.sh} "$@"

    echo "✅ Maps storage started"
  '';

  configure-maps-storage = pkgs.writeShellScriptBin "configure-maps-storage" ''
    echo "🔧 Configuring maps storage..."

    ${env}
    
    mkdir -p .dev/minio
    exec ${pkgs.bash}/bin/bash ${./scripts/configure.sh} "$@"

    echo "✅ Maps storage configured"
  '';

  stop-maps-storage = pkgs.writeShellScriptBin "stop-maps-storage" ''
    echo "🛑 Stopping maps storage..."

    if [ -f .dev/minio.pid ]; then
      kill $(cat .dev/minio.pid) 2>/dev/null || true
      rm .dev/minio.pid
    fi
    pkill -f "minio server" || true
    
    echo "✅ Maps storage stopped"
  '';

  clean-maps-storage = pkgs.writeShellScriptBin "clean-maps-storage" ''
    echo "🧹 Cleaning maps storage..."
    
    ${stop-maps-storage}/bin/stop-maps-storage
    rm -rf .dev/minio
    
    echo "✅ Maps storage cleaned"
  '';

in {
  inherit start-maps-storage stop-maps-storage clean-maps-storage;
}