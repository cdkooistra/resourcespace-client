{ pkgs, lib, config, inputs, ... }:

{
  dotenv.enable = true;

  languages.rust = {
    enable = true;
    channel = "stable";
    # mold.enable = true;
  };

  packages = with pkgs; [
    openssl
    just
  ];
}
