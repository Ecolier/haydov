# nix/lib/env-utils.nix
{ pkgs }:
{
  # Convert attribute set to shell environment exports
  envToShellExports = envVars: 
    pkgs.lib.concatStringsSep "\n" 
      (pkgs.lib.mapAttrsToList 
        (name: value: "export ${name}=\"${toString value}\"") 
        envVars);

  # Merge multiple environment configs
  mergeEnvConfigs = configs:
    pkgs.lib.foldl' (acc: config: acc // config) {} configs;

  # Filter environment variables by prefix
  filterEnvByPrefix = prefix: envVars:
    pkgs.lib.filterAttrs (name: value: pkgs.lib.hasPrefix prefix name) envVars;

  # Mask secrets in environment display (for debugging)
  maskSecrets = secretKeys: envVars:
    pkgs.lib.mapAttrs (name: value:
      if builtins.elem name secretKeys 
      then "${builtins.substring 0 4 value}..."
      else value
    ) envVars;
}