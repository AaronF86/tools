{ pkgs }:

pkgs.rustPlatform.buildRustPackage {
  pname = "bannify";
  version = "0.1.0";

  src = builtins.path {
    path = ../.;
    filter = path: type:
      let
        baseName = baseNameOf path;
      in
        baseName != ".git" &&
        baseName != "target" &&
        baseName != "result";
  };

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = with pkgs; [
    pkgs.rustPlatform.cargoSetupHook
  ];

  buildInputs = with pkgs; [
    openssl
    zlib
  ];

  buildPhase = ''
    export CARGO_HOME=$(mktemp -d)
    cargo build --release
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp target/release/bannify $out/bin/bannify
  '';

  postInstall = ''
    mkdir -p $out/share/bannify
    if [ ! -f "$HOME/.config/bannify_config.toml" ]; then
      mkdir -p "$HOME/.config"
      cp $src/src/default_languages.toml "$HOME/.config/bannify_config.toml"
      echo "Installed default bannify config to ~/.config/bannify_config.toml"
    fi
  '';

  meta = with pkgs.lib; {
    description = "being a Comment Banner Tool";
    homepage = "https://github.com/AaronF86/tools/bannify";
    license = licenses.mit;
    maintainers = with maintainers; [ AaronF86 ];
    platforms = platforms.all;
  };
}
