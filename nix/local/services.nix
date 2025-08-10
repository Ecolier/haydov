{ pkgs, env }:
let
  messageBroker = import ../../common/services/message-broker { inherit pkgs env; };
  mapsStorage = import ../../maps/services/storage { inherit pkgs env; };

  start-services = pkgs.writeShellScriptBin "start-services" ''
    echo "🚀 Starting local services..."

    ${mapsStorage.start-maps-storage}/bin/start-maps-storage
    ${messageBroker.start-message-broker}/bin/start-message-broker
    
    echo "✅ Local services started"
  '';

  stop-services = pkgs.writeShellScriptBin "stop-services" ''
    echo "🛑 Stopping local services..."
    
    ${messageBroker.stop-message-broker}/bin/stop-message-broker
    ${mapsStorage.stop-maps-storage}/bin/stop-maps-storage

    echo "✅ Local services stopped"
  '';

  clean-services = pkgs.writeShellScriptBin "clean-services" ''
    echo "🧹 Cleaning local services..."
    
    ${messageBroker.clean-message-broker}/bin/clean-message-broker
    ${mapsStorage.clean-maps-storage}/bin/clean-maps-storage
    rm -rf .dev
    
    echo "✅ Local services cleaned"
  '';

in {
  inherit start-services stop-services clean-services;
  inherit (messageBroker) start-message-broker stop-message-broker clean-message-broker;
  inherit (mapsStorage) start-maps-storage stop-maps-storage clean-maps-storage;
}