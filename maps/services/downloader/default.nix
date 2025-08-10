{ pkgs, env }:
let
  download-maps = pkgs.writeShellScriptBin "download-maps" ''
    ${env}
    exec ${pkgs.bash}/bin/bash ${./scripts/download-maps.sh} "$@"
  '';

in {
  inherit download-maps;
}