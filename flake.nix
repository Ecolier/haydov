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
            # Your existing tools
            nodejs
            nodePackages.pnpm
            rustToolchain
            kubectl
            skaffold
            helm
            kustomize
            docker
            
            # Local service binaries
            rabbitmq-server
            minio
            minio-client
            
            # Process management
            overmind  # or use foreman/hivemind
            
            # Development utilities
            jq
            curl
            wget
            pkg-config
            openssl
          ];

          shellHook = ''
            echo "🚀 Haydov development environment ready!"
            echo ""
            echo "Available services:"
            echo "  📨 RabbitMQ (local): http://localhost:15672"
            echo "  🗄️  MinIO (local): http://localhost:9000"
            echo "  📊 MinIO Console: http://localhost:9001"
            echo ""
            
            # Create local data directories
            mkdir -p .dev/{rabbitmq,minio}
            
            # Set environment variables to match your Helm deployments
            export MESSAGE_BROKER_HOST="localhost"
            export MESSAGE_BROKER_PORT="5672"
            export MESSAGE_BROKER_USERNAME="admin"
            export MESSAGE_BROKER_PASSWORD="haydov_default_password"
            export MESSAGE_BROKER_SCHEMA="amqp"
            
            export GEOGRAPHY_STORAGE_BASE_URL="http://localhost:9000"
            export GEOGRAPHY_STORAGE_USERNAME="admin"
            export GEOGRAPHY_STORAGE_PASSWORD="haydov_default_password"
            
            export NODE_ENV=development
            export RUST_LOG=debug
            
            echo "🔧 Quick commands:"
            echo "  nix run .#start-services     # Start RabbitMQ + MinIO"
            echo "  nix run .#stop-services      # Stop all services"
            echo "  nix run .#setup-dev          # Set up development"
            echo "  overmind start               # Start all with process manager"
          '';
        };

        packages = {
          # Start local services script
          start-services = pkgs.writeShellScriptBin "start-services" ''
            echo "🚀 Starting local development services..."
            
            # Create data directories
            mkdir -p .dev/{rabbitmq,minio}
            
            # Start RabbitMQ
            echo "📨 Starting RabbitMQ..."
            RABBITMQ_NODE_PORT=5672 \
            RABBITMQ_MANAGEMENT_PORT=15672 \
            RABBITMQ_MNESIA_BASE="$(pwd)/.dev/rabbitmq" \
            RABBITMQ_LOG_BASE="$(pwd)/.dev/rabbitmq" \
            rabbitmq-server -detached
            
            # Wait for RabbitMQ to start
            sleep 5
            
            # Configure RabbitMQ user (matching your Helm values)
            rabbitmqctl add_user admin haydov_default_password 2>/dev/null || true
            rabbitmqctl set_user_tags admin administrator
            rabbitmqctl set_permissions -p / admin ".*" ".*" ".*"
            
            # Start MinIO
            echo "🗄️  Starting MinIO..."
            MINIO_ROOT_USER=admin \
            MINIO_ROOT_PASSWORD=haydov_default_password \
            minio server .dev/minio \
              --address :9000 \
              --console-address :9001 &
            
            echo "✅ Services started!"
            echo "  RabbitMQ Management: http://localhost:15672 (admin/haydov_default_password)"
            echo "  MinIO Console: http://localhost:9001 (admin/haydov_default_password)"
            echo "  MinIO API: http://localhost:9000"
          '';

          # Stop services script
          stop-services = pkgs.writeShellScriptBin "stop-services" ''
            echo "🛑 Stopping local services..."
            
            # Stop RabbitMQ
            rabbitmqctl stop 2>/dev/null || true
            
            # Stop MinIO
            pkill -f "minio server" || true
            
            echo "✅ Services stopped!"
          '';

          # Setup development environment
          setup-dev = pkgs.writeShellScriptBin "setup-dev" ''
            echo "🏗️  Setting up Haydov development environment..."
            
            # Install workspace dependencies
            pnpm install --frozen-lockfile
            
            # Create local development secrets if they don't exist
            if [ ! -f "services/geography/importer/.env.secret" ]; then
              echo "📝 Creating local development secrets..."
              cp services/geography/importer/.env.secret.defaults services/geography/importer/.env.secret
            fi
            
            # Start services
            nix run .#start-services
            
            # Wait for services to be ready
            echo "⏳ Waiting for services to be ready..."
            sleep 10
            
            # Configure MinIO for development (create buckets, etc.)
            echo "🪣 Setting up MinIO buckets..."
            mc alias set local http://localhost:9000 admin haydov_default_password
            mc mb local/geography-data 2>/dev/null || true
            
            echo "✅ Development environment ready!"
            echo ""
            echo "Services running:"
            echo "  📨 RabbitMQ: http://localhost:15672"
            echo "  🗄️  MinIO: http://localhost:9000"
            echo ""
            echo "Next steps:"
            echo "  1. Run your importer: nix run .#run-local importer"
            echo "  2. Run your dispatcher: nix run .#run-local dispatcher"
            echo "  3. Or use containers: skaffold dev"
          '';

          # Run services locally
          run-local = pkgs.writeShellScriptBin "run-local" ''
            case "$1" in
              "importer")
                echo "🚀 Starting geography-importer locally..."
                cd services/geography/importer
                pnpm nx dev geography-importer
                ;;
              "dispatcher")
                echo "🚀 Starting geography-dispatcher locally..."
                cd services/geography/dispatcher  
                cargo run
                ;;
              *)
                echo "Usage: run-local [importer|dispatcher]"
                ;;
            esac
          '';
        };

        apps = {
          start-services = {
            type = "app";
            program = "${self.packages.${system}.start-services}/bin/start-services";
          };
          stop-services = {
            type = "app";
            program = "${self.packages.${system}.stop-services}/bin/stop-services";
          };
          setup-dev = {
            type = "app";
            program = "${self.packages.${system}.setup-dev}/bin/setup-dev";
          };
          run-local = {
            type = "app";
            program = "${self.packages.${system}.run-local}/bin/run-local";
          };
        };
      });
}