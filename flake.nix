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
      defaultBuildInputs =
        pkgs.lib.optionals pkgs.stdenv.isDarwin (
          with pkgs.darwin.apple_sdk.frameworks; [
            CoreFoundation
            Security
            SystemConfiguration
            IOKit
          ]
        )
        ++ [
          pkgs.openssl
          pkgs.pkg-config
        ];
      name = "stockpile";
      src = self;
      rustPackage = naersk-lib.buildPackage {
        pname = name;
        inherit src;
        buildInputs = defaultBuildInputs;
        release = true;
      };
    in {
      packages = {
        default = rustPackage;
        test = naersk-lib.buildPackage {
          inherit name src;
          buildInputs = defaultBuildInputs;
          mode = "test";
        };
        checks = naersk-lib.buildPackage {
          pname = name;
          src = ./.;
          buildInputs = defaultBuildInputs;
          mode = "check";
        };
        dockerImage = pkgs.dockerTools.buildImage {
          inherit name;
          tag = "latest";
          copyToRoot = [rustPackage];
          config = {
            Cmd = ["${rustPackage}/bin/${name}"];
            WorkingDir = "/";
          };
        };
      };
      devShell = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          (import ./devenv.nix)
        ];
      };
    });
}
