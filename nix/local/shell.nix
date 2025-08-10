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

  env = envUtils.envToShellExports allEnvVars;

  services = import ./services.nix { inherit pkgs env; };
  jobs = import ../../maps/services/downloader { inherit pkgs env; };

in {
  devShell = pkgs.mkShell {
    buildInputs = with pkgs; [
      shared.rustToolchain  # ✅ Use shared.rustToolchain, not rustToolchain
      nodejs_20
      minio
      rabbitmq-server
      erlang
      bacon
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
      echo "  clean-services  # Clean all services data"
      echo "  download-maps   # Download map files"
    '';
  };

  packages = services // jobs;

  apps = {
    start-services = {
      type = "app";
      program = "${services.start-services}/bin/start-services";
    };
    stop-services = {
      type = "app";
      program = "${services.stop-services}/bin/stop-services";
    };
    clean-services = {
      type = "app";
      program = "${services.clean-services}/bin/clean-services";
    };
    download-maps = {
      type = "app";
      program = "${jobs.download-maps}/bin/download-maps";
    };
  };
}