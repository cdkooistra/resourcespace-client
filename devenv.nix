{ pkgs, lib, config, inputs, ... }:

{
  cachix.enable = false;
  dotenv.enable = true;

  languages.rust = {
    enable = true;
    channel = "stable";
    mold.enable = true;
  };

  packages = with pkgs; [
    openssl
  ];
}
