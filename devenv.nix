{pkgs, ...}: {
  # This is your devenv configuration
  packages =
    [
      pkgs.gum
    ]
    ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (
      with pkgs.darwin.apple_sdk; [
        frameworks.CoreFoundation
        frameworks.Security
        frameworks.SystemConfiguration
        frameworks.IOKit
      ]
    );

  languages.rust.enable = true;

  scripts.stockpile.exec = ''
    ./result/bin/stockpile
  '';

  pre-commit.hooks = {
    alejandra.enable = true;
    shellcheck.enable = true;
    shfmt.enable = true;
    statix.enable = true;
    taplo.enable = true;
  };
}
