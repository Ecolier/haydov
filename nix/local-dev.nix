# nix/local-dev.nix
{ pkgs }:
let
  shared = import ./shared.nix { inherit pkgs; };

  packages = {
    start-local-services = pkgs.writeShellScriptBin "start-local-services" ''
      echo "🚀 Starting local services..."
      
      # Create data directories
      mkdir -p ~/.local/share/{postgres,redis,minio,rabbitmq}
      
      # Initialize PostgreSQL if needed
      if [ ! -d ~/.local/share/postgres/data ]; then
        initdb -D ~/.local/share/postgres/data
      fi
      
      # Start services
      pg_ctl -D ~/.local/share/postgres/data -l ~/.local/share/postgres/postgres.log start || true
      redis-server --daemonize yes --dir ~/.local/share/redis || true
      minio server ~/.local/share/minio --console-address ":9001" &
      rabbitmq-server -detached || true
      
      echo "✅ Local services started"
    '';

    stop-local-services = pkgs.writeShellScriptBin "stop-local-services" ''
      echo "🛑 Stopping local services..."
      pg_ctl -D ~/.local/share/postgres/data stop || true
      redis-cli shutdown || true
      pkill -f minio || true
      rabbitmqctl stop || true
      echo "✅ Local services stopped"
    '';
  };

in {
  devShell = pkgs.mkShell {
    buildInputs = with pkgs; [
      rustToolchain nodejs_20 postgresql_15 redis minio rabbitmq-server git
    ];
    shellHook = ''
      echo "🏠 Local development environment ready!"
      echo ""
      echo "Services available:"
      echo "  📊 PostgreSQL: localhost:5432"
      echo "  🗄️  Redis: localhost:6379" 
      echo "  📦 MinIO: localhost:9000 (console: localhost:9001)"
      echo "  🐰 RabbitMQ: localhost:5672 (management: localhost:15672)"
      echo ""
      echo "Commands:"
      echo "  start-local-services  # Start all services"
      echo "  stop-local-services   # Stop all services"
    '';
  };

  inherit packages;

  apps = {
    start-local = {
      type = "app";
      program = "${packages.start-local-services}/bin/start-local-services";
    };
    stop-local = {
      type = "app";
      program = "${packages.stop-local-services}/bin/stop-local-services";
    };
  };
}