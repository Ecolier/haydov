{ pkgs, env }:
let
  start-message-broker = pkgs.writeShellScriptBin "start-message-broker" ''
    echo "🐰 Starting message broker..."

    ${env}
    
    mkdir -p .dev/rabbitmq/{logs,mnesia,config}
    
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
      default_user = $COMMON_MESSAGE_BROKER_USERNAME
      default_pass = $COMMON_MESSAGE_BROKER_PASSWORD
    EOF

    exec ${pkgs.bash}/bin/bash ${./scripts/start.sh} "$@"
    
    echo "✅ Message broker started"
  '';

  stop-message-broker = pkgs.writeShellScriptBin "stop-message-broker" ''
    echo "🛑 Stopping message broker..."

    export RABBITMQ_LOG_BASE="$(pwd)/.dev/rabbitmq/logs"
    export RABBITMQ_MNESIA_BASE="$(pwd)/.dev/rabbitmq/mnesia"
    export RABBITMQ_NODENAME=rabbit@localhost
    
    rabbitmqctl stop || true
    
    echo "✅ Message broker stopped"
  '';

  clean-message-broker = pkgs.writeShellScriptBin "clean-message-broker" ''
    echo "🧹 Cleaning message broker..."
    
    ${stop-message-broker}/bin/stop-message-broker
    rm -rf .dev/rabbitmq
    
    echo "✅ Message broker cleaned"
  '';

in {
  inherit start-message-broker stop-message-broker clean-message-broker;
}