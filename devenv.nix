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
    ./result/bin/pokeapi-ingest-rust
  '';

  pre-commit.hooks = {
    alejandra.enable = true;
    cargo-check.enable = true;
    clippy.enable = true;
    rustfmt.enable = true;
    shellcheck.enable = true;
    shfmt.enable = true;
    statix.enable = true;
    taplo.enable = true;
  };
}
