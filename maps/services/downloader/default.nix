{ pkgs }:
let
  download-maps = pkgs.writeShellScriptBin "download-maps" ''
    exec ${pkgs.bash}/bin/bash ${./scripts/download-maps.sh} "$@"
  '';

in {
  inherit download-maps;
}