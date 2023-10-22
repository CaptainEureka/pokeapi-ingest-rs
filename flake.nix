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
      defaultPackage = naersk-lib.buildPackage {
        pname = "stockpile";
        src = ./.;
        buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin (
          with pkgs.darwin.apple_sdk.frameworks; [
            CoreFoundation
            Security
            SystemConfiguration
            IOKit
          ]
        );
      };
      devShell = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          (import ./devenv.nix)
        ];
      };
    });
}
