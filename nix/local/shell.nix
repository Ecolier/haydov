# nix/local-dev.nix
{ pkgs }:
let
  shared = import ../shared.nix { inherit pkgs; };
  envUtils = import ../utils.nix { inherit pkgs; };
  envConfig = import ./env.nix;
  
  allEnvVars = envUtils.mergeEnvConfigs [
    envConfig.infrastructure
    envConfig.runtime
    envConfig.secrets
  ];

  envString = envUtils.envToShellExports allEnvVars;

  packages = {
    start-services = pkgs.writeShellScriptBin "start-services" ''
      echo "🚀 Starting local services..."

      ${envString}
      
      # Create data directories
      mkdir -p .dev/{minio,rabbitmq/{logs,mnesia,config}}

      echo "📦 Starting MinIO..."
      MINIO_ROOT_USER=admin MINIO_ROOT_PASSWORD=minio_password \
        minio server .dev/minio --console-address ":9001" &
      echo $! > .dev/minio.pid
      
      echo "🐰 Starting RabbitMQ..."
      export RABBITMQ_LOG_BASE="$(pwd)/.dev/rabbitmq/logs"
      export RABBITMQ_MNESIA_BASE="$(pwd)/.dev/rabbitmq/mnesia"
      export RABBITMQ_CONFIG_FILE="$(pwd)/.dev/rabbitmq/config/rabbitmq"
      export RABBITMQ_ENABLED_PLUGINS_FILE="$(pwd)/.dev/rabbitmq/config/enabled_plugins"
      export RABBITMQ_NODE_PORT=5672
      export RABBITMQ_NODENAME=rabbit@localhost
      export ERL_CRASH_DUMP="$(pwd)/.dev/rabbitmq/erl_crash.dump"

      touch .dev/rabbitmq/config/enabled_plugins
      cat > .dev/rabbitmq/config/rabbitmq.conf << EOF
        management.tcp.port = 15672
        management.tcp.ip = 127.0.0.1
        listeners.tcp.default = 5672
        loopback_users = none
      EOF

      rabbitmq-server -detached

      # Wait for RabbitMQ to be ready
      echo "⏳ Waiting for RabbitMQ to start..."
      max_attempts=30
      attempt=0
      
      while [ $attempt -lt $max_attempts ]; do
        if rabbitmqctl status >/dev/null 2>&1; then
          echo "✅ RabbitMQ is ready!"
          break
        fi
        
        echo "   Attempt $((attempt + 1))/$max_attempts - waiting 2s..."
        sleep 2
        attempt=$((attempt + 1))
      done
      
      if [ $attempt -eq $max_attempts ]; then
        echo "❌ RabbitMQ failed to start within 60 seconds"
        exit 1
      fi
      
      # Enable management plugin
      echo "🔧 Enabling RabbitMQ management plugin..."
      rabbitmq-plugins enable rabbitmq_management
      
      echo "✅ Local services started"
    '';

    stop-services = pkgs.writeShellScriptBin "stop-services" ''
      echo "🛑 Stopping local services..."

      export RABBITMQ_LOG_BASE="$(pwd)/.dev/rabbitmq/logs"
      export RABBITMQ_MNESIA_BASE="$(pwd)/.dev/rabbitmq/mnesia"
      export RABBITMQ_NODENAME=rabbit@localhost
      
      echo "📦 Stopping MinIO..."
      if [ -f .dev/minio.pid ]; then
        kill $(cat .dev/minio.pid) 2>/dev/null || true
        rm .dev/minio.pid
      fi
      pkill -f "minio server" || true
      
      echo "🐰 Stopping RabbitMQ..."
      rabbitmqctl stop || true
      
      echo "✅ Local services stopped"
    '';

    clean-services = pkgs.writeShellScriptBin "clean-services" ''
      echo "🧹 Cleaning local services..."
      
      ${packages.stop-services}/bin/stop-services
      rm -rf .dev
      
      echo "✅ Local services cleaned"
    '';
  };

in {
  devShell = pkgs.mkShell {
    buildInputs = with pkgs; [
      shared.rustToolchain  # ✅ Use shared.rustToolchain, not rustToolchain
      nodejs_20
      minio
      rabbitmq-server
      erlang
    ] ++ shared.commonInputs;
    
    shellHook = ''
      echo "🏠 Local development environment ready!"
      echo ""
      echo "Services available:"
      echo "  📦 MinIO: localhost:9000 (console: localhost:9001)"
      echo "  🐰 RabbitMQ: localhost:5672 (management: localhost:15672)"
      echo ""
      echo "Commands:"
      echo "  start-services  # Start all services"
      echo "  stop-services   # Stop all services"
      echo "  clean-services  # Clean all data"
    '';
  };

  inherit packages;

  apps = {
    start-services = {
      type = "app";
      program = "${packages.start-services}/bin/start-services";
    };
    stop-services = {
      type = "app";
      program = "${packages.stop-services}/bin/stop-services";
    };
    clean-services = {
      type = "app";
      program = "${packages.clean-services}/bin/clean-services";
    };
  };
}