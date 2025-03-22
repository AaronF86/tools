{
  description = "Rust Project with Nix Flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        packages = {
          default = import ./nix/build.nix { inherit pkgs; };
          bannify = import ./nix/build.nix { inherit pkgs; };
        };

        devShells.default = import ./nix/develp.nix { inherit pkgs; };

        nixosModules.bannify = import ./nix/options.nix { inherit pkgs; };
      }
    );
}
