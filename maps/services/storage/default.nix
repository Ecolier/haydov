{ pkgs }:
let
  start-maps-storage = pkgs.writeShellScriptBin "start-maps-storage" ''
    echo "📦 Starting maps storage..."

    mkdir -p .dev/

    export MINIO_ROOT_USER=$STORAGE_USERNAME
    export MINIO_ROOT_PASSWORD=$STORAGE_PASSWORD
    nohup ${pkgs.bash}/bin/bash ${./scripts/start.sh} "$@" > .dev/minio.log 2>&1 &
    echo $! > .dev/minio.pid

    echo "✅ Maps storage started"
  '';

  configure-maps-storage = pkgs.writeShellScriptBin "configure-maps-storage" ''
    echo "🔧 Configuring maps storage..."
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
  devShell = pkgs.mkShell {
    buildInputs = with pkgs; [
      minio
      minio-client
    ];
    
    shellHook = ''
      echo "📦 Maps storage: localhost:9000"
      echo ""
      echo "Commands:"
      echo "  start-maps-storage"
      echo "  configure-maps-storage"
      echo "  stop-maps-storage"
      echo "  clean-maps-storage"
      echo ""
    '';
  };

  packages = {
    inherit start-maps-storage configure-maps-storage stop-maps-storage clean-maps-storage;
  };
  
  apps = {
    start-maps-storage = {
      type = "app";
      program = "${start-maps-storage}/bin/start-maps-storage";
    };
    configure-maps-storage = {
      type = "app";
      program = "${configure-maps-storage}/bin/configure-maps-storage";
    };
    stop-maps-storage = {
      type = "app";
      program = "${stop-maps-storage}/bin/stop-maps-storage";
    };
    clean-maps-storage = {
      type = "app";
      program = "${clean-maps-storage}/bin/clean-maps-storage";
    };
  };
}