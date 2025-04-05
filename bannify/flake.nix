#-----------------------------------------------#
#                                               #
#  flake.nix - entry point for Nix environment  #
#                                               #
#-----------------------------------------------#


{
  description = "Rust Project with Nix Flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
        Bannify = {
          url = "github:aaronf86/tools?dir=bannify";
          flake = true;
        };
  };

outputs = { self, nixpkgs, flake-utils, Bannify, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        packages = {
          default = import ./nix/build.nix { inherit pkgs; };
        };

        devShells.default = import ./nix/develp.nix { inherit pkgs Bannify; };

        nixosModules.bannify = import ./nix/options.nix { inherit pkgs; };
      }
    );
}