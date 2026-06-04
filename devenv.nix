{ pkgs, ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";
    mold.enable = true;
  };

  # used for rendering docs
  languages.python.enable = true;

  packages = with pkgs; [
    openssl
    just
  ];
}
