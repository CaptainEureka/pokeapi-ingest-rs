{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    devenv.url = "github:cachix/devenv";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    devenv,
  } @ inputs:
    inputs.utils.lib.eachDefaultSystem (system: let
      pkgs = import inputs.nixpkgs {inherit system;};
      naersk-lib = pkgs.callPackage inputs.naersk {};
    in {
      defaultPackage = naersk-lib.buildPackage ./.;
      devShell = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          ({pkgs, ...}: {
            # This is your devenv configuration
            packages = [pkgs.hello];

            enterShell = ''
              hello
            '';

            languages.rust.enable = true;

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

            processes.run.exec = "hello";
          })
        ];
      };
    });
}
