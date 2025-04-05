#------------------------------------------#
#                                          #
# nix/develop.nix - Dev Shell              #
#                                          #
#------------------------------------------#

{ pkgs, Bannify }:

let
  bannify = Bannify.packages.${pkgs.system}.default;
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    clippy
    rustfmt
    cargo-watch
    rust-analyzer
    git
    bannify
  ];

  shellHook = ''
    echo "🚀 Entering Rust development environment"
    echo "👉 Use 'cargo watch' to automatically run commands on file changes."
    echo "👉 Example: 'cargo watch -x \"clippy\" -x \"fmt\"'"

    export CARGO_INCREMENTAL=1
    export RUST_BACKTRACE=1
  '';
}
