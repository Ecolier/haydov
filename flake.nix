{
  description = "Haydov development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        messageBroker = import ./common/services/message-broker { inherit pkgs; };
        mapsStorage = import ./maps/services/storage { inherit pkgs; };

        haydov-tui = pkgs.buildGoModule {
          pname = "haydov-tui";
          version = "0.1.0";
          src = ./.;
          vendorHash = null;
          subPackages = [ "cmd/" ];
          meta = {
            description = "Haydov development TUI";
          };
        };

        start-services = pkgs.writeShellScriptBin "start-services" ''
            echo "➡ Starting all services..."
            (cd common/services/message-broker && start-message-broker)
            (cd maps/services/storage && start-maps-storage)
            echo "✅ All services started"
          '';

        stop-services = pkgs.writeShellScriptBin "stop-services" ''
            echo "➡ Stopping all services..."
            (cd common/services/message-broker && stop-message-broker)
            (cd maps/services/storage && stop-maps-storage)
            echo "🛑 All services stopped"
        '';

        clean-services = pkgs.writeShellScriptBin "clean-services" ''
            echo "➡ Cleaning all services..."
            (cd common/services/message-broker && clean-message-broker)
            (cd maps/services/storage && clean-maps-storage)
            echo "🧹 All services cleaned"
        '';
        
      in {
        devShells = {
          messageBroker = messageBroker.devShell;
          mapsStorage = mapsStorage.devShell;
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              git
              go
            ];
            packages = [
                start-services
                stop-services
                clean-services
                messageBroker.packages.start-message-broker
                messageBroker.packages.stop-message-broker
                messageBroker.packages.clean-message-broker
                mapsStorage.packages.start-maps-storage
                mapsStorage.packages.stop-maps-storage
                mapsStorage.packages.clean-maps-storage
                haydov-tui
              ];
            shellHook = ''
              echo "🌎 Haydov development environment!"
              echo ""
              echo "Commands:"
              echo "  start-services"
              echo "  stop-services"
              echo "  clean-services"
              echo ""
            '';
            inputsFrom = [
              messageBroker.devShell
              mapsStorage.devShell
            ];
          };
        };

        packages = messageBroker.packages // mapsStorage.packages // {
          start-services = start-services;
          stop-services = stop-services;
          haydov = haydov-tui;
        };
        apps = messageBroker.apps // mapsStorage.apps;

        formatter = pkgs.nixpkgs-fmt;
      });
}