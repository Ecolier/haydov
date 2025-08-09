{ pkgs }:
{
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" "rust-analyzer" ];
    targets = [ "wasm32-wasip2" ];
  };

  commonInputs = with pkgs; [
    git
    curl
    jq
    yq
  ];
}